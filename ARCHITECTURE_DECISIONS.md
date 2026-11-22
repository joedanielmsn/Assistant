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


## ADR-016 — AI-first CachyOS-derived Linux distribution

**Status:** Approved  
**Date:** 2026-11-22

### Decision

Build the project's AI-powered Linux distribution on top of CachyOS as its technical foundation, while creating an independently designed and maintained distribution identity. This is not intended to be a simple CachyOS reskin.

### Approved direction

- Use CachyOS as the upstream foundation and selectively reuse valuable infrastructure, packages, kernel work, hardware support and installation tooling where they meet the project's needs.
- Create an independent distribution identity, including its name and branding, visual language, desktop defaults, settings experience, system utilities and user workflows. Do not simply inherit CachyOS's overall design.
- Build the initial flagship desktop experience on KDE Plasma, with a substantially redesigned and integrated user experience. Keep the architecture modular enough to replace or extend desktop components later.
- Make assistant integration a first-class part of the operating system experience from the beginning. The distribution is deliberately AI-first; users who do not want an AI-focused operating system can use CachyOS itself.
- Integrate assistant capabilities into desktop and system workflows rather than treating the assistant as an unrelated optional application.
- Preserve the ability to disable or pause AI services without breaking ordinary operating-system and desktop functionality.
- Maintain our own distribution repositories, build and testing pipeline, release identity, documentation and quality-control process. CachyOS upstream updates are inputs, not automatic approvals for our releases.
- Use a curated rolling-release model: validate updates through our own testing channel before promoting them to our stable channel.
- Keep developer productivity, gaming, laptop hardware compatibility, installation, recovery and performance as core distribution requirements.
- Prefer upstream components and minimise long-lived custom patches where practical to reduce maintenance burden.

### Design constraints

- The operating system must remain functional without AI inference, cloud access or a hosted account.
- AI features must respect the platform's central permission broker, user identity and resource manager. System integration does not grant the assistant unrestricted administrative authority.
- Privileged system operations remain explicitly brokered; a user-level assistant service is the default, with privileged system components limited to narrowly defined OS integration needs.
- The distribution must not require telemetry or mandatory cloud services.
- Hardware-specific optimisations must not make the distribution unusable on ordinary supported hardware.

### Not decided

Distribution name and branding, exact CachyOS components to adopt or replace, repository and build infrastructure, package promotion/testing policy, desktop-shell modifications, kernel variants, installer configuration, filesystem/partitioning defaults, Secure Boot implementation, hardware profiles, recovery tooling, update/rollback mechanics and release support policy remain subject to further research and implementation design.


## ADR-017 — Privileged system operations use central broker authorization

**Status:** Approved  
**Date:** 2026-11-22

### Decision

Use the platform's central permission broker as the authoritative authorization and approval layer for assistant-initiated privileged system operations. Do not require a second interactive authorization prompt from the operating system for every operation.

### Mandatory safeguards

- The broker must authenticate the initiating identity, evaluate the requested operation and applicable policy, and record the decision before execution.
- Privileged execution must be delegated to a narrowly scoped OS helper or integration component; the ordinary assistant core and model workers must not run with broad administrator/root privileges.
- The helper exposes only predefined, structured operations with validated arguments and targets. It must not provide arbitrary shell execution, unrestricted root/administrator access or a general-purpose privilege escalation interface.
- Plugins, specialists, automations and models cannot invoke the helper directly or bypass the broker.
- OS-enforced process isolation, access controls and least-privilege boundaries remain mandatory. The decision removes a duplicate interactive OS authorization step; it does not remove operating-system security controls.
- Deny by default when the broker, trusted identity context or policy evaluation is unavailable or ambiguous.
- Record the authorized operation and execution outcome in the appropriate audit history, subject to data-minimization rules.

### Rationale

A single authoritative authorization experience avoids duplicate prompts while preserving a narrow and independently constrained privileged execution boundary. The helper's limited operation surface reduces the impact of a compromised or misbehaving assistant component.

### Not decided

The exact helper architecture, operation catalogue, platform-specific implementation, privilege lifecycle, IPC protocol and audit storage remain implementation decisions.

## ADR-018 — Initial platform support is x86-64 only

**Status:** Approved  
**Date:** 2026-11-22

### Decision

The initial supported hardware architecture for Local Assistant 2.0 and its associated desktop/server/distribution deliverables is **x86-64 (AMD64) only**.

### Constraints

- Initial official builds, installation images, packaging, testing and support target x86-64 systems.
- ARM64 and other CPU architectures are outside the initial support commitment; do not add them to release or compatibility promises.
- Keep architecture-dependent code behind portable interfaces where practical, so future support can be evaluated without committing to it now.
- Hardware-vendor and GPU support remain separate compatibility dimensions; x86-64 support does not imply that every GPU or device is supported.

### Rationale

Concentrating initial development, validation and release engineering on one CPU architecture reduces build, testing, packaging and support complexity.

### Not decided

Minimum CPU instruction-set baseline, supported processor generations, GPU/accelerator matrix and exact hardware certification requirements remain open for compatibility research.


## ADR-019 — Minimum CPU instruction-set baseline: x86-64-v3

**Status:** Approved  
**Date:** 2026-11-22

### Decision

Set **x86-64-v3** as the minimum CPU instruction-set baseline for initial official builds and supported systems.

### Constraints

- x86-64-v3 compatibility is required for officially supported releases.
- x86-64-v4 may be used for optional optimized builds or runtime dispatch when the processor supports it; it is not a system-wide requirement.
- CPUs that support only the original x86-64 baseline or x86-64-v2 are outside the initial support commitment.
- Validate CPU feature availability before installing or launching architecture-specific binaries where practical, and provide clear compatibility diagnostics.
- Keep portable source code and architecture-specific optimizations separated so future targets can be evaluated without weakening the v3 baseline.

### Rationale

A modern baseline aligns with the CachyOS-derived performance direction and allows broader use of contemporary CPU optimizations while keeping x86-64-v4 optional.

### Not decided

Exact processor-generation support lists, compiler flags, package repository variants, runtime dispatch strategy and hardware certification tests remain implementation and release-engineering decisions.


## ADR-020 — Desktop application framework: Qt 6 + QML

**Status:** Approved  
**Date:** 2026-11-22

### Decision

Use **Qt 6 with QML/Qt Quick** as the framework for the primary cross-platform desktop application.

### Constraints and direction

- Target Linux and Windows desktop clients from a shared application codebase where practical.
- Use QML/Qt Quick for the desktop presentation layer and Qt's platform integration capabilities where appropriate.
- Keep the desktop application as a client of the authoritative assistant core; it must not become a second source of truth for identity, permissions, task state or resource policy.
- Design the application to support the primary desktop experience and optional dashboard, tray controls, notifications and floating companion surfaces.
- Integrate with KDE Plasma while retaining a viable Windows desktop experience.
- Keep platform-specific integrations behind adapters so desktop UI logic remains portable.
- Preserve accessibility, keyboard navigation, screen-reader support and clear privacy/awareness indicators.

### Rationale

Qt 6 + QML aligns with the KDE Plasma foundation selected for the AI-first Linux distribution, offers a mature cross-platform desktop toolkit and supports rich desktop interfaces without making the assistant core dependent on a web frontend.

### Not decided

Exact Qt modules, QML architecture, UI state-management approach, build system, packaging, platform-specific tray/overlay implementations and accessibility implementation details remain open for implementation planning.

## ADR-021 — Initial core, worker and runtime technology architecture

**Status:** Approved  
**Date:** 2026-11-22

### Decision

Adopt the following initial technology architecture for Local Assistant 2.0. These selections establish the implementation direction; detailed schemas, versions, APIs, deployment configurations and security policies still require implementation specifications and validation.

| Area | Approved technology/direction |
|---|---|
| Authoritative core | Rust |
| Core asynchronous runtime | Tokio |
| AI and specialist workers | Python, running as separate supervised processes |
| Desktop application | Qt 6 + QML (ADR-020) |
| Local LLM inference | llama.cpp |
| Specialized inference | ONNX Runtime, selected per model/task and validated execution provider |
| Personal desktop persistence | SQLite |
| Shared-server persistence | PostgreSQL |
| Initial local full-text search | SQLite FTS5 |
| Rust database access candidate | SQLx, subject to platform/backend validation |
| Client API | Versioned HTTP-compatible structured API and WebSockets |
| Internal worker protocol | Versioned JSON messages over supervised local IPC |
| Portable plugin execution | WebAssembly using Wasmtime as the initial runtime |
| Native integration workers | Isolated processes with OS-enforced restrictions; Bubblewrap and Linux kernel controls on Linux, AppContainer where compatible on Windows |
| Workflow execution | Internal durable workflow engine backed by database state |
| Event delivery | Internal typed event system initially; introduce an external broker only if measured needs justify it |

### Core and worker boundary

- Rust owns authoritative platform state, orchestration, trusted identity, authorization, the permission broker and resource policy.
- Python provides AI and specialist capabilities but is not an authority or security boundary.
- Python workers run separately from the core, are supervised and restartable, and receive only scoped task inputs and authorized context.
- Worker crashes, dependency failures or restarts must not crash or compromise the core.
- All worker requests and results use versioned, language-neutral contracts.
- Models and workers may propose actions; deterministic validation and the central permission broker govern execution.
- Worker concurrency, memory, CPU, GPU and execution time remain bounded by the resource manager.

### Worker IPC

Use versioned JSON messages over supervised local IPC as the initial internal worker protocol. Requests and responses must support correlation IDs, trusted execution context, deadlines, cancellation, structured errors and protocol-version negotiation. Standard output is reserved for protocol messages and standard error for diagnostics. Large payloads should use scoped, access-controlled temporary-file references rather than embedding large binary data in JSON.

The public/client API remains distinct from internal worker IPC. gRPC/Protocol Buffers may be reconsidered if measured throughput, schema or streaming requirements justify the additional infrastructure.

### Inference architecture

- A central model manager owns model discovery, routing, lifecycle, resource accounting and concurrency.
- llama.cpp is the initial local LLM backend.
- ONNX Runtime is the specialized inference backend where a model and a validated execution provider make it appropriate.
- Backends implement a common internal inference contract; no single runtime is forced to handle every model type.
- Share runtime/model instances where practical and avoid unnecessary duplicate GPU/VRAM allocations.
- Backend and execution-provider support must be validated per OS, GPU vendor, driver and model. Do not promise universal hardware acceleration.
- Gaming Mode and all existing resource-protection rules remain mandatory.

### Persistence and search

- SQLite is the initial personal desktop database; PostgreSQL is the shared-server database.
- Keep domain logic independent of database-specific SQL and maintain backend-specific migrations where required.
- SQLite WAL may be used for desktop concurrency, respecting its single-writer and filesystem constraints.
- Package a maintained SQLite release containing applicable WAL fixes; pin and test the actual shipped version.
- PostgreSQL row-level security may provide defence in depth, but application authorization remains mandatory. Database roles and table ownership must not accidentally bypass intended policies.
- SQLite FTS5 is the initial local full-text search implementation. Vector search, embeddings and reranking remain subject to separate evaluation.

### Plugin and native-worker isolation

- Use Wasmtime/WebAssembly as the preferred portable sandbox for community plugins where the required capabilities fit its component/host interface.
- Native integration workers remain a separate, more restricted execution path for OS-specific capabilities.
- Process separation alone is not a sandbox. Apply OS-enforced filesystem, network, process and resource restrictions.
- Linux and Windows sandbox implementations must be validated independently; Bubblewrap and AppContainer are initial candidates, not guarantees of identical isolation.
- Every privileged operation continues to pass through the central permission broker. Sandboxes do not replace authorization.

### Durable workflows and events

Begin with an internal typed event system and a database-backed durable workflow engine, including checkpoints, cancellation, bounded retries and idempotency/deduplication controls. Use a transactional outbox where needed to avoid losing committed state transitions. Do not claim exactly-once external effects where integrations cannot guarantee them. Consider NATS, Temporal or another external system only when measured deployment and scaling requirements justify it.

### Constraints and validation

- This is an approved initial technology direction, not permission to skip detailed design, threat modelling, compatibility testing or performance measurement.
- Pin and audit dependencies; establish supported version policies and update procedures.
- Validate Linux and Windows packaging, worker supervision, sandboxing, GPU/runtime combinations, recovery and upgrade/rollback behaviour.
- Keep hardware-specific functionality behind platform/backend adapters.
- Licensing remains deferred and must be reviewed before architecture finalisation and public release.
- The existing `~/local-assistant` project remains reference-only; this architecture does not authorize code reuse or dependency on that build.

### Not decided

Exact crate and package versions, Rust/Python API schemas, Python environment distribution, process launch/supervision details, IPC framing, inference model catalogue, GPU execution-provider matrix, database schemas and migrations, encryption/key management, plugin component ABI/manifest, sandbox profiles, workflow schema, build system, packaging, update/rollback implementation and licensing.

## ADR-022 — Build, repository, packaging and release architecture

**Status:** Approved  
**Date:** 2026-11-22

### Decision

Use a coordinated platform monorepo and a separate repository for the AI-first Linux distribution. The platform and distribution share compatible components and interfaces but retain independent release schedules and validation pipelines.

### Repository boundaries

**Platform monorepo** contains:
- Rust core and shared Rust libraries.
- Qt 6/QML desktop application.
- Python AI and specialist workers.
- Official plugins, plugin SDK and shared tool interfaces.
- Platform packaging, developer tooling and release scripts.
- Platform architecture and developer documentation.

**Separate Linux distribution repository** contains:
- CachyOS-derived image and installer configuration.
- Distribution-specific kernel and desktop customizations.
- Distribution package definitions and integration configuration.
- Distribution repository, signing, testing-channel and release-channel configuration.

The distribution may consume versioned platform packages or release artifacts. It must not require platform and OS releases to happen simultaneously.

### Coordinated component build systems

| Component | Initial build direction |
|---|---|
| Rust core and Rust tools | Cargo workspace |
| Qt 6/QML desktop | CMake with Ninja as the initial build executor |
| Python workers | uv-managed, locked and reproducible environments |
| Linux distribution | Arch/CachyOS-compatible package build tooling and PKGBUILDs |
| Windows desktop distribution | Qt deployment tooling and a maintained Windows installer/package process |

Use component-appropriate tools rather than forcing the entire project through one build system. Pin toolchain and dependency versions and maintain reproducible build inputs where practical.

### CI and release validation

Use GitHub Actions as the initial CI/CD platform, with workflows that support:
- Fast pull-request checks: formatting, static analysis, unit tests and interface/schema validation.
- Linux and Windows build/test matrices.
- Integration tests for core, desktop and worker communication.
- Separate hardware-dependent inference validation; ordinary CI must not require a GPU.
- Reviewed, tagged release builds with retained artifacts, test reports and diagnostic logs.
- Independent distribution build, promotion and release workflows.

Hardware-specific GPU tests run on controlled compatible machines and cover the supported CPU/GPU/driver combinations. CI success alone is not evidence of hardware acceleration or sandbox strength.

### Packaging and updates

- Linux desktop and core components should use native packages as the primary integration path, with additional universal formats considered only when they provide a clear distribution benefit.
- The Linux distribution maintains its own signed package repositories and controlled testing/stable promotion process. Package signature verification must remain enabled and use trusted signing keys.
- Windows deployment packages the Qt application and required runtime dependencies, while managing the core and Python workers as separately versioned components where practical.
- Python worker environments are pinned and isolated; do not depend on arbitrary user-installed Python packages.
- Platform and distribution updates must support compatibility checks, staged rollout, failure reporting and a documented recovery/rollback strategy.
- Build artifacts must be traceable to reviewed source revisions and dependency/toolchain inputs.

### Constraints

- One platform source tree does not imply one binary, one installer or one release cadence.
- The Linux distribution is independently tested and maintained; upstream CachyOS changes are not automatically promoted.
- Linux and Windows remain first-class platform targets, subject to their respective integration and validation requirements.
- Build, package and update mechanisms must preserve the permission-broker, worker-isolation and data-ownership boundaries.
- Do not weaken package-signature verification to simplify development or installation.
- This decision establishes the initial direction; exact workflow definitions, package formats, signing infrastructure and release policies remain detailed design work.

### Not decided

Exact monorepo directory layout, branch/review policy, CMake/Ninja versions, Cargo toolchain policy, Python worker distribution format, GitHub Actions runner strategy, artifact retention, release versioning, package names and dependencies, Windows installer technology, Linux package promotion mechanics, signing-key custody/rotation, update/rollback implementation and support policy.


## ADR-023 — Final cross-cutting product, privacy and deployment decisions

**Status:** Approved architectural direction  
**Date:** 2026-11-22

### Identity and deployment

- Personal desktop authentication uses the operating-system user identity by default, with optional additional authentication for sensitive operations.
- Shared-server deployments initially use built-in accounts with passkey support. External identity-provider integration is deferred to a later release while preserving an extensible authentication boundary.
- Initial shared-server deployment support targets Linux servers. Windows server support is not part of the initial target.

### Data protection, encryption and recovery

- Private local data receives application-level encryption by default, alongside OS account protections and any available full-disk encryption. The implementation must remain configurable.
- A user-controlled master password is the selected primary key-unlock approach.
- A separately generated recovery key provides the initial lost-key recovery path. Recovery procedures must explain the consequences of losing both the master password and recovery key.
- Retention is configurable by data category, with sensible defaults and distinct treatment for conversations, task data, logs, caches and derived indexes.
- Users can define sensitivity levels and policies for conversations, projects, folders and workspaces. Policies may impose stricter storage, cloud-routing, worker-access and retention constraints.

### Cloud and synchronization

- Cloud inference behaviour is configurable, supporting explicit-only use, policy-governed automatic fallback and confirmation-before-use modes.
- Provider connectivity supports OpenAI-compatible APIs, dedicated provider adapters, custom endpoints and self-hosted inference servers.
- Synchronization conflict handling automatically merges changes only where safe and reliable; meaningful conflicts are presented for user resolution.
- The project will support self-hosted synchronization. A project-operated managed synchronization service is not currently planned.

### Models, specialists and autonomy

- Model routing is automatic by default, with user overrides and advanced configuration.
- Model management supports both assistant-managed downloads/verification/lifecycle and compatible external runtimes.
- The specialist ecosystem includes built-in, user-created and community-provided specialists.
- Autonomy is configurable by agent, tool and workspace. The initial default remains risk-based: low-risk permitted actions may proceed automatically; sensitive actions require approval.

### Hardware and resource management

- NVIDIA receives first priority for initial inference compatibility validation, reflecting the initial development target, while AMD and Intel support remain architectural goals requiring independent validation.
- Resource management combines automatic adaptation with advanced manual CPU, RAM, GPU and VRAM limits and workload profiles.
- Gaming Mode supports both configurable automatic game detection and manual activation/override.

### Plugins and integrations

- Plugin trust levels include verified, community and local extensions. Installation never grants blanket access; all plugins remain subject to declared capabilities, review, isolation and the central permission broker.
- Support an official plugin registry and independent third-party registries.
- Maintain a curated official integration catalogue alongside community integrations.

### Desktop interaction

- The desktop companion supports both an optional floating companion presentation and a minimal floating control-panel presentation.
- Voice architecture supports speech recognition and spoken responses as well as real-time conversational voice, with selectable modes and visible privacy controls.
- Application/window metadata awareness and screen-content awareness remain separately configurable and permissioned.

### Linux distribution identity and lifecycle

- Initially build and present the OS as a distinct distribution based on CachyOS infrastructure, with the long-term goal of establishing a fully independent distribution identity, name and ecosystem.
- Substantially redesign KDE Plasma while retaining it as the initial desktop foundation.
- The installer provides beginner and advanced modes.
- Offer Btrfs and ext4, recommending Btrfs with snapshot-based recovery support.
- Use conventional curated rolling package updates rather than image-based atomic updates. Btrfs snapshots and documented package/system recovery remain important safeguards; this choice does not remove the existing testing/stable promotion process.

### Telemetry, security reporting and audit

- The project collects no telemetry, including anonymous opt-in telemetry, as the product direction.
- Provide both private vulnerability reporting/coordinated disclosure and a public issue tracker, with clear guidance about which channel to use.
- Audit detail and retention are configurable, with data minimization and access controls.

### Licensing and bundled software

- Do not select the core licence until a project-wide licensing evaluation has considered distribution, dependencies, contributions and compatibility.
- The plugin ecosystem accepts compatible open-source licences rather than requiring every extension to use the core's eventual licence.
- Distribution defaults favour open-source software while allowing clearly labelled optional proprietary applications and drivers.

### Constraints and deferred specifications

These decisions establish product direction, not final cryptographic or implementation specifications. Still required are threat modelling, encryption/key derivation and recovery design, database isolation, detailed retention/deletion guarantees, cloud data disclosure controls, sync protocol and self-hosted deployment design, model/runtime compatibility testing, plugin registry governance, OS installer/update/recovery engineering, vulnerability response procedures and a complete licence/dependency audit.

The choice of a master password plus recovery key must be reconciled with unattended desktop startup and secure key caching. No plaintext key material may be exposed to ordinary workers or plugins. A recovery key must not become a hidden bypass of user/workspace authorization.



## ADR-024 — Security threat model and enforceable worker isolation

**Status:** Approved technical direction; implementation validation required  
**Date:** 2026-11-22

### Threat model

Use a documented threat model, maintained per release and revisited when trust boundaries or capabilities change. Cover at minimum:
- Malicious or compromised plugins, specialist workers, model outputs, documents, browser pages and tool responses.
- Prompt injection and instruction-like content in untrusted inputs.
- Compromised desktop clients, local IPC callers, credentials, update channels, package registries and build dependencies.
- Cross-user/workspace data disclosure, confused-deputy behaviour and background-task privilege drift.
- Malicious or compromised administrators in shared-server deployments, while documenting limits of protection from a machine/root administrator.
- Denial of service through resource exhaustion, runaway workflows, model loading and malicious packages.
- Supply-chain compromise, signing-key compromise, downgrade/replay attacks and vulnerable dependencies.
- Physical device loss and offline access to copied data.

Use STRIDE-style threat analysis and OWASP ASVS 5.0 as a security verification reference for applicable API and service controls. This is a baseline, not a claim of certification.

### Mandatory security boundaries

- The central permission broker remains the authoritative policy decision point for assistant-initiated privileged actions.
- No model, plugin, specialist or automation can mint identity, widen scope, grant itself permissions or invoke privileged helpers directly.
- Treat all external content and model-generated plans/tool arguments as untrusted data.
- Validate identity, workspace membership, resource ownership, path/target, operation and policy at execution time.
- Use least privilege, deny-by-default behaviour, bounded execution and cancellation.
- Do not expose unrestricted host sockets, D-Bus, credentials, home directories, process namespaces or privileged file descriptors to untrusted workers.
- Do not place long-lived secrets in command-line arguments, environment variables, logs, crash reports or worker-readable temporary files.
- Fail closed when a required sandbox or authorization control is unavailable. Any compatibility mode that weakens isolation must be explicit, visibly reported and disabled for untrusted community plugins by default.

### Linux native-worker isolation

Use layered, unprivileged isolation:
- Bubblewrap mount/user/PID/IPC/network namespaces with an explicitly constructed minimal filesystem view.
- Landlock rules to restrict filesystem and supported network access further.
- Seccomp-BPF filters to reduce the syscall surface.
- Drop capabilities, set no-new-privileges, apply resource limits/cgroups where available, and use read-only mounts by default.
- Expose only task-specific directories and explicitly brokered communication endpoints.
- Do not pass unrestricted D-Bus, session bus, host agent sockets, display server or desktop-control sockets into a worker. Use narrow proxies or dedicated broker APIs when a capability is required.
- Validate the actual kernel ABI/features at runtime; do not assume all supported Linux systems provide identical controls.

Bubblewrap arguments and mounts define the effective boundary; its presence alone is not proof of a secure sandbox. Landlock and seccomp are additional restrictions, not substitutes for correct namespace and mount design.

### Windows native-worker isolation

Use AppContainer where compatible, with explicit capability grants and ACLs, low-integrity/restricted execution where applicable, network restrictions and Job Objects for process/resource lifecycle controls. Validate packaged and unpackaged deployment modes independently. If a required worker cannot run within the intended isolation boundary, do not silently fall back to an unrestricted process.

### WebAssembly plugin isolation

- Wasmtime is the preferred portable plugin runtime.
- No ambient filesystem, network, environment, clock, process or host API access by default.
- Grant explicit, narrowly scoped host capabilities through versioned interfaces.
- Apply fuel/instruction, memory, table, stack, execution-time and concurrency limits.
- Version and validate host-call contracts; all privileged host calls route through the permission broker.
- Treat a WASM sandbox as one layer, not as proof that plugin logic or its inputs are trustworthy.

### Security verification gates

Before a component or plugin class is promoted:
- Automated tests verify filesystem, network, process, IPC and privilege boundaries.
- Test negative cases and attempted boundary escapes, not only successful workflows.
- Fuzz parsers, IPC framing, manifests, protocol decoders and untrusted input boundaries.
- Run dependency and vulnerability scanning, static analysis, secret scanning and produce an SBOM.
- Test worker crash, timeout, cancellation, restart and resource-exhaustion behaviour.
- Review permission-broker bypass attempts and cross-user/workspace isolation.
- Conduct independent security review before broad public release and after material changes to the trust model.
- Publish a private vulnerability reporting path and coordinated disclosure policy.

References: [OWASP ASVS 5.0](https://asvs.owasp.org/), [Bubblewrap](https://github.com/containers/bubblewrap), [Linux Landlock](https://docs.kernel.org/userspace-api/landlock.html), [Linux seccomp](https://docs.kernel.org/userspace-api/seccomp_filter.html), [Microsoft application isolation](https://learn.microsoft.com/en-us/windows/security/book/application-security-application-isolation).

### Not decided

Exact syscall profiles, kernel feature minimums, per-worker mount/capability profiles, Windows capability sets, Wasmtime configuration, resource quotas, threat-model templates and test harnesses require implementation-specific validation.

## ADR-025 — Encryption, key hierarchy and database isolation

**Status:** Approved technical direction; cryptographic review and implementation validation required  
**Date:** 2026-11-22

### Key hierarchy

Use envelope encryption:
- Generate cryptographically random data-encryption keys (DEKs); do not derive content-encryption keys directly from human passwords.
- Encrypt data with DEKs using authenticated encryption.
- Use a password-derived key-encryption key (KEK) to wrap the user's root/private DEK hierarchy.
- Generate an independent, high-entropy recovery key that can separately unwrap the recovery-protected root key. The recovery key is not an authorization token and does not grant access outside the user's existing identity and scopes.
- Derive domain-separated subkeys for user-private data, workspace data, credentials and other distinct purposes. Do not reuse one raw key across unrelated purposes.
- Version all encrypted envelopes and key metadata to support safe algorithm/parameter upgrades and rotation.
- Bind relevant identity, tenant, workspace, object type, object ID and format version as authenticated associated data to prevent ciphertext substitution across scopes.
- Use established, maintained cryptographic libraries and standard constructions; never design custom cryptographic algorithms or protocols.

### Password and recovery handling

- Use Argon2id for master-password key derivation, with a unique random salt per key hierarchy and parameters stored in versioned metadata.
- Start from the RFC 9106 memory-constrained profile (64 MiB, three passes, four lanes) as a compatibility baseline, then benchmark and raise parameters where supported without making recovery impractical. Revisit parameters as hardware changes.
- Keep the recovery key separate from the master password; show it once during setup, require the user to confirm safe storage, and provide a deliberate recovery test.
- If both master password and recovery key are lost, encrypted data is unrecoverable. State this plainly; do not provide a hidden backdoor.
- A master-password change rewraps the root key; it must not require re-encrypting all user data.
- Key rotation must support rewrapping and, where necessary, staged data-key rotation without losing access to backups.

### Session unlock and key custody

- The master password is required to unlock the private-data key hierarchy after a fresh OS login by default.
- Keep unwrapped keys only in the core's protected memory while unlocked; never pass root keys or password-derived keys to model/plugin workers.
- Locking the assistant, logging out, or reaching a configured inactivity policy clears in-memory key material on a best-effort basis and stops/pauses private-data tasks that cannot safely continue.
- Optional per-device convenience unlock using the OS credential vault may be offered as an explicit opt-in, clearly explaining that it delegates unlock trust to the OS account. It is off by default and can be revoked.
- Unattended tasks must not silently bypass a locked user's encryption boundary. They may continue only with explicitly scoped non-sensitive data or a separately approved device/workload unlock policy.

### Cryptographic construction direction

- Use a reviewed authenticated-encryption construction for records and metadata, and a streaming authenticated-encryption API for large files/attachments.
- XChaCha20-Poly1305 and libsodium secretstream are the initial candidates for the encrypted payload layer, subject to Rust binding maintenance, platform packaging, independent review and interoperability tests.
- Use a reviewed KDF for deriving subkeys from random root keys; keep contexts/domain labels distinct.
- Encrypt backups and synchronized private payloads before they leave the originating device.
- Do not treat TLS or encrypted disks as substitutes for application-level end-to-end encryption.

### Database isolation

**Personal desktop:**
- Keep one private database per OS user/installation with restrictive filesystem permissions.
- Use full database encryption (SQLCipher or an equivalently reviewed SQLite encryption approach) for sensitive local persistence; verify FTS5/build compatibility and maintenance before selecting the exact library.
- Encrypt attachments and large blobs independently; temporary plaintext is minimized and stored in private task directories.
- Derived indexes and caches must be encrypted or treated as rebuildable sensitive data with equivalent access restrictions.

**Shared server:**
- Use PostgreSQL with application-enforced authorization on every query path and tenant/workspace scope.
- Use PostgreSQL row-level security as defence in depth, with non-owner runtime roles, no BYPASSRLS, and FORCE ROW LEVEL SECURITY where appropriate. Do not rely on RLS as the sole authorization mechanism.
- Encrypt private user payloads and workspace-sensitive payloads at the application layer with distinct key scopes; encrypt storage/backups and require TLS in transit.
- Keep user-private keys separate from workspace keys. Workspace membership changes and device revocation trigger the required key-distribution/rotation process.
- Prevent privileged database maintenance identities from becoming ordinary application identities; document that a server/root administrator can ultimately access host memory and infrastructure.

### Search, deletion and retention

- Decrypt only within an authorized core path for search/retrieval; workers receive only the minimum authorized plaintext context.
- Minimize sensitive plaintext in database indexes, logs, caches, crash dumps and telemetry (telemetry is disabled by product decision).
- Treat indexes, embeddings, summaries, temporary files and caches as derived sensitive data; propagate correction/deletion/revocation or rebuild them.
- Document backup expiry separately from live deletion. Do not claim immediate erasure from immutable/offline backups.
- Use cryptographic erasure only where key scope and backup design make the guarantee technically defensible.

References: [RFC 9106 Argon2](https://www.rfc-editor.org/rfc/rfc9106.html), [libsodium key derivation](https://doc.libsodium.org/key_derivation), [libsodium secretstream](https://doc.libsodium.org/secret-key_cryptography/secretstream).

### Not decided

Final cryptographic library/version, exact AEAD and envelope formats, SQLCipher integration, password parameter tuning, memory-locking strategy, OS-vault adapters, database schema, key rotation schedule and backup erasure guarantees require code-level review and independent security validation.

## ADR-026 — Self-hosted end-to-end encrypted synchronization

**Status:** Approved technical direction  
**Date:** 2026-11-22

- Synchronization is self-hosted only; no project-operated managed sync service is planned.
- A sync server stores and relays encrypted private payloads and metadata required for delivery. It must not need plaintext access to user-private content.
- Require authenticated device enrollment, per-device cryptographic identity, revocation and auditable device lists.
- Enroll a new device only through an existing trusted device or the user's recovery process, with explicit user approval.
- Use TLS 1.3 or a later secure standard for transport, in addition to application-level end-to-end encryption.
- Use separate cryptographic key boundaries for user-private data and workspace-shared data. Workspace key distribution is limited to authorized members/devices.
- Device revocation prevents future access and triggers rotation/re-encryption of affected keys where feasible. Explain that revocation cannot erase plaintext or keys already copied to a previously authorized, offline or compromised device.
- Synchronize versioned objects with causal/version metadata. Merge only data types with well-defined, tested merge semantics; surface ambiguous or destructive conflicts.
- Preserve conflict copies and history until user resolution or retention policy expiry.
- Treat synchronization as replication, not backup. Backups remain a separate encrypted, versioned recovery system.
- Support offline operation and bounded retry; reconnect must not silently replay non-idempotent external actions.
- Document metadata visible to the self-hosted server, including account/device identifiers, sizes, timing and routing information.

### Not decided

Exact wire protocol, device key agreement, server implementation, object version format, conflict algorithms by data type, workspace key rotation, discovery/onboarding UX and operational deployment profiles require detailed design and cryptographic review.

## ADR-027 — Hardware compatibility and inference backend policy

**Status:** Approved technical direction; hardware matrix remains validation-driven  
**Date:** 2026-11-22

### Support policy

- x86-64-v3 remains the initial CPU baseline.
- CPU inference is the universal fallback; GPU acceleration is an optional capability detected and validated at runtime.
- NVIDIA receives first validation priority. AMD and Intel remain supported-target goals, but each OS/driver/backend combination must pass testing before being advertised as supported.
- Never infer GPU capability from vendor name alone. Detect device identity, driver/runtime versions, memory capacity and available execution providers.
- Distinguish the display GPU from the compute GPU on hybrid-graphics laptops. Allow user override and avoid forcing the discrete GPU when it would harm battery life or game performance.
- Model placement and offload must account for actual free VRAM, shared memory, context/KV cache, other processes and Gaming Mode.
- Support partial CPU/GPU offload and quantized models where the backend/model format permits; fall back cleanly when acceleration is unavailable or unstable.
- No model download or automatic backend installation may silently replace drivers or weaken system security.

### Initial backend validation matrix

- llama.cpp: CPU baseline; NVIDIA CUDA first; AMD HIP and Vulkan; Intel SYCL/Vulkan where supported by tested hardware and builds.
- ONNX Runtime: CPU baseline; NVIDIA CUDA; Intel OpenVINO; Windows GPU provider selection validated against current support status. DirectML is not assumed to be the long-term default because its provider documentation describes sustained engineering and points Windows deployments toward WinML.
- Keep backend adapters replaceable. Do not expose provider-specific details as permanent user-facing model identities.

### Validation requirements

Maintain a public compatibility matrix covering OS, CPU, GPU, driver, runtime/backend version, model format and tested capabilities. Include:
- Load/unload reliability and memory accounting.
- Token throughput/latency and specialized-model latency.
- Peak VRAM/RAM, CPU usage, thermals and power behaviour.
- Hybrid GPU routing, sleep/resume, driver reset and device loss.
- Gaming Mode contention and recovery.
- CPU fallback and graceful degradation.
- Reproducible benchmark methodology and representative model sizes.

The initial RTX 3050 4 GB development system is a useful constrained validation target, not a universal minimum GPU requirement.

References: [llama.cpp build backends](https://github.com/ggml-org/llama.cpp/blob/master/docs/build.md), [ONNX Runtime execution providers](https://onnxruntime.ai/docs/execution-providers/), [ONNX Runtime DirectML status](https://onnxruntime.ai/docs/execution-providers/DirectML-ExecutionProvider.html).

### Not decided

Exact minimum RAM/GPU memory recommendations, supported driver versions, official backend build matrix, model catalogue, performance thresholds and certified hardware list must be established by measured tests.

## ADR-028 — Conventional rolling updates, snapshots and secure release metadata

**Status:** Approved technical direction  
**Date:** 2026-11-22

### Operating-system update model

- Use conventional curated rolling package updates through signed native packages and repositories.
- Maintain testing and stable promotion channels; upstream updates are inputs, not automatic releases.
- Require package signature verification and trusted-key validation. Never recommend disabling signature checks to work around repository or keyring problems.
- Perform preflight checks for disk space, package conflicts, critical service health and known incompatible transitions.
- On Btrfs installations, provide supported pre-update snapshots and recovery entries for the relevant system subvolumes.
- On ext4 installations, provide documented package-cache downgrade and backup/recovery procedures; do not promise snapshot rollback.
- Snapshot rollback is not automatically a complete rollback of bootloader/EFI state, external data, firmware changes, databases or user files. Document the boundaries and test boot consistency.
- Keep a recovery environment and documented offline repair procedure.
- Do not automatically roll back user data or assistant databases as part of an OS rollback without explicit compatibility and backup handling.

### Platform application, plugins and model assets

- Use signed release artifacts and verifiable provenance.
- Adopt The Update Framework (TUF) or a compatible implementation for application/plugin registry metadata and downloadable artifacts where appropriate, to defend against compromised mirrors, stale metadata and rollback attacks.
- Maintain offline or threshold-protected root trust, delegated roles, key rotation and metadata expiry.
- Use transparency/provenance tooling such as Sigstore where useful for build attestations; it supplements rather than replaces trusted update metadata and policy.
- Keep OS package-signing trust and application/plugin registry trust separate.
- Plugin and model downloads are data/code supply-chain inputs and must be verified before installation or activation.
- Updates that change plugin capabilities or permissions require renewed review and authorization.

### Recovery and release quality

- Test clean install, upgrade, interrupted upgrade, failed boot, package downgrade, snapshot restore, data restore and recovery media.
- Retain known-good artifacts and package versions sufficient for supported recovery windows.
- Publish release notes, compatibility warnings, known issues and recovery steps.
- Use staged rollout and halt promotion when critical regressions or security issues appear.
- Generate SBOMs and preserve source revision, toolchain, dependency and build provenance for each release.

References: [TUF specification](https://theupdateframework.github.io/specification/latest/), [Arch package signing](https://wiki.archlinux.org/title/Pacman/Package_signing), [Sigstore](https://docs.sigstore.dev/about/overview/).

### Not decided

Exact snapshot tooling/layout, boot integration, retention count, package cache policy, TUF role thresholds, signing-key custody ceremony, rollout percentages, supported recovery window and Windows update/rollback implementation remain detailed design work.

## ADR-029 — Plugin supply-chain trust, registry and licence governance

**Status:** Approved technical direction  
**Date:** 2026-11-22

### Plugin trust and lifecycle

- Plugin signatures establish publisher/integrity provenance, not that a plugin is benign or safe.
- Require a versioned manifest declaring publisher, package identity, version, dependencies, requested capabilities, network destinations, resource requirements and supported platform/runtime.
- Verify package digest, signature, provenance and registry metadata before installation.
- Use separate trust levels for verified, community and local plugins; present the level and requested capabilities clearly.
- Keep new or updated plugins inactive until required permissions are reviewed. Material changes to capabilities, dependencies or publisher identity trigger renewed review.
- Run plugins in the narrowest suitable Wasmtime or native-worker sandbox; enforce per-plugin resource and network restrictions.
- Route all privileged host capabilities through typed broker APIs.
- Uninstall/disable revokes grants and scoped credentials and cancels subscriptions/scheduled tasks. Preserve user data unless explicitly deleted.
- Support official and independent third-party registries. Registry selection does not bypass signature, manifest, sandbox or broker checks.

### Licence and supply-chain governance

- Do not choose the core project licence yet. Complete a project-wide review before architecture sign-off and public release.
- Maintain SPDX identifiers and licence expressions for first-party components and dependencies.
- Generate and publish an SBOM for release artifacts; include transitive dependencies and bundled assets.
- Review Rust crates, Python packages, Qt modules, native libraries, codecs, fonts, icons, themes, kernels, drivers, firmware, model weights, datasets and documentation assets separately.
- Establish component-specific allow/deny/review rules. Do not assume the application, OS distribution, plugin SDK, plugins, model weights and datasets share one licence.
- Require provenance and licence information for official plugins and models. Community submissions must disclose source and licence; unresolved or incompatible rights block official promotion.
- Use automated licence/SBOM checks as triage, not as a substitute for legal review.
- Evaluate contributor terms, patent grants, trademark/branding, source-distribution obligations, notices, attribution and model-use restrictions before final licence selection.
- Plugin authors may use compatible open-source licences as approved in ADR-023; compatibility must be checked against the eventual host/API distribution arrangement.

References: [SPDX licence list](https://spdx.org/licenses/), [Apache licence compatibility FAQ](https://www.apache.org/foundation/license-faq.html), [Sigstore overview](https://docs.sigstore.dev/about/overview/).

### Not decided

Core licence, contributor agreement/CLA policy, plugin SDK licence, official model catalogue licence policy, trademark policy, registry moderation rules and exact SBOM tooling remain deferred until the project-wide review.


## ADR-030 — Core service lifecycle, API contracts and desktop integration

**Status:** Approved initial implementation direction  
**Date:** 2026-11-22

### Core service lifecycle

- Linux personal desktop: run the authoritative core as a per-user systemd user service, with explicit user-controlled startup/stop behaviour.
- Linux headless server: run the core as a dedicated unprivileged system service account under systemd; privileged OS operations remain delegated to the narrow helper.
- Windows personal desktop: run the core as a per-user background process started at login and supervised by the desktop installation. Do not run the ordinary core or model workers as an elevated Windows service.
- Standalone desktop profile may start/stop the same core with the UI lifecycle.
- UI closure does not stop a persistent desktop core; explicit stop/quit and background-task policy govern shutdown.
- Inference and specialist workers remain supervised child processes with independent lifecycle and bounded restart policies.

### API and contract architecture

- Use Axum on Tokio as the initial Rust HTTP/API framework, with Tower middleware for cross-cutting request controls.
- Publish a versioned OpenAPI 3.1 description for the client-facing HTTP API; use JSON Schema-compatible versioned contracts for worker messages and plugin manifests.
- Use explicit major API versioning and a compatibility/deprecation policy. Streaming/WebSocket subprotocols are versioned independently.
- HTTP-compatible request/response operations and WebSockets remain the public client contract. Keep the API independent of desktop UI internals and database schema.
- The API authenticates each client session and authorizes every operation using trusted identity/workspace context. A local connection is not trusted solely because it originates on the same machine.
- Remote access is disabled by default and remains subject to the existing VPN/tunnel and authentication direction. Do not expose an unauthenticated network listener.

### Local IPC and worker protocol

- Linux local client transport uses per-user Unix domain sockets with restrictive permissions and peer-identity checks.
- Windows local client transport uses named pipes with explicit ACLs and client identity checks.
- Local client sessions use short-lived, revocable scoped credentials after OS peer identity is established.
- Worker processes use the approved versioned JSON protocol over supervised standard input/output pipes initially. Standard output is protocol-only; standard error is diagnostics-only.
- Enforce maximum message sizes, framing, schema validation, correlation IDs, deadlines, cancellation and structured errors. Large data uses scoped temporary references, not unbounded inline JSON.
- Workers receive only the minimum task context and capabilities. A worker protocol message is never an authorization grant.

### Desktop application structure

- Use Qt Quick/QML and Qt Quick Controls for the interface, with a thin C++ application/transport layer.
- Use Qt Network for HTTP operations and Qt WebSockets for streaming; add platform-specific transport adapters for local IPC.
- Use modular QML feature areas and typed client-side view models. Keep business rules, authorization, durable state and orchestration in the core.
- Linux desktop integration such as tray, notifications, portal/screen-capture and KDE-specific features remains behind platform adapters; Windows uses equivalent adapters.
- Avoid direct database access from the UI and avoid embedding the core through an in-process Rust FFI boundary in the initial architecture.
- Preserve accessibility, keyboard navigation, screen-reader support and visible awareness/microphone indicators.

### Persistence operational constraints

- SQLite databases remain on a local filesystem, not network shares.
- SQLite WAL is permitted for same-host concurrent readers/writers; it is not used across network filesystems. Use correct SQLite backup/checkpoint procedures rather than copying a live database file without its WAL state.
- PostgreSQL is the only initial shared-server database backend.
- Use SQLx migrations and explicit versioned migration tests for both supported database backends.

References: [Axum](https://github.com/tokio-rs/axum), [Tokio networking](https://docs.rs/tokio/latest/tokio/net/), [Qt Quick](https://doc.qt.io/qt-6/qtquick-index.html), [Qt Quick Controls](https://doc.qt.io/qt-6/qtquickcontrols-index.html), [Qt WebSockets](https://doc.qt.io/qt-6/qtwebsockets-index.html), [SQLite WAL constraints](https://www.sqlite.org/wal.html).

### Not decided

Exact crate versions, API resource names, OpenAPI schema, token format, socket/pipe naming and ACLs, UI module tree, view-model conventions, migration naming and service unit details remain implementation specifications.

## ADR-031 — Linux installer, filesystem and boot security direction

**Status:** Approved initial direction; boot-chain validation required  
**Date:** 2026-11-22

### Installer

- Use Calamares as the initial distribution installer framework, reusing compatible CachyOS installer knowledge/configuration selectively while maintaining our own branding, configuration and modules.
- Provide two guided paths: beginner installation with safe defaults and advanced installation with manual partitioning and detailed controls.
- Beginner mode defaults to UEFI/GPT, Btrfs and a clearly explained automatic partition layout.
- Offer ext4 as an alternative.
- Destructive disk operations require an explicit summary and confirmation; identify the selected target disk clearly and warn about data loss.
- The installer must validate hardware, boot mode, disk layout, available space, network requirements and selected package profile before committing changes.
- Keep installation logs available locally for user-controlled troubleshooting export; do not upload them automatically.

### Disk encryption

- Offer optional full-system disk encryption using LUKS2 on supported Linux installations, particularly recommended for laptops and portable devices.
- Disk-encryption credentials are distinct from the assistant's application master password and recovery key.
- Explain recovery-key/passphrase consequences and provide a recovery procedure before enabling encryption.
- Do not silently enable, disable or reconfigure existing disk encryption.

### Boot security and Secure Boot

- Secure Boot is a supported target, not a requirement that every user must enable.
- Preserve an already-enabled Secure Boot configuration; the installer must not silently instruct users to disable it as a routine workaround.
- Design a verifiable signed boot chain covering the relevant boot manager, kernel and update path. Key enrollment and recovery must be explicit and documented.
- Test key rotation and kernel/bootloader updates so a normal package update does not leave the machine unable to boot.
- Do not claim Secure Boot support until the official installation media, boot chain, update hooks and recovery process pass end-to-end tests.
- The exact bootloader, signing/enrollment model and initial installer-media trust chain remain to be validated against the selected CachyOS components and gaming compatibility requirements.

### Recovery

- Btrfs is the recommended default because it supports the planned snapshot-assisted recovery workflow.
- ext4 remains supported, with documented package downgrade and backup-based recovery rather than snapshot claims.
- Snapshot tooling must preserve bootability and clearly distinguish system rollback from user-data restoration.
- Provide a recovery environment and documented offline repair path.

References: [Calamares](https://calamares.io/about/), [Calamares partitioning guide](https://calamares.codeberg.page/guide/partitions/), [CachyOS Secure Boot guidance](https://wiki.cachyos.org/configuration/secure_boot_setup/).

### Not decided

Exact Calamares modules, partition/subvolume layout, bootloader, UEFI key enrollment model, LUKS2 defaults/crypt parameters, Secure Boot signing custody and snapshot tooling require prototype validation before distribution implementation.
