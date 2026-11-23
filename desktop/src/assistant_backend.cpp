#include "assistant_backend.h"

#include <QDir>
#include <QJsonDocument>
#include <QJsonArray>
#include <QJsonValue>
#include <QStandardPaths>

AssistantBackend::AssistantBackend(QObject *parent)
    : QObject(parent)
{
    connect(&m_socket, &QLocalSocket::readyRead,
            this, &AssistantBackend::readSocket);

    connect(&m_socket, &QLocalSocket::connected,
            this, &AssistantBackend::socketConnected);

    connect(&m_socket, &QLocalSocket::disconnected,
            this, &AssistantBackend::socketDisconnected);

    connect(&m_socket, &QLocalSocket::errorOccurred,
            this, [this](QLocalSocket::LocalSocketError) {
        if (m_socket.state() == QLocalSocket::UnconnectedState)
            setStatus("Core connection error: " + m_socket.errorString());
    });
}

bool AssistantBackend::connected() const
{
    return m_connected;
}

bool AssistantBackend::busy() const
{
    return m_busy;
}

QString AssistantBackend::status() const
{
    return m_status;
}

QVariantList AssistantBackend::messages() const
{
    return m_messages;
}

void AssistantBackend::setStatus(const QString &status)
{
    if (m_status == status)
        return;

    m_status = status;
    emit statusChanged();
}

void AssistantBackend::setConnected(bool connected)
{
    if (m_connected == connected)
        return;

    m_connected = connected;
    emit connectedChanged();
}

void AssistantBackend::setBusy(bool busy)
{
    if (m_busy == busy)
        return;

    m_busy = busy;
    emit busyChanged();
}

void AssistantBackend::connectToCore()
{
    if (m_socket.state() != QLocalSocket::UnconnectedState)
        return;

    const QString runtimeDir =
        qEnvironmentVariable("XDG_RUNTIME_DIR");

    if (runtimeDir.isEmpty()) {
        setStatus("XDG_RUNTIME_DIR is not available");
        return;
    }

    const QString socketPath =
        runtimeDir + "/local-assistant/assistantd.sock";

    setStatus("Connecting to assistant core...");
    m_socket.connectToServer(socketPath);
}

void AssistantBackend::socketConnected()
{
    setConnected(true);
    setStatus("Connected to Local Assistant 2.0");

    sendRequest("initialize", QJsonObject{}, m_nextRequestId++);
}

void AssistantBackend::socketDisconnected()
{
    setConnected(false);
    setBusy(false);
    m_conversationId.clear();
    setStatus("Disconnected from assistant core");
}

void AssistantBackend::sendRequest(
    const QString &method,
    const QJsonObject &params,
    int requestId)
{
    m_pendingMethods.insert(requestId, method);

    QJsonObject request{
        {"jsonrpc", "2.0"},
        {"method", method},
        {"params", params},
        {"id", requestId}
    };

    QByteArray data =
        QJsonDocument(request).toJson(QJsonDocument::Compact);
    data.append('\n');

    m_socket.write(data);
}

void AssistantBackend::readSocket()
{
    m_buffer.append(m_socket.readAll());

    while (true) {
        const qsizetype newline = m_buffer.indexOf('\n');

        if (newline < 0)
            break;

        const QByteArray line = m_buffer.left(newline).trimmed();
        m_buffer.remove(0, newline + 1);

        if (line.isEmpty())
            continue;

        QJsonParseError error;
        const QJsonDocument document =
            QJsonDocument::fromJson(line, &error);

        if (error.error != QJsonParseError::NoError ||
            !document.isObject()) {
            setStatus("Received invalid data from assistant core");
            continue;
        }

        handleMessage(document.object());
    }
}

void AssistantBackend::handleMessage(const QJsonObject &message)
{
    if (message.contains("id")) {
        const int id = message.value("id").toInt(-1);
        const QString method = m_pendingMethods.take(id);

        if (message.contains("error")) {
            setStatus("Core error: " +
                message.value("error").toObject()
                    .value("message").toString());
            return;
        }

        const QJsonObject result =
            message.value("result").toObject();

        if (method == "initialize") {
            setStatus("Creating conversation...");

            sendRequest(
                "conversation.create",
                QJsonObject{},
                m_nextRequestId++
            );
            return;
        }

        if (method == "conversation.create") {
            m_conversationId =
                result.value("conversationId").toString();

            if (m_conversationId.isEmpty()) {
                setStatus("Core returned an invalid conversation ID");
                return;
            }

            setStatus("Ready");
            return;
        }

        if (method == "chat.send") {
            if (!result.value("accepted").toBool()) {
                setBusy(false);
                setStatus("Core did not accept the message");
            }
            return;
        }

        return;
    }

    const QString method = message.value("method").toString();
    const QJsonObject params = message.value("params").toObject();

    if (method == "chat.delta") {
        appendToLastAssistantMessage(
            params.value("delta").toString()
        );
    } else if (method == "chat.completed") {
        setBusy(false);
        setStatus("Ready");
    } else if (method == "chat.failed") {
        setBusy(false);
        setStatus("Generation failed: " +
            params.value("error").toString());
    } else if (method == "chat.cancelled") {
        setBusy(false);
        setStatus("Generation cancelled");
    }
}

void AssistantBackend::addMessage(
    const QString &role,
    const QString &content)
{
    m_messages.append(QVariantMap{
        {"role", role},
        {"content", content}
    });

    emit messagesChanged();
}

void AssistantBackend::appendToLastAssistantMessage(
    const QString &delta)
{
    if (m_messages.isEmpty() || delta.isEmpty())
        return;

    QVariantMap last = m_messages.last().toMap();

    if (last.value("role").toString() != "assistant")
        return;

    last["content"] = last.value("content").toString() + delta;
    m_messages.last() = last;

    emit messagesChanged();
}

void AssistantBackend::sendMessage(const QString &text)
{
    const QString cleaned = text.trimmed();

    if (cleaned.isEmpty() || !m_connected ||
        m_conversationId.isEmpty() || m_busy)
        return;

    addMessage("user", cleaned);
    addMessage("assistant", "");

    QJsonObject params{
        {"conversationId", m_conversationId},
        {"text", cleaned}
    };

    sendRequest("chat.send", params, m_nextRequestId++);

    setBusy(true);
    setStatus("Generating...");
}

void AssistantBackend::newConversation()
{
    if (!m_connected || m_busy)
        return;

    m_messages.clear();
    m_conversationId.clear();
    emit messagesChanged();

    setStatus("Creating conversation...");

    sendRequest(
        "conversation.create",
        QJsonObject{},
        m_nextRequestId++
    );
}
