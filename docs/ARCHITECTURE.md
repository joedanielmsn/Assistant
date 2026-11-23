# Local Assistant 2.0 — Architecture

## 1. Architectural Overview

The project is divided into a Rust core, a Qt/QML desktop client, and
replaceable platform and inference providers.

The Rust core is authoritative. The desktop client presents the interface
and communicates with the core through a documented local protocol.

## 2. Rust Core

Responsibilities:
- Application identity and configuration.
- Conversation and message state.
- Context assembly.
- Inference coordination.
- Agent and task orchestration.
- Cancellation and worker supervision.
- Persistence and memory services.
- Permission and policy enforcement.

The core must not depend on the desktop UI to maintain authoritative state.

## 3. Desktop Application

Technology:
- Qt 6.
- C++ integration layer.
- QML interface.

Responsibilities:
- Chat and conversation presentation.
- User input and interaction.
- Settings and status presentation.
- Display of permissions and awareness state.
- Sending commands to the core over IPC.

The UI must not independently own authoritative conversation history,
inference state, or agent permissions.

## 4. Local IPC

Initial Linux transport:
- Unix-domain socket.
- Socket location under `$XDG_RUNTIME_DIR/local-assistant/`.
- Private runtime directory and socket permissions.

Protocol:
- JSON-RPC 2.0.
- Newline-delimited JSON frames.
- Streaming notifications for generation events.

Initial methods:
- `initialize`
- `conversation.create`
- `chat.send`
- `chat.cancel`

Generation notifications:
- `chat.delta`
- `chat.completed`
- `chat.cancelled`
- `chat.failed`

The protocol should evolve with explicit request validation, structured errors,
request/task identifiers, and versioned capability negotiation.

Future Windows transport:
- Replace the Linux socket transport with a Windows named-pipe provider.
- Keep the application-level protocol independent of the transport.

## 5. Inference Architecture

Use a provider-neutral Rust interface.

Initial provider:
- Ollama.

Initial model:
- `assistant-qwen:latest`.

The internal architecture should not assume Ollama-specific message or
generation behaviour outside its adapter.

Potential future providers include llama.cpp and compatible local inference
services.

Streaming and cancellation are first-class requirements.

The selected context default is 16K tokens, with a planned user-configurable
range of 4K–16K.

## 6. State and Storage Boundaries

Keep the following concepts separate:

### Conversation history
- Messages and metadata for each conversation.
- Conversation creation, selection, renaming, and deletion.
- Persistent local storage.

### User memory
- Durable user-approved facts and preferences.
- Retrieval when relevant.
- Inspection, correction, and deletion controls.

### Adaptive profile
- Learned interaction preferences.
- Confidence and evidence tracking.
- Distinguish explicit preferences from inferred patterns.

### Skills
- Versioned capabilities and workflows.
- Tool schemas and permission requirements.
- Evaluation status and activation history.

These systems may share storage infrastructure, but must have distinct
schemas, retention rules, and user controls.

## 7. Screen Awareness Engine

Screen awareness is a core assistant-mode capability and should be enabled
by default while Assistant Mode is active.

The engine should maintain useful current desktop context without requiring
heavyweight visual inference on every frame.

Preferred information sources:
- Window and application metadata.
- Accessibility information.
- Event-driven context changes.
- OCR or visual processing when needed.

Linux providers:
- XDG Desktop Portal.
- PipeWire.
- KDE/KWin.
- AT-SPI.
- Tesseract OCR.

Privacy requirements:
- Visible active state.
- Immediate pause and disable.
- App/window exclusions.
- Sensitive-content protections.
- No capture outside active, unpaused Assistant Mode.
- Local processing by default.

Keep platform capture providers replaceable for Windows.

## 8. Agent and Worker Architecture

The core supervises agent tasks and workers.

Requirements:
- Structured tool schemas.
- Strict argument validation.
- Explicit environment allowlists.
- Resource bounds.
- Cancellation and process-tree cleanup.
- Permission checks before consequential actions.
- Isolation where appropriate.

Investigate systemd user services, Bubblewrap, and Landlock.

## 9. Adaptive Evolution

Personality and skill adaptation must remain separate from core identity,
safety rules, and permission enforcement.

The assistant may learn preferred tone, response length, workflows, and
repeated task patterns.

New skills must be evaluated before activation. New permissions and
consequential capabilities require appropriate user approval.

Do not autonomously rewrite core application code or modify model weights.

See `ADAPTIVE_EVOLUTION.md`.

## 10. Security and Privacy

- Local-first processing.
- No telemetry or automatic diagnostic uploads.
- Explicit permission boundaries.
- Visible controls for sensitive capabilities.
- Isolated worker execution where appropriate.
- Encryption intentionally deferred; preserve future integration boundaries.

## 11. Current Implementation Boundary

Implemented prototype:
- Rust core and Ollama provider.
- Unix socket IPC.
- JSON-RPC request/notification flow.
- Streaming generation.
- Conversation creation and in-memory conversation state.
- Initial Qt/QML client and C++ bridge.

Not yet implemented:
- Persistent storage.
- Full conversation navigation.
- User memory.
- Screen awareness.
- Adaptive skill activation.
- Worker isolation.
- Windows transport and providers.
