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
