# Phase 0 — Exit Review and Transition Decision

**Review date:** 2026-11-22  
**Repository:** `joedanielmsn/AssistantV2`  
**Decision:** Proceed to Phase 1 secure-core implementation preparation and then the main working prototype, with the deferred risks below retained as explicit gates.

## 1. Review outcome

Phase 0 has produced enough Linux prototype evidence to begin the first secure core foundation. Continuing broad exploratory tests now has diminishing value because several outstanding checks depend on production components that do not yet exist.

This is a **controlled transition**, not a declaration that all Phase 0 acceptance criteria passed. SP-01, SP-03, SP-04, SP-05 and SP-06 remain incomplete as full cross-platform or production acceptance items. SP-08 through SP-11 are partially or wholly deferred.

The user explicitly requested that testing be kept focused on genuinely necessary risks and that implementation begin sooner. This review records that direction.

## 2. What is accepted as sufficient to proceed

- Linux prototype evidence exists for encrypted SQLite/SQLCipher/FTS5, local IPC, worker supervision, native sandbox primitives, PostgreSQL isolation, Wasmtime capabilities and local inference.
- The core authority and central permission-broker architecture is approved.
- The initial CachyOS-only delivery target and x86-64-v3 baseline are approved.
- Repository governance is settled: one platform monorepo, planning under `plan/`, implementation at root, direct focused commits to `main`.
- The initial inference model candidates and configurable context-length requirement are documented.
- Major limitations and deferred tests are now explicit in this folder.

## 3. Items that remain open

| Item | Status | Gate |
|---|---|---|
| SP-01 Windows build and retained dependency patch review | Deferred | Before cross-platform claims or final persistence acceptance |
| SP-02 trusted DB identity, membership protection and role design | Open design/integration | Before shared-server release; core identity rules apply from the first repository/API |
| SP-03 client enrollment and broker integration | Open | Before treating any desktop client as trusted |
| SP-04 trusted context, cooperative cancellation, temp references and production resource budgets | Open | Before enabling real worker tools |
| SP-05 cgroup descendant cleanup, supervisor restart, fail-closed integration and production policies | Open | Before untrusted workers execute |
| SP-06 Component Model/WIT, broker integration and revocation | Open | Before third-party plugin support |
| SP-07 full-context memory behavior and production reliability | Deferred | Revisit only if implementation evidence exposes a concrete issue |
| SP-08 Qt client prototype | Deferred | Before desktop UI milestone |
| SP-09 actual boot rollback/recovery | Not proven | Mandatory before bootloader/update/recovery integration |
| SP-10 installer/LUKS2 | Not tested | Mandatory before installer/distribution work |
| SP-11 CI/release supply chain | Not tested | Basic CI before meaningful implementation; signing/SBOM/provenance before releases |

## 4. Phase 1 scope

The first working prototype is the **secure local core foundation**, not a fully autonomous assistant and not an OS installer.

Initial implementation should establish:

1. A clean Rust workspace and pinned toolchain/dependencies.
2. A runnable core process with structured configuration and lifecycle.
3. Trusted per-OS-user identity context and explicit user/workspace/task scope types.
4. A deny-by-default central permission broker with RBAC + ABAC policy inputs.
5. A minimal authenticated local IPC boundary, initially Linux Unix-domain sockets.
6. A narrow operation registry and mock/safe built-in operations; no arbitrary shell or unrestricted OS executor.
7. Structured audit events with sensitive-data minimization.
8. A supervised worker interface only after identity, authorization and sandbox contracts are in place; no unrestricted worker execution.
9. A basic build/test/format/lint workflow suitable for CachyOS Linux.
10. Configuration and health/status surfaces sufficient to run and inspect the prototype.

The first milestone should be small enough to compile and run end-to-end. Do not start the Qt UI, model tool execution, broad plugin system, system-wide daemon, privileged helper, bootloader changes or installer as part of this initial foundation.

## 5. Mandatory implementation gates

These requirements are not waived by the Phase 0 transition:

- Every request is associated with trusted identity and scope created by the core.
- Every privileged or sensitive operation is authorized by the central permission broker.
- Missing policy/identity context means deny.
- Worker/model/plugin input is untrusted and cannot assert identity or permissions.
- Untrusted workers cannot start unless the complete mandatory sandbox is successfully established.
- Resource budgets and cancellation are explicit; uncertain non-idempotent actions are not blindly retried.
- Database access must not rely on caller-controlled tenant context as proof of identity.
- Context-length selection is validated against available resources and never silently changed.
- No OS-level boot/update/partitioning operation is included in the first prototype.

## 6. Phase 0 closure classification

**Engineering discovery:** Sufficient to transition.  
**All original spike acceptance criteria:** Not complete.  
**Cross-platform acceptance:** Not complete and intentionally deferred.  
**Production security acceptance:** Not complete; enforced through implementation gates and later review.  
**Authorization to begin Phase 1:** Approved by the user's explicit instruction to document Phase 0 and begin the main working prototype.

The original detailed exit checklist in `plan/PHASE_0_TECHNICAL_SPIKES.md` should not be rewritten as if every item passed. This transition review records a scoped, risk-aware path forward.

## 7. Next action

Create and review the Phase 1 secure-core specification, then bootstrap the minimal Rust workspace at the repository root. Keep each change focused, buildable and committed directly to `main`.