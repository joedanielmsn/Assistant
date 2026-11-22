#include "chatcontroller.h"

#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <QQmlContext>

int main(int argc, char *argv[])
{
    QGuiApplication app(argc, argv);
    QGuiApplication::setApplicationName(QStringLiteral("Local Assistant"));
    QGuiApplication::setOrganizationName(QStringLiteral("Local Assistant"));

    ChatController chatController;
    QQmlApplicationEngine engine;
    engine.rootContext()->setContextProperty(
        QStringLiteral("chatController"), &chatController);
    engine.loadFromModule("LocalAssistant", "Main");

    if (engine.rootObjects().isEmpty())
        return 1;
    return app.exec();
}
