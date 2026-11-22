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
| Modular core and selective process isolation | Approved | Modular main core; isolate components only for independent lifecycle/resource needs, untrusted or sensitive work, failure containment, or specialised runtime/hardware requirements |\n| Gaming-mode resource protection | Approved | Aggressively prioritize the game; avoid assistant GPU inference, optionally retain a tiny CPU-only model and lightweight tools, enforce adaptive CPU/RAM budgets, and pause nonessential background work |

See [ARCHITECTURE_DECISIONS.md](./ARCHITECTURE_DECISIONS.md) for the decision records.

## Stage 1A — Explored / under evaluation

| Topic | Current understanding | Status |
|---|---|---|
| Overall architecture | Modular core with selectively isolated workers; avoid unnecessary microservices overhead | Approved; see ADR-005 |
| Core/client model | One authoritative core; desktop, voice, companion and other interfaces act as clients | Proposed; awaiting decision |
| IPC | Linux Unix-domain sockets/peer credentials, Windows named pipes/ACLs, structured RPC options researched | No transport selected |
| Process/security boundaries | Main core for orchestration and policy; separate workers only where lifecycle, isolation, failure-containment or specialised-runtime needs justify them | Direction approved; exact topology and sandboxing remain open |
| Data storage isolation | Hybrid strategy based on sensitivity and operational requirements | Direction approved; concrete implementation undecided |

## Next research topic

**Core/client communication and IPC**

Define how desktop and other clients authenticate to the authoritative core, how requests carry trusted user/workspace context, and how local and remote clients communicate safely across Linux, Windows and future headless/server deployments. Compare IPC/RPC options without prematurely selecting a transport.

## Deferred / undecided

- Technology stack and programming languages.
- Desktop/UI framework.
- Backend/runtime architecture details.
- IPC transport and remote protocol.
- Authentication and session technology.
- Database, vector store and concrete tenancy strategy.
- Encryption and key management.
- Exact resource scheduler implementation and workload budgets.
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
7. Keep the GitHub repository as the project documentation source of truth.
