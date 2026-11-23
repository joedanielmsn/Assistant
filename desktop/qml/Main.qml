import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

ApplicationWindow {
    id: window

    width: 1100
    height: 760
    minimumWidth: 720
    minimumHeight: 520
    visible: true
    title: "Local Assistant 2.0"

    color: "#111318"

    property color panelColor: "#191c23"
    property color borderColor: "#292e38"
    property color accentColor: "#8b9cff"
    property color textColor: "#edf0f7"
    property color mutedColor: "#9299aa"

    ColumnLayout {
        anchors.fill: parent
        spacing: 0

        Rectangle {
            Layout.fillWidth: true
            Layout.preferredHeight: 66
            color: panelColor
            border.color: borderColor

            RowLayout {
                anchors.fill: parent
                anchors.leftMargin: 22
                anchors.rightMargin: 22

                ColumnLayout {
                    spacing: 2

                    Label {
                        text: "Local Assistant"
                        color: textColor
                        font.pixelSize: 19
                        font.bold: true
                    }

                    Label {
                        text: "Version 2.0 · Local AI"
                        color: mutedColor
                        font.pixelSize: 11
                    }
                }

                Item { Layout.fillWidth: true }

                Rectangle {
                    radius: 12
                    color: assistant.connected ? "#19382d" : "#3a2528"
                    implicitWidth: statusLabel.implicitWidth + 22
                    implicitHeight: 30

                    Label {
                        id: statusLabel
                        anchors.centerIn: parent
                        text: assistant.connected ? "● Connected" : "● Offline"
                        color: assistant.connected ? "#8ce0b5" : "#f09b9b"
                        font.pixelSize: 11
                    }
                }

                Button {
                    text: "New chat"
                    onClicked: assistant.newConversation()
                    enabled: assistant.connected && !assistant.busy
                }
            }
        }

        RowLayout {
            Layout.fillWidth: true
            Layout.fillHeight: true
            spacing: 0

            Rectangle {
                Layout.preferredWidth: 220
                Layout.fillHeight: true
                color: "#15181e"
                border.color: borderColor

                ColumnLayout {
                    anchors.fill: parent
                    anchors.margins: 18
                    spacing: 14

                    Label {
                        text: "WORKSPACE"
                        color: mutedColor
                        font.pixelSize: 10
                        font.bold: true
                    }

                    Rectangle {
                        Layout.fillWidth: true
                        height: 42
                        radius: 8
                        color: "#252a35"

                        RowLayout {
                            anchors.fill: parent
                            anchors.leftMargin: 12
                            Label {
                                text: "◉"
                                color: accentColor
                            }
                            Label {
                                text: "Assistant"
                                color: textColor
                                font.pixelSize: 13
                            }
                        }
                    }

                    Label {
                        text: "Your conversations"
                        color: mutedColor
                        font.pixelSize: 11
                        Layout.topMargin: 12
                    }

                    Label {
                        text: "Current chat"
                        color: textColor
                        font.pixelSize: 12
                    }

                    Item { Layout.fillHeight: true }

                    Label {
                        text: "MODEL"
                        color: mutedColor
                        font.pixelSize: 10
                        font.bold: true
                    }

                    Label {
                        text: "assistant-qwen:latest"
                        color: textColor
                        font.pixelSize: 11
                        wrapMode: Text.Wrap
                        Layout.fillWidth: true
                    }

                    Label {
                        text: "16K context · Ollama"
                        color: mutedColor
                        font.pixelSize: 10
                    }
                }
            }

            ColumnLayout {
                Layout.fillWidth: true
                Layout.fillHeight: true
                spacing: 0

                Rectangle {
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    color: window.color

                    ListView {
                        id: chatList
                        anchors.fill: parent
                        anchors.margins: 24
                        spacing: 18
                        clip: true
                        model: assistant.messages

                        delegate: Item {
                            width: chatList.width
                            height: messageColumn.implicitHeight

                            ColumnLayout {
                                id: messageColumn
                                width: parent.width
                                spacing: 6

                                Label {
                                    text: modelData.role === "user" ? "YOU" : "LOCAL ASSISTANT"
                                    color: modelData.role === "user" ? accentColor : "#8ce0b5"
                                    font.pixelSize: 10
                                    font.bold: true
                                }

                                Rectangle {
                                    Layout.maximumWidth: Math.min(chatList.width * 0.88, 760)
                                    Layout.preferredWidth: Math.min(
                                        Math.max(messageText.implicitWidth + 32, 100),
                                        Math.min(chatList.width * 0.88, 760)
                                    )
                                    Layout.preferredHeight: messageText.implicitHeight + 24
                                    radius: 12
                                    color: modelData.role === "user" ? "#252d43" : panelColor
                                    border.color: borderColor

                                    Text {
                                        id: messageText
                                        anchors.fill: parent
                                        anchors.margins: 12
                                        text: modelData.content.length > 0
                                            ? modelData.content
                                            : (assistant.busy && modelData.role === "assistant"
                                                ? "Thinking..."
                                                : "")
                                        color: textColor
                                        font.pixelSize: 14
                                        wrapMode: Text.Wrap
                                        textFormat: Text.PlainText
                                    }
                                }
                            }
                        }

                        onCountChanged: Qt.callLater(function() {
                            positionViewAtEnd()
                        })
                    }

                    Column {
                        anchors.centerIn: parent
                        spacing: 10
                        visible: assistant.messages.length === 0

                        Label {
                            anchors.horizontalCenter: parent.horizontalCenter
                            text: "Welcome to Local Assistant 2.0"
                            color: textColor
                            font.pixelSize: 23
                            font.bold: true
                        }

                        Label {
                            anchors.horizontalCenter: parent.horizontalCenter
                            text: "Your private, locally running AI assistant."
                            color: mutedColor
                            font.pixelSize: 13
                        }
                    }
                }

                Rectangle {
                    Layout.fillWidth: true
                    Layout.preferredHeight: 116
                    color: panelColor
                    border.color: borderColor

                    ColumnLayout {
                        anchors.fill: parent
                        anchors.margins: 16
                        spacing: 8

                        RowLayout {
                            Layout.fillWidth: true

                            TextArea {
                                id: messageInput
                                Layout.fillWidth: true
                                Layout.fillHeight: true
                                placeholderText: "Message Local Assistant..."
                                color: textColor
                                placeholderTextColor: mutedColor
                                wrapMode: TextEdit.Wrap
                                font.pixelSize: 14
                                selectByMouse: true
                                background: Rectangle {
                                    radius: 9
                                    color: "#222630"
                                    border.color: messageInput.activeFocus
                                        ? accentColor : borderColor
                                }

                                Keys.onReturnPressed: function(event) {
                                    if (!event.modifiers || event.modifiers === Qt.NoModifier) {
                                        event.accepted = true
                                        if (!assistant.busy) {
                                            assistant.sendMessage(messageInput.text)
                                            messageInput.clear()
                                        }
                                    }
                                }

                                Keys.onEnterPressed: function(event) {
                                    if (!assistant.busy) {
                                        assistant.sendMessage(messageInput.text)
                                        messageInput.clear()
                                    }
                                }
                            }

                            Button {
                                Layout.preferredWidth: 92
                                Layout.fillHeight: true
                                text: assistant.busy ? "Working..." : "Send"
                                enabled: assistant.connected && !assistant.busy
                                onClicked: {
                                    assistant.sendMessage(messageInput.text)
                                    messageInput.clear()
                                }
                            }
                        }

                        Label {
                            text: assistant.status
                            color: mutedColor
                            font.pixelSize: 10
                        }
                    }
                }
            }
        }
    }
}
