# Local Assistant 2.0 — Master Plan

Last updated: 2026-11-23

## 1. Vision

Build a capable, natural, privacy-focused AI assistant that runs primarily
on the user's own computer.

The assistant should understand requests, use tools, coordinate specialist
agents, understand the user's active desktop when permitted, remember
user-approved information, and adapt its workflows over time.

The goal is not to build every underlying technology ourselves. The goal
is to deliver a cohesive product by combining our own architecture with
mature, reliable open-source components wherever appropriate.

The product must remain useful as a desktop assistant independently of
any future operating-system integration.

## 2. Product Strategy

Three products are envisioned, in this order.

### Product A — Linux Desktop Assistant

The first complete product, initially targeting CachyOS and KDE Plasma.

Deliverables:

- Rust assistant core.
- Qt 6/QML desktop application.
- Local inference and model management.
- Persistent conversations and context.
- Reusable tool system.
- Multi-agent coordination.
- User memory and personalization.
- Screen Awareness Engine.
- Skills and workflow automation.
- Settings, permissions, and privacy controls.
- Linux packaging and installer.

### Product B — Windows Desktop Assistant

Begin after Product A is complete.

Reuse the shared core and product architecture where practical. Implement
replaceable Windows-specific IPC, capture, accessibility, and platform
providers.

Deliver a tested Windows desktop application and installer.

### Product C — AI-integrated Operating System

Explore a separate CachyOS-derived AI-integrated operating system only
after both desktop products are complete.

The desktop assistant must not depend on OS integration to function.
An independently hosted package repository or server is not required for
the initial OS product.

## 3. Foundational Development Principles

### 3.1 Reuse First

The user is a solo developer. Development time should be spent on product
capabilities and integration rather than recreating mature infrastructure.

For every significant subsystem, evaluate:

1. Existing open-source applications and libraries.
2. Whether an existing component can be integrated behind our interfaces.
3. Whether a small adapter or extension is sufficient.
4. Whether a custom implementation is genuinely necessary.

Prefer:

- Mature and actively maintained projects.
- Local operation and privacy.
- Compatible licenses.
- Low resource consumption.
- Small integration surfaces.
- Replaceable components.
- Good Linux support and a credible cross-platform path.

Do not add dependencies merely because they are popular.

### 3.2 Preserve Architectural Ownership

External components should implement capabilities, not dictate the entire
product architecture.

Examples:

- Our Tool Registry remains the internal tool interface.
- MCP may be used through an adapter for external tools.
- Our Agent Coordinator owns product-level routing and task lifecycle.
- Existing orchestration libraries may be evaluated as implementation
  components, but should not silently become the product's architecture.
- Qt/QML remains the desktop experience.
- Linux capture and accessibility systems should be accessed through
  replaceable providers.

Avoid coupling core application state to an external framework.

### 3.3 User Approval for Major Decisions

The assistant must ask the user before committing to major decisions.

Approval is required for:

- Adopting a major framework or orchestration platform.
- Adding a substantial runtime dependency or background service.
- Selecting a license with meaningful distribution consequences.
- Replacing a major architectural component.
- Changing the supported platforms or hardware baseline.
- Changing inference architecture or primary model strategy.
- Introducing cloud processing or external data transmission.
- Changing product direction or roadmap priorities.

Before asking, present:

- The problem being solved.
- Viable alternatives.
- Advantages and disadvantages.
- Maintenance and licensing implications.
- Performance and privacy implications.
- A recommended option.

Do not treat a recommendation as approval.

Routine implementation details within an approved design can proceed
without repeatedly interrupting the user.

### 3.4 Local-First Privacy

- Local inference and processing by default.
- No telemetry or automatic diagnostic uploads.
- No external transmission without appropriate user authorization.
- Visible controls for sensitive capabilities.
- User control over memory and personalization.
- Permission checks must remain authoritative in the core.

### 3.5 Incremental, Working Development

- Prefer complete vertical slices over disconnected abstractions.
- Keep the application runnable throughout development.
- Combine related implementation steps when quality is preserved.
- Validate meaningful batches rather than performing unnecessary checks
  after every tiny edit.
- Maintain useful documentation and Git checkpoints.
- Prioritize reaching a usable product before broad polishing campaigns.

## 4. Target Architecture

### Rust Core

Owns:

- Application state.
- Conversations and context.
- Inference coordination.
- Agent registry and coordinator.
- Tool registry and permissions.
- Task lifecycle and cancellation.
- Memory and personalization.
- Skill management.
- Platform service interfaces.

### Desktop Client

Qt 6, C++, and QML.

Owns presentation and user interaction, not authoritative agent permissions
or application state.

### Inference

Initial provider: Ollama.

Current model: `assistant-qwen:latest` (Qwen 3.5 2B Q4_K_M).

Current context target: 16K, configurable from 4K to 16K.

Keep inference providers replaceable. Evaluate alternatives only when
there is a concrete capability or performance reason.

### IPC

Current Linux transport: Unix-domain socket.

Application protocol: JSON-RPC 2.0 over NDJSON.

Keep transport independent from the application-level protocol so Windows
can use an appropriate replaceable transport.

### Tool System

A common interface for built-in and external capabilities.

Tool definitions should contain:

- Unique identity and version.
- Description.
- Argument schema.
- Required capabilities.
- Execution limits.
- Structured results.
- Logging and lifecycle information.

External tool ecosystems should be integrated through adapters.

### Agent System

The Agent Coordinator owns:

- Task classification.
- Specialist selection.
- Delegation.
- Task lifecycle.
- Result aggregation.
- Failure handling.
- Execution limits.
- Permission preservation.

Specialists should be replaceable and should use the shared tool interface.

### Platform Services

Use replaceable providers for:

- Workspace access.
- Screen capture.
- Window metadata.
- Accessibility.
- Notifications.
- Operating-system integration.

## 5. Reuse and Integration Strategy by Subsystem

These are evaluation areas, not pre-approved dependencies.

| Subsystem | Existing solutions to evaluate |
|---|---|
| Agent orchestration | LangGraph and Rust-native alternatives |
| Tool interoperability | Model Context Protocol (MCP) |
| Local inference | Ollama and compatible local runtimes |
| Desktop capture | XDG Desktop Portal and PipeWire |
| KDE integration | KDE/KWin interfaces |
| Accessibility | AT-SPI |
| OCR | Tesseract and existing OCR libraries |
| Workflow automation | n8n and smaller embeddable alternatives |
| Vector search | Qdrant and embedded alternatives |
| Persistence | Existing SQLite ecosystem |
| Document extraction | Mature format-specific libraries |
| Search and indexing | Existing local indexing and search engines |

Evaluate the smallest solution that satisfies the actual requirement.
A standalone server or large framework is not automatically preferable
to a lightweight library.

No candidate in this table is approved for integration solely by being
listed here.

## 6. Tool and Agent Capability Roadmap

### Tool System

- Request-aware execution context.
- Workspace selection and propagation.
- Project exploration and file search.
- Code inspection.
- Safe file modification workflows.
- External tool adapters.
- Explicit permissions and resource limits.
- Execution logging and cancellation.

### Agent Coordinator

- General coordinator.
- Research specialist.
- Coding specialist.
- System specialist.
- Delegated task contracts.
- Bounded specialist execution.
- Result aggregation.
- Recovery and retry limits.
- Principal and permission preservation.

### Worker Supervision

- Task cancellation.
- Timeouts.
- Resource limits.
- Worker lifecycle tracking.
- Failure isolation.
- Controlled retries.
- No capability escalation through delegation.

## 7. Memory, Personalization, and Skills

Keep separate:

- Conversation history.
- Durable user-approved memory.
- Adaptive interaction preferences.
- Versioned skills and workflows.

Users must be able to inspect and control retained information.

Skills should be evaluated before activation. New permissions and
consequential capabilities require appropriate approval.

Do not autonomously rewrite core application code or modify model weights.

## 8. Screen Awareness Engine

Screen awareness is intended to operate while Assistant Mode is active,
with visible status and immediate user controls.

Use existing operating-system facilities where possible:

- XDG Desktop Portal.
- PipeWire.
- KDE/KWin.
- AT-SPI.
- OCR engines when appropriate.

Prefer event-driven desktop context and accessibility information over
unnecessary continuous heavyweight visual inference.

Requirements:

- Visible active indicator.
- Pause and disable controls.
- App/window exclusions.
- Sensitive-content protections.
- No capture while inactive or paused.
- Local processing by default.
- Replaceable providers for Windows.

## 9. Product Completion and Release

### Linux

- Complete assistant functionality.
- Reliable IPC and lifecycle.
- Model and settings management.
- Memory and privacy controls.
- Screen awareness.
- Agent and skill workflows.
- Performance and stability.
- Dependency and license inventory.
- Installer and documentation.

### Windows

- Shared product behaviour.
- Windows-specific platform providers.
- Desktop awareness.
- Installer and validation.

### AI-integrated OS

Only begin after both desktop products are complete.

## 10. Hardware Baseline

Current development hardware:

- AMD CPU.
- 16 GB DDR5 RAM.
- NVIDIA RTX 3050 4 GB.

Current inference baseline:

- Ollama.
- `assistant-qwen:latest`.
- Qwen 3.5 2B Q4_K_M.
- 16K context tested.

Initial CPU target: x86-64-v3.
Optional optimization: x86-64-v4.
Older x86-64-only and ARM64 systems are outside the initial baseline.

Do not increase resource requirements without a concrete product benefit
and user approval.

## 11. Deferred Decisions

- Encryption: defer until late development or a later update.
- Release licensing: decide during release planning.
- Windows implementation details: decide when Product B begins.
- OS integration details: decide only when Product C begins.
- Major third-party framework adoption: requires explicit user approval.
