# Local Assistant 2.0

A locally hosted, privacy-focused AI desktop assistant built around a
provider-neutral Rust core and a native Qt/QML desktop application.

## Current Development Status

**Phase 1 — First working desktop prototype**

Currently working:
- Rust assistant core.
- Ollama inference integration.
- `assistant-qwen:latest` as the selected primary model.
- 16,384-token context successfully tested.
- Unix-domain socket IPC using JSON-RPC 2.0 over NDJSON.
- Streaming responses from the Rust core.
- Conversation creation and independent conversation IDs.
- Initial Qt 6/QML desktop interface.
- C++ desktop-to-core bridge with JSON-RPC request tracking.

Not yet implemented:
- Persistent conversation storage.
- Conversation list, renaming, and deletion in the UI.
- User memory and preference management.
- Screen Awareness Engine.
- Adaptive skill development.
- Model manager and complete settings interface.
- Windows desktop implementation.

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
and future agent capabilities. The desktop application is a client of the core.

## Project Principles

- Independent implementation; no dependency on the previous project.
- Local-first operation and privacy.
- Provider-neutral inference architecture.
- Clear separation between core, desktop, and platform-specific services.
- Streaming and cancellation as first-class capabilities.
- User-controlled memory, personalization, and permissions.
- No telemetry or automatic diagnostic uploads.
- Safety and permission boundaries must not be bypassed by adaptive features.

## Development Roadmap

1. Complete the Linux desktop application on CachyOS.
2. Implement persistent conversations, settings, memory, and model management.
3. Build the Screen Awareness Engine and agent capabilities.
4. Stabilize the Linux product and create its installer.
5. Develop the Windows desktop implementation and installer.
6. Consider a separate AI-integrated CachyOS-derived OS product after both
   desktop products are complete.

See `docs/PROJECT_PLAN.md` and `docs/ARCHITECTURE.md`.
