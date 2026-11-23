# Local Assistant 2.0

A locally hosted, privacy-focused AI desktop assistant built around a
provider-neutral Rust core and a native Qt/QML desktop application.

## Current Development Status

**Linux desktop prototype — persistent conversations and initial agent/tool system working.**

Verified foundations:
- Rust assistant daemon and Ollama inference integration.
- Primary model: `assistant-qwen:latest`.
- 16,384-token context tested; configurable 4K–16K context setting.
- Unix-domain socket IPC using JSON-RPC 2.0 over NDJSON.
- Local client authentication using Linux peer credentials.
- Streaming responses and task cancellation infrastructure.
- SQLite conversation and message persistence.
- Desktop conversation creation, listing, selection, renaming, and deletion.
- Qt 6/QML interface with C++ IPC bridge.
- Agent registry with General, Research, Coding, and System definitions.
- Shared runtime tool registry with strict argument-schema validation.
- Calculator tool successfully executed through the live desktop-to-daemon agent path.
- Per-agent capability boundaries and request/task context propagation.

The initial agent system is operational, but multi-agent coordination and
specialist delegation are not yet implemented.

See `docs/DEVELOPMENT_STATUS.md` for the detailed status.

## Architecture

- **Core:** Rust
- **Desktop:** Qt 6, C++, and QML
- **Initial inference provider:** Ollama
- **IPC:** JSON-RPC 2.0 over newline-delimited JSON
- **Linux transport:** Unix-domain socket
- **Initial platform:** CachyOS / Linux
- **Primary model:** `assistant-qwen:latest`
- **Default context:** 16K tokens

Rust owns application state, conversation orchestration, inference coordination,
permissions, and agent execution. The desktop application is a client of the core.

## Project Principles

- Independent implementation; no dependency on the previous project.
- Local-first operation and privacy.
- Provider-neutral inference architecture.
- Clear separation between core, desktop, and platform-specific services.
- Streaming and cancellation as first-class capabilities.
- User-controlled memory, personalization, and permissions.
- No telemetry or automatic diagnostic uploads.
- Safety and permission boundaries must not be bypassed by agents or adaptive features.

## Development Roadmap

1. Complete the Linux desktop application on CachyOS.
2. Finish persistence, settings, model management, memory, and reliability.
3. Build the Screen Awareness Engine and coordinated agent/skill system.
4. Stabilize the Linux product and create its installer.
5. Develop the Windows desktop implementation and installer.
6. Consider a separate AI-integrated CachyOS-derived OS product only after both
   desktop products are complete.

See `docs/PROJECT_PLAN.md` and `docs/ARCHITECTURE.md`.
