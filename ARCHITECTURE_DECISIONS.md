# Architecture Decisions — Local Assistant 2.0

**Last updated:** 22 November 2026  
**Status:** Architecture discovery in progress. The entries below are explicitly approved directions, not implementation selections.

This file records decisions that the user has approved. Proposals and unresolved implementation choices belong in the research tracker until approved.

---

## ADR-001 — Shared core with four deployment profiles

**Status:** Approved  
**Date:** 2026-11-22

### Decision
Support four deployment profiles using shared core capabilities:

1. Desktop service: core starts at login and remains available when the main UI closes; UI connects independently.
2. Standalone desktop: core starts with the GUI and stops when the application closes.
3. Headless/server: no GUI; authenticated local or remote clients.
4. Optional system-integrated mode: deeper OS integration may be added later, with privileged actions brokered.

### Constraints
- These are profiles of one platform, not separate products/codebases.
- Linux desktop defaults to a per-user service, not a privileged system-wide daemon.
- Windows desktop uses per-user background processes and a separate interactive desktop process; avoid running the entire assistant as a privileged Windows service.
- Local IPC is not implicitly trusted.
- Background work is permission-controlled and resource-managed.

### Not decided
Service manager, IPC transport, authentication technology, exact process boundaries and implementation details.

---

## ADR-002 — Support personal and shared multi-user deployments

**Status:** Approved  
**Date:** 2026-11-22

### Decision
Support both personal per-OS-user desktop installations and shared multi-user server deployments.

### Personal installation
Each OS user has an independent core and separate private conversations, memory, settings, permissions and background tasks.

### Shared server
A managed core serves authenticated users with strict data separation, per-user permissions, workspace membership and fair resource allocation. A user may belong to multiple workspaces.

### Mandatory constraints
- Private data stays private unless explicitly shared.
- Every operation carries trusted user and workspace authorization context.
- Background tasks retain the initiating owner's identity, scope and authorization.
- Resource scheduling is bounded machine-wide and fair across users.
- Server administration is separate from unrestricted access to private user data.
- Initial delivery prioritizes the personal per-user core while preserving identity/workspace interfaces for future shared-server operation.
- Test multiple simulated users early.

### Not decided
Database isolation, authentication, encryption and scheduler implementation.

---

## ADR-003 — Four-scope data ownership and persistence model

**Status:** Approved  
**Date:** 2026-11-22

### Decision
Use four logical ownership scopes. They do not imply four separate databases.

| Scope | Owns |
|---|---|
| Installation | Machine/server configuration, installed components and runtimes, hardware capabilities, machine-wide resource limits and server security configuration |
| User | Private conversations, personal memory, preferences, personal knowledge/documents, personal credentials, permissions and personal automation |
| Workspace | Explicitly shared projects, documents, workspace knowledge, shared integrations, tasks and workspace policies |
| Task/session | Temporary execution state, files/results, agent context, approval state and task execution history |

### Mandatory constraints
- User data is private by default; sharing must be explicit, scoped and revocable.
- Personal memory and workspace knowledge are distinct.
- Using information is not the same as saving it permanently; conversation content is not automatically long-term memory.
- Agents receive only authorized personal and workspace context.
- Trusted identity and workspace context are established by the core, not self-asserted by agents or plugins.
- Authorization applies to every resource operation and considers subject, resource, scope, acting component, action and relevant conditions.
- Background tasks retain owner/workspace context, permissions, operation scope, expiry and cancellation state; delayed tasks may require authorization revalidation.
- Shared inference compute must not cause cross-user sharing of conversations, memory, retrieval results, caches, summaries, tool outputs or approval state.
- Export, deletion, retention and workspace membership revocation must account for derived indexes, caches and delayed work.
- Backup expiry/deletion limitations must be documented honestly.
- Server administration does not confer unrestricted access to private user data.

### Storage direction
Evaluate a hybrid isolation approach based on sensitivity and operational requirements. Separate stores may offer stronger boundaries at higher overhead; shared stores require consistently enforced authorization. This is a direction for further evaluation, not a selected database design.

### Not decided
Database/storage technology, tenancy mechanism, encryption/key management, exact retention periods and backup lifecycle.


## ADR-004 — Central permission broker and authorization lifecycle

**Status:** Approved  
**Date:** 2026-11-22

### Decision

All privileged actions initiated by the assistant platform must pass through one central permission broker. Agents, plugins, integrations, tools and automations cannot bypass the broker, grant themselves permissions or self-assert an authorization identity.

| Dimension | Approved decision |
|---|---|
| Authorization | Hybrid RBAC + ABAC: roles establish baseline access; contextual attributes and policy conditions evaluate each action |
| Approval | Risk-based: low-risk actions may run automatically when policy permits; sensitive, consequential or destructive actions require confirmation |
| Scope | User-wide: grants belong to the individual user who issued them and may apply across that user's authorized contexts; they are not installation-wide and never transfer to another user |
| Lifetime | Configurable: permanent, session-scoped, task-scoped or time-limited grants, with expiry and revocation |

### Mandatory enforcement rules

- Evaluate every privileged operation using trusted identity and context: requesting component, user, workspace where applicable, target resource, action, risk and applicable policy conditions.
- User-wide grants do not override data ownership, workspace membership, resource-level restrictions or other security boundaries.
- A user's grant never authorizes access to another user's private data or grants privileges to another user.
- More restrictive applicable resource, workspace, application or policy restrictions take precedence over a broader user-wide grant.
- Deny access by default when identity, scope or authorization is unclear.
- Background and delayed tasks retain their initiating user's identity, scope and authorization context; revalidate permissions when appropriate before execution.
- Provide understandable permission records, expiry and revocation.
- Provide previews for consequential actions, audit history, individual action/task cancellation and an emergency stop.
- Restrict access to audit records and minimize unnecessary sensitive content.

Risk-based autonomy is configurable. Harmless, reversible actions may run automatically only when the applicable user policy permits them. Users may choose ask-before-every-action behaviour.

### Not decided

The concrete policy engine, permission storage format, authentication/session technology, risk-classification implementation, approval UI, audit storage and enforcement implementation remain open for detailed design and technology research.

---
---

## ADR-005 — Modular core with selectively isolated workers

**Status:** Approved  
**Date:** 2026-11-22

### Decision

Use a **modular core with selectively isolated processes**, rather than a monolithic single process or a broad microservices architecture.

The main core owns orchestration, authorization, resource policy and shared platform state. Components remain modules in the core unless a concrete isolation or lifecycle requirement justifies a process boundary.

A component should become a separate process when one or more of these conditions apply:

1. It needs an independent lifecycle or resource-management boundary.
2. It handles untrusted code or sensitive operations that require stronger isolation.
3. Its failure must not be allowed to crash or destabilize the main core.
4. It requires a specialised runtime, hardware access or dependency environment.

Otherwise, keep it inside the core as a module. Process separation alone is not a security sandbox; untrusted workers require actual OS-enforced restrictions and controlled interfaces.

### Initial boundary direction

The following is an initial architectural baseline, not an irrevocable process-by-process commitment:

| Component | Initial direction |
|---|---|
| Main orchestrator | Main core |
| Permission broker | Main core |
| Resource manager | Main core |
| Memory and knowledge | Main core initially |
| Event system | Main core initially |
| Desktop UI | Separate client process |
| Inference runtime | Independent process |
| Specialist agents | On-demand, bounded worker pool |
| Plugins | Restricted isolated processes |
| Browser/desktop automation | Isolated workers |
| Voice processing | On-demand worker |

### Operational constraints

- A worker failure must not bring down the main core; workers should be restartable where appropriate.
- Specialist workers should be on-demand and bounded, not permanently spawned without need.
- Share inference runtimes where practical; avoid duplicating large model instances unnecessarily.
- Worker execution remains subject to the central permission broker and resource manager.
- Process boundaries, sandbox strength and lifecycle may be refined as concrete workloads and platform constraints are researched.

### Not decided

Exact process topology, worker protocol, IPC transport, sandbox technologies, service managers, restart policy and platform-specific implementation remain undecided.

## Decision status rules

- **Approved:** explicitly confirmed by the user.
- **Proposed / under evaluation:** researched or recommended but not yet approved.
- **Deferred / undecided:** intentionally left open for a later decision.
- Do not promote proposals or deferred items to approved without explicit user confirmation.


## ADR-006 — Gaming-mode resource protection

**Status:** Approved  
**Date:** 2026-11-22

### Decision

Gaming Mode uses aggressive resource protection to prioritize the game while retaining an optional minimal assistant capability on the CPU.

### Approved behaviour

- Avoid GPU inference during Gaming Mode and release assistant GPU/model allocations when needed so the GPU remains available to the game.
- Optionally keep a very small CPU-only model, with an initial target range around 0.4B–0.8B parameters. Actual model selection depends on quality, memory use, runtime support and tool-use reliability.
- Preserve basic web search and selected lightweight tools where practical.
- Apply strict CPU and RAM budgets so assistant activity does not materially compete with the game.
- Pause or defer nonessential background work such as indexing, maintenance, model downloads and nonessential automations.
- Restore normal scheduling and model availability when Gaming Mode ends.

### Implementation guidance

- Do not rely on a tiny language model as the sole tool router or security authority. Use deterministic routing, schemas, validation and the central permission broker for tool execution.
- Resource protection must be adaptive: the assistant should not assume that a fixed CPU/RAM allowance is safe for every game or machine.
- Provide a reliable way to enter and leave Gaming Mode, and recover gracefully if a game exits unexpectedly or a mode transition is interrupted.
- Keep essential safety, user control and explicitly permitted lightweight functions available.

### Not decided

Exact CPU/RAM budgets, model/runtime choice, tool allowlist, game detection mechanism and transition implementation remain subject to hardware testing and implementation research.


## ADR-007 — Core/client communication, IPC and remote access

**Status:** Approved  
**Date:** 2026-11-22

### Decision

Use one authoritative core as the platform's control and security boundary. Desktop interfaces, CLI clients, browser interfaces, companion surfaces and future clients communicate with the core through a versioned, language-neutral API. Clients do not access private stores, privileged tools or protected resources directly.

| Dimension | Approved direction |
|---|---|
| Common client API | Versioned HTTP-compatible API with structured request/response operations |
| Live events | WebSocket channel for streaming responses, task progress, state changes and notifications |
| Linux local IPC | Unix domain sockets, with OS peer-identity checks and restrictive socket permissions |
| Windows local IPC | Named pipes, with explicit access-control configuration and client identity checks |
| Local trust | Local IPC is not implicitly trusted; establish an authenticated client session and authorize each operation |
| Remote access | Disabled by default; optional LAN access and secure remote access through a user-configured VPN or tunnel |
| Shared-server authentication | Built-in accounts with passkey support, plus optional external identity-provider integration |
| Authorization | Core-side authorization on every operation, enforced through the central permission broker and trusted user/workspace context |

### Mandatory constraints

- The same API contract should serve desktop, headless, browser and future mobile clients.
- Keep API contracts independent of a particular programming language or transport implementation.
- Remote access must be explicitly enabled and securely authenticated; do not expose unauthenticated development endpoints.
- Authentication establishes identity; it does not itself grant resource or action permissions.
- Worker and plugin identities are constrained by the core and cannot impersonate users or bypass the permission broker.
- Clients must handle API version negotiation and unsupported capabilities explicitly.
- Internal worker communication may use a specialised transport later if justified by measured requirements, while preserving trusted identity and authorization context.

### Not decided

Concrete HTTP server/library, WebSocket implementation, API schema format, local socket/pipe path and ACL details, passkey implementation, external identity-provider protocols, session/token format and lifecycle, LAN exposure controls, VPN/tunnel integration, API compatibility policy and worker-specific transport remain subject to implementation research.


## ADR-008 — Adaptive resource management and workload scheduling

**Status:** Approved  
**Date:** 2026-11-22

### Decision

Use a hybrid resource-management strategy: enforce fixed safety boundaries while dynamically scheduling workloads according to priority, current resource pressure and available capacity.

| Dimension | Approved direction |
|---|---|
| Scheduling | Hybrid fixed safety limits with adaptive priority scheduling |
| User-facing modes | Normal, Performance, Gaming and Battery/Quiet, with automatic adaptation |
| Model lifecycle | Retain frequently used models; unload idle models when memory pressure rises; support configurable idle timeouts |
| Multi-user fairness | Weighted fair scheduling per user, individual concurrency limits and priority for interactive tasks |

### Mandatory behaviour

- Coordinate CPU, RAM, GPU and VRAM management through a central resource manager.
- Observe actual resource pressure and foreground workload demand; do not rely only on static utilization thresholds.
- Fixed safety limits remain in force even when adaptive scheduling changes workload allocations.
- Prioritize foreground responsiveness and active user-requested work over deferrable background activity.
- Pause, defer or reduce nonessential indexing, maintenance, downloads and automations under resource pressure.
- Avoid unnecessary duplicate model instances; share runtimes where practical.
- Model loading and unloading must account for memory pressure, expected reuse and configurable idle timeouts.
- Gaming Mode continues to follow ADR-006: aggressively prioritize the game, avoid assistant GPU inference, optionally retain a tiny CPU-only model and lightweight tools, enforce adaptive CPU/RAM limits, and pause nonessential background work.
- Performance and Battery/Quiet modes must respect system stability and user-selected preferences.
- Shared-server scheduling must prevent one user from monopolizing resources while preserving interactive responsiveness and avoiding indefinite starvation.
- Resource controls should use platform-native mechanisms where appropriate, behind platform adapters rather than leaking OS-specific behaviour into core scheduling policy.

### Not decided

Exact CPU/RAM/GPU/VRAM safety limits, scheduling weights and algorithms, mode-specific budgets, adaptation thresholds, model idle timeouts, GPU preemption/unload behaviour, power/thermal policy, platform APIs and hardware-specific validation remain implementation research topics.


## ADR-009 — Computer interaction and tool execution architecture

**Status:** Approved  
**Date:** 2026-11-22

### Decision

Use a layered computer-interaction strategy. Prefer the most reliable structured interface available, in this order:

1. Native application APIs and structured integrations.
2. OS accessibility interfaces.
3. Browser automation using semantic controls and locators.
4. Visual screen understanding with keyboard and pointer interaction as a fallback.

Use a common internal tool interface with adapters for external protocols such as MCP. MCP is supported as an integration path, not mandated as the platform's sole internal tool contract.

### Host access and execution safeguards

The user selected broad host access rather than a fixed folder allowlist. This does **not** grant tools ambient or unrestricted access.

- File, process, terminal and system operations must be individually evaluated by the central permission broker.
- Every operation carries trusted user, workspace, component, target-resource and action context.
- The broker applies the approved risk-based policy and user-wide grant rules from ADR-004; broader user grants never bypass resource ownership or other restrictions.
- Terminal requests must be structured, validated and attributable; previews and confirmation are required according to risk.
- Execution should occur in isolated, restartable workers where appropriate, with OS-enforced restrictions and resource limits.
- Validate paths and targets at execution time; do not rely solely on model-generated descriptions or earlier checks.
- Read-only operations and modifying/executing operations must be distinguishable.
- Tool schemas, timeouts, cancellation, execution identifiers and auditable outcomes are required.
- Untrusted tool output, web content and files are data, not instructions that can override system policy.

### Browser identity

Use a separate assistant-managed browser profile by default. Access to the user's existing browser profile is an explicit, separately authorized capability.

### Recovery

Use operation-level undo where supported and checkpoints for consequential or bulk changes. Clearly identify actions that cannot be reliably undone; do not promise rollback where the external system provides no such guarantee.

### Not decided

Concrete tool schema, MCP adapter design, terminal command policy, worker sandbox technology, OS-specific accessibility and browser automation libraries, exact approval thresholds, and implementation-specific undo/checkpoint mechanisms remain open.

---

## ADR-010 — Desktop awareness, voice and unified desktop experience

**Status:** Approved  
**Date:** 2026-11-22

### Decision

Provide one primary desktop application with optional dashboard, tray controls, notifications and floating companion surfaces. These are clients or views of the authoritative core, not separate authorities or independent permission systems.

### Desktop awareness

The user selected continuous screen awareness while the assistant is running, with user-configurable opt-out.

This is continuous awareness, **not permission for covert recording**. Mandatory safeguards:

- Show a persistent, unmistakable indicator whenever screen content is being captured or analysed.
- Provide immediate pause and disable controls.
- Provide application/window exclusions and configurable capture scope.
- Do not silently retain screen recordings or captured frames. Default to transient processing and no persistent capture history.
- Make any feature that stores screenshots, extracts persistent content or shares screen data a separate, explicit permission and retention choice.
- Respect OS capture permissions and platform privacy controls.
- Keep processing local by default where practical; disclose and obtain permission before sending screen content to a cloud service.
- Do not capture the lock screen, secure desktop or other OS-protected surfaces where the platform prohibits it.
- Application metadata awareness and screen-content awareness must be separately controllable.

### Voice

Push-to-talk is the default. Optional wake-word activation may be explicitly enabled. Voice controls must include visible microphone/listening status, immediate mute/disable, configurable local-first recognition and synthesis, and no default permanent recording or retention of raw audio.

### Accessibility and user control

The desktop experience must support keyboard navigation, clear focus handling and screen-reader accessibility. Notifications must be configurable, support quiet hours and avoid excessive interruptions.

### Not decided

Desktop and UI frameworks, tray/overlay implementation, notification APIs, screen-capture adapters, exact indicator design, wake-word engine, speech recognition/synthesis technologies, audio retention settings and platform-specific privacy integration remain open.



## ADR-011 — Intelligence and orchestration architecture

**Status:** Approved  
**Date:** 2026-11-22

### Decision

Use a **central, stateful orchestrator with bounded adaptive execution**. The orchestrator owns each task's overall plan, execution state, delegation, recovery and final result. Specialist agents are on-demand workers that perform bounded delegated work and return structured results; they do not independently control the overall assistant or acquire authority.

### Mandatory behaviour

- The orchestrator classifies requests, plans work, selects suitable models/tools/specialists, coordinates execution, verifies outcomes and reports completion.
- Plans may be revised as new information arrives, but execution is bounded by explicit step, time, token and resource limits.
- Sequential execution is the default. Parallelize only independent work when it materially improves the result, within the approved resource manager's limits.
- Specialist contracts define scoped inputs, authorized context, expected structured outputs, evidence, status and error reporting.
- Model outputs may propose actions, tool calls and plans; deterministic schemas, validation, authorization and the central permission broker enforce what can actually execute. Models are never security authorities.
- Route models according to task capability, complexity, latency, privacy/cloud policy and current resource availability. Respect the approved model lifecycle and Gaming Mode constraints.
- Persist sufficient task state and checkpoints to support interruption, recovery and safe resumption. Retries must account for operations that may have completed despite a timeout; do not blindly repeat non-idempotent actions.
- Isolate worker failures from the core, support cancellation and bounded retries, and return actionable failure information when recovery is not safe.
- Evaluate orchestration quality using task completion, tool selection and argument correctness, unnecessary calls, verification, recovery, latency and resource consumption.
- Preserve trusted user/workspace identity and authorization context across every delegation and tool operation.

### Not decided

Concrete model-routing algorithms, planner implementation, task-state schema, retry/idempotency mechanisms, specialist taxonomy, evaluation datasets/benchmarks, model/runtime choices and orchestration libraries remain open.

---

## ADR-012 — Layered memory and governed knowledge architecture

**Status:** Approved  
**Date:** 2026-11-22

### Decision

Use a **layered, scope-aware memory system with governed writes and authorization-aware retrieval**. Memory and knowledge repositories are related but distinct: memory stores useful interaction-derived information, while knowledge repositories preserve and retrieve source material such as documents, code and project files.

### Information layers

- **Working memory:** current conversation and active-task context; temporary and bounded.
- **Episodic memory:** useful summaries of past interactions and experiences; persistent only under the approved memory policy.
- **Semantic memory:** confirmed facts, preferences and relationships; persistent only under the approved memory policy.
- **Procedural memory:** reusable workflows and methods; save or activate only after the required user approval.
- **Knowledge repositories:** source documents, code, project files and reference material, indexed separately from personal memory and governed by their source scope.

### Approved lifecycle and safeguards

- Conversation history does not automatically become permanent memory. Using information is not the same as saving it.
- Memory writes use a review-based default: the assistant may identify useful candidates, but asks the user to approve them before persistent saving. Separately approved automatic rules may be introduced later.
- Memory records should preserve provenance, confidence, timestamps and temporal validity where applicable. Corrections and conflicting information must be reconciled or versioned rather than silently overwriting established facts.
- Project knowledge uses user-selected project locations. The assistant may monitor approved locations for changes and update indexes, but must not silently expand monitoring scope.
- Original source documents remain authoritative; indexes, embeddings and summaries are derived, rebuildable data.
- Retrieval must apply trusted user/workspace authorization before content is exposed to a model. Retrieved content, metadata, caches and derived summaries must not cross user or workspace boundaries.
- Retrieved documents, memories and tool outputs are untrusted data, not instructions or sources of authorization.
- Updates, corrections, revocation and deletion must propagate to derived indexes, caches and summaries, subject to the documented backup-retention lifecycle.
- Use keyword, semantic and reranking techniques only where they improve measured retrieval quality enough to justify their latency and resource costs.
- Keep working context bounded and construct task-specific context from only the authorized, relevant information needed by the current task.

### Not decided

Concrete memory schemas, database and vector/index technologies, embedding and reranking models, retrieval algorithms, conflict-resolution implementation, memory retention periods, deletion guarantees, project watcher mechanisms and user-review interface remain open.



## ADR-013 — Event-driven automation and governed workflows

**Status:** Approved  
**Date:** 2026-11-22

### Decision

Use event-driven automation with durable, bounded workflows integrated with the central orchestrator, resource manager and permission broker.

### Approved behaviour

- Prefer explicit events, user requests and scheduled triggers over continuous polling or unnecessary model inference.
- Persist workflow state, scheduled waits, checkpoints and approval pauses so workflows can survive core restarts.
- Every automation has a trusted owner and user/workspace scope. Shared workflows require explicit workspace ownership and policy.
- Background execution retains initiating identity, scope and authorization context; revalidate permissions before delayed or consequential actions when appropriate.
- Apply explicit limits for execution time, steps, retries, concurrency and resource consumption.
- Provide preview, approval checkpoints, pause, cancellation, execution history and actionable failure reporting.
- Use idempotency keys, deduplication or equivalent safeguards for retried external operations. Do not promise exactly-once effects where external systems cannot guarantee them.
- Respect resource-manager priorities; defer nonessential automation during Gaming Mode or resource pressure.
- Workflow learning may propose reusable procedures, but saving or activating learned workflows remains subject to the approved memory-governance and user-review rules.

### Not decided

Workflow definition format, scheduler implementation, event transport, persistence schema, trigger catalogue, retry policy details and automation UI remain open.


## ADR-014 — Open, capability-based plugin and integration ecosystem

**Status:** Approved  
**Date:** 2026-11-22

### Decision

Provide an open plugin ecosystem with a public community registry and support for third-party and locally developed extensions, governed by explicit capability declarations, trust verification, isolation and central permission enforcement.

### Approved behaviour

- Plugins declare identity, publisher, version, dependencies, requested capabilities, network needs, resource requirements and permission requirements through a manifest.
- Support publisher verification, package integrity checks, signatures and provenance tracking. Clearly distinguish verified, community and locally developed/unverified extensions; verification is not a guarantee that software is harmless.
- Plugin installation does not grant blanket access. Every privileged operation is evaluated by the central permission broker under the user's grants and all applicable data/workspace boundaries.
- Run plugins in restricted, isolated and restartable workers with OS-enforced controls appropriate to their capabilities; process separation alone is not a sandbox.
- Use the common internal tool contract with adapters for external protocols such as MCP. MCP is supported but is not the sole internal plugin or tool architecture.
- Scope integration credentials to the relevant service, user/workspace and capability. Protect secrets from unrelated plugins and workers; never treat possession of a credential as authorization for other resources.
- Review material permission, dependency and capability changes during plugin updates. Preserve version and provenance information and provide rollback where feasible.
- Uninstalling or disabling an extension must revoke its credentials and grants and cancel or detach its scheduled tasks and event subscriptions.
- Treat plugin output and package content as untrusted data, not as instructions or sources of authority.

### Not decided

Manifest schema, registry governance, signing infrastructure, package format, sandbox technologies, update/rollback mechanics, plugin SDK, compatibility policy and review/moderation operations remain open.


## ADR-015 — Local-first synchronization, backup and portability

**Status:** Approved  
**Date:** 2026-11-22

### Decision

Keep the platform fully useful locally and support opt-in synchronization through either a managed service or a self-hosted deployment. Use end-to-end encryption for private synchronized data, with separate controlled key-sharing for explicitly shared workspace data. Treat synchronization, backup and export/import as distinct capabilities.

### Synchronization

- Sync is optional; local operation must remain functional offline and without a hosted account.
- Support both an optional managed synchronization service and self-hosted synchronization using the same interoperable architecture.
- Encrypt private data on authorized devices before transmission so the synchronization service does not need plaintext access.
- Keep user-private and workspace-shared data under distinct authorization and key boundaries. Workspace sharing requires explicit membership and controlled key distribution.
- Support device enrollment, device revocation and documented recovery procedures for lost devices or keys.
- Resolve safe structured changes automatically where reliable; preserve version history and surface meaningful conflicts for user resolution. Do not rely on timestamp-only last-write-wins where it could silently discard meaningful edits.
- Sync protocols must preserve ownership, provenance, versioning and authorization metadata and must not bypass the permission broker or workspace boundaries.

### Backup and recovery

- Provide encrypted, versioned backups with configurable retention and multiple destinations, including optional offline/offsite copies.
- Protect backup integrity and deletion controls; support verification and regular restore testing.
- Provide selective restoration where feasible and clearly report conflicts or data that cannot be restored.
- Distinguish deletion from live stores and derived indexes from eventual expiration in retained backups; document retention and deletion limits honestly.

### Export and import portability

- Provide user-readable exports and complete backup/migration archives as distinct formats or modes.
- Define documented, versioned formats for conversations, memory, projects, settings, automation definitions, workspace data and attachments/metadata where applicable.
- Support full migration and selective import, with reports for unsupported or omitted data.
- Imported plugins and executable automation remain inactive and untrusted until reviewed and explicitly authorized; importing data never silently grants permissions or activates code.

### Not decided

Sync protocol, managed-service operations, cryptographic protocols and key recovery design, backup storage formats, retention periods, export schemas, migration tooling, conflict-resolution algorithms and deletion guarantees remain open.
