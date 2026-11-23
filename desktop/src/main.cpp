#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <QQmlContext>

#include "assistant_backend.h"

int main(int argc, char *argv[])
{
    QGuiApplication app(argc, argv);

    QGuiApplication::setApplicationName("Local Assistant 2.0");
    QGuiApplication::setOrganizationName("LocalAssistant");

    AssistantBackend backend;

    QQmlApplicationEngine engine;
    engine.rootContext()->setContextProperty("assistant", &backend);
    engine.loadFromModule("LocalAssistant", "Main");

    if (engine.rootObjects().isEmpty())
        return 1;

    backend.connectToCore();

    return app.exec();
}
