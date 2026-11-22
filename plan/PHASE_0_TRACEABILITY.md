# Phase 0 — Requirements Traceability Matrix

**Status:** Baseline retained; traceability updates continue alongside Phase 1 implementation  
**Date:** 2026-11-22  
**Authority:** Approved ADR-001 through ADR-035 and `IMPLEMENTATION_ROADMAP.md`  
**Implementation status:** Phase 1 secure local core prototype started; requirements-to-code mapping is being expanded incrementally

This matrix connects the approved product requirements to architecture decisions, implementation ownership, verification evidence and delivery phases. It is a planning and review artifact; listing a test does not mean that the test or feature already exists.

## Traceability

| ID | Requirement / outcome | Governing ADRs | Primary components / ownership | Required verification evidence | Roadmap phase |
|---|---|---|---|---|---|
| TR-001 | Support personal desktop, standalone, headless/server and future system-integrated deployment profiles | 001, 002, 030 | Core lifecycle, platform service adapters, deployment packaging | Start/stop/restart tests for each supported profile; verify UI and core lifecycle independence | 0, 1, 5, 8 |
| TR-002 | Enforce installation, user, workspace and task/session data ownership; private by default | 002, 003, 004, 011, 012, 013 | Identity/context service, repositories, authorization middleware, task/workflow state | Cross-user and cross-workspace negative tests across APIs, workers, indexes, caches, exports and background tasks | 1, 2, 4, 7, 8 |
| TR-003 | Route every privileged or sensitive assistant operation through one authoritative permission broker | 004, 009, 017, 024 | Permission broker, policy engine, privileged helper, tool gateway | Broker-bypass tests; deny-by-default tests; helper interface fuzzing; approval and revocation tests | 0, 1, 6, 7, 9, 10 |
| TR-004 | Keep the core authoritative while isolating untrusted, sensitive or independently managed workers | 005, 011, 014, 021, 024, 030 | Rust core, worker supervisor, Python workers, plugin runtimes, platform sandbox adapters | Worker crash/timeout containment; OS sandbox conformance and adversarial negative tests | 0, 1, 4, 6, 7, 10 |
| TR-005 | Provide authenticated, versioned client and worker communication on Linux and Windows | 007, 021, 030 | Axum API, WebSocket service, UDS/named-pipe adapters, worker IPC | Protocol compatibility tests; peer identity and ACL negative tests; framing, size-limit, cancellation and deadline tests on both OSes | 0, 1, 5, 8 |
| TR-006 | Protect user data with encryption, scoped keys, recovery and safe lifecycle handling | 003, 025, 032 | Key-management module, rusqlite/SQLCipher, encrypted attachments, OS credential adapter | Argon2id parameter and key-wrap tests; wrong-key/tamper tests; lock/unlock and recovery drills; key-isolation tests | 0, 2, 8, 10 |
| TR-007 | Provide personal SQLite persistence and shared-server PostgreSQL persistence with correct isolation | 003, 021, 025, 032 | Repository/domain interfaces, rusqlite + SQLCipher + FTS5, SQLx/PostgreSQL | Linux/Windows build matrix; migrations; encrypted backup/restore; WAL/checkpoint; PostgreSQL RLS and application-authorization tests | 0, 2, 8 |
| TR-008 | Preserve data lifecycle semantics across retention, deletion, export, backups and derived indexes | 003, 012, 015, 025, 026 | Data lifecycle service, export/import, backup, index/cache managers | Deletion propagation tests; restore tests; backup-expiry behaviour tests; export completeness and omission reports | 2, 4, 8, 10 |
| TR-009 | Run local models through a common backend interface with CPU fallback and controlled lifecycle | 006, 008, 021, 027 | Model manager, llama.cpp, ONNX Runtime adapters, hardware detection | Model load/unload tests; CPU fallback tests; backend compatibility matrix; resource accounting and recovery tests | 0, 3, 10 |
| TR-010 | Protect foreground work and gaming workloads through adaptive CPU/RAM/GPU/VRAM scheduling | 006, 008, 027 | Resource manager, OS telemetry adapters, Gaming Mode controller | Controlled workload benchmarks; GPU release verification; adaptive budget and background-deferral tests | 0, 3, 10 |
| TR-011 | Orchestrate bounded tasks, specialists, verification, cancellation and safe recovery | 005, 011, 013, 021 | Orchestrator, task store, specialist contracts, worker supervisor | Plan-bound tests; structured-output validation; timeout/cancellation tests; retry/idempotency and interrupted-task recovery tests | 1, 4, 7 |
| TR-012 | Provide governed working, episodic, semantic and procedural memory | 003, 012 | Memory services, approval workflow, provenance/version metadata | Approval-before-write tests; conflict/correction tests; scope filtering; deletion propagation and provenance checks | 2, 4 |
| TR-013 | Provide project-scoped knowledge retrieval with source provenance and authorization-aware context | 003, 009, 012 | Project repository manager, indexer, FTS5 retrieval, context assembler | Unauthorized retrieval negative tests; source traceability; index rebuild and stale-source tests; prompt-injection handling tests | 2, 4, 6 |
| TR-014 | Provide a cross-platform Qt desktop client without making the UI an authority | 007, 010, 020, 030 | Qt 6/QML application, C++ transport/view-model layer, platform adapters | API contract tests; UI/core lifecycle tests; accessibility checks; Linux/Windows package smoke tests | 5 |
| TR-015 | Make desktop awareness, screen capture and voice visible, controllable and privacy-preserving | 010, 034, 035 | Awareness service, capture adapters, voice worker, UI indicators and controls | Indicator-state tests; pause/disable and exclusion tests; retention-default tests; cloud-disclosure and consent tests | 5, 6, 10 |
| TR-016 | Enable brokered computer interaction using layered APIs, accessibility and isolated fallbacks | 009, 017, 024 | Typed tool gateway, OS adapters, browser/desktop workers, privileged helper | Per-operation authorization tests; sandbox tests; target revalidation; undo/checkpoint tests; no-arbitrary-shell tests | 1, 6, 10 |
| TR-017 | Support durable event-driven workflows with bounded execution and safe retries | 013, 021 | Event system, workflow engine, durable task store, transactional outbox | Restart/resume tests; duplicate-event tests; idempotency tests; approval pause and cancellation tests | 4, 7 |
| TR-018 | Support third-party plugins with declared capabilities, isolation and lifecycle governance | 014, 021, 024, 029 | Plugin SDK, manifest validator, Wasmtime runtime, native worker sandbox, registry client | Manifest/schema tests; capability-denial tests; sandbox conformance; signature/provenance validation; update permission-diff tests | 0, 7, 10 |
| TR-019 | Support optional self-hosted, end-to-end encrypted synchronization without a managed hosted service | 015, 025, 026 | Sync server/client, device identity and key management, conflict resolver | Protocol security review; ciphertext-only private payload checks; device revocation/key rotation tests; conflict and offline-device tests | 8, 10 |
| TR-020 | Maintain fair, bounded shared-server resource scheduling and administrative separation | 002, 003, 008, 025 | Server identity/policy layer, scheduler, PostgreSQL access layer, admin interface | Multi-user load tests; starvation/monopolization tests; admin privacy-boundary tests | 2, 3, 8 |
| TR-021 | Build an independently maintained CachyOS-derived, AI-first Linux distribution | 016, 018, 019, 022, 031, 033 | Separate distro repository, package definitions, installer, desktop integration | Clean-install tests; supported CPU baseline checks; hardware compatibility tests; independent release-pipeline validation | 9, 10 |
| TR-022 | Provide safe conventional rolling updates, signed artifacts and documented recovery | 016, 022, 028, 031, 033 | Package/repository pipeline, update client, snapshot/recovery integration, signing infrastructure | Signature/provenance verification; failed-update drills; Btrfs and ext4 recovery tests; Secure Boot end-to-end tests before support claims | 0, 9, 10 |
| TR-023 | Keep local inference as default and require explicit policy for cloud disclosure | 003, 034 | Model router, sensitivity policy, provider adapters, credential store | Default-local tests; cloud-block tests for restricted scopes; provider disclosure and separate screen/voice authorization tests | 1, 3, 4, 6, 10 |
| TR-024 | Do not collect telemetry or upload diagnostics automatically | 023, 034, 035 | Logging/diagnostics, update and network clients, build dependency policy | Network egress review; dependency scan for telemetry SDKs; diagnostic preview/redaction tests; no automatic crash-upload tests | 0, 1, 10 |
| TR-025 | Maintain supply-chain integrity, licensing compliance and traceable releases | 014, 022, 028, 029 | CI/CD, dependency inventory, SBOM generator, signing and release pipeline | SPDX/SBOM validation; licence compatibility review; pinned dependency checks; signed artifact and provenance verification | 0, 7, 9, 10 |
| TR-026 | Support only the approved initial x86-64-v3 CPU baseline and make compatibility claims evidence-based | 018, 019, 027 | Build configuration, runtime capability detector, installer preflight, hardware test lab | Build/runtime baseline checks; x86-64-v2 rejection tests; x86-64-v4 optional-path tests; documented tested hardware matrix | 0, 3, 9, 10 |

## Cross-cutting verification rules

Every implementation milestone must include, where applicable:

1. Functional and regression tests.
2. Authorization, ownership and data-isolation tests.
3. Failure, timeout, cancellation and resource-pressure tests.
4. Separate Linux and Windows validation where both are claimed as supported.
5. Dependency, vulnerability and licence scanning appropriate to the component.
6. Documentation, operational diagnostics and recovery procedures.
7. Review of any new or changed trust boundary.
8. Evidence for every compatibility, privacy and security claim.

## Phase 0 completion checklist

- [ ] Review every row with the architecture and roadmap.
- [ ] Assign a stable component owner/module to each requirement.
- [ ] Convert verification descriptions into test plans with explicit pass/fail criteria.
- [ ] Identify requirements that need a prototype before implementation.
- [ ] Record unresolved risks and decisions; do not silently weaken approved requirements.
- [ ] Link the matrix from the implementation roadmap.
- [ ] Obtain user approval of the Phase 0 engineering baseline before starting Phase 1.

## Notes and limitations

- This is a first-pass traceability baseline. Test names, thresholds and detailed ownership will be refined during Phase 0.
- A requirement mapped to a phase is not necessarily implemented only in that phase; later phases may extend or revalidate it.
- Secure Boot, cryptographic configuration, sandbox strength and GPU support must not be described as implemented or certified until their acceptance tests pass.
- The final project licence remains intentionally deferred until the project-wide dependency and distribution audit.
