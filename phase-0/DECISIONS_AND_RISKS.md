# Phase 0 — Decisions, Constraints and Deferred Risks

**Date:** 2026-11-22  
**Authority:** User-approved architecture directions and recorded Phase 0 prototype evidence.

This document distinguishes approved product/architecture direction from provisional implementation candidates and unresolved risks.

## Approved project direction

1. **Fresh codebase:** Local Assistant 2.0 starts as a new independent codebase. The old `~/local-assistant` project is reference material only; no reuse, migration, extension or dependency.
2. **One platform repository:** `joedanielmsn/AssistantV2` is the platform monorepo. Planning remains under `plan/`; implementation is at the repository root.
3. **Workflow:** Develop directly on `main`, use focused commits, keep `main` buildable, do not force-push or rewrite shared history.
4. **Initial platform:** CachyOS Linux only for initial development, validation and product focus. Broader base-Linux support is a later objective. Windows testing is deferred.
5. **CPU baseline:** x86-64-v3 minimum; x86-64-v4 may be an optional optimized target. Original x86-64-only and ARM64 are outside the initial baseline.
6. **Deployment direction:** Personal per-OS-user desktop installations first; preserve architecture for future shared multi-user server deployments.
7. **Core authority:** The Rust core owns trusted identity, authorization, orchestration and privileged-operation decisions.
8. **Permission architecture:** One central permission broker is the only authorization path for privileged OS operations. Use hybrid RBAC + ABAC. Workers and models are untrusted requesters, never authorities.
9. **Data ownership:** Installation, user, workspace and task/session scopes; private by default; every operation carries trusted user/workspace context.
10. **Persistence direction:** Personal encrypted SQLite using rusqlite + bundled SQLCipher + FTS5; PostgreSQL + SQLx for shared-server persistence. Prototype results are provisional and do not finalize key management or DB roles.
11. **Worker direction:** Rust/Tokio supervisor with separate Python worker processes and versioned JSON-over-stdio protocol.
12. **Sandbox direction:** Linux Bubblewrap namespaces, minimal mounts, Landlock, seccomp, dropped capabilities, no-new-privileges, RLIMITs and delegated cgroup v2. Missing mandatory isolation must fail closed.
13. **Wasm plugin direction:** Embedded Wasmtime with explicit host capabilities and no ambient filesystem/network access by default.
14. **Inference direction:** Qwen 3.5 2B Q4_K_M primary candidate and 0.8B Q8_0 fallback candidate, both provisional.
15. **Context setting:** User-facing 4K–16K slider, 1K steps, provisional 8K default, persistent selection and runtime memory validation.
16. **OS integration:** Long-term CachyOS integration is a goal, but privileged OS modifications and boot/update integration are not part of the first core prototype.

## Hardware and runtime baseline

Development machine: ASUS TUF A15; NVIDIA RTX 3050 Mobile 4 GB; 16 GB DDR5; AMD Ryzen 7 170 (8 cores/16 threads); CachyOS x86_64. Observed kernel during spikes: 7.3.5-2-cachyos. Observed NVIDIA driver: 620.73.04. Observed Ollama: 0.40.17.

This is a development baseline, not a broad compatibility claim. No AMD GPU inference or Intel GPU backend support is established by the available evidence.

## High-priority implementation risks

| Risk | Why it matters | Required implementation control |
|---|---|---|
| Trusted identity propagation | PostgreSQL custom RLS context can be set by the application role; workers can forge claims | Core authenticates principal; derive scope from trusted membership; ignore worker identity as authority; bind context to each operation |
| Permission-broker bypass | A model, worker or plugin must not self-authorize | All privileged tools call one broker; deny by default; test direct executor and helper bypass attempts |
| Sandbox fail-open | Bubblewrap/Landlock/seccomp/cgroup setup may be unavailable or partially applied | Treat mandatory controls as a transaction; do not start untrusted worker unless complete policy is established |
| Worker descendant cleanup | Process-group test is not proof of cgroup cleanup | Production worker cgroup owns all descendants; supervisor teardown empties/removes cgroup and verifies no survivors |
| Worker timeout ambiguity | A timed-out action may have completed before worker termination | No blind retry of non-idempotent actions; use operation IDs, state reconciliation and idempotency contracts |
| Database encryption/key management | SQLCipher prototype uses fixed test keys, not production key hierarchy | Define key derivation/wrapping, recovery, lock/unlock, rotation and zeroization before personal-data release |
| SQLCipher build patch | Extension disabling used a disposable local crate patch | Review and retain a maintainable patch or supported build configuration; verify locked clean build |
| Same-UID client trust | OS UID does not distinguish trusted UI from same-user plugin/worker | Explicit client enrollment/session policy; IPC peer credentials are only one authentication input |
| Wasm host capability | Prototype host function was not broker-connected | Host assigns plugin identity; capability grants are explicit and broker-mediated; no ambient WASI |
| Model tool-call reliability | Five controlled checks do not establish reliability | Validate and authorize every tool call in core; schema-check arguments; require approval by policy; treat model output as untrusted |
| Context memory fit | Short prompts do not establish fully populated context memory safety | Runtime estimates/validates memory and handles load failure safely; do not silently lower selected context |
| Boot/recovery | Preliminary audit is not a rollback drill | Keep OS boot/update integration out of the core prototype; complete SP-09 in disposable VM before Phase 9 |
| CI/release supply chain | SP-11 has no execution evidence | Add basic CI early; later require SBOM, provenance, protected signing keys and promotion gates |

## Explicitly deferred validation

The following are deliberately not required before starting the first secure core prototype:

- Windows builds, named-pipe/AppContainer validation and Windows packaging.
- Other Linux distributions and ARM64.
- Broad adversarial sandbox escape campaigns.
- Deliberate cgroup `memory.max` OOM and recovery testing.
- Production supervisor/cgroup lifecycle integration tests before the supervisor implementation exists.
- Full Wasmtime Component Model/WIT compatibility campaign.
- Full Qt packaging/accessibility matrix.
- GRUB/Secure Boot/installer work before the OS distribution phase.
- Exhaustive model latency/context stress benchmarks absent a concrete implementation issue.
- Final licensing decision; licensing review remains deferred to architecture/release planning as previously requested.

Deferral does not waive a security requirement. For example, fail-closed sandbox behavior is an implementation acceptance condition even though the disposable fail-closed experiment is deferred.

## Non-negotiable security rules

- Models propose; the trusted core validates; the permission broker authorizes; the executor performs only authorized actions.
- No worker, plugin or model can choose its own trusted user/workspace identity.
- No privileged operation bypasses the central permission broker.
- No untrusted worker runs unrestricted when a mandatory sandbox primitive is missing.
- RLS is defense in depth, not authentication.
- A signature or sandbox label is not proof that a plugin is safe.
- No automatic retry of uncertain non-idempotent actions.
- No secrets, production credentials, private databases, model weights or local user data in Git.
- Do not claim untested platform or hardware support.

## Change control

Any change to these approved directions requires an explicit new decision record. Routine implementation choices may use conservative defaults, but must preserve the trust boundaries and constraints above.