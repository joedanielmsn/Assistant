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
- Agent registry and orchestration.
- Tool registration, validation, and execution.
- Cancellation and future worker supervision.
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
- Linux peer-credential authentication.

Protocol:
- JSON-RPC 2.0.
- Newline-delimited JSON frames.
- Request validation and bounded frame handling.
- Streaming notifications for generation events.
- Request and task identifiers.

Initial methods include:
- `initialize`
- `conversation.create`
- Chat send and cancellation operations.

Generation notifications include:
- `chat.delta`
- `chat.completed`
- `chat.cancelled`
- `chat.failed`

Further work includes protocol version negotiation, complete error handling,
reconnection behaviour, and lifecycle hardening.

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

Streaming and cancellation are first-class requirements.

The selected context default is 16K tokens, with a configurable range
of 4K–16K.

## 6. State and Storage Boundaries

Keep the following concepts separate:

### Conversation history
- Messages and metadata for each conversation.
- Conversation creation, selection, renaming, and deletion.
- Persistent local SQLite storage.

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

## 7. Agent and Tool Architecture

The core owns a shared runtime tool registry and an agent registry.

Current agent definitions:
- General.
- Research.
- Coding.
- System.

The initial General Agent uses the inference planner to choose between
responding directly and requesting an available tool.

Tool execution:
- Tools are registered with metadata and argument schemas.
- Arguments are validated before handlers execute.
- Tool execution requires the registered capability.
- Results include structured status, output, errors, and duration.
- Execution is logged.

`AgentContext` is derived from a registered agent definition and the
authenticated request context. It carries the agent identity, original
principal, request ID, task ID, and the agent's registered capabilities.

The agent cannot grant itself additional capabilities. Delegation must
preserve the original principal and must not expand privileges.

The initial General Agent and calculator path are working. The following
remain planned:
- Agent Coordinator and specialist delegation.
- Delegated task lifecycle and result aggregation.
- Complete central authorization for consequential tools.
- Supervised, resource-bounded, isolated worker execution.

## 8. Screen Awareness Engine

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

The current permission broker and agent capability checks are foundational,
not a substitute for complete authorization and isolation of future OS tools.

## 11. Current Implementation Boundary

Implemented and verified:
- Rust core and Ollama provider.
- Unix socket IPC with local peer authentication.
- JSON-RPC request validation and bounded frames.
- Streaming generation and task cancellation infrastructure.
- Persistent SQLite conversations and task context.
- Desktop conversation management and context settings.
- Agent registry and shared runtime tool registry.
- Strict tool argument validation and structured execution results.
- General Agent context and capability enforcement.
- Live calculator execution through the desktop-to-daemon path.

Not yet implemented:
- Agent Coordinator and specialist delegation.
- Complete worker supervision and isolation.
- Complete IPC reliability and reconnection hardening.
- User memory and adaptive profile.
- Screen Awareness Engine.
- Windows transport and platform providers.
