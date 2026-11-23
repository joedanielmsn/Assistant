# Local Assistant 2.0 — Project Plan

## 1. Product Vision

Build a capable, locally hosted AI desktop assistant with a natural
personality, useful desktop awareness, extensible skills, and user-controlled
personalization.

The assistant should work primarily on the user's own machine. Its core
should remain independent of any one inference provider or desktop interface.

## 2. Product Principles

- Local-first, privacy-focused operation.
- Fresh, independent implementation.
- Rust core is authoritative for application state and orchestration.
- Native desktop experience using Qt and QML.
- Provider-neutral inference architecture.
- Modular platform-specific services.
- Streaming, cancellation, and reliable task management.
- Transparent permissions and user control.
- No telemetry or automatic diagnostic uploads.
- Adaptive features must not override safety or permission boundaries.

Encryption is intentionally deferred until late development or a later
update. Architecture should avoid preventing its future addition.

## 3. Product Sequence

### Product A — Linux Desktop

Complete the CachyOS/Linux desktop product first.

Deliverables:

- Rust assistant core.
- Qt/QML desktop application.
- Local inference and model management.
- Conversation management and persistence.
- User memory and personalization.
- Screen Awareness Engine.
- Agent and skill system.
- Settings and permission controls.
- Linux packaging and installer.

### Product B — Windows Desktop

After the Linux desktop product is complete:

- Implement Windows platform services.
- Preserve shared architecture and product behaviour where practical.
- Implement Windows IPC and desktop-awareness providers.
- Build and test a Windows installer.

### Product C — AI-integrated Operating System

Only after both desktop products are complete, consider a separate
AI-integrated CachyOS-derived OS product.

Keep the desktop assistant independent from OS integration. Do not require
an independently hosted package repository or server for the initial OS
product.

## 4. Development Phases

### Phase 1 — Working Core and Desktop Prototype

Status: **Working prototype established.**

Completed:

- Fresh Rust project.
- Ollama provider integration.
- Primary model selected.
- 16K context tested successfully.
- JSON-RPC 2.0 / NDJSON IPC over Unix-domain sockets.
- Local peer authentication.
- Request validation and bounded frame handling.
- Persistent SQLite conversations and task context.
- Streaming response notifications.
- Core task cancellation infrastructure.
- Qt/QML desktop shell and C++ IPC bridge.
- Conversation management.
- Context-length settings and persistence.

Remaining:

- Complete IPC reconnect and disconnect recovery.
- Expand lifecycle and failure-path integration tests.

### Phase 2 — Reusable Tool System

Status: **Initial implementation working; expansion in progress.**

Completed foundations:

- Runtime tool registry.
- Built-in calculator.
- Tool metadata and JSON argument schemas.
- Strict recursive schema validation.
- Capability-checked tool execution.
- Structured execution status, output, errors, and duration.
- Read-only system diagnostics.
- Read-only workspace listing and text-file reading.
- Workspace path restrictions and bounded output.

Next:

- Standardize tool execution context and lifecycle.
- Add desktop-selected workspace propagation.
- Add project inspection and code-reading tools.
- Add safe, permission-aware file modification workflows later.
- Add provider-backed research tools.
- Keep arbitrary shell execution outside the default tool surface.

### Phase 3 — Agent Architecture

Status: **Initial planner and orchestrator implemented.**

Completed foundations:

- General, Research, Coding, and System agent definitions.
- Shared authenticated `AgentContext`.
- Planner-driven tool selection.
- Iterative multi-step tool execution.
- Tool-result feedback into subsequent planning.
- Duplicate-call protection and per-turn call limits.
- Cancellation checks between execution stages.
- Desktop tool activity events.

Next:

1. Implement Agent Coordinator.
2. Route tasks to appropriate specialists.
3. Define delegated task contracts and lifecycle.
4. Aggregate specialist results.
5. Add failure recovery and delegation limits.
6. Preserve principal identity and capability boundaries.
7. Supervise workers with resource limits and isolation.
8. Add evaluation before activating new skills.

### Phase 4 — Desktop Foundation

Status: **In progress.**

Completed foundations:

- Persistent conversation and message database.
- Conversation list, selection, renaming, and deletion.
- Persisted settings and configurable 4K–16K context length.
- Task context persistence and context assembly.
- Basic desktop chat and message-copy interaction.
- Tool activity display and cancellation control.

Next:

- Workspace selection UI and request propagation.
- Reliable reconnection and startup/shutdown handling.
- Complete settings and model manager.
- User-facing task-state improvements.
- Additional desktop and IPC reliability work.

### Phase 5 — Memory and Personalization

Planned:

1. Conversation history: messages belonging to a particular chat.
2. User memory: durable information and preferences the user allows the
   assistant to retain.
3. Adaptive profile: learned interaction preferences, with confidence
   and user controls.
4. Skills: versioned capabilities with descriptions, requirements,
   permissions, and evaluation results.

Provide ways to inspect, correct, disable, and delete retained information.

### Phase 6 — Screen Awareness Engine

Planned. Screen awareness should be enabled by default while Assistant Mode
is active.

Requirements:

- Visible active indicator.
- Immediate pause and disable controls.
- App/window exclusions and sensitive-content protections.
- No capture while Assistant Mode is inactive or paused.
- Local processing by default.
- No cloud disclosure without separate explicit authorization.
- Resource-aware context updates rather than unnecessary continuous
  heavyweight vision inference.

Initial Linux integration:

- XDG Desktop Portal and PipeWire for permission-aware capture.
- KDE/KWin for desktop and window context.
- AT-SPI for accessibility information.
- Tesseract OCR when appropriate.
- Replaceable capture-provider interface for Windows.

### Phase 7 — Linux Product Completion

Planned:

- Complete settings and user-facing controls.
- Complete Screen Awareness and agent capabilities.
- Performance and reliability stabilization.
- Essential automated tests and integration checks.
- Dependency and license inventory.
- Linux packaging and installer.
- Installation, upgrade, and troubleshooting documentation.

### Phase 8 — Windows Product

Planned:

- Windows platform providers and IPC transport.
- Desktop-awareness implementation.
- Windows packaging and installer.
- Feature parity review and testing.

### Phase 9 — AI-integrated OS Exploration

Begin only after the Linux and Windows desktop products are complete.

## 5. Hardware and Performance Baseline

Initial development target:

- x86-64-v3 baseline.
- x86-64-v4 may be an optional optimized target.
- Older x86-64-only CPUs and ARM64 are outside the initial baseline.

Current tested development machine:

- AMD CPU.
- 16 GB DDR5 RAM.
- NVIDIA RTX 3050 with 4 GB VRAM.

Primary model:

- `assistant-qwen:latest`.
- Qwen 3.5 2B Q4_K_M.
- Ollama inference.
- 16,384-token context tested with 100% GPU model placement.

Context configuration:

- Minimum: 4K.
- Default: 16K.
- Maximum: 16K.
- Adjustment step: 1K.

Retain resource checks and a safe fallback if future configurations cannot
run reliably.

## 6. Deferred Decisions

- Encryption: defer until late development or a later update.
- Release licensing: decide during release planning.
- Independent package repository/server: not required for the initial OS
  product.
- Broad test, formatting, and Clippy campaigns: expand during stabilization,
  not at the expense of reaching a working product prototype.

## 7. Reuse-First Development and Decision Approval

Local Assistant 2.0 is developed by a solo developer. We should not
reimplement mature infrastructure when an established open-source
component can provide the capability without compromising product
experience, privacy, performance, security, or maintainability.

For each major subsystem:

1. Research existing open-source applications and libraries.
2. Compare maintenance, license, platform support, resource usage, and
   integration complexity.
3. Prefer a small adapter behind our own replaceable interface.
4. Build custom functionality only when existing solutions do not fit.

Major framework, dependency, licensing, architecture, cloud-processing,
hardware-baseline, and product-direction choices require explicit user
approval after presenting alternatives and a recommendation.

Routine implementation choices within approved architecture do not need
repeated confirmation.

See `docs/MASTER_PLAN.md` and `docs/ACTION_PLAN.md`. The living open-source
technology inventory and evaluation checklist is in
`docs/INTEGRATION_CATALOG.md`.
