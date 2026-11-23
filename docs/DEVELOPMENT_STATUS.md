# Development Status

Last updated: 2026-11-23

## Current Milestone

Persistent Linux desktop prototype with a working initial agent/tool execution
path. Multi-agent coordination is the next architecture milestone.

## Completed and Verified

### Project foundation
- Independent project created at `~/Project_Assistant`.
- Rust core crate and Qt desktop application.
- Qt 6.12 development environment verified.

### Inference
- Provider interface and Ollama integration.
- Primary model: `assistant-qwen:latest` (Qwen 3.5 2B Q4_K_M).
- 8K, 12K, and 16K context tests completed.
- 16K selected as the default; configurable 4K–16K range.
- Model placement reported as 100% GPU during testing.

### Core, persistence, and IPC
- Rust daemon with Unix-domain socket IPC.
- JSON-RPC 2.0 over NDJSON.
- Local client authentication using Linux peer credentials.
- Request validation and bounded frame handling.
- Request and task identifiers propagated through generation.
- SQLite conversation and message persistence.
- Conversation loading and history assembly.
- Task context persistence and context assembly.
- Streaming generation notifications.
- Core task cancellation infrastructure.
- Background task analysis and generation lifecycle logging.
- Permission broker and capability definitions.

### Desktop
- Qt 6/QML application and C++ local-socket bridge.
- Conversation creation and independent conversation IDs.
- Conversation list, selection, renaming, and deletion.
- Settings interface with persisted context-length selection.
- Chat message text selection and assistant-message copying.
- Successful GUI-to-core chat and multi-conversation use.

### Agent and tool foundation
- Runtime `ToolRegistry` with built-in calculator.
- Tool metadata, registration, and discovery.
- Recursive JSON argument-schema validation.
- Required fields, types, nested objects, arrays, and boundary validation.
- Tool capability checks and structured execution results.
- Execution status, duration, and logging.
- Agent definitions and runtime registry.
- Built-in General, Research, Coding, and System agent definitions.
- `AgentContext` inherits authenticated principal, request ID, task ID,
  and registered agent capabilities.
- General Agent is connected to the live generation path.
- Agent capabilities are checked before planning and used for tool execution.
- 34 focused agent tests passed.
- Live desktop-to-daemon calculator execution succeeded.

## Current Limitations and Work in Progress

### Agent architecture
- General Agent currently acts as the entry point.
- Specialist definitions are registered but do not yet have independent
  specialist execution workflows.
- Agent Coordinator and delegation protocol are not implemented.
- Result aggregation and delegated-task lifecycle are not implemented.
- Tool permissions are capability-scoped, but consequential OS tools still
  require the complete central authorization and isolation design.

### Core and desktop reliability
- Full IPC hardening and reconnect/disconnect recovery remain.
- A user-facing cancellation control remains to be added.
- More integration testing is required for task lifecycle and failure cases.
- Workspace-aware context integration remains incomplete.

### Not yet implemented
- Complete user memory and adaptive profile management.
- Screen Awareness Engine.
- Supervised and isolated worker execution.
- Complete model manager.
- Windows implementation and installer.
- Linux release packaging and installer.

## Immediate Next Steps

1. Build the Agent Coordinator and delegation protocol.
2. Preserve identity, task context, and capability boundaries across delegation.
3. Add structured specialist results, aggregation, and failure handling.
4. Integrate the complete permission-broker path before adding consequential tools.
5. Continue desktop and IPC reliability work.
6. Bring Assistant Mode and the Screen Awareness Engine into the desktop product
   with visible state and user controls.

## Current Development Environment

- OS: CachyOS Linux.
- Desktop: KDE Plasma on Wayland.
- Rust and Cargo: 1.100.0.
- Qt: 6.12.0.
- CMake: 4.5.1.
- Ninja: 1.13.2.
- Initial inference backend: Ollama.
- Primary model: `assistant-qwen:latest`.
- Tested hardware: AMD CPU, 16 GB DDR5 RAM, NVIDIA RTX 3050 4 GB.

## Git Checkpoint

The persistent-core checkpoint is preserved on
`checkpoint/linux-core-2026-11-23`.

The agent/tool implementation is being developed separately on
`feature/agent-tool-system`.
