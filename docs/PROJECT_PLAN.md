# Local Assistant 2.0 — Project Plan

## 1. Product Vision

Build a capable, locally hosted AI desktop assistant with a natural personality,
useful desktop awareness, extensible skills, and user-controlled personalization.

The assistant should work primarily on the user's own machine. Its core should
remain independent of any one inference provider or desktop interface.

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

Encryption is intentionally deferred until late development or a later update.
Architecture should avoid preventing its future addition.

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
- Rebuild platform-specific services for Windows.
- Preserve shared architecture and product behaviour where practical.
- Implement Windows IPC and desktop-awareness providers.
- Build and test a Windows installer.

The Windows application is a separate platform implementation, not a
requirement to share one binary with Linux.

### Product C — AI-integrated Operating System

Only after both desktop products are complete, consider a separate
AI-integrated CachyOS-derived OS product.

Initial direction:
- Build on upstream CachyOS/Arch and existing repositories.
- Keep the desktop assistant independent from OS integration.
- Do not require an independently hosted package repository or server
  for the initial OS product.

## 4. Development Phases

### Phase 1 — Working Core and Desktop Prototype

Status: Core prototype and initial GUI working.

Completed:
- Fresh Rust project.
- Ollama provider integration.
- Primary model selected.
- 16K context benchmark completed successfully.
- JSON-RPC 2.0 / NDJSON IPC.
- Unix-domain socket server.
- Conversation creation.
- Streaming response notifications.
- Chat cancellation support in the core IPC design.
- Initial Qt/QML desktop shell.
- C++ IPC client bridge.
- JSON-RPC request tracking.
- Basic new-conversation flow.

Remaining:
- Improve error and disconnect handling.
- Add a visible cancellation control.
- Verify conversation isolation and multi-turn history.
- Add persistent conversation storage.

### Phase 2 — Desktop Foundation

- Persistent conversation database.
- Conversation list, selection, renaming, and deletion.
- Reliable reconnection and startup/shutdown handling.
- Settings architecture and persistence.
- Model manager.
- Configurable context length from 4K to 16K, in 1K increments.
- Clear generation status and cancellation controls.
- Better message rendering and conversation navigation.

### Phase 3 — Memory and Personalization

Keep these systems separate:

1. Conversation history: messages belonging to a particular chat.
2. User memory: durable information and preferences the user allows the
   assistant to retain.
3. Adaptive profile: learned interaction preferences, with confidence and
   user controls.
4. Skills: versioned capabilities with descriptions, requirements,
   permissions, and evaluation results.

Provide ways to inspect, correct, disable, and delete retained information.

### Phase 4 — Screen Awareness Engine

Screen awareness should be enabled by default while Assistant Mode is active.

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

### Phase 5 — Agent and Skill System

- Structured tool definitions and validation.
- Task and worker supervision.
- Explicit permission boundaries.
- Cancellation and cleanup of worker processes.
- Resource limits and isolation.
- Skill evaluation before activation.
- User approval for new permissions and consequential capabilities.

Investigate systemd user services, Bubblewrap, and Landlock as appropriate.

### Phase 6 — Linux Product Completion

- Complete settings and user-facing controls.
- Performance and reliability stabilization.
- Essential automated tests and integration checks.
- Dependency and license inventory.
- Linux packaging and installer.
- Documentation for installation, upgrades, and troubleshooting.

### Phase 7 — Windows Product

- Windows platform providers and IPC transport.
- Desktop-awareness implementation.
- Windows packaging and installer.
- Feature parity review and testing.

### Phase 8 — AI-integrated OS Exploration

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
- Planned adjustment step: 1K.

Retain resource checks and a safe fallback if future configurations cannot
run reliably.

## 6. Deferred Decisions

- Encryption: defer until late development or a later update.
- Release licensing: decide during release planning.
- Independent package repository/server: not required for the initial OS product.
- Broader test, formatting, and Clippy campaigns: expand during stabilization,
  not at the expense of reaching a working product prototype.
