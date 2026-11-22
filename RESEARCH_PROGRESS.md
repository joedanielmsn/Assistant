# Research Progress — Local Assistant 2.0

**Last updated:** 22 November 2026  
**Current stage:** Stage 1A — Architecture discovery  
**Implementation status:** Not started. No technology stack selected.

---

## Approved

| Topic | Status | Outcome |
|---|---|---|
| Deployment profiles | Approved | Shared platform supports desktop service, standalone desktop, headless/server and optional future system-integrated profiles |
| Multi-user architecture | Approved | Support personal per-OS-user installations and shared multi-user servers, with explicit identities/workspaces and strict private-data boundaries |
| Data ownership and persistence | Approved | Four logical scopes: installation, user, workspace and task/session; private by default, explicit sharing, owner-scoped background tasks and lifecycle controls |
| Central permission broker | Approved | Hybrid RBAC + ABAC; risk-based approval; user-wide grants bounded by user/data ownership; configurable lifetimes; deny-by-default, previews, audit, cancellation, revocation and emergency stop |
| Modular core and selective process isolation | Approved | Modular main core; isolate components only for independent lifecycle/resource needs, untrusted or sensitive work, failure containment, or specialised runtime/hardware requirements |
| Gaming-mode resource protection | Approved | Aggressively prioritize the game; avoid assistant GPU inference, optionally retain a tiny CPU-only model and lightweight tools, enforce adaptive CPU/RAM budgets, and pause nonessential background work |
| Core/client communication and IPC | Approved | Versioned HTTP-compatible API plus WebSocket events; Linux Unix-domain sockets; Windows named pipes; local identity verification; remote access opt-in via LAN or user-configured VPN/tunnel; built-in accounts with passkeys and optional external identity providers |
| Resource management and scheduling | Approved | Hybrid fixed safety limits plus adaptive priority scheduling; Normal/Performance/Gaming/Battery-Quiet modes; pressure-aware model unloading with configurable idle timeouts; weighted fair per-user scheduling and concurrency limits |
| Computer interaction and tool execution | Approved | Layered strategy: native APIs, accessibility, semantic browser automation, then visual interaction; common internal tool interface with MCP adapters; broad host access still brokered per operation; separate assistant browser profile; operation-level undo and checkpoints |
| Desktop experience and awareness | Approved | One primary desktop app with optional dashboard/tray/notifications/companion; continuous screen awareness with persistent indicators, immediate pause/disable, exclusions, transient processing by default and explicit permission for retention/cloud sharing; push-to-talk default with optional wake word |
| Intelligence and orchestration | Approved | Central stateful orchestrator with bounded adaptive execution; sequential default, selective parallelism, structured specialist contracts, deterministic enforcement, verified outcomes and safe recovery |
| Memory and knowledge | Approved | Layered scope-aware memory; review-based persistent memory writes; user-selected project monitoring; authorization-aware retrieval, provenance and governed lifecycle |


## Additional approved architecture areas — 22 November 2026

The user approved the computer-interaction and desktop-experience directions recorded in ADR-009 and ADR-010 and Masterplan Sections 28–29.

Key user selections:
- Broad host access rather than a fixed folder allowlist, with every operation still subject to central permission-broker evaluation and execution safeguards.
- Continuous screen awareness while the assistant is running, with a user-configurable opt-out. This is not covert recording; visible indicators, immediate controls, exclusions, transient processing by default and explicit retention/cloud-sharing permissions are mandatory.

Other choices in these areas follow the assistant's recommendations. Concrete libraries, UI frameworks, sandbox technologies and platform adapters remain undecided.

See [ARCHITECTURE_DECISIONS.md](./ARCHITECTURE_DECISIONS.md) for the decision records.

## Stage 1A — Explored / under evaluation

| Topic | Current understanding | Status |
|---|---|---|
| Overall architecture | Modular core with selectively isolated workers; avoid unnecessary microservices overhead | Approved; see ADR-005 |
| Core/client model | One authoritative core; desktop, CLI, browser, companion and future clients use a common versioned API | Approved; see ADR-007 |
| Resource scheduling policy | Fixed safety boundaries with adaptive priority scheduling | Approved; see ADR-008 |
| Performance modes | Normal, Performance, Gaming and Battery/Quiet, with automatic adaptation | Approved; exact budgets and adaptation rules deferred |
| Model lifecycle | Retain frequently used models; unload idle models under memory pressure with configurable idle timeouts | Approved; exact thresholds/timeouts deferred |
| Shared-server fairness | Weighted fair scheduling per user, per-user concurrency limits and interactive-task priority | Approved; weights and algorithms deferred |
| Client API and events | HTTP-compatible structured API plus WebSocket event stream | Approved direction; concrete libraries/schema deferred |
| Local IPC | Linux Unix-domain sockets with peer checks; Windows named pipes with explicit ACLs; local connections are not implicitly trusted | Approved direction; exact paths/ACL implementation deferred |
| Remote access and authentication | Remote access off by default; optional LAN/VPN/tunnel; built-in accounts with passkeys and optional external identity providers | Approved direction; concrete protocols/session implementation deferred |
| Process/security boundaries | Main core for orchestration and policy; separate workers only where lifecycle, isolation, failure-containment or specialised-runtime needs justify them | Direction approved; exact topology and sandboxing remain open |
| Data storage isolation | Hybrid strategy based on sensitivity and operational requirements | Direction approved; concrete implementation undecided |

## Recently approved architecture domains — 22 November 2026

The user approved the recommended directions for both intelligence/orchestration and memory/knowledge.

- **Intelligence/orchestration:** bounded adaptive central orchestration; sequential execution by default; selective parallelism; structured specialist contracts; deterministic validation and permission enforcement; task checkpoints, safe retries and outcome verification.
- **Memory/knowledge:** layered working, episodic, semantic and procedural memory plus separate knowledge repositories; review-based persistent memory writes; user-selected project monitoring; provenance and temporal validity; authorization-aware retrieval and propagation of updates/deletion.

See ADR-011 and ADR-012 in [ARCHITECTURE_DECISIONS.md](./ARCHITECTURE_DECISIONS.md) and Masterplan Sections 30–31.

## Recently approved architecture domains — 22 November 2026

The user approved the recommended directions for automation/workflows, the plugin ecosystem, and synchronization/backup/portability.

- **Automation:** event-driven durable workflows with explicit ownership, bounded execution, approval checkpoints, safe retries and resource-aware scheduling.
- **Plugins/integrations:** open community ecosystem with manifests, publisher/provenance verification, signatures, isolated workers, scoped credentials and mandatory broker enforcement.
- **Sync/portability:** local-first optional managed or self-hosted sync; end-to-end encryption for private data; controlled workspace key boundaries; separate encrypted backups and portable export/import; safe conflict handling.

See ADR-013 through ADR-015 in [ARCHITECTURE_DECISIONS.md](./ARCHITECTURE_DECISIONS.md) and Masterplan Sections 32–34.

## Deferred / undecided

- Technology stack and programming languages.
- Desktop/UI framework.
- Backend/runtime architecture details.
- Concrete IPC paths/ACLs, API schema and protocol implementation.
- Authentication/passkey protocols, session/token technology and remote-access integration.
- Database, vector store and concrete tenancy strategy.
- Encryption and key management.
- Exact resource scheduler implementation, workload budgets, adaptation thresholds and fairness weights.
- Exact retention, backup and deletion periods.
- Licensing (intentionally deferred; revisit at architecture finalisation and before public release).

---

## Working rules

1. Research before implementation.
2. Do not assume technologies from the previous repository or earlier experiments.
3. Keep approved decisions separate from proposals and deferred choices.
4. Inspect existing repository files before changing them.
5. Preserve established requirements when updating the master plan.
6. Use an autonomy-first decision process: the assistant should choose sound defaults for routine implementation, security and management details, and document the rationale without asking for approval of every detail.
7. Ask the user only about consequential product/design choices, significant performance trade-offs, or decisions with multiple genuinely comparable options. Explain these briefly and present clear choices.
8. Never treat a consequential user-facing or product-direction choice as approved without the user's input.
9. Keep the GitHub repository as the project documentation source of truth.


## Phase 1 — Core communication and IPC

**Status:** Complete — architecture direction approved on 22 November 2026.

Approved:
- One authoritative core; desktop, CLI, browser, companion and future clients communicate through a common versioned API.
- Versioned HTTP-compatible request/response API with WebSocket events for streaming, progress, state changes and notifications.
- Linux local IPC uses Unix domain sockets with OS peer-identity checks and restrictive permissions.
- Windows local IPC uses named pipes with explicit access-control configuration and client identity checks.
- Local IPC is not implicitly trusted; establish an authenticated client session and authorize every operation.
- Remote access is disabled by default; optional LAN access and secure remote access through a user-configured VPN or tunnel.
- Shared-server authentication uses built-in accounts with passkey support and optional external identity-provider integration.
- The core remains responsible for trusted identity, authorization and permission-broker enforcement for client and worker operations.

Deferred implementation details:
- HTTP/WebSocket libraries, API schema, socket/pipe paths and ACLs.
- Passkey and external identity-provider protocols, session/token lifecycle.
- LAN exposure controls, VPN/tunnel integration and API compatibility policy.
- Any specialised internal worker transport.

## Next phase

**Phase 2 — Resource manager and scheduling.** Research adaptive CPU/RAM/GPU/VRAM budgeting, task priorities, model lifecycle, gaming-mode transitions, foreground responsiveness and fair scheduling across users. Bring only major product/performance trade-offs to the user at the end of the phase.


## Phase 2 — Resource management and scheduling

**Status:** Complete — architecture direction approved on 22 November 2026.

Approved:
- Hybrid resource management: fixed safety boundaries with adaptive priority scheduling.
- Built-in Normal, Performance, Gaming and Battery/Quiet modes, complemented by automatic adaptation.
- Pressure-aware model lifecycle: retain frequently used models, unload idle models under memory pressure, and support configurable idle timeouts.
- Weighted fair scheduling per user, individual concurrency limits and interactive-task priority.
- Central coordination of CPU, RAM, GPU and VRAM, with platform-native resource controls behind platform adapters.
- Existing Gaming Mode behaviour in ADR-006 remains mandatory.

Deferred implementation details:
- Exact resource budgets, mode-specific limits and adaptation thresholds.
- Scheduling weights, queue algorithms and starvation prevention parameters.
- Model idle timeouts and GPU unload/preemption implementation.
- Power/thermal policies and concrete Linux/Windows APIs.
- Hardware-specific performance validation.

## Phase 3 — Intelligence and orchestration architecture

**Status:** Complete — architecture direction approved on 22 November 2026.

Approved:
- Central, stateful orchestrator with bounded adaptive execution and explicit step/time/token/resource limits.
- Sequential execution by default; selective parallelism for independent work.
- Structured specialist contracts with scoped context and structured results.
- Deterministic validation, authorization and permission-broker enforcement; models are not security authorities.
- Resource- and policy-aware model routing.
- Durable task state/checkpoints, safe retries, cancellation, worker-failure isolation and outcome verification.

Deferred implementation details:
- Model-routing algorithms, planner implementation, task-state schema and retry/idempotency mechanisms.
- Specialist taxonomy, evaluation datasets/benchmarks, model/runtime choices and orchestration libraries.

## Phase 4 — Memory and knowledge architecture

**Status:** Complete — architecture direction approved on 22 November 2026.

Approved:
- Separate working, episodic, semantic and procedural memory layers from source knowledge repositories.
- Review-based approval before persistent memory writes by default.
- User-selected project locations for indexing and monitoring; no silent expansion of monitoring scope.
- Provenance, confidence, timestamps and temporal validity; reconcile or version conflicting facts.
- Authorization-aware retrieval before content reaches models, with strict user/workspace isolation.
- Original documents remain authoritative; indexes/embeddings/summaries are derived and rebuildable.
- Governed update, revocation and deletion propagation; bounded task-specific context.

Deferred implementation details:
- Memory schemas, database/vector/index technology, embeddings, reranking and retrieval algorithms.
- Conflict resolution, retention/deletion guarantees, project watchers and review interface.



## Phase 5 — Automation and plugin ecosystem

**Status:** Complete — architecture directions approved on 22 November 2026.

Approved:
- Event-driven durable workflows integrated with the central orchestrator, resource manager and permission broker.
- Explicit ownership and user/workspace scope for automations; background jobs retain initiating authorization context.
- Bounded execution, approval checkpoints, pause/cancellation, audit history and safe retry/deduplication safeguards.
- Open community plugin ecosystem with manifests, publisher/provenance verification, signatures and clear trust levels.
- Isolated plugin workers with OS-enforced restrictions; every privileged operation remains brokered.
- Scoped integration credentials; updates re-evaluate material permission changes; uninstall revokes grants/credentials and removes scheduled/event hooks.
- Common internal tool contract with MCP adapters, not MCP-only architecture.

Deferred implementation details:
- Workflow format, scheduler, event transport, persistence schema and automation UI.
- Plugin manifest/SDK, registry governance, package/signing infrastructure, sandbox technology, compatibility and moderation operations.

## Phase 6 — Synchronization, backup and portability

**Status:** Complete — architecture directions approved on 22 November 2026.

Approved:
- Local-first operation with optional synchronization.
- Both managed and self-hosted sync deployments.
- End-to-end encryption for private synchronized data, with separate controlled key boundaries for shared workspace data.
- Device enrollment/revocation and documented key/device recovery.
- Safe automatic merging where reliable; preserve versions and surface meaningful conflicts.
- Separate encrypted, versioned backup/recovery and user-readable export/import capabilities.
- Multiple backup destinations, integrity verification and regular restore testing.
- Documented versioned portable formats; imported plugins and executable automations remain inactive until reviewed and authorized.

Deferred implementation details:
- Sync protocol, managed-service operations, cryptographic and key-recovery protocols.
- Backup formats, retention periods, export schemas, migration tooling, conflict-resolution algorithms and deletion guarantees.


## Phase 7 — AI-first Linux distribution architecture

**Status:** Complete — major architecture direction approved on 22 November 2026.

Approved:
- Build on CachyOS as the technical foundation while creating an independently designed and maintained distribution, not a simple reskin.
- Establish independent branding, visual language, desktop defaults, settings experience, system utilities and workflows.
- Use KDE Plasma as the initial desktop foundation with a substantially redesigned experience; retain modularity for future changes.
- Make assistant integration a first-class part of the OS from the beginning. The distribution is deliberately AI-first.
- AI services can be paused or disabled without breaking ordinary OS and desktop functionality.
- Keep privileged system operations subject to the central permission broker; system integration does not grant unrestricted authority.
- Maintain our own repositories, build/testing pipeline, release identity, documentation and quality controls.
- Use a curated rolling-release model with our own testing and stable channels; upstream CachyOS updates are not automatically promoted.
- Preserve developer, gaming, laptop hardware compatibility, installation and recovery as core requirements.

Deferred implementation details:
- Distribution name/branding and exact CachyOS components to reuse or replace.
- Repository/build infrastructure, signing and package promotion policy.
- KDE customisation and desktop integration implementation.
- Kernel variants, installer, filesystem/partitioning defaults and Secure Boot.
- Hardware profiles, recovery, rollback and release-support policy.

See ADR-016 in [ARCHITECTURE_DECISIONS.md](./ARCHITECTURE_DECISIONS.md) and Masterplan Section 35.

## Architecture discovery status

The six assistant-platform architecture domains and the AI-powered Linux distribution direction have now been researched and approved at the architectural level. The next stage is architecture consolidation and cross-cutting review before implementation planning. No implementation technology stack has been finalised.

## Architecture consolidation — in progress

Cross-cutting review is underway before implementation planning.

Newly approved:
- Privileged OS operations use the central permission broker as the authoritative authorization layer; no second interactive OS authorization prompt is required for every operation. Narrow OS helpers and OS-enforced least-privilege controls remain mandatory (ADR-017).
- Initial supported hardware architecture is x86-64 (AMD64) only. ARM64 and other architectures are outside the initial support commitment (ADR-018).

Review areas:
- Security boundaries and privileged OS integration.
- CPU/GPU and device compatibility, including laptops and gaming systems.
- Deployment, packaging, updates, rollback and release engineering.
- Linux/Windows consistency and the AI distribution's relationship to the shared platform.
- Technology-stack evaluation against the approved architecture.

Technology selection remains open; implementation has not started.


### Additional approved platform constraint — 22 November 2026

- **CPU baseline:** x86-64-v3 is the minimum instruction-set baseline for official builds and supported systems. x86-64-v4 remains optional for compatible processors; older x86-64 CPUs without v3 are outside initial support (ADR-019).

The cross-cutting hardware review continues with GPU, hybrid-graphics, laptop, power-management and gaming compatibility. Technology selection and implementation planning remain open.


## Phase 8 — Architecture consolidation and technology selection

**Status:** In progress.

Completed cross-cutting decisions:
- Privileged system operations use the central permission broker as the authoritative authorization layer; narrowly scoped OS helpers and OS-enforced least-privilege controls remain mandatory (ADR-017).
- Initial supported CPU architecture is x86-64 only (ADR-018).
- Minimum CPU instruction-set baseline is x86-64-v3; x86-64-v4 remains optional (ADR-019).
- Primary desktop application framework is Qt 6 + QML (ADR-020).

Technology candidates under evaluation; not yet approved as a complete stack:
- Rust + Tokio for the authoritative core.
- Python for AI and specialist workers.
- llama.cpp for local LLM inference.
- ONNX Runtime for specialized inference.
- SQLite for personal desktop persistence and PostgreSQL for shared-server persistence.
- SQLite FTS5 for initial local full-text search.
- Versioned HTTP-compatible API and WebSockets.
- Internal typed event system and durable database-backed workflows initially.

Remaining consolidation work:
- Validate complete technology combinations and platform support.
- Evaluate sandboxing, plugin execution, build/release infrastructure and deployment packaging.
- Complete hardware compatibility and performance constraints.
- Resolve data/storage, cryptography, sync and workflow implementation dependencies.
- Revisit licensing before architecture finalisation and again before public release.

No implementation stack has been approved as a whole, and implementation has not started.

## Phase 8 — Architecture consolidation and technology selection

**Status:** In progress — initial technology architecture approved on 22 November 2026; implementation specifications and validation remain.

### Newly approved

- Rust + Tokio for the authoritative core.
- Python AI/specialist workers in separate supervised, restartable processes; Python is not a security or authorization authority.
- Versioned JSON over supervised local IPC as the initial worker protocol; gRPC/Protocol Buffers deferred unless measured requirements justify them.
- llama.cpp as the initial local LLM backend and ONNX Runtime for suitable specialized inference, behind a common model-manager/backend interface.
- SQLite for personal desktop persistence; PostgreSQL for shared-server persistence; SQLite FTS5 for initial local full-text search.
- SQLx as the initial Rust database-access candidate, subject to validation.
- Wasmtime/WebAssembly as the preferred portable plugin sandbox where capabilities fit; native integrations use separately isolated OS-restricted workers.
- Bubblewrap plus Linux kernel controls as Linux native-worker sandbox candidates; Windows AppContainer where compatible. Platform parity and actual isolation strength require validation.
- Internal typed event system and durable database-backed workflow engine initially; external event/workflow infrastructure deferred until justified by measured needs.
- Versioned HTTP-compatible API and WebSockets remain the client-facing communication direction.

### Remaining Phase 8 work

- Define worker protocol schemas, process supervision, lifecycle, cancellation, deadlines and failure recovery.
- Validate Python runtime/dependency packaging and Rust/Python compatibility across Linux and Windows.
- Threat-model and test WebAssembly plugin capabilities, native worker sandbox profiles and privileged helper boundaries.
- Validate llama.cpp and ONNX Runtime model/provider combinations across supported CPU/GPU hardware, including hybrid graphics and the RTX 3050 4 GB development target.
- Define database schemas, migrations, encryption/key management, authorization enforcement, backup and deletion behaviour.
- Specify workflow persistence, transactional outbox, retries, idempotency and event delivery.
- Evaluate build system, package formats, signing, update/rollback and reproducible release processes.
- Revisit licensing before architecture finalisation and public release.

The initial technology direction is approved, but detailed implementation specifications, compatibility/security validation and release engineering are not complete. Implementation has not started.

### Additional approved consolidation decision — repository, build and release architecture

**ADR-022 approved on 22 November 2026.**

- Use a platform monorepo for the Rust core, Qt/QML desktop, Python workers, plugins/SDK and shared platform tooling.
- Maintain the CachyOS-derived Linux distribution in a separate repository with its own build, testing and release cadence.
- Use Cargo workspaces for Rust, CMake + Ninja for Qt, and uv-managed locked environments for Python workers.
- Use GitHub Actions as the initial CI/CD platform, with Linux/Windows build matrices and separate hardware-dependent inference validation.
- Prefer native Linux packages; maintain signed distribution repositories and controlled testing/stable promotion.
- Package Windows desktop deployment with required Qt dependencies and separately managed core/worker components where practical.
- Require pinned dependencies, traceable release artifacts, compatibility checks, staged updates and documented recovery/rollback.

### Remaining build/release research

- Define repository layout, branch/review policy and component version compatibility.
- Specify CI workflows, runner strategy, artifact retention and release provenance.
- Select Windows installer and exact Linux package formats.
- Design signing-key custody/rotation, package promotion and update/rollback mechanisms.
- Validate clean builds, reproducibility, upgrades and recovery on supported Linux and Windows systems.
- Keep licensing review scheduled for architecture finalisation and public release.


### Final cross-cutting questionnaire — decisions recorded 22 November 2026

**Status:** User decisions recorded in ADR-023; Phase 8 remains in progress pending detailed security, compatibility and implementation validation.

Approved directions:
- OS identity by default on personal desktops; optional stronger authentication for sensitive operations.
- Built-in accounts/passkeys and Linux-only initial shared-server target; external identity providers deferred.
- Application-level encryption enabled by default; user-controlled master password and recovery key.
- Configurable category-specific retention and user-defined sensitivity policies.
- Configurable cloud inference modes and broad provider/custom-endpoint compatibility.
- Self-hosted synchronization only; safe automatic merges and surfaced conflicts.
- Automatic model routing with manual overrides; internal and external runtime support.
- Built-in, user-created and community specialists; configurable autonomy.
- NVIDIA-first initial inference validation while retaining AMD/Intel support goals.
- Automatic and manual resource/Gaming Mode controls.
- Verified/community/local plugin trust levels, official and third-party registries, and maintained official integrations.
- Multiple desktop companion and voice interaction modes; independently configurable metadata and screen awareness.
- CachyOS-derived distribution initially, with a long-term independent identity; redesigned KDE, dual-mode installer, Btrfs/ext4 and conventional curated rolling updates.
- No telemetry; private vulnerability reporting plus public issue tracking; configurable audit policies.
- Core licence remains deferred for a project-wide review; compatible open-source plugin licences and optional clearly labelled proprietary distribution components.

Important interpretation:
- Conventional rolling updates do not eliminate the need for Btrfs snapshots, package recovery, controlled testing/stable promotion or documented rollback.
- Master-password and recovery-key design must account for unattended desktop startup without exposing key material to workers/plugins or weakening user/workspace authorization.
- Self-hosted sync is the approved product direction; no managed hosted sync service is currently planned.

Remaining Phase 8 work:
- Threat modelling, sandbox validation and privileged-helper review.
- Cryptographic design, key derivation/storage/recovery, database isolation and deletion/retention guarantees.
- Self-hosted synchronization protocol and conflict-resolution specification.
- Cloud disclosure and sensitive-data enforcement.
- Hardware/runtime compatibility and performance testing across supported CPU/GPU combinations.
- Installer, conventional rolling update, snapshot/package rollback and recovery design.
- Plugin signing, registry governance, sandbox profiles and SDK contracts.
- Project-wide licence and dependency compatibility audit before architecture finalisation and public release.

See ADR-023 and Masterplan Section 42. Implementation has not started.


### Phase 8 technical consolidation — ADR-024 through ADR-029

**Status:** Initial cross-cutting technical directions recorded 22 November 2026. Detailed implementation and independent validation remain mandatory.

Completed:
- **Security/threat model (ADR-024):** Threat-model coverage, ASVS 5.0 reference, layered Linux isolation (Bubblewrap, Landlock, seccomp and resource controls), Windows AppContainer path, Wasmtime capability restrictions, fail-closed behaviour and security verification gates.
- **Encryption/data isolation (ADR-025):** Envelope encryption, Argon2id master-password derivation, separate recovery key, scoped user/workspace keys, protected in-memory key lifecycle, opt-in OS-vault convenience unlock, encrypted SQLite direction and application-layer PostgreSQL protection.
- **Synchronization (ADR-026):** Self-hosted only. This supersedes the managed synchronization service option in ADR-015. E2EE private payloads, trusted device enrollment/revocation, separate workspace key boundaries and safe conflict handling.
- **Hardware (ADR-027):** CPU fallback, NVIDIA-first validation, AMD/Intel support goals, backend-specific compatibility matrix, hybrid graphics and resource-aware inference testing.
- **Updates/recovery (ADR-028):** Conventional curated rolling updates, signed native packages, Btrfs snapshots with honest rollback limits, ext4 recovery procedures, TUF-style secure metadata for applicable app/plugin artifacts, SBOM/provenance.
- **Plugin/licence governance (ADR-029):** Manifest and capability verification, signatures as provenance rather than safety proof, official and third-party registries, SPDX metadata, SBOMs and project-wide licence review.

Research references consulted:
- OWASP ASVS 5.0: https://asvs.owasp.org/
- RFC 9106 Argon2: https://www.rfc-editor.org/rfc/rfc9106.html
- libsodium key derivation and secretstream: https://doc.libsodium.org/key_derivation and https://doc.libsodium.org/secret-key_cryptography/secretstream
- Linux Landlock and seccomp: https://docs.kernel.org/userspace-api/landlock.html and https://docs.kernel.org/userspace-api/seccomp_filter.html
- Bubblewrap: https://github.com/containers/bubblewrap
- Microsoft application isolation: https://learn.microsoft.com/en-us/windows/security/book/application-security-application-isolation
- llama.cpp backend build guide: https://github.com/ggml-org/llama.cpp/blob/master/docs/build.md
- ONNX Runtime execution providers: https://onnxruntime.ai/docs/execution-providers/
- TUF: https://theupdateframework.github.io/specification/latest/
- Arch package signing: https://wiki.archlinux.org/title/Pacman/Package_signing
- Sigstore: https://docs.sigstore.dev/about/overview/
- SPDX: https://spdx.org/licenses/

Remaining before architecture sign-off:
- Review and validate cryptographic formats, Rust crypto bindings, SQLCipher/FTS5 compatibility, secure key lifecycle and recovery UX.
- Build and test Linux/Windows sandbox profiles, including adversarial negative tests and independent review.
- Define self-hosted sync wire protocol, device enrollment and workspace key rotation.
- Establish tested hardware/driver/runtime compatibility and performance thresholds.
- Validate Btrfs snapshot/boot recovery, ext4 recovery, conventional rolling update failure handling and Windows update/rollback.
- Define TUF role/key custody, signing ceremony, plugin registry moderation and SBOM tooling.
- Complete project-wide licence/dependency and asset/model rights audit.

No implementation has started. These are architecture directions and validation requirements, not claims that security controls have already been implemented or certified.


### Additional Phase 8 integration decisions — ADR-030 and ADR-031

Recorded 22 November 2026:
- Core lifecycle: per-user systemd service on Linux desktop; unprivileged system service account on Linux server; per-user background process on Windows.
- API: Axum/Tokio, versioned HTTP-compatible API and WebSockets, OpenAPI 3.1, JSON Schema-compatible worker/plugin contracts.
- Local IPC: Linux Unix sockets and Windows named pipes with peer identity/ACL checks; supervised worker JSON over stdio.
- Desktop: Qt Quick/QML, Qt Quick Controls, thin C++ transport/view-model layer, Qt Network and Qt WebSockets.
- SQLite stays on local filesystems; WAL is same-host only. PostgreSQL is the shared-server backend.
- Installer: Calamares with project-owned configuration/branding, beginner guided and advanced manual modes; UEFI/GPT and Btrfs recommended defaults, ext4 option.
- Optional LUKS2 system encryption is separate from assistant data encryption.
- Secure Boot is a supported target, not a mandatory setting; signed installation/update/recovery chain must be validated before claiming support.

Research references:
- Axum: https://github.com/tokio-rs/axum
- Tokio local IPC: https://docs.rs/tokio/latest/tokio/net/
- Qt Quick and Controls: https://doc.qt.io/qt-6/qtquick-index.html and https://doc.qt.io/qt-6/qtquickcontrols-index.html
- Qt WebSockets: https://doc.qt.io/qt-6/qtwebsockets-index.html
- SQLite WAL constraints: https://www.sqlite.org/wal.html
- Calamares: https://calamares.io/about/
- CachyOS Secure Boot: https://wiki.cachyos.org/configuration/secure_boot_setup/

Remaining before Phase 8 sign-off:
- Prototype/test the local client transport adapters and worker IPC framing on Linux and Windows.
- Verify SQLCipher FTS5 build configuration and migration/backup behaviour.
- Select the distribution bootloader and Secure Boot key enrollment/signing design through an end-to-end prototype.
- Validate Calamares partitioning and LUKS2 workflows on representative UEFI systems.
- Finalize API/schema and service lifecycle specifications.
- Complete independent security review and project-wide licensing audit.

No implementation has started. These decisions establish design defaults; platform and distribution validation remains a prerequisite to implementation milestones.


### Additional implementation architecture refinements — ADR-032 and ADR-033

Recorded 22 November 2026:
- Use rusqlite with bundled SQLCipher and FTS5 for personal desktop databases; run synchronous work on a bounded blocking pool.
- Use SQLx PostgreSQL for shared-server persistence; do not enable SQLx SQLite in the same binary by default.
- Test the exact SQLCipher build and FTS5 configuration on both Linux and Windows.
- GRUB is the initial Linux distribution bootloader default, selected for broad compatibility and Btrfs snapshot boot integration.
- Secure Boot must retain signature verification/lockdown; prefer signed shim/MOK or another reviewed chain preserving existing firmware keys. Exact signing/enrollment remains a prototype validation task.

Research references:
- rusqlite SQLCipher feature documentation: https://docs.rs/rusqlite/latest/rusqlite/
- SQLx SQLite driver documentation: https://docs.rs/sqlx/latest/sqlx/sqlite/
- GNU GRUB Secure Boot manual: https://www.gnu.org/software/grub/manual/grub/grub.html
- Arch Secure Boot guidance: https://wiki.archlinux.org/title/Secure_Boot
- GRUB Btrfs snapshot integration: https://man.archlinux.org/man/grub-btrfs.8.en

Remaining:
- Verify Cargo feature unification and native library linkage in the real workspace.
- Validate SQLCipher FTS5, backup/restore and WAL across platforms.
- Prototype GRUB snapshot boot and Secure Boot end-to-end, including kernel and module updates.


### Cloud inference privacy policy — ADR-034

Recorded 22 November 2026:
- Local-only is the default until cloud use is configured.
- Automatic cloud fallback remains disabled until explicitly enabled.
- Sensitivity and workspace restrictions override global provider settings.
- Screen, voice and credential content are excluded from cloud transmission by default and require separate explicit authorization.
- Cloud provider secrets are protected and unavailable to plugins/workers.
- Provider-side retention/training cannot be guaranteed by the assistant and must be disclosed.
- Cloud requests are user-authorized product traffic, not telemetry.

See ADR-034 and Masterplan Section 48.


### No-telemetry and diagnostics policy — ADR-035

Recorded 22 November 2026:
- No analytics, usage tracking, behavioural profiling, remote crash reporting or automatic diagnostic uploads.
- Diagnostics remain local by default; user-controlled preview/export is allowed.
- Functional network requests for configured sync, cloud inference, updates and plugin/model downloads are disclosed and are not telemetry.
- Server audit records remain narrowly scoped and separate from product analytics.

See ADR-035 and Masterplan Section 49.
