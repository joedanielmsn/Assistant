# Local Assistant 2.0 — Master Project Plan

**Version:** 2.0  
**Last updated:** 23 November 2026  
**Status:** Revised project direction approved  
**Authority:** Product and architecture overview. See ARCHITECTURE_DECISIONS.md for binding decisions and IMPLEMENTATION_ROADMAP.md for execution order.

## 1. Product vision

Local Assistant 2.0 is a local-first, model-backed desktop assistant built around a trusted Rust core, replaceable inference backends, supervised workers, a Qt 6/QML desktop client, persistent conversations, governed tools and desktop awareness.

The immediate objective is a reliable end-to-end model-backed assistant. Build that first, then grow it into a useful daily desktop product without compromising user control, privacy or system reliability.

### Product sequence

1. **Linux desktop:** Build and finish the assistant backend, agent and desktop application on CachyOS/Linux. Establish a reliable installation, update and user experience.
2. **Windows desktop:** After Linux is complete, use it as the working reference and rebuild the platform-specific Windows implementation. Do not develop both OS implementations in parallel.
3. **Cross-platform desktop release:** Complete Linux and Windows desktop applications with an easy installer appropriate to each OS. “Universal” means a straightforward OS-appropriate installation experience, not one binary shared across operating systems.
4. **AI-integrated CachyOS:** Only after both desktop applications are complete, create a separate CachyOS-derived/repackaged OS experience with deep assistant integration.

The desktop assistant must remain independently installable and useful. The future OS integration consumes assistant capabilities through stable interfaces; it must not become a dependency of the desktop product.

### Distribution constraints

- Build on CachyOS and its upstream ecosystem.
- Initially use CachyOS/Arch and other existing repositories.
- Do not plan around self-hosting an independent package repository or repository server.
- The OS product is a later product, not a parallel workstream during desktop development.

## 2. Project principles

- Local-first operation and user-controlled data.
- Rust core is authoritative for identity, policy, state and privileged operations.
- Models, workers, plugins and UI are untrusted requesters, not authorities.
- All privileged operations pass through one central permission broker.
- Build the first functioning model-backed path early.
- Keep provider and platform boundaries replaceable without speculative complexity.
- Streaming and cancellation are first-class from the beginning.
- Prefer event-driven system interfaces over expensive polling.
- Make privacy and capture state visible and controllable.
- Keep early tests proportional; defer broad validation to stabilization.
- No telemetry, hidden cloud dependency or automatic diagnostic upload.

## 3. Initial platform and hardware

Initial development target is CachyOS/Linux, KDE Plasma, x86-64-v3 minimum CPU baseline. x86-64-v4 may be an optional optimized target. Original x86-64-only CPUs and ARM64 are outside the initial support baseline. Qt 6 + QML is the desktop framework; Rust is the core and inference integration language; Ollama is the first inference integration.

The initial development machine has an NVIDIA RTX 3050 with 4 GB VRAM and 16 GB DDR5 RAM. The AMD CPU model remains to be confirmed.

Design implications:
- Keep model loading, context size, concurrency and screen processing resource-aware.
- Provide CPU fallback where practical.
- Context length is configurable from 4K to 16K tokens, with 16K (16,384 tokens) selected as the current default after local RTX 3050 4GB testing. The UI must expose a persisted 4K–16K slider in 1K increments and validate resource use.
- Validate selected models and context settings against available resources; never assume every selectable setting fits.

## 4. High-level architecture

~~~text
Qt 6 / QML Desktop Client
          |
          | QLocalSocket + JSON-RPC 2.0 over NDJSON
          v
       Rust Core
  ├── Identity and request context
  ├── Central permission broker
  ├── Conversation and context manager
  ├── Agent orchestrator
  ├── Inference provider interface
  ├── Tool registry and validation
  ├── Screen Awareness Engine
  ├── Resource manager
  ├── Persistence and memory
  └── Worker supervisor
          |
          ├── Ollama backend (first provider)
          ├── Supervised specialist workers
          └── Brokered OS capabilities
~~~

The diagram is logical, not a requirement that every module run in one process. Use process separation where it materially improves isolation, fault containment or resource control.

### 4.1 Rust core

The core owns application lifecycle, configuration, trusted identity, authorization, request context, conversation history, context assembly, orchestration, inference coordination, tool validation, screen-awareness coordination, persistence, resource scheduling and worker supervision.

The UI displays and requests state; it does not independently own authoritative conversation history or authorization decisions.

### 4.2 Local client protocol

Use QLocalSocket for local Qt-to-core communication, JSON-RPC 2.0 for request/response semantics and NDJSON framing.

Requirements:
- Versioned methods and events.
- Explicit request IDs and task IDs.
- Streaming events for incremental output and task progress.
- Cancellation messages and deterministic task cleanup.
- Bounded frame sizes and payload validation.
- Local peer identity checks and per-user socket permissions.
- Reconnect behaviour that does not duplicate requests or messages.

An HTTP/WebSocket gateway may be considered later; it is not part of the initial local desktop path.

### 4.3 Inference architecture

Implement a provider-neutral inference interface inside the Rust core. Ollama is the first backend to deliver a working model-backed prototype quickly.

- Keep internal provider types independent of Ollama-specific types.
- An OpenAI-compatible /v1/chat/completions wire shape may be used as a common adapter contract where useful; internal architecture must remain provider-neutral.
- Streaming, cancellation, model metadata and errors are part of the provider contract.
- llama.cpp may be evaluated as a future backend.
- Python remains available for specialist workers where useful; it is not the authoritative owner of conversation state or primary inference architecture.

### 4.4 Request context and authorization

Propagate a shared request context across core modules and worker boundaries. Initial conceptual fields: principal, workspace, task ID and request ID. Extend only when a concrete requirement appears. Workers receive minimum necessary context and capabilities. Model output never grants authority.

All privileged OS operations are mediated by the central permission broker. It evaluates identity, scope, capability, policy and approval requirements, and fails closed.

### 4.5 Worker supervision and isolation

Keep a supervisor abstraction. On Linux, investigate/use systemd-run --user where useful for cgroup and lifecycle controls. Use Bubblewrap and Landlock for filesystem/process confinement where applicable.

Workers require:
- Explicit environment allowlist; never inherit the full parent environment.
- Bounded memory, CPU, task count, time and output.
- Cancellation on timeout, explicit cancellation or relevant owner disconnect.
- Process-tree termination and temporary-file cleanup.
- No broad host filesystem, network or process access by default.
- Validation of all worker output before use.

## 5. Agent and tool architecture

The first agent is a conversational model-backed assistant. Expand into tools only after the model path works.

Tool execution requires typed schemas, strict argument validation, broker authorization, capability/scope checks, approval for consequential operations, bounded planning/retries/deadlines, cancellation and idempotency protection where retries are possible.

Constrained generation or grammar support may be used when a backend supports it, but schema validation remains mandatory. Maintain a lightweight evaluation set for common tool-selection and argument errors.

## 6. Conversation, context and memory

The Rust core owns conversation history. The client sends a conversation identifier and new user input; the core loads history, assembles authorized context, invokes the model and persists the resulting messages.

Initial requirements:
- Persistent conversations and stable IDs.
- Ordered user, assistant and tool messages.
- Streaming assistant responses.
- Correct handling of interrupted, cancelled and failed turns.
- Context setting from 4K–16K, current selected default 16K; persist the setting and validate resource use.
- Context selection respecting user/workspace/task boundaries.
- Basic memory only after reliable conversation persistence and retrieval.

Long-term memory must be governed and user-controllable. Do not silently make every conversation detail permanent. Encryption is deferred; storage design should not prevent adding it later, but it must not delay the first working model.

## 7. Screen Awareness Engine

Screen awareness is a major Assistant Mode capability and is **enabled by default while Assistant Mode is active**.

Build a dedicated engine integrated with the context manager and agent, not a one-off screenshot prompt.

Requirements:
- Maintain useful current desktop context while Assistant Mode is active.
- Prefer event-driven window, accessibility and application metadata.
- Use OCR or visual processing when needed, not continuous heavyweight vision inference on every frame.
- Adapt processing frequency and model use to hardware/workload constraints.
- Clearly indicate when awareness is active.
- Provide immediate pause and disable controls.
- Support application/window exclusions and sensitive-content protections.
- Keep captured content local by default; no cloud disclosure without separate explicit authorization.
- Do not capture when Assistant Mode is inactive or awareness is paused.
- Keep OS-specific capture providers replaceable for the later Windows implementation.

Default-on does not remove user control. Visible status, exclusions and immediate pause/disable are mandatory.

## 8. Desktop application

Use Qt 6 and QML. Keep the client thin and connected to the Rust core through the local protocol.

Build in this order: chat with streaming; conversation list/history; model selection and management; settings including context length and Assistant Mode; task status and cancellation; screen-awareness indicator/exclusions/pause; later permission prompts, dashboard, tray and companion modes.

Do not build elaborate UI surfaces before backend and agent work end-to-end.

## 9. Resource management

Track CPU, RAM, GPU and VRAM where available. Bound concurrent model and worker activity, avoid duplicate model loads, support unload and CPU fallback where practical, validate context/model settings against available memory, and defer nonessential screen/background work under resource pressure. Add workload profiles only when behaviour can be measured and explained.

## 10. Security and privacy

- One central permission broker for privileged operations.
- Deny by default for capabilities not granted.
- Treat model, worker and plugin output as untrusted.
- Keep local IPC private to the user and authenticate peer identity.
- Use explicit environment and capability allowlists for workers.
- Provide visible capture state and controls for screen awareness.
- No telemetry or automatic diagnostic uploads.
- Local inference is the initial default.
- Encryption/key management is deferred to a later stage; reassess before public release or when the data-risk assessment requires it.
- Licensing selection is deferred until release planning; maintain a dependency/licence inventory.

## 11. Quality and validation strategy

During initial development, use the smallest checks that protect forward progress:
- Build/run smoke checks for newly integrated components.
- Lightweight fake-Ollama server test for the Rust inference/chat path.
- Essential CI for Rust, Python and Qt build integration.
- Focused tests for protocol framing, authorization and cancellation as those paths arrive.

Do not front-load exhaustive test suites, broad Clippy/format campaigns or full cross-platform validation before the first model-backed system works. A dedicated stabilization phase expands tests, security review and release validation.

## 12. Delivery milestones

1. Preserve rushed build as read-only reference and establish clean implementation workspace.
2. Deliver working Rust-core-to-Ollama response with streaming and cancellation.
3. Add core-owned conversations and context assembly.
4. Deliver reliable conversational agent and structured tool foundation.
5. Build Screen Awareness Engine and integrate it into Assistant Mode.
6. Complete Linux desktop application and Linux installation/update experience.
7. Rebuild platform-specific implementation for Windows using finished Linux product as reference.
8. Complete Linux + Windows desktop applications and OS-appropriate installers.
9. Build separate AI-integrated CachyOS product using upstream repositories.
10. Stabilize, audit, decide licensing and prepare public release.

See IMPLEMENTATION_ROADMAP.md for phase details and acceptance criteria.

## 13. Explicitly deferred

- Encryption and master-password/key-management implementation during early development.
- Final project licence selection.
- Independent package repository infrastructure.
- AI-integrated CachyOS work before both desktop apps are complete.
- Parallel Windows development during the Linux build.
- Remote gateway, shared multi-user server and hosted synchronization.
- Broad plugin marketplace and advanced automation before the core agent is reliable.
- ARM64 and CPUs below x86-64-v3.
- Exhaustive test campaigns before first working model integration.

## 14. Governance

This is the product and architecture overview. ARCHITECTURE_DECISIONS.md records binding choices and deferred questions. IMPLEMENTATION_ROADMAP.md records execution order and status.

When a decision changes, update the relevant decision record, revise this plan if product direction/architecture changes, revise the roadmap if delivery order/scope changes, and remove contradictory text rather than appending a competing plan.
