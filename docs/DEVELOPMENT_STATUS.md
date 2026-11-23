# Local Assistant 2.0 — Development Status

Last updated: 2026-11-23

## Current milestone

The Linux prototype has a working Rust daemon, Qt/QML desktop client,
persistent conversations, local inference, and an initial agent/tool
execution pipeline.

The current focus is to turn the initial agent/tool foundation into a
reusable, extensible system before expanding desktop features.

## Implemented

### Core and inference

- Rust daemon and provider abstraction.
- Ollama inference integration.
- Selected model: `assistant-qwen:latest`.
- 16K default context, configurable from 4K to 16K.
- Context manager with mandatory-message protection and history budgeting.
- Planner prompt budgeting.
- SQLite conversation and task-context persistence.

### IPC and desktop

- JSON-RPC 2.0 over NDJSON.
- Linux Unix-domain socket transport.
- Local peer-credential authentication.
- Streaming generation notifications.
- Task cancellation infrastructure.
- Qt 6/QML desktop and C++ IPC bridge.
- Conversation management.
- Tool activity events and cancellation controls.

### Agents and tools

- Agent registry with General, Research, Coding, and System definitions.
- Shared runtime tool registry.
- Tool metadata and JSON argument schemas.
- Strict recursive argument validation.
- Capability checks before execution.
- Structured tool results and execution logging.
- Planner decisions between direct responses and tool execution.
- Multi-step planning using previous tool results.
- Duplicate tool-call protection.
- Maximum of four tool calls per turn.
- Bounded tool output passed back into planning.
- Cancellation checks between planning and tool execution.

### Built-in tools

- Calculator.
- `system_info`: read-only operating-system, architecture, hostname,
  and memory information.
- `workspace_list`: lists entries inside the configured workspace.
- `workspace_read`: reads UTF-8 text files inside the configured workspace.

Workspace access currently uses `LOCAL_ASSISTANT_WORKSPACE`, captured when
the daemon initializes its built-in tools.

Workspace access is read-only. Paths must be relative and remain inside
the canonical workspace root. Symlinks are not followed during directory
listing, sensitive paths are blocked, file reads are limited to 64 KiB,
and directory listings are limited to 100 entries.

## Current limitations

- Workspace selection is environment-configured rather than selected
  through the desktop and propagated with each request.
- General, Research, Coding, and System definitions exist, but a complete
  coordinator and specialist delegation lifecycle are not implemented.
- Tool handlers are synchronous; cancellation cannot interrupt a handler
  already executing.
- The inference provider's JSON planning call is blocking.
- Complete worker supervision, isolation, and resource limits remain future
  work.
- IPC reconnect and lifecycle recovery need further hardening.
- User memory and adaptive profile are not implemented.
- Screen Awareness Engine is not implemented.
- Windows transport and platform providers are not implemented.

## Current development environment

- CachyOS / KDE Plasma.
- Rust core.
- Qt 6 desktop.
- Ollama inference.
- AMD CPU, 16 GB DDR5 RAM, NVIDIA RTX 3050 4 GB.
- Primary model: `assistant-qwen:latest`.

## Immediate next steps

1. Propagate workspace selection from the desktop through IPC and request
   context into tool execution.
2. Improve agent-specific routing and specialist workflows.
3. Add useful coding-oriented project inspection tools.
4. Add research capabilities behind explicit provider and permission
   boundaries.
5. Continue toward a coordinator and delegated-task lifecycle.
6. Improve execution supervision, cancellation, and isolation.
7. Continue integrating the growing tool system into the desktop.
