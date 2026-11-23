#include "assistant_backend.h"

#include <QDir>
#include <QJsonDocument>
#include <QJsonArray>
#include <QJsonValue>
#include <QStandardPaths>
#include <QCoreApplication>
#include <QFileInfo>

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
        if (m_socket.state() == QLocalSocket::UnconnectedState &&
            !m_startedCore && !m_shuttingDown) {
            startCore();
        }
    });

    m_connectionTimer.setInterval(100);

    connect(&m_connectionTimer, &QTimer::timeout, this, [this]() {
        if (m_socket.state() == QLocalSocket::ConnectedState) {
            m_connectionTimer.stop();
            return;
        }

        if (++m_connectionAttempts > 50) {
            m_connectionTimer.stop();
            setStatus("Timed out waiting for assistant core");
            return;
        }

        if (m_socket.state() == QLocalSocket::UnconnectedState) {
            const QString runtimeDir =
                qEnvironmentVariable("XDG_RUNTIME_DIR");

            if (!runtimeDir.isEmpty()) {
                m_socket.connectToServer(
                    runtimeDir + "/local-assistant/assistantd.sock"
                );
            }
        }
    });

    connect(&m_coreProcess, &QProcess::errorOccurred,
            this, [this](QProcess::ProcessError error) {
        if (error == QProcess::FailedToStart) {
            m_startedCore = false;
            m_connectionTimer.stop();
            setStatus("Could not start assistant core: " +
                      m_coreProcess.errorString());
        }
    });

    connect(&m_coreProcess,
            qOverload<int, QProcess::ExitStatus>(&QProcess::finished),
            this, [this](int exitCode, QProcess::ExitStatus) {
        if (m_startedCore && !m_shuttingDown) {
            m_startedCore = false;
            m_connectionTimer.stop();
            setStatus("Assistant core stopped unexpectedly (exit code " +
                      QString::number(exitCode) + ")");
        }
    });
}

AssistantBackend::~AssistantBackend()
{
    m_shuttingDown = true;
    m_connectionTimer.stop();

    // Prevent socket teardown from invoking callbacks on a
    // backend whose members are being destroyed.
    QObject::disconnect(&m_socket, nullptr, this, nullptr);
    m_socket.abort();

    stopOwnedCore();
}

void AssistantBackend::startCore()
{
    if (m_startedCore ||
        m_coreProcess.state() != QProcess::NotRunning)
        return;

    QDir projectRoot(QCoreApplication::applicationDirPath());

    if (!projectRoot.cdUp() || !projectRoot.cdUp()) {
        setStatus("Could not locate the project directory");
        return;
    }

    const QStringList candidates{
        projectRoot.filePath("core/assistantd/target/debug/assistantd"),
        projectRoot.filePath("core/assistantd/target/release/assistantd")
    };

    QString executable;

    for (const QString &candidate : candidates) {
        if (QFileInfo(candidate).isExecutable()) {
            executable = candidate;
            break;
        }
    }

    if (executable.isEmpty()) {
        setStatus("Assistant core executable not found. Build the Rust core first.");
        return;
    }

    setStatus("Starting assistant core...");

    m_coreProcess.setProgram(executable);
    m_coreProcess.setArguments({"--ipc"});
    m_coreProcess.setProcessChannelMode(QProcess::ForwardedChannels);

    m_startedCore = true;
    m_connectionAttempts = 0;

    m_coreProcess.start();

    if (!m_coreProcess.waitForStarted(3000)) {
        m_startedCore = false;
        setStatus("Failed to start assistant core: " +
                  m_coreProcess.errorString());
        return;
    }

    m_connectionTimer.start();
}

void AssistantBackend::stopOwnedCore()
{
    m_shuttingDown = true;
    m_connectionTimer.stop();

    if (!m_startedCore ||
        m_coreProcess.state() == QProcess::NotRunning)
        return;

    m_coreProcess.terminate();

    if (!m_coreProcess.waitForFinished(1500)) {
        m_coreProcess.kill();
        m_coreProcess.waitForFinished(1000);
    }

    m_startedCore = false;
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

QVariantList AssistantBackend::conversations() const
{
    return m_conversations;
}

QString AssistantBackend::activeConversationId() const
{
    return m_conversationId;
}

int AssistantBackend::contextLengthTokens() const
{
    return m_contextLengthTokens;
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

void AssistantBackend::setActiveConversationId(const QString &id)
{
    if (m_conversationId == id)
        return;

    m_conversationId = id;
    emit activeConversationChanged();
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

    setStatus("Checking assistant core...");

    // First try an existing server. If connection fails, start our own.
    m_socket.connectToServer(socketPath);

    if (!m_socket.waitForConnected(300)) {
        m_socket.abort();
        startCore();
    }
}

void AssistantBackend::socketConnected()
{
    m_connectionTimer.stop();
    setConnected(true);
    setStatus("Connected to Local Assistant 2.0");

    sendRequest("initialize", QJsonObject{}, m_nextRequestId++);
}

void AssistantBackend::socketDisconnected()
{
    if (m_shuttingDown)
        return;

    setConnected(false);
    setBusy(false);

    setActiveConversationId(QString{});

    m_messages.clear();
    m_conversations.clear();

    emit messagesChanged();
    emit conversationsChanged();

    m_pendingMethods.clear();
    m_pendingConversationIds.clear();
    m_buffer.clear();

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
        const QString conversationId = m_pendingConversationIds.take(id);

        if (message.contains("error")) {
            setBusy(false);
            setStatus("Core error: " +
                message.value("error").toObject()
                    .value("message").toString());
            return;
        }

        const QJsonObject result =
            message.value("result").toObject();

        if (method == "initialize") {
            loadConversations();
            loadSettings();
            return;
        }

        if (method == "settings.get" || method == "settings.update") {
            const int value = result.value("contextLength").toInt(16384);
            if (value >= 4096 && value <= 16384 && value % 1024 == 0 &&
                m_contextLengthTokens != value) {
                m_contextLengthTokens = value;
                emit contextLengthTokensChanged();
            }

            if (method == "settings.update")
                setStatus("Settings saved");

            return;
        }

        if (method == "conversation.list") {
            const QJsonArray array =
                result.value("conversations").toArray();

            QVariantList updated;

            for (const QJsonValue &value : array) {
                const QJsonObject conversation = value.toObject();

                updated.append(QVariantMap{
                    {"id", conversation.value("id").toString()},
                    {"title", conversation.value("title").toString()}
                });
            }

            m_conversations = updated;
            emit conversationsChanged();

            if (m_conversationId.isEmpty()) {
                if (m_conversations.isEmpty()) {
                    newConversation();
                    return;
                }

                const QString firstId =
                    m_conversations.first().toMap().value("id").toString();

                selectConversation(firstId);
                return;
            }

            bool activeExists = false;

            for (const QVariant &entry : m_conversations) {
                if (entry.toMap().value("id").toString() ==
                    m_conversationId) {
                    activeExists = true;
                    break;
                }
            }

            if (!activeExists) {
                if (m_conversations.isEmpty()) {
                    setActiveConversationId(QString{});
                    m_messages.clear();
                    emit messagesChanged();
                    newConversation();
                } else {
                    selectConversation(
                        m_conversations.first().toMap()
                            .value("id").toString()
                    );
                }
            }

            return;
        }

        if (method == "conversation.create") {
            const QString newId =
                result.value("conversationId").toString();

            if (newId.isEmpty()) {
                setStatus("Core returned an invalid conversation ID");
                return;
            }

            setActiveConversationId(newId);
            m_messages.clear();
            emit messagesChanged();

            setStatus("Loading conversations...");
            loadConversations();
            return;
        }

        if (method == "conversation.get") {
            const QJsonArray array =
                result.value("messages").toArray();

            QVariantList loadedMessages;

            for (const QJsonValue &value : array) {
                const QJsonObject messageObject = value.toObject();

                loadedMessages.append(QVariantMap{
                    {"role", messageObject.value("role").toString()},
                    {"content", messageObject.value("content").toString()}
                });
            }

            m_messages = loadedMessages;
            emit messagesChanged();

            setStatus("Ready");
            return;
        }

        if (method == "conversation.rename") {
            setStatus("Conversation renamed");
            loadConversations();
            return;
        }

        if (method == "conversation.delete") {
            if (conversationId == m_conversationId) {
                setActiveConversationId(QString{});
                m_messages.clear();
                emit messagesChanged();
            }

            setStatus("Conversation deleted");
            loadConversations();
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
        loadConversations();
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

void AssistantBackend::loadConversations()
{
    if (!m_connected)
        return;

    sendRequest(
        "conversation.list",
        QJsonObject{},
        m_nextRequestId++
    );
}

void AssistantBackend::loadSettings()
{
    if (!m_connected)
        return;

    sendRequest("settings.get", QJsonObject{}, m_nextRequestId++);
}

void AssistantBackend::setContextLengthTokens(int value)
{
    if (!m_connected)
        return;

    if (value < 4096 || value > 16384 || value % 1024 != 0) {
        setStatus("Context length must be between 4K and 16K");
        return;
    }

    sendRequest(
        "settings.update",
        QJsonObject{{"contextLength", value}},
        m_nextRequestId++
    );

    setStatus("Saving settings...");
}

void AssistantBackend::selectConversation(const QString &conversationId)
{
    if (!m_connected || m_busy || conversationId.isEmpty())
        return;

    if (m_conversationId == conversationId &&
        !m_messages.isEmpty())
        return;

    setActiveConversationId(conversationId);

    m_messages.clear();
    emit messagesChanged();

    setStatus("Loading conversation...");

    sendRequest(
        "conversation.get",
        QJsonObject{{"conversationId", conversationId}},
        m_nextRequestId++
    );
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

    setActiveConversationId(QString{});

    m_messages.clear();
    emit messagesChanged();

    setStatus("Creating conversation...");

    sendRequest(
        "conversation.create",
        QJsonObject{},
        m_nextRequestId++
    );
}



void AssistantBackend::renameConversation(
    const QString &conversationId,
    const QString &title)
{
    const QString cleaned = title.trimmed();

    if (!m_connected || m_busy || conversationId.isEmpty())
        return;

    if (cleaned.isEmpty() || cleaned.size() > 100) {
        setStatus("Name must contain 1 to 100 characters");
        return;
    }

    const int requestId = m_nextRequestId++;
    m_pendingConversationIds.insert(requestId, conversationId);

    sendRequest(
        "conversation.rename",
        QJsonObject{
            {"conversationId", conversationId},
            {"title", cleaned}
        },
        requestId
    );

    setStatus("Renaming conversation...");
}

void AssistantBackend::deleteConversation(const QString &conversationId)
{
    if (!m_connected || m_busy || conversationId.isEmpty())
        return;

    const int requestId = m_nextRequestId++;
    m_pendingConversationIds.insert(requestId, conversationId);

    sendRequest(
        "conversation.delete",
        QJsonObject{{"conversationId", conversationId}},
        requestId
    );

    setStatus("Deleting conversation...");
}
