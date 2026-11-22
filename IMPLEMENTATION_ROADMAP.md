# Local Assistant 2.0 — Implementation Roadmap

**Status:** Proposed execution plan based on approved architecture ADR-001 through ADR-035  
**Date:** 2026-11-22  
**Implementation status:** Not started  
**Source of truth:** `ARCHITECTURE_DECISIONS.md` and `AssistantV2_masterplan.md`

This roadmap sequences implementation so that identity, authorization, privacy and resource controls exist before agents, plugins or desktop automation can use them. It is a delivery plan, not authorization to begin coding.

## Delivery principles

1. Build a secure local platform before adding broad autonomy, plugins or OS integration.
2. The Rust core is authoritative. Desktop clients, Python workers and plugins never become authorization authorities.
3. Implement the permission broker, identity/scope propagation and audit path before privileged tools.
4. Keep the first usable product local-first and useful offline. Cloud inference is opt-in and local-only is the initial default.
5. Validate Linux and Windows boundaries independently. A successful Linux test does not establish Windows support.
6. GPU tests run on controlled hardware; ordinary CI remains GPU-independent.
7. Keep the platform monorepo and Linux distribution repository independent in build and release cadence.
8. No telemetry, automatic diagnostic uploads or hidden hosted-service dependency.
9. Treat every milestone as incomplete until its acceptance criteria and security gates pass.

## Phase 0 — Architecture freeze and engineering preparation

**Goal:** Turn approved architecture decisions into a buildable, reviewable engineering specification.

Deliverables:
- Reconcile ADR-001 through ADR-035 and maintain a traceability matrix from requirements to components, tests and release gates.
- Maintain the detailed [Phase 0 requirements traceability matrix](./docs/phase-0/PHASE_0_TRACEABILITY.md) and keep it synchronized with architecture and implementation changes.
- Define monorepo layout, branch/review policy, coding standards, supported toolchains and dependency update policy.
- Maintain the [engineering baseline](./docs/engineering/ENGINEERING_BASELINE.md) for branch/review workflow, coding standards, dependency policy, tests and CI gates.
- Maintain the proposed [platform and distribution repository layout](./docs/project/REPOSITORY_LAYOUT.md); settle repository governance before code bootstrap.
- Establish Linux/Windows CI, formatting, linting, static analysis, unit tests, integration-test scaffolding, secret scanning and SBOM generation.
- Define versioned API, worker protocol and plugin-manifest schema conventions.
- Maintain the [contracts and versioning policy](./docs/engineering/CONTRACTS_AND_VERSIONING.md) for client APIs, worker messages, plugin manifests and internal events.
- Create a threat-model register and data-classification catalogue.
- Maintain the [security foundation](./docs/engineering/SECURITY_FOUNDATION.md), including data handling rules and abuse-case verification.
- Perform technical spikes for SQLCipher + rusqlite + FTS5, SQLx/PostgreSQL, local IPC, Wasmtime, native sandbox profiles, Qt packaging and Secure Boot.
- Track prototype outcomes and pass/fail gates in [Phase 0 Technical Spikes](./docs/phase-0/PHASE_0_TECHNICAL_SPIKES.md).
- Establish the licensing inventory process; do not select the core licence until the full dependency and distribution review.

Exit criteria:
- Clean reproducible development builds on Linux and Windows.
- Documented component boundaries and trust boundaries.
- Critical technical spikes have pass/fail results and documented alternatives.
- No unresolved architectural contradiction blocks the secure local MVP.

## Phase 1 — Secure core foundation

**Goal:** Build the smallest trustworthy platform core before connecting models or privileged tools.

Deliverables:
- Rust + Tokio core process and lifecycle management.
- Linux per-user systemd service and Windows per-user background-process lifecycle.
- Versioned local client API and authenticated sessions over Unix sockets/named pipes.
- Trusted OS-user identity, user/workspace/task context propagation and deny-by-default authorization.
- Central permission broker with RBAC + ABAC policy evaluation, grant expiry/revocation and approval records.
- Audit event pipeline with local retention controls and sensitive-data minimization.
- Narrow privileged-helper interface with no arbitrary shell or general-purpose root/admin capability.
- Worker supervisor, protocol framing, deadlines, cancellation, resource limits and crash isolation.
- Local-only diagnostics and explicit export; no telemetry.

Exit criteria:
- Every core operation has trusted identity and scope.
- Automated tests demonstrate that a client, worker or model cannot bypass the broker.
- Worker crashes and timeouts do not crash the core or cause unsafe retries.
- Privileged operations fail closed when authorization is unavailable.
- Linux and Windows local IPC identity checks pass negative tests.

## Phase 2 — Encrypted persistence and data ownership

**Goal:** Make private data safe and correctly scoped before building long-term memory or multi-user features.

Deliverables:
- Personal SQLite database using rusqlite + SQLCipher + FTS5.
- Shared-server PostgreSQL access using SQLx, initially exercised with simulated users.
- Versioned schema migrations and upgrade tests.
- Envelope-encryption key hierarchy, Argon2id password wrapping, independent recovery key and key-version metadata.
- Master-password unlock/lock lifecycle; optional OS-vault convenience unlock remains opt-in.
- User-private and workspace-shared key separation.
- Encrypted attachments, temporary task directories and protected caches/indexes.
- Configurable retention, export, deletion and backup-expiry semantics.
- PostgreSQL row-level security as defence in depth, with application authorization always required.

Exit criteria:
- A locked or unauthorized user cannot retrieve private plaintext through APIs, workers, indexes or caches.
- Database backups can be restored and verified.
- Losing both the master password and recovery key is clearly documented as unrecoverable.
- Data deletion and backup expiry behaviour is tested and accurately described.
- SQLite never operates in WAL mode across network filesystems.

## Phase 3 — Model runtime and resource manager

**Goal:** Provide useful local inference without destabilizing the host or competing with games.

Deliverables:
- Central model manager and common inference-backend interface.
- llama.cpp local LLM runtime; ONNX Runtime for suitable specialized models.
- Model catalogue, verified downloads, quantization metadata and compatibility checks.
- CPU fallback, model lifecycle, bounded concurrency and duplicate-model avoidance.
- CPU/RAM/GPU/VRAM monitoring and adaptive scheduling.
- Normal, Performance, Gaming and Battery/Quiet workload profiles.
- NVIDIA-first validation with AMD and Intel backend adapters and an explicit tested compatibility matrix.
- Hybrid-graphics selection and user override.
- Gaming Mode game detection, manual override, GPU release and background-task deferral.

Exit criteria:
- Model load/unload and resource accounting are reliable.
- CPU fallback works when GPU support is unavailable or fails.
- Gaming Mode demonstrably releases assistant GPU allocations and respects configured CPU/RAM limits.
- Hardware support claims match measured test results.

## Phase 4 — Orchestration, memory and knowledge

**Goal:** Deliver the central assistant intelligence with governed context and persistent task state.

Deliverables:
- Central stateful orchestrator with bounded planning, delegation and verification.
- Built-in specialist contracts and supervised Python worker pool.
- Task checkpoints, deadlines, cancellation, bounded retries and idempotency handling.
- Working, episodic, semantic and procedural memory layers.
- User-reviewed persistent memory writes by default.
- Project-scoped knowledge repositories, source provenance and rebuildable indexes.
- SQLite FTS5 initial retrieval; evaluate embeddings/vector search only through measured quality and resource tests.
- Context assembly that filters by user/workspace authorization before content reaches a model.

Exit criteria:
- Agents receive only authorized, task-relevant context.
- Specialist output cannot grant permissions or alter identity.
- Task recovery does not blindly repeat non-idempotent actions.
- Memory corrections and deletion propagate to derived indexes/caches.

## Phase 5 — Desktop application MVP

**Goal:** Deliver a useful cross-platform UI as a client of the secure core.

Deliverables:
- Qt 6/QML desktop application with Qt Quick Controls.
- Thin C++ transport/view-model layer using the versioned API and WebSockets.
- Chat, conversation history, task progress, model selection and settings.
- Tray controls, notifications, dashboard and floating companion/control-panel modes.
- Permission prompts, approval previews, audit history and emergency stop.
- Accessibility, keyboard navigation and screen-reader support.
- Linux KDE and Windows platform adapters.

Exit criteria:
- UI closure does not unexpectedly stop the persistent core.
- Every action is authorized by the core, not trusted because it came from the UI.
- The UI remains responsive during long-running tasks and worker restarts.
- Linux and Windows package smoke tests pass.

## Phase 6 — Tools, computer interaction and voice

**Goal:** Add useful real-world interaction only after broker, identity and sandbox boundaries are proven.

Deliverables:
- Common typed tool interface and MCP adapters.
- File, application and process operations with operation-specific scopes and previews.
- Native APIs and accessibility interfaces before browser/visual fallback.
- Isolated assistant-managed browser profile.
- Browser and desktop automation workers under OS-enforced isolation.
- Undo/checkpoints for supported consequential operations.
- Push-to-talk voice, speech recognition and spoken responses; real-time conversational mode later.
- Screen awareness with persistent indicator, pause/disable, exclusions and independent metadata/content settings.

Exit criteria:
- No untrusted worker has broad host filesystem, process or network access.
- Sensitive/destructive actions require the correct approval.
- Screen and microphone indicators accurately reflect capture state.
- Screen/audio content is not sent to cloud without separate explicit authorization.
- Undo limitations are clearly communicated.

## Phase 7 — Durable automation and plugin ecosystem

**Goal:** Allow extensibility without weakening core security.

Deliverables:
- Durable database-backed workflow engine and typed event system.
- Scheduled/event-triggered tasks with bounded execution, approvals and cancellation.
- Transactional outbox, deduplication and idempotency protections.
- Versioned plugin manifest and SDK.
- Wasmtime component runtime with explicit capabilities and resource limits.
- Native plugin worker path with Linux/Windows sandbox profiles.
- Official registry plus third-party registry support.
- Signature/provenance verification, trust labels, permission review and update reapproval.
- Disable/uninstall revocation and task/subscription cleanup.

Exit criteria:
- Plugin signatures are not misrepresented as proof of safety.
- Plugin sandbox tests pass on each supported OS.
- Plugin updates cannot silently expand permissions.
- Disabling/uninstalling revokes grants and credentials.
- Workflow restart/retry behaviour is tested against duplicate external effects.

## Phase 8 — Shared server and self-hosted synchronization

**Goal:** Extend the personal platform to multiple users and trusted devices.

Deliverables:
- Linux headless server deployment under an unprivileged service account.
- Built-in accounts/passkeys; external identity providers remain a later extension.
- PostgreSQL tenancy, workspace membership and fair resource scheduling.
- Self-hosted sync service with end-to-end encrypted private payloads.
- Device enrollment/revocation and workspace-key distribution/rotation.
- Safe merge rules, conflict copies and explicit conflict resolution.
- Encrypted versioned backup and portable export/import.
- Server administration separated from private user-data access.

Exit criteria:
- Automated cross-user and cross-workspace isolation tests pass.
- A server administrator role does not expose private content through ordinary APIs.
- Revoked devices cannot receive future keys/data; limitations of already-copied offline data are documented.
- Sync is not presented as a backup substitute.
- Multiple simulated users cannot monopolize shared resources.

## Phase 9 — AI-first Linux distribution alpha

**Goal:** Integrate the platform into an independently maintained CachyOS-derived OS.

Deliverables:
- Separate distribution repository and independent package/build/release pipeline.
- Calamares-based beginner and advanced installer.
- Btrfs default, ext4 alternative, optional LUKS2.
- GRUB default and Btrfs snapshot recovery integration.
- Secure Boot signed-chain prototype, key enrollment/recovery and update tests.
- Redesigned KDE Plasma experience, settings and system utilities.
- Per-user assistant integration and narrowly scoped privileged OS helper.
- Native signed repositories, testing/stable promotion and conventional rolling updates.
- Recovery environment and documented package/snapshot recovery.

Exit criteria:
- Clean install, upgrade, failed update, snapshot restore and recovery tests pass.
- Secure Boot is advertised only for configurations that pass full end-to-end validation.
- AI services can be disabled without breaking normal OS operation.
- No mandatory cloud account or telemetry.
- Gaming, developer workflows, laptop power management and graphics compatibility are validated.

## Phase 10 — Hardening, beta and public release

**Goal:** Prove the system is safe, maintainable and supportable before public stable release.

Deliverables:
- Independent security review and remediation.
- Threat-model refresh and sandbox escape/broker bypass test campaign.
- Full cross-platform compatibility matrix and performance benchmarks.
- Backup/restore drills, migration tests and update/rollback exercises.
- Vulnerability disclosure process, security advisories and incident response.
- Complete SPDX/SBOM and licence audit for code, dependencies, assets, model weights, datasets and distribution packages.
- Final core licence decision after compatibility and legal review.
- Release documentation, support policy, known limitations and upgrade guidance.

Exit criteria:
- No unresolved critical/high security findings.
- All published hardware/platform claims have reproducible evidence.
- Licence and distribution obligations are reviewed and satisfied.
- Recovery, upgrade and rollback paths are documented and tested.
- Public release artifacts are signed, traceable and reproducible to the extent practical.

## Cross-phase release gates

A phase cannot be considered complete merely because its feature works on the developer laptop. Each phase must pass:
- Functional tests and regression tests.
- Authorization and data-isolation tests.
- Failure, cancellation and resource-pressure tests.
- Linux and Windows tests where the component is cross-platform.
- Dependency, vulnerability and licence scanning appropriate to the phase.
- Documentation and recovery procedures.
- Review of any changed threat boundary.

## Risks and explicit constraints

| Risk | Mitigation |
|---|---|
| Permission broker becomes a bottleneck or single point of failure | Keep policy evaluation deterministic, bounded, observable locally and fail closed; isolate broker interfaces and test failure behaviour. |
| Sandbox weakens across Linux/Windows | Maintain separate profiles and conformance tests; never claim parity without evidence. |
| SQLCipher/native dependency integration becomes brittle | Prototype early, pin the full native dependency chain and test Linux/Windows builds and restore. |
| GPU backend fragmentation | Publish only tested combinations; preserve CPU fallback and avoid driver replacement. |
| Secure Boot updates break bootability | Prototype the complete chain before stable; retain a known-good recovery path. |
| Self-hosted sync leaks metadata or stale keys | Minimize server-visible metadata, rotate keys on revocation and document offline-device limits. |
| Plugin ecosystem increases supply-chain risk | Capability declarations, signature/provenance checks, sandboxing, brokered actions and staged registry governance. |
| Licensing conflicts emerge late | Maintain SPDX/SBOM from the first dependency; complete project-wide review before public release. |
| Scope grows faster than validation capacity | Keep the first usable release local-first; defer hosted sync, Windows server, ARM64 and unvalidated backends. |

## Deferred product decisions

- Final distribution name, branding and independent ecosystem identity.
- Final core licence after the full dependency and distribution audit.
- External identity-provider integration for shared servers.
- Any future managed synchronization service (not currently planned).
- ARM64 or other CPU architectures (outside initial support).
- Additional bootloaders, GPU backends and model families beyond the validated initial matrix.

## Immediate next action

Begin Phase 0 only after the architecture consistency review is accepted. The first engineering work should be small, testable technical spikes—not a broad feature implementation. The first spikes are encrypted SQLite/FTS5, Linux/Windows local IPC, worker sandbox conformance, Secure Boot/GRUB recovery and the GPU backend matrix.
