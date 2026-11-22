#include "chatcontroller.h"

#include <QDir>
#include <QJsonDocument>
#include <QJsonObject>
#include <QLocalSocket>
#include <QStandardPaths>
#include <QUuid>
#include <QVariantMap>

ChatController::ChatController(QObject *parent) : QObject(parent) {}

QVariantList ChatController::messages() const { return m_messages; }
bool ChatController::busy() const { return m_busy; }
QString ChatController::statusText() const { return m_statusText; }

void ChatController::setBusy(bool value)
{
    if (m_busy == value)
        return;
    m_busy = value;
    emit busyChanged();
}

void ChatController::setStatusText(const QString &value)
{
    if (m_statusText == value)
        return;
    m_statusText = value;
    emit statusTextChanged();
}

void ChatController::appendMessage(const QString &role, const QString &content)
{
    m_messages.append(QVariantMap{
        {QStringLiteral("role"), role},
        {QStringLiteral("content"), content}
    });
    emit messagesChanged();
}

QJsonArray ChatController::boundedHistory() const
{
    QJsonArray result;
    int totalBytes = 0;
    int count = 0;
    for (qsizetype i = m_history.size() - 1; i >= 0 && count < 8; --i) {
        const QJsonObject item = m_history.at(i).toObject();
        const QString content = item.value(QStringLiteral("content")).toString();
        const int messageBytes = content.toUtf8().size();
        if (totalBytes + messageBytes > 11000)
            break;
        result.prepend(item);
        totalBytes += messageBytes;
        ++count;
    }
    return result;
}

void ChatController::sendMessage(const QString &rawText)
{
    const QString text = rawText.trimmed();
    if (m_busy || text.isEmpty())
        return;
    if (text.toUtf8().size() > 10000) {
        setStatusText(QStringLiteral("Please keep each message under 10,000 UTF-8 bytes."));
        return;
    }

    appendMessage(QStringLiteral("user"), text);
    m_history.append(QJsonObject{
        {QStringLiteral("role"), QStringLiteral("user")},
        {QStringLiteral("content"), text}
    });
    setBusy(true);
    setStatusText(QStringLiteral("Thinking with your local model…"));

    const QString runtimeDir = qEnvironmentVariable("XDG_RUNTIME_DIR");
    if (runtimeDir.isEmpty()) {
        finishWithError(QStringLiteral("XDG_RUNTIME_DIR is not set. Launch from your desktop session."));
        return;
    }
    const QString socketPath = QDir(runtimeDir).filePath(
        QStringLiteral("assistant-v2/core.sock"));

    auto *socket = new QLocalSocket(this);
    m_socket = socket;

    connect(socket, &QLocalSocket::connected, this, [this, socket]() {
        m_responseBuffer.clear();
        const QJsonObject payload{{QStringLiteral("messages"), boundedHistory()}};
        const QJsonObject request{
            {QStringLiteral("version"), 1},
            {QStringLiteral("id"), QUuid::createUuid().toString(QUuid::WithoutBraces)},
            {QStringLiteral("operation"), QStringLiteral("agent.chat")},
            {QStringLiteral("payload"), payload}
        };
        socket->write(QJsonDocument(request).toJson(QJsonDocument::Compact) + '\n');
        socket->flush();
    });

    connect(socket, &QLocalSocket::readyRead, this, [this, socket]() {
        m_responseBuffer.append(socket->readAll());
        const qsizetype newline = m_responseBuffer.indexOf('\n');
        if (newline < 0)
            return;
        const QByteArray responseLine = m_responseBuffer.left(newline);
        m_responseBuffer.remove(0, newline + 1);

        QJsonParseError parseError;
        const QJsonDocument document = QJsonDocument::fromJson(responseLine, &parseError);
        if (parseError.error != QJsonParseError::NoError || !document.isObject()) {
            finishWithError(QStringLiteral("The core returned an invalid response."));
        } else {
            const QJsonObject response = document.object();
            if (!response.value(QStringLiteral("ok")).toBool()) {
                const QString message = response.value(QStringLiteral("error")).toObject()
                    .value(QStringLiteral("message")).toString(
                        QStringLiteral("The request was rejected."));
                finishWithError(message);
            } else {
                const QJsonObject result = response.value(QStringLiteral("result")).toObject();
                const QString answer = result.value(QStringLiteral("message")).toString();
                if (answer.isEmpty()) {
                    finishWithError(QStringLiteral("The model returned an empty response."));
                } else {
                    appendMessage(QStringLiteral("assistant"), answer);
                    m_history.append(QJsonObject{
                        {QStringLiteral("role"), QStringLiteral("assistant")},
                        {QStringLiteral("content"), answer}
                    });
                    setStatusText(QStringLiteral("Connected • %1").arg(
                        result.value(QStringLiteral("model")).toString(
                            QStringLiteral("local model"))));
                    setBusy(false);
                }
            }
        }

        socket->disconnectFromServer();
        socket->deleteLater();
        m_socket = nullptr;
    });

    connect(socket, &QLocalSocket::errorOccurred, this,
            [this, socket](QLocalSocket::LocalSocketError) {
        if (m_busy) {
            finishWithError(QStringLiteral(
                "Cannot reach assistantd. Run ./run.sh and check .run/assistantd.log."));
            socket->deleteLater();
            m_socket = nullptr;
        }
    });

    socket->connectToServer(socketPath);
}

void ChatController::finishWithError(const QString &message)
{
    appendMessage(QStringLiteral("system"), message);
    setStatusText(message);
    setBusy(false);
}

void ChatController::clearConversation()
{
    if (m_busy)
        return;
    m_messages.clear();
    m_history = QJsonArray{};
    emit messagesChanged();
    setStatusText(QStringLiteral("New conversation"));
}
