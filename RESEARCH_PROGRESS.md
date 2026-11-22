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

## Next research topic

**Intelligence architecture**

Research the central orchestrator, specialist invocation, model routing, context construction, tool selection, bounded parallelism and reliable task execution. Preserve the approved resource manager and permission-broker boundaries. Routine implementation defaults will be selected autonomously; only consequential product or performance trade-offs will be brought to the user.

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

## Next phase

**Phase 3 — Intelligence architecture.** Research the central orchestrator, specialist invocation, model routing, context construction, tool selection, bounded parallelism and reliable task execution. Bring only major product/design/performance trade-offs to the user at the end of the phase.
