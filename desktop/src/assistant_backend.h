#pragma once

#include <QObject>
#include <QHash>
#include <QVariantList>
#include <QVariantMap>
#include <QJsonObject>
#include <QLocalSocket>

class AssistantBackend : public QObject
{
    Q_OBJECT

    Q_PROPERTY(bool connected READ connected NOTIFY connectedChanged)
    Q_PROPERTY(bool busy READ busy NOTIFY busyChanged)
    Q_PROPERTY(QString status READ status NOTIFY statusChanged)
    Q_PROPERTY(QVariantList messages READ messages NOTIFY messagesChanged)

public:
    explicit AssistantBackend(QObject *parent = nullptr);

    bool connected() const;
    bool busy() const;
    QString status() const;
    QVariantList messages() const;

    Q_INVOKABLE void connectToCore();
    Q_INVOKABLE void sendMessage(const QString &text);
    Q_INVOKABLE void newConversation();

signals:
    void connectedChanged();
    void busyChanged();
    void statusChanged();
    void messagesChanged();

private slots:
    void readSocket();
    void socketConnected();
    void socketDisconnected();

private:
    void setStatus(const QString &status);
    void setConnected(bool connected);
    void setBusy(bool busy);
    void sendRequest(const QString &method, const QJsonObject &params,
                     int requestId);
    void handleMessage(const QJsonObject &message);
    void addMessage(const QString &role, const QString &content);
    void appendToLastAssistantMessage(const QString &delta);

    QLocalSocket m_socket;
    QByteArray m_buffer;
    QVariantList m_messages;
    QString m_status = "Disconnected";
    QString m_conversationId;
    bool m_connected = false;
    bool m_busy = false;
    int m_nextRequestId = 1;
    QHash<int, QString> m_pendingMethods;
};
