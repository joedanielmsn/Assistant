# Local Assistant 2.0 — Implementation Roadmap

**Status:** Active; revised sequence approved 23 November 2026  
**Governing documents:** AssistantV2_masterplan.md · ARCHITECTURE_DECISIONS.md

This roadmap prioritizes the first working model-backed assistant, then a finished Linux and Windows desktop product. The future CachyOS OS product is not a dependency of the desktop application.

## Execution principles

1. Get an end-to-end model response early.
2. Keep the first implementation small, observable and runnable.
3. Rust core owns conversation state, authorization, orchestration and inference integration.
4. Streaming and cancellation are part of the initial request lifecycle.
5. Keep inference provider-neutral while implementing Ollama first.
6. Finish Linux before starting the Windows implementation.
7. Screen awareness is a core Assistant Mode feature, enabled by default while Assistant Mode is active, with visible status and immediate user controls.
8. Defer encryption, licensing decisions and broad stabilization.
9. Use essential smoke checks, focused tests, CI and a fake-Ollama test; do not let exhaustive testing block the first working system.
10. Start AI-integrated CachyOS work only after both desktop applications are complete.

## Phase 0 — Plan and workspace reset

**Goal:** Establish one coherent plan and protect existing work as reference.

Deliverables:
- Rewrite master plan, architecture decision register and roadmap to describe one consistent product sequence.
- Preserve rushed implementation as reference material in a clearly separated repository location; do not discard code or history.
- Establish a clean implementation workspace independent of the reference build.
- Record useful lessons from the prior implementation and Claude's static review without treating unverified observations as confirmed runtime defects.
- Align supporting documents and remove stale contradictions.
- Confirm actual repository layout before moving or renaming source files.

Exit criteria:
- One authoritative master plan and roadmap.
- Decisions accurately marked approved, deferred or open.
- Reference code preserved and new implementation has a clean boundary.
- Documentation-only changes do not silently restructure source code.

## Phase 1 — First working model-backed core

**Goal:** Make the new Rust core send a user message to a local model and return a response through the desktop-facing contract.

Deliverables:
- Rust core lifecycle and configuration.
- Provider-neutral inference trait and request/response types.
- Ollama as first provider.
- Local model selection/configuration sufficient for first working path.
- Request context with principal, workspace, task ID and request ID.
- Local IPC skeleton using QLocalSocket and JSON-RPC 2.0 over NDJSON.
- Request IDs, bounded framing and useful errors.
- Basic streaming from model through core to client.
- Cancellation wired through request lifecycle.
- Minimal desktop/development client to submit a prompt and display output.
- Lightweight fake-Ollama integration test that does not require a GPU.

Exit criteria:
- Prompt reaches local Ollama through Rust inference layer.
- Response reaches the client.
- Streaming works for a normal response.
- Cancellation stops work cleanly.
- Core remains usable after model error or cancelled request.

**Immediate priority:** Do not expand into memory, plugins, distro work or elaborate UI before this works.

## Phase 2 — Conversation ownership and dependable agent loop

**Goal:** Turn the first model call into a coherent conversational agent.

Deliverables:
- Rust core owns conversation history and message ordering.
- Client sends conversation ID and new user text rather than authoritative full history.
- Core assembles context and persists user/assistant messages.
- Correct handling of interrupted, cancelled and failed turns.
- Context-length setting from 4K to 16K tokens, provisional default 8K.
- Resource validation for context and model configuration.
- Basic agent orchestrator and task lifecycle.
- Typed tool schemas, strict argument validation and small tool registry.
- Model output remains untrusted; tool calls require broker authorization.
- Lightweight evaluation examples for tool selection and malformed arguments.

Exit criteria:
- Conversations survive application restart.
- Core constructs context from authorized stored messages.
- Failed/cancelled turns do not leave misleading or duplicate history.
- Tool proposals cannot bypass validation or permission checks.

## Phase 3 — Core reliability and OS boundaries

**Goal:** Establish reliable local operation before expanding tools.

Deliverables:
- Central permission broker for privileged operations.
- Consistent request-context propagation.
- Per-user local socket permissions and peer identity checks.
- Worker supervisor abstraction.
- Linux worker lifecycle using systemd-run --user where appropriate.
- Bubblewrap and Landlock isolation for worker filesystem/process boundaries where applicable.
- Explicit worker environment allowlist.
- Bounded time, memory, CPU, task count and output.
- Process-tree cleanup on timeout, cancellation and owner disconnect.
- Local audit events for security-relevant operations.
- Essential GitHub Actions CI for Rust checks, Python checks and Qt build integration.

Exit criteria:
- Core operations have trusted identity and request scope.
- Unauthorized privileged operations fail closed.
- Worker crash, timeout and cancellation do not crash core or leave uncontrolled processes.
- Essential CI catches basic integration regressions.

## Phase 4 — Linux desktop application and Screen Awareness Engine

**Goal:** Build the Linux desktop experience around the working core.

Deliverables:
- Qt 6/QML application using QLocalSocket and versioned JSON-RPC/NDJSON.
- Responsive chat UI with streaming, conversation history and task status.
- Settings and model management.
- Context-length control with resource validation.
- Assistant Mode and dedicated Screen Awareness Engine.
- Screen awareness enabled by default whenever Assistant Mode is active.
- Event-driven window/application/accessibility context collection.
- OCR/visual processing only when needed and within resource limits.
- Persistent visible active indicator, pause/disable controls and application/window exclusions.
- Local-only processing by default; separate authorization before cloud disclosure.
- Clear errors, reconnection and cancellation behaviour.
- Linux packaging, installation, update and uninstall experience.

Exit criteria:
- Linux desktop app is useful for daily conversational use.
- Core and UI communicate through documented local protocol.
- Persistent conversations, settings and model selection work.
- Screen awareness behaves as specified and remains under user control.
- Linux installation, update and removal paths are usable and documented.

## Phase 5 — Complete Linux desktop product

**Goal:** Finish and stabilize Linux before starting Windows.

Deliverables:
- Complete core and desktop features required for first daily-usable product.
- Basic memory and project context, with user control over persistent memory.
- Reliable model lifecycle and resource management for initial supported hardware.
- Core tool capabilities introduced incrementally behind permission broker.
- Accessibility, notifications and task recovery.
- Linux update/rollback behaviour appropriate to chosen packaging.
- User documentation and troubleshooting guidance.
- Focused regression and security checks for implemented features.

Exit criteria:
- Linux product has coherent feature set and documented limitations.
- Installation, launch, update and uninstall exercised on target CachyOS environment.
- No known critical issue prevents normal daily use.
- Linux implementation is stable enough to serve as Windows reference.

## Phase 6 — Windows desktop rebuild

**Goal:** Port the finished product without maintaining two active platform builds during Linux development.

Deliverables:
- Rebuild Windows-specific service lifecycle, local IPC, identity, worker supervision and OS integration.
- Retain shared Rust core, provider abstraction, agent contracts and Qt/QML UI where portable.
- Implement Windows-specific IPC and security boundaries; do not assume Linux socket/sandbox behaviour applies.
- Implement Windows screen-awareness providers and user controls.
- Validate model/runtime integration on Windows.
- Provide easy Windows installer, update and uninstall experience.
- Maintain Windows-specific compatibility and test record.

Exit criteria:
- Windows app provides agreed core feature set.
- Windows authorization and process isolation independently validated.
- Installer/update/uninstall are usable.
- Platform differences are isolated behind explicit adapters and documented contracts.

## Phase 7 — Cross-platform desktop release

**Goal:** Deliver finished Linux and Windows applications as one coherent product.

Deliverables:
- Consistent user experience and feature expectations.
- OS-appropriate installers, not an assumed single universal binary.
- Versioning, update channels and release notes.
- Platform dependency and packaging audits.
- Backup/export/import and recovery behaviour.
- Accessibility and performance review.
- Full release test matrix and documentation.

Exit criteria:
- Linux and Windows desktop products are complete.
- Each has supported installer and update path.
- Cross-platform contracts are stable and documented.
- Public release readiness review is complete.

## Phase 8 — AI-integrated CachyOS product

**Goal:** Create the separate AI-integrated CachyOS experience after desktop completion on both OSes.

Deliverables:
- Separate CachyOS-derived product definition and integration workspace.
- Repackage/customize CachyOS rather than create a new distribution from scratch.
- Deep assistant integration with desktop, system settings and carefully scoped OS capabilities.
- Continue using upstream CachyOS/Arch and other existing repositories.
- No self-hosted package repository or repository server required initially.
- Keep assistant installable and useful outside this OS product.
- Provide clear disable/uninstall controls and preserve normal OS functionality.

Exit criteria:
- Product is a CachyOS-based experience with assistant integration.
- It consumes upstream packages without independent repository infrastructure.
- Assistant integration can be disabled without breaking normal OS operation.
- Installation, update and recovery are documented.

## Phase 9 — Stabilization and release governance

**Goal:** Complete deeper engineering work without delaying the first working model.

Deliverables:
- Broader unit, integration, regression and end-to-end coverage.
- Security review of broker, IPC, worker isolation, screen capture and tool execution.
- Resource-pressure and failure testing across supported hardware.
- Dependency inventory, SBOM and licence compatibility review.
- Final licensing decision.
- Revisit encryption and key management before public release or when required by data-risk assessment.
- Release signing, update integrity and vulnerability response process.
- Performance/compatibility matrix.
- User documentation, migration and recovery guidance.

Exit criteria:
- No unresolved critical security or reliability issue.
- Published support claims have reproducible evidence.
- Licensing and distribution obligations reviewed.
- Release artifacts and update paths traceable and documented.

## Explicitly deferred

- Encryption implementation during early build.
- Final licence selection until release planning.
- Windows implementation before Linux product completion.
- AI-integrated CachyOS work before both desktop apps are complete.
- Independent package repository hosting.
- Shared multi-user server and hosted synchronization.
- Broad plugin marketplace and complex durable automation before core agent is dependable.
- ARM64 and CPUs below x86-64-v3.
- Exhaustive test campaigns before first working model integration.

## Immediate next action

Proceed with Phase 1: build the smallest end-to-end Rust-core-to-Ollama path with streaming and cancellation. Preserve the rushed implementation as reference, but do not let archival/restructuring work block the first working model.
