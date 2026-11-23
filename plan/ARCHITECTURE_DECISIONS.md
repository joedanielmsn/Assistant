# Local Assistant 2.0 — Architecture Decision Register

**Last updated:** 23 November 2026  
**Status:** Consolidated approved decisions and explicit deferrals  
**Authority:** Binding architecture choices. Product vision and delivery order are defined in AssistantV2_masterplan.md; execution is defined in IMPLEMENTATION_ROADMAP.md.

This register replaces the earlier discovery-stage register. It removes contradictory proposals and distinguishes approved choices from open implementation details.

## ADR-001 — Fresh implementation and reference preservation

**Status: Approved**

Start the new implementation independently. The rushed implementation is reference material and a source of lessons only. Do not extend, migrate or depend on its code as the foundation. Preserve source and history before any archival move.

## ADR-002 — Linux-first, sequential Windows development

**Status: Approved**

Build and finish the desktop product on CachyOS/Linux first. Only after Linux is complete, use it as the reference to rebuild the platform-specific Windows implementation. Do not maintain simultaneous active Linux and Windows implementation tracks during the Linux phase.

Shared architecture should isolate platform-specific concerns where practical; this does not authorize parallel Windows development.

## ADR-003 — Desktop product before AI-integrated CachyOS

**Status: Approved**

Complete Linux and Windows desktop applications, including OS-appropriate installers, before beginning the separate AI-integrated CachyOS product.

The future OS product is a CachyOS-derived/repackaged product, not a prerequisite for the desktop assistant. Initially use upstream CachyOS/Arch and other existing repositories. Do not build or host independent package repository infrastructure at the outset.

## ADR-004 — Rust core is authoritative

**Status: Approved**

The Rust core owns application state, identity, authorization, request lifecycle, conversation history, context assembly, orchestration, inference coordination, persistence and worker supervision. UI clients and workers are clients/requesters and cannot become authorities.

## ADR-005 — Central permission broker

**Status: Approved**

All privileged OS operations pass through one central permission broker. It evaluates authenticated principal, workspace/task scope, requested capability, policy and approval requirements. It fails closed when authorization is unavailable or invalid.

No model, tool, plugin, worker or UI component may bypass this broker.

## ADR-006 — Local client protocol

**Status: Approved**

Use QLocalSocket for Qt-to-core local communication, JSON-RPC 2.0 for method/event semantics and NDJSON framing.

The protocol supports versioning, request IDs, bounded frames, streaming events, cancellation, structured errors and reconnect-safe behaviour. An HTTP/WebSocket gateway is a possible later extension, not part of the initial local path.

## ADR-007 — Provider-neutral inference with Ollama first

**Status: Approved**

Implement an inference abstraction in Rust with provider-neutral internal types and lifecycle. Ollama is the first provider/backend integration to deliver a working model-backed prototype quickly.

An OpenAI-compatible /v1/chat/completions wire contract may be used as a common adapter where supported, but internal design must not depend on that API or one provider. llama.cpp may be evaluated as a later backend.

## ADR-008 — Conversation history belongs to the core

**Status: Approved**

The core owns authoritative conversation history and message ordering. The client submits a conversation identifier and new user text; the core loads prior messages, assembles authorized context, calls the model and persists results.

The client is not required to resend or authoritatively maintain the full transcript.

## ADR-009 — Streaming and cancellation are first-class

**Status: Approved**

Streaming output and cancellation are part of the initial request lifecycle. Cancellation propagates from client to core, inference backend and supervised worker as applicable. Disconnects, timeouts and failures must not leave uncontrolled work or misleading conversation state.

## ADR-010 — Shared request context

**Status: Approved**

Use a consistent request context across core modules and worker boundaries. Initial conceptual fields: principal, workspace, task ID and request ID. Extend only when justified by a concrete requirement. Propagate minimum necessary context and never trust worker-supplied identity.

## ADR-011 — Supervised and constrained workers

**Status: Approved direction; implementation details validated incrementally**

Keep a worker supervisor abstraction. On Linux, use systemd-run --user where appropriate for lifecycle and cgroup resource controls. Evaluate Bubblewrap and Landlock for filesystem/process confinement. Do not couple the architecture permanently to systemd.

Workers receive an explicit environment allowlist, narrow capabilities, bounded resources and bounded execution time. Cancel and clean them up on timeout, owner cancellation or relevant disconnect. Worker output is untrusted and validated by the core.

## ADR-012 — Agent tool reliability

**Status: Approved**

Use structured tool schemas, strict argument validation, capability checks and broker authorization. Use constrained generation/grammar support when available, but never rely on model formatting alone for safety. Maintain a small evaluation set for common tool-selection and argument errors.

## ADR-013 — Screen Awareness Engine

**Status: Approved**

Screen awareness is a major Assistant Mode capability and is enabled by default while Assistant Mode is active.

Implement a dedicated Screen Awareness Engine integrated with the context manager and agent. Prefer event-driven window, application and accessibility metadata; use OCR/vision as needed rather than continuously invoking heavyweight visual inference. Resource use adapts to hardware.

Mandatory controls:
- Persistent, clear indication when awareness is active.
- Immediate pause and disable controls.
- Application/window exclusions and sensitive-content protections.
- Local processing by default.
- Separate explicit authorization before captured content is disclosed to a cloud service.
- No capture while Assistant Mode is inactive or awareness is paused.

## ADR-014 — Desktop framework

**Status: Approved**

Use Qt 6 and QML for the desktop application. The UI is a client of the Rust core over the local protocol. Keep transport/view-model responsibilities thin; the core remains authoritative for state and permissions.

## ADR-015 — Initial CPU and platform baseline

**Status: Approved**

Initial support baseline is x86-64-v3. x86-64-v4 may be an optional optimized target. Original x86-64-only CPUs and ARM64 are outside the initial support baseline.

The first product implementation target is CachyOS/Linux. Windows is a later sequential implementation phase after Linux completion.

## ADR-016 — Context-length configuration

**Status: Approved**

Expose a context-length setting from 4K to 16K tokens. Use 8K as provisional default until realistic context and memory testing supports another default. Validate settings against available system/GPU memory and fail or degrade safely when resources are insufficient.

## ADR-017 — Resource-aware model operation

**Status: Approved direction**

Track and bound model/worker resource use. Avoid duplicate model loads, support unload and CPU fallback where practical, and account for limited VRAM. Screen awareness and background work adapt under resource pressure. Do not claim hardware compatibility without testing.

## ADR-018 — Persistence and memory ownership

**Status: Approved direction**

The core owns persistent conversations and context assembly. Add basic memory only after conversation persistence and retrieval are reliable. Persistent memory is governed and user-controllable. Filter context by user/workspace/task authorization before it reaches a model.

Exact database engine, schema and indexing strategy remain implementation choices unless separately approved.

## ADR-019 — Encryption deferred

**Status: Deferred by explicit decision**

Do not implement encryption, master-password unlock or key management in the early build. Encryption is deferred until near the end of development or a later update. Keep data boundaries modular enough to add a future protection layer, but do not spend early implementation effort on it.

Reassess before public release based on the actual data-risk model. This is not a decision that encryption will never be needed.

## ADR-020 — No telemetry by default

**Status: Approved**

No telemetry, automatic diagnostic uploads or hidden hosted-service dependency. Diagnostics remain local unless the user explicitly exports or shares them.

## ADR-021 — Initial model and development hardware

**Status: Approved initial target**

Ollama is the first inference integration. Initial development hardware includes an NVIDIA RTX 3050 with 4 GB VRAM and 16 GB system RAM. Keep the first working path practical for constrained hardware; do not assume large models or high context settings will fit.

The exact AMD CPU model and broader hardware compatibility matrix remain to be confirmed.

## ADR-022 — Essential CI and lightweight model-path testing

**Status: Approved**

Add GitHub Actions CI for essential Rust checks, Python checks and Qt build integration. Add a lightweight fake-Ollama stub-server integration test for inference/chat without requiring GPU hardware.

During initial development, keep validation proportional: build/run smoke checks and focused tests maintain momentum. Defer exhaustive suites, broad lint/format campaigns and full security/performance validation to stabilization. CI must not become a blocker to the first working backend.

## ADR-023 — Licensing deferred

**Status: Deferred by explicit decision**

Do not choose the final project licence during the early architecture/build phase. Maintain a dependency and licence inventory for later compatibility review. Make the final licensing decision before public distribution.

## ADR-024 — Future packaging and installers

**Status: Open; decide during platform packaging work**

Provide an easy installer appropriate to each OS. Do not assume one binary or package format serves all operating systems. Evaluate Linux packaging options and Windows installer options when Linux reaches packaging work. No final packaging format is selected here.

## ADR-025 — Future CachyOS repository strategy

**Status: Approved direction**

The AI-integrated CachyOS product initially consumes upstream CachyOS/Arch repositories and other existing repositories. Do not require a self-hosted package repository or repository server. Revisit only if a concrete distribution need makes independent infrastructure necessary.

## ADR-026 — Remote access, shared server and synchronization

**Status: Deferred**

A remote client gateway, shared multi-user server and synchronization service are not part of the initial desktop product. Do not let these complicate the first working backend or Linux desktop release.

## ADR-027 — Plugin ecosystem and advanced automation

**Status: Deferred beyond the initial agent**

Build the conversational agent and a narrow, validated tool foundation first. A broad plugin marketplace, third-party registry, durable automation engine and complex workflow learning are later capabilities requiring their own threat and permission reviews.

## ADR-028 — Platform-specific boundaries

**Status: Approved principle**

Keep platform-specific service lifecycle, local IPC identity, sandboxing, screen capture, packaging and OS integration behind explicit adapters where practical. Linux remains the sole active platform workstream until complete. Windows requires independent validation; Linux success is not evidence of Windows security or compatibility.

## ADR-029 — Prior implementation review

**Status: Approved process**

The prior rushed build is reference-only. Claude's review observations may inform the new design, but static-review findings are not automatically verified runtime defects. Do not spend time patching the reference build unless specific information is needed to understand or preserve it.

---

## Deferred decisions register

Intentionally not decided in this revision:
- Exact Linux package format and Windows installer technology.
- Exact database engine and schema details.
- Exact model catalogue, default model and later inference backends.
- Full memory taxonomy and retrieval implementation.
- Remote gateway, shared server and synchronization.
- Broad plugin ecosystem and advanced automation.
- Final encryption design and timing beyond the current deferral.
- Final licence and release governance details.
- Future CachyOS branding, installer details and release pipeline.

## Change control

When a binding decision changes, update this register and revise the master plan and roadmap in place. Remove contradictory old text rather than appending a competing decision. Keep all three governing documents aligned.
