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

See [ARCHITECTURE_DECISIONS.md](./ARCHITECTURE_DECISIONS.md) for the decision records.

## Stage 1A — Explored / under evaluation

| Topic | Current understanding | Status |
|---|---|---|
| Overall architecture | Hybrid modular core; separate processes only where lifecycle, security isolation or hardware access justify them | Proposed; not finally approved |
| Core/client model | One authoritative core; desktop, voice, companion and other interfaces act as clients | Proposed; awaiting decision |
| IPC | Linux Unix-domain sockets/peer credentials, Windows named pipes/ACLs, structured RPC options researched | No transport selected |
| Process/security boundaries | Trusted core, authenticated clients, restricted workers, independent AI workloads and external environments | Proposed; requires further design |
| Data storage isolation | Hybrid strategy based on sensitivity and operational requirements | Direction approved; concrete implementation undecided |

## Next research topic

**Central permission broker and authorization lifecycle**

Research and define:
- Identity, authentication, authorization, approval, execution and audit as distinct stages.
- Central enforcement for agents, plugins, integrations, tools and automation.
- Policy approaches: RBAC, ABAC, capability-based permissions and practical hybrid options.
- Scope, duration, expiry, revocation and delegated authority.
- Approval tiers, action previews, risk/reversibility and user-configurable autonomy.
- Background task authorization, cancellation, audit trails and emergency stop.
- A practical initial-release design that preserves future extensibility.

Present a recommendation and explicit decision checkpoint. Do not record it as approved until the user confirms.

## Deferred / undecided

- Technology stack and programming languages.
- Desktop/UI framework.
- Backend/runtime architecture details.
- IPC transport and remote protocol.
- Authentication and session technology.
- Database, vector store and concrete tenancy strategy.
- Encryption and key management.
- Resource scheduler implementation.
- Exact retention, backup and deletion periods.
- Licensing (intentionally deferred; revisit at architecture finalisation and before public release).

---

## Working rules

1. Research before implementation.
2. Do not assume technologies from the previous repository or earlier experiments.
3. Keep approved decisions separate from proposals and deferred choices.
4. Inspect existing repository files before changing them.
5. Preserve established requirements when updating the master plan.
6. Ask for explicit approval at decision checkpoints.
7. Keep the GitHub repository as the project documentation source of truth.
