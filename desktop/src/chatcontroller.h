#pragma once

#include <QObject>
#include <QJsonArray>
#include <QLocalSocket>
#include <QPointer>
#include <QString>
#include <QVariantList>

class ChatController final : public QObject
{
    Q_OBJECT
    Q_PROPERTY(QVariantList messages READ messages NOTIFY messagesChanged)
    Q_PROPERTY(bool busy READ busy NOTIFY busyChanged)
    Q_PROPERTY(QString statusText READ statusText NOTIFY statusTextChanged)

public:
    explicit ChatController(QObject *parent = nullptr);
    QVariantList messages() const;
    bool busy() const;
    QString statusText() const;

    Q_INVOKABLE void sendMessage(const QString &text);
    Q_INVOKABLE void clearConversation();

signals:
    void messagesChanged();
    void busyChanged();
    void statusTextChanged();

private:
    void appendMessage(const QString &role, const QString &content);
    void setBusy(bool busy);
    void setStatusText(const QString &text);
    void finishWithError(const QString &message);
    QJsonArray boundedHistory() const;

    QVariantList m_messages;
    QJsonArray m_history;
    QPointer<QLocalSocket> m_socket;
    QString m_statusText = QStringLiteral("Ready to connect to your local core");
    bool m_busy = false;
};
