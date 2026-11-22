import QtQuick
import QtQuick.Controls
import QtQuick.Controls.Material
import QtQuick.Layouts

ApplicationWindow {
    visible: true
    width: 1240
    height: 820
    minimumWidth: 900
    minimumHeight: 620
    title: "Local Assistant 2.0"
    color: "#0b1020"
    Material.theme: Material.Dark
    Material.accent: "#8b7cff"

    RowLayout {
        anchors.fill: parent
        spacing: 0

        Rectangle {
            Layout.preferredWidth: 258
            Layout.fillHeight: true
            color: "#10172a"
            ColumnLayout {
                anchors.fill: parent
                anchors.margins: 22
                spacing: 20

                RowLayout {
                    spacing: 10
                    Rectangle {
                        width: 36; height: 36; radius: 12
                        color: "#7766e8"
                        Text {
                            anchors.centerIn: parent
                            text: "L"; color: "white"
                            font.pixelSize: 20; font.bold: true
                        }
                    }
                    ColumnLayout {
                        spacing: 0
                        Label {
                            text: "LOCAL ASSISTANT"
                            color: "#f1f2ff"
                            font.pixelSize: 13; font.bold: true
                        }
                        Label {
                            text: "VERSION 2.0"
                            color: "#858da8"
                            font.pixelSize: 10; font.letterSpacing: 1.4
                        }
                    }
                }

                Button {
                    text: "+   New conversation"
                    Layout.fillWidth: true
                    highlighted: true
                    onClicked: chatController.clearConversation()
                }

                Label {
                    text: "WORKSPACE"
                    color: "#79829e"
                    font.pixelSize: 10; font.bold: true
                    font.letterSpacing: 1.5
                    topPadding: 12
                }

                Rectangle {
                    Layout.fillWidth: true
                    height: 48; radius: 10
                    color: "#1a2340"
                    RowLayout {
                        anchors.fill: parent
                        anchors.leftMargin: 12
                        spacing: 10
                        Text { text: "◈"; color: "#a99dff"; font.pixelSize: 18 }
                        Label { text: "Personal"; color: "#e4e7f6" }
                        Item { Layout.fillWidth: true }
                        Label { text: "LOCAL"; color: "#8d84db"; font.pixelSize: 9 }
                    }
                }

                Item { Layout.fillHeight: true }

                Rectangle {
                    Layout.fillWidth: true
                    radius: 12
                    color: "#171f35"
                    implicitHeight: 118
                    ColumnLayout {
                        anchors.fill: parent
                        anchors.margins: 14
                        spacing: 7
                        Label {
                            text: "MODEL"; color: "#8992ad"
                            font.pixelSize: 10; font.bold: true
                            font.letterSpacing: 1.2
                        }
                        Label {
                            text: "Qwen 3.5 · 2B"
                            color: "#f1f2ff"; font.pixelSize: 14; font.bold: true
                        }
                        Label {
                            text: "Local inference · 8K context"
                            color: "#a4abc1"; font.pixelSize: 11
                        }
                        Label {
                            text: "No cloud connection"
                            color: "#76c9a2"; font.pixelSize: 10
                        }
                    }
                }
                Label { text: "Prototype build"; color: "#626d88"; font.pixelSize: 10 }
            }
        }

        Rectangle {
            Layout.fillWidth: true
            Layout.fillHeight: true
            color: "#0b1020"
            ColumnLayout {
                anchors.fill: parent
                anchors.leftMargin: 38
                anchors.rightMargin: 38
                anchors.topMargin: 24
                anchors.bottomMargin: 22
                spacing: 16

                RowLayout {
                    Layout.fillWidth: true
                    ColumnLayout {
                        spacing: 3
                        Label {
                            text: "Assistant"
                            color: "#f2f3ff"; font.pixelSize: 23; font.bold: true
                        }
                        Label {
                            text: "A private conversation with your local model"
                            color: "#8e98b3"; font.pixelSize: 12
                        }
                    }
                    Item { Layout.fillWidth: true }
                    Rectangle {
                        radius: 14
                        color: "#142c2b"; border.color: "#24534a"
                        implicitWidth: statusLabel.implicitWidth + 24
                        implicitHeight: 30
                        Label {
                            id: statusLabel
                            anchors.centerIn: parent
                            text: chatController.busy ? "●  GENERATING" : "●  LOCAL MODE"
                            color: chatController.busy ? "#f2c879" : "#7fd3ad"
                            font.pixelSize: 10; font.bold: true
                        }
                    }
                }

                Rectangle { Layout.fillWidth: true; height: 1; color: "#202943" }

                Item {
                    Layout.fillWidth: true
                    Layout.fillHeight: true

                    ColumnLayout {
                        anchors.centerIn: parent
                        width: Math.min(parent.width, 760)
                        spacing: 12
                        visible: chatController.messages.length === 0
                        Label {
                            Layout.alignment: Qt.AlignHCenter
                            text: "What can I help you with?"
                            color: "#f1f2ff"; font.pixelSize: 29; font.weight: Font.DemiBold
                        }
                        Label {
                            Layout.alignment: Qt.AlignHCenter
                            text: "Your first model-backed conversation starts here."
                            color: "#929bb5"; font.pixelSize: 13
                        }
                    }

                    ListView {
                        id: chatList
                        anchors.fill: parent
                        clip: true
                        spacing: 14
                        model: chatController.messages
                        delegate: Item {
                            width: chatList.width
                            height: bubble.implicitHeight + 12
                            Rectangle {
                                id: bubble
                                anchors.left: modelData.role === "user" ? undefined : parent.left
                                anchors.right: modelData.role === "user" ? parent.right : undefined
                                width: Math.min(chatList.width * 0.82, 790)
                                radius: 14
                                color: modelData.role === "user" ? "#25264a"
                                    : (modelData.role === "system" ? "#38252d" : "#151f34")
                                border.color: modelData.role === "user" ? "#454278" : "#26324c"
                                implicitHeight: messageText.implicitHeight + 28
                                Text {
                                    id: messageText
                                    anchors.fill: parent
                                    anchors.margins: 14
                                    text: modelData.content
                                    color: modelData.role === "system" ? "#f0b6b6" : "#e7eaf7"
                                    font.pixelSize: 14
                                    wrapMode: Text.Wrap
                                    textFormat: Text.PlainText
                                }
                            }
                        }
                        onCountChanged: Qt.callLater(function() { chatList.positionViewAtEnd() })
                    }
                }

                Rectangle {
                    Layout.fillWidth: true
                    radius: 16
                    color: "#151e33"
                    border.color: "#303b59"
                    implicitHeight: 116
                    ColumnLayout {
                        anchors.fill: parent
                        anchors.margins: 12
                        spacing: 6
                        TextArea {
                            id: composer
                            Layout.fillWidth: true
                            Layout.fillHeight: true
                            placeholderText: "Message your local assistant…"
                            color: "#f0f2ff"
                            placeholderTextColor: "#77829f"
                            wrapMode: TextEdit.Wrap
                            selectByMouse: true
                            background: Rectangle { color: "transparent" }
                            enabled: !chatController.busy
                            Keys.onReturnPressed: function(event) {
                                if (event.modifiers & Qt.ShiftModifier) {
                                    insert(cursorPosition, "\n")
                                } else {
                                    chatController.sendMessage(text)
                                    clear()
                                }
                            }
                        }
                        RowLayout {
                            Layout.fillWidth: true
                            Label {
                                text: chatController.statusText
                                color: "#818ca8"; font.pixelSize: 10
                                elide: Text.ElideRight
                                Layout.fillWidth: true
                            }
                            Button {
                                text: chatController.busy ? "Working…" : "Send  ↑"
                                highlighted: true
                                enabled: !chatController.busy && composer.text.trim().length > 0
                                onClicked: {
                                    chatController.sendMessage(composer.text)
                                    composer.clear()
                                }
                            }
                        }
                    }
                }
                Label {
                    Layout.alignment: Qt.AlignHCenter
                    text: "Local prototype · Model responses cannot execute system actions"
                    color: "#5f6a85"; font.pixelSize: 10
                }
            }
        }
    }
}
