cmake_minimum_required(VERSION 3.25)

project(LocalAssistantDesktop VERSION 0.1.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(Qt6 6.8 REQUIRED COMPONENTS Quick Qml Network)

qt_standard_project_setup(REQUIRES 6.8)

qt_add_executable(local-assistant
    src/main.cpp
    src/assistant_backend.h
    src/assistant_backend.cpp
)

qt_add_qml_module(local-assistant
    URI LocalAssistant
    VERSION 1.0
    QML_FILES
        qml/Main.qml
)

target_link_libraries(local-assistant PRIVATE
    Qt6::Quick
    Qt6::Qml
    Qt6::Network
)
