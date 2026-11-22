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
