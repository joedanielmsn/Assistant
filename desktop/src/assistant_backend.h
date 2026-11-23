#pragma once

#include <QObject>
#include <QHash>
#include <QVariantList>
#include <QVariantMap>
#include <QJsonObject>
#include <QLocalSocket>
#include <QProcess>
#include <QTimer>

class AssistantBackend : public QObject
{
    Q_OBJECT

    Q_PROPERTY(bool connected READ connected NOTIFY connectedChanged)
    Q_PROPERTY(bool busy READ busy NOTIFY busyChanged)
    Q_PROPERTY(QString status READ status NOTIFY statusChanged)
    Q_PROPERTY(QVariantList messages READ messages NOTIFY messagesChanged)
    Q_PROPERTY(QVariantList toolEvents READ toolEvents NOTIFY toolEventsChanged)
    Q_PROPERTY(QVariantList conversations READ conversations NOTIFY conversationsChanged)
    Q_PROPERTY(QString activeConversationId READ activeConversationId NOTIFY activeConversationChanged)
    Q_PROPERTY(int contextLengthTokens READ contextLengthTokens NOTIFY contextLengthTokensChanged)

public:
    explicit AssistantBackend(QObject *parent = nullptr);
    ~AssistantBackend() override;

    bool connected() const;
    bool busy() const;
    QString status() const;
    QVariantList messages() const;
    QVariantList toolEvents() const;
    QVariantList conversations() const;
    QString activeConversationId() const;
    int contextLengthTokens() const;

    Q_INVOKABLE void connectToCore();
    Q_INVOKABLE void loadSettings();
    Q_INVOKABLE void setContextLengthTokens(int value);
    Q_INVOKABLE void sendMessage(const QString &text);
    Q_INVOKABLE void cancelGeneration();
    Q_INVOKABLE void newConversation();
    Q_INVOKABLE void loadConversations();
    Q_INVOKABLE void selectConversation(const QString &conversationId);
    Q_INVOKABLE void renameConversation(const QString &conversationId, const QString &title);
    Q_INVOKABLE void deleteConversation(const QString &conversationId);

signals:
    void connectedChanged();
    void busyChanged();
    void statusChanged();
    void messagesChanged();
    void toolEventsChanged();
    void conversationsChanged();
    void activeConversationChanged();
    void contextLengthTokensChanged();

private slots:
    void readSocket();
    void socketConnected();
    void socketDisconnected();

private:
    void setStatus(const QString &status);
    void setConnected(bool connected);
    void setBusy(bool busy);
    void setActiveConversationId(const QString &id);
    void startCore();
    void stopOwnedCore();

    void sendRequest(const QString &method, const QJsonObject &params,
                     int requestId);
    void handleMessage(const QJsonObject &message);
    void addMessage(const QString &role, const QString &content);
    void appendToLastAssistantMessage(const QString &delta);

    QLocalSocket m_socket;
    QProcess m_coreProcess;
    QTimer m_connectionTimer;

    int m_connectionAttempts = 0;
    bool m_startedCore = false;
    bool m_shuttingDown = false;
    QByteArray m_buffer;
    QVariantList m_messages;
    QVariantList m_toolEvents;
    QVariantList m_conversations;

    QString m_status = "Disconnected";
    QString m_conversationId;
    QString m_activeTaskId;

    bool m_connected = false;
    bool m_busy = false;
    int m_contextLengthTokens = 16384;

    int m_nextRequestId = 1;
    QHash<int, QString> m_pendingMethods;
    QHash<int, QString> m_pendingConversationIds;
};
