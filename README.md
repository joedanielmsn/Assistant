# Local Assistant 2.0

A locally hosted, privacy-focused AI desktop assistant built around a
provider-neutral Rust core and a native Qt/QML desktop application.

## Current Development Status

**Linux desktop prototype — persistent conversations, streaming inference,
and the initial iterative agent/tool execution system working.**

Verified foundations:

- Rust assistant daemon and Ollama inference integration.
- Primary model: `assistant-qwen:latest` (Qwen 3.5 2B Q4_K_M).
- 16,384-token context tested; configurable 4K–16K context setting.
- Unix-domain socket IPC using JSON-RPC 2.0 over NDJSON.
- Local client authentication using Linux peer credentials.
- Streaming responses and task cancellation infrastructure.
- SQLite conversation storage and task context.
- Qt 6/QML desktop interface with C++ IPC bridge.
- Agent registry: General, Research, Coding, and System.
- Shared runtime tool registry with strict argument-schema validation.
- Capability-checked tool execution and structured execution results.
- Planner-driven, multi-step tool execution with duplicate-call protection.
- Tool execution notifications in the desktop interface.
- Built-in calculator, read-only system diagnostics, and workspace tools.
- Workspace file access restricted to a configured root directory.
- Context budgeting for generation and tool planning.

The current system provides the foundations for specialist agents. Full
multi-agent delegation, specialist coordination, and supervised worker
isolation remain future work.

See:
- `docs/DEVELOPMENT_STATUS.md`
- `docs/ARCHITECTURE.md`
- `docs/PROJECT_PLAN.md`

## Architecture

- **Core:** Rust
- **Desktop:** Qt 6, C++, and QML
- **Initial inference provider:** Ollama
- **IPC:** JSON-RPC 2.0 over newline-delimited JSON
- **Linux transport:** Unix-domain socket
- **Initial platform:** CachyOS / Linux
- **Primary model:** `assistant-qwen:latest`
- **Default context:** 16K tokens

Rust owns application state, conversation orchestration, inference
coordination, permissions, and agent execution. The desktop application
is a client of the core.

## Project Principles

- Independent implementation; no dependency on the previous project.
- Local-first operation and privacy.
- Provider-neutral inference architecture.
- Clear separation between core, desktop, and platform-specific services.
- Streaming and cancellation as first-class capabilities.
- User-controlled memory, personalization, and permissions.
- No telemetry or automatic diagnostic uploads.
- Safety and permission boundaries must not be bypassed by agents or
  adaptive features.

## Development Roadmap

1. Complete the reusable tool system and agent execution architecture.
2. Integrate workspace selection and specialist workflows into the desktop.
3. Complete desktop settings, memory, model management, and reliability.
4. Build the Screen Awareness Engine and coordinated agent/skill system.
5. Stabilize the Linux product and create its installer.
6. Develop the Windows desktop implementation and installer.
7. Consider a separate AI-integrated CachyOS-derived OS product only after
   both desktop products are complete.

See `docs/PROJECT_PLAN.md` and `docs/ARCHITECTURE.md`.

## Planning Documents

- `docs/MASTER_PLAN.md` — long-term vision, architecture, product roadmap,
  and reuse-first development principles.
- `docs/ACTION_PLAN.md` — implementation milestones, immediate work queue,
  acceptance criteria, and approval gates.
- `docs/PROJECT_PLAN.md` — product phases and development sequence.
- `docs/DEVELOPMENT_STATUS.md` — current implementation status.
- `docs/INTEGRATION_CATALOG.md` — researched open-source candidates across the assistant, desktop, AI, security, and eventual OS stack, with evaluation criteria and sequencing.
