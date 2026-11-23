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
    property string selectedConversationId: ""
    property string selectedConversationTitle: ""
    property int draftContextLength: 16384

    Connections {
        target: assistant

        function onContextLengthTokensChanged() {
            if (settingsDialog.visible)
                window.draftContextLength = assistant.contextLengthTokens
        }
    }

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
                    text: "Settings"
                    enabled: assistant.connected
                    onClicked: {
                        window.draftContextLength = assistant.contextLengthTokens
                        settingsDialog.open()
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

                    ListView {
                        id: conversationList
                        Layout.fillWidth: true
                        Layout.fillHeight: true
                        clip: true
                        spacing: 5
                        model: assistant.conversations

                        delegate: Rectangle {
                            width: conversationList.width
                            height: 46
                            radius: 7

                            color: modelData.id === assistant.activeConversationId
                                ? "#292f40" : "transparent"

                            border.color: modelData.id === assistant.activeConversationId
                                ? accentColor : "transparent"

                            RowLayout {
                                anchors.fill: parent
                                anchors.leftMargin: 8
                                anchors.rightMargin: 4
                                spacing: 2

                                Item {
                                    Layout.fillWidth: true
                                    Layout.fillHeight: true

                                    Label {
                                        anchors.fill: parent
                                        verticalAlignment: Text.AlignVCenter
                                        text: modelData.title
                                        color: textColor
                                        font.pixelSize: 11
                                        elide: Text.ElideRight
                                    }

                                    MouseArea {
                                        anchors.fill: parent
                                        enabled: assistant.connected && !assistant.busy
                                        cursorShape: Qt.PointingHandCursor
                                        onClicked: assistant.selectConversation(modelData.id)
                                    }
                                }

                                ToolButton {
                                    text: "✎"
                                    font.pixelSize: 13
                                    implicitWidth: 27
                                    implicitHeight: 30
                                    enabled: assistant.connected && !assistant.busy
                                    onClicked: {
                                        window.selectedConversationId = modelData.id
                                        window.selectedConversationTitle = modelData.title
                                        renameField.text = modelData.title
                                        renameDialog.open()
                                    }
                                    ToolTip.visible: hovered
                                    ToolTip.text: "Rename"
                                }

                                ToolButton {
                                    text: "×"
                                    font.pixelSize: 17
                                    implicitWidth: 25
                                    implicitHeight: 30
                                    enabled: assistant.connected && !assistant.busy
                                    onClicked: {
                                        window.selectedConversationId = modelData.id
                                        window.selectedConversationTitle = modelData.title
                                        deleteDialog.open()
                                    }
                                    ToolTip.visible: hovered
                                    ToolTip.text: "Delete"
                                }
                            }
                        }

                        Label {
                            anchors.centerIn: parent
                            visible: conversationList.count === 0
                            text: assistant.connected
                                ? "No conversations yet"
                                : "Waiting for assistant..."
                            color: mutedColor
                            font.pixelSize: 11
                        }
                    }

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
                        anchors.leftMargin: 24
                        anchors.rightMargin: 24
                        anchors.bottomMargin: 24
                        anchors.topMargin: assistant.toolEvents.length > 0 ? 82 : 24
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

                                RowLayout {
                                    Layout.fillWidth: true

                                    Label {
                                        Layout.fillWidth: true
                                        text: modelData.role === "user" ? "YOU" : "LOCAL ASSISTANT"
                                        color: modelData.role === "user" ? accentColor : "#8ce0b5"
                                        font.pixelSize: 10
                                        font.bold: true
                                    }

                                    ToolButton {
                                        visible: modelData.role === "assistant" && modelData.content.length > 0
                                        text: "Copy"
                                        font.pixelSize: 11
                                        implicitHeight: 28

                                        onClicked: {
                                            messageText.selectAll()
                                            messageText.copy()
                                            messageText.deselect()
                                        }

                                        ToolTip.visible: hovered
                                        ToolTip.text: "Copy response"
                                    }
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

                                    TextEdit {
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
                                        wrapMode: TextEdit.Wrap
                                        textFormat: TextEdit.PlainText
                                        readOnly: true
                                        selectByMouse: true
                                        selectByKeyboard: true
                                        persistentSelection: true
                                    }
                                }
                            }
                        }

                        onCountChanged: Qt.callLater(function() {
                            positionViewAtEnd()
                        })
                    }

                    Rectangle {
                        anchors.top: parent.top
                        anchors.left: parent.left
                        anchors.right: parent.right
                        anchors.topMargin: 12
                        anchors.leftMargin: 24
                        anchors.rightMargin: 24
                        height: 58
                        radius: 9
                        color: "#202630"
                        border.color: borderColor
                        visible: assistant.toolEvents.length > 0
                        z: 2

                        Column {
                            anchors.fill: parent
                            anchors.leftMargin: 12
                            anchors.rightMargin: 12
                            anchors.topMargin: 7
                            anchors.bottomMargin: 6
                            spacing: 3

                            Label {
                                text: "TOOL ACTIVITY"
                                color: mutedColor
                                font.pixelSize: 9
                                font.bold: true
                            }

                            Label {
                                width: parent.width
                                elide: Text.ElideRight
                                color: textColor
                                font.pixelSize: 11
                                text: {
                                    const event = assistant.toolEvents[assistant.toolEvents.length - 1]
                                    if (!event)
                                        return ""

                                    let action = "Selected"
                                    if (event.event === "tool.started")
                                        action = "Running"
                                    else if (event.event === "tool.completed")
                                        action = "Completed"
                                    else if (event.event === "tool.failed")
                                        action = "Failed"

                                    return action + ": " + event.tool +
                                        "  ·  Step " + event.callIndex
                                }
                            }
                        }
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
                                visible: !assistant.busy
                                text: "Send"
                                enabled: assistant.connected
                                onClicked: {
                                    assistant.sendMessage(messageInput.text)
                                    messageInput.clear()
                                }
                            }

                            Button {
                                Layout.preferredWidth: 92
                                Layout.fillHeight: true
                                visible: assistant.busy
                                text: "Stop"
                                enabled: assistant.connected
                                onClicked: assistant.cancelGeneration()
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


    Dialog {
        id: renameDialog
        title: "Rename conversation"
        modal: true
        anchors.centerIn: parent
        standardButtons: Dialog.Ok | Dialog.Cancel

        onAccepted: assistant.renameConversation(
            window.selectedConversationId,
            renameField.text
        )

        contentItem: ColumnLayout {
            width: 340

            Label {
                text: "Enter a new name (up to 100 characters):"
                color: window.textColor
                wrapMode: Text.Wrap
            }

            TextField {
                id: renameField
                Layout.fillWidth: true
                maximumLength: 100
                selectByMouse: true
                placeholderText: "Conversation name"
                onAccepted: renameDialog.accept()
            }
        }
    }

    Dialog {
        id: deleteDialog
        title: "Delete conversation"
        modal: true
        anchors.centerIn: parent
        standardButtons: Dialog.Yes | Dialog.No

        onAccepted: assistant.deleteConversation(
            window.selectedConversationId
        )

        contentItem: Label {
            width: 340
            text: "Delete \"" + window.selectedConversationTitle +
                  "\" and all its saved messages? This cannot be undone."
            color: window.textColor
            wrapMode: Text.Wrap
        }
    }


    Dialog {
        id: settingsDialog
        title: "Settings"
        modal: true
        anchors.centerIn: parent
        width: 420
        standardButtons: Dialog.Ok | Dialog.Cancel

        onOpened: assistant.loadSettings()

        onAccepted: assistant.setContextLengthTokens(
            window.draftContextLength
        )

        contentItem: ColumnLayout {
            spacing: 14

            Label {
                text: "Context length"
                color: window.textColor
                font.pixelSize: 15
                font.bold: true
            }

            Label {
                text: "Controls how much conversation context the model can use."
                color: window.mutedColor
                wrapMode: Text.Wrap
                Layout.fillWidth: true
            }

            Label {
                text: Math.round(window.draftContextLength / 1024) + "K tokens"
                color: window.accentColor
                font.pixelSize: 22
                font.bold: true
            }

            Slider {
                Layout.fillWidth: true
                from: 4096
                to: 16384
                stepSize: 1024
                snapMode: Slider.SnapAlways
                value: window.draftContextLength

                onMoved: {
                    window.draftContextLength =
                        Math.round(value / 1024) * 1024
                }
            }

            RowLayout {
                Layout.fillWidth: true

                Label {
                    text: "4K"
                    color: window.mutedColor
                }

                Item { Layout.fillWidth: true }

                Label {
                    text: "16K"
                    color: window.mutedColor
                }
            }

            Label {
                text: "Changes are saved when you press OK and affect the next response."
                color: window.mutedColor
                wrapMode: Text.Wrap
                Layout.fillWidth: true
            }
        }
    }


}
