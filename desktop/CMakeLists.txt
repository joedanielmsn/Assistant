cmake_minimum_required(VERSION 3.21)
project(LocalAssistantDesktop VERSION 0.1.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_AUTOMOC ON)

find_package(Qt6 6.5 REQUIRED COMPONENTS Core Gui Qml Quick QuickControls2 Network)
qt_standard_project_setup(REQUIRES 6.5)

qt_add_executable(local-assistant
    src/main.cpp
    src/chatcontroller.cpp
    src/chatcontroller.h
)

qt_add_qml_module(local-assistant
    URI LocalAssistant
    VERSION 1.0
    QML_FILES qml/Main.qml
)

target_link_libraries(local-assistant PRIVATE
    Qt6::Core Qt6::Gui Qt6::Qml Qt6::Quick
    Qt6::QuickControls2 Qt6::Network
)
