# Local Assistant 2.0 — Architecture

## 1. Architectural Overview

The project is divided into a Rust core, a Qt/QML desktop client, and
replaceable platform and inference providers.

The Rust core is authoritative. The desktop client presents the interface
and communicates with the core through a documented local protocol.

The implementation is a fresh, independent codebase. The previous project
is reference material only.

## 2. Rust Core

Responsibilities:

- Application identity and configuration.
- Conversation and message state.
- Context assembly and budgeting.
- Inference coordination.
- Agent registry and orchestration.
- Tool registration, validation, and execution.
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
- Display of tool activity and permissions.
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

Generation notifications include:

- `chat.delta`
- `chat.completed`
- `chat.cancelled`
- `chat.failed`
- `tool.event`

Task cancellation is supported through the chat cancellation operation.

Future Windows transport will use a named-pipe provider while keeping the
application-level protocol independent of transport.

## 5. Inference and Context Architecture

Use a provider-neutral Rust interface.

Initial provider: Ollama.

Initial model: `assistant-qwen:latest`.

The selected context default is 16K tokens, with a configurable range of
4K–16K.

The context manager protects mandatory system instructions and the current
user request while budgeting optional history and task context.

The planner also checks its prompt budget before requesting a tool decision.

Token estimation is currently heuristic rather than an exact model tokenizer.
Provider-specific message and generation behaviour should remain inside
provider adapters.

## 6. State and Storage Boundaries

Keep these concepts separate:

### Conversation history

Messages and metadata belonging to a particular chat, persisted locally.

### User memory

Durable information and preferences the user allows the assistant to retain.

### Adaptive profile

Learned interaction preferences with confidence and evidence tracking.

### Skills

Versioned capabilities and workflows with schemas, permission requirements,
and evaluation status.

These systems may share storage infrastructure but must have distinct
schemas, retention rules, and user controls.

## 7. Agent and Tool Architecture

The core owns a shared runtime tool registry and an agent registry.

Current agent definitions:

- General.
- Research.
- Coding.
- System.

### Tool registry

Tools are registered with:

- Unique names.
- Descriptions and versions.
- JSON argument schemas.
- Required capabilities.
- Synchronous execution handlers.

The registry validates arguments before invoking handlers, checks the
execution capability, and returns structured results containing status,
output, errors, and duration.

### Agent context

`AgentContext` is derived from a registered agent definition and the
authenticated request context. It carries agent identity, original
principal, request ID, task ID, and registered capabilities.

An agent cannot grant itself additional capabilities. Delegation must
preserve the original principal and must not expand privileges.

### Planner and orchestrator

The planner chooses between answering directly and requesting an available
tool. The orchestrator executes approved tool calls and feeds bounded,
structured results back into subsequent planning decisions.

Current execution safeguards include:

- Maximum of four tool calls per turn.
- Duplicate name-and-argument call protection.
- Bounded tool output.
- Cancellation checks between planning and execution.
- Structured tool lifecycle notifications.

### Built-in tools

- Calculator.
- `system_info` for read-only system diagnostics.
- `workspace_list` for read-only directory listing.
- `workspace_read` for bounded UTF-8 text-file reading.

Workspace tools currently use `LOCAL_ASSISTANT_WORKSPACE`, captured when
the daemon initializes its built-in registry.

Workspace path handling uses a canonical root, relative paths, traversal
rejection, symlink escape checks, sensitive-path restrictions, a 64 KiB
file-read limit, and a 100-entry directory-listing limit.

The workspace tools do not provide arbitrary shell execution or file
modification.

### Planned agent capabilities

- Desktop-selected workspace propagation.
- Agent Coordinator and specialist delegation.
- Coding and research workflows.
- Delegated task lifecycle and result aggregation.
- Central authorization for consequential tools.
- Supervised, resource-bounded, isolated worker execution.

## 8. Screen Awareness Engine

Screen awareness is a core assistant-mode capability.

The engine should maintain useful current desktop context without requiring
heavyweight visual inference on every frame.

Preferred information sources:

- Window and application metadata.
- Accessibility information.
- Event-driven context changes.
- OCR or visual processing when needed.

Initial Linux providers:

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

Capture providers must remain replaceable for Windows.

## 9. Adaptive Evolution

Personality and skill adaptation must remain separate from core identity,
safety rules, and permission enforcement.

The assistant may learn preferred tone, response length, workflows, and
repeated task patterns.

New skills must be evaluated before activation. New permissions and
consequential capabilities require appropriate user approval.

Do not autonomously rewrite core application code or modify model weights.

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

Implemented:

- Rust core and Ollama provider.
- Unix socket IPC with local peer authentication.
- JSON-RPC request validation and bounded frames.
- Streaming generation and task cancellation infrastructure.
- Persistent SQLite conversations and task context.
- Desktop conversation management and context settings.
- Agent registry and shared runtime tool registry.
- Strict tool argument validation and structured execution results.
- Planner-driven iterative tool execution.
- Calculator and read-only system/workspace tools.
- Desktop tool activity events.

Not yet implemented:

- Desktop workspace selection and request-level workspace propagation.
- Complete Agent Coordinator and specialist delegation.
- Complete worker supervision and isolation.
- Fully interruptible tool handlers and planning calls.
- IPC reconnect and lifecycle hardening.
- User memory and adaptive profile.
- Screen Awareness Engine.
- Windows transport and platform providers.
