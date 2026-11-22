# Local Assistant 2.0 — Phase 0 Technical Spike Plan

**Status:** Planned; experiments not yet executed  
**Date:** 2026-11-22  
**Purpose:** Resolve high-risk compatibility and security assumptions before production implementation.  
**Rule:** A spike is disposable validation code, not a production feature. Record environment, versions, commands, results, limitations and reproducibility instructions.

## 1. Execution rules

- Run spikes in isolated branches or disposable workspaces; do not merge prototype code into production modules.
- Pin toolchain, dependency and native-library versions in each report.
- Test Linux and Windows independently wherever both are supported.
- Record negative results and rejected alternatives, not just successful demonstrations.
- Never weaken security controls to make a spike pass.
- Do not use real user data, production credentials, signing keys or private model assets.
- A spike passes only when every mandatory acceptance criterion is met.
- If a platform cannot meet a security criterion, record the limitation and propose a stronger supported design; do not silently downgrade the requirement.

## 2. Spike register

| ID | Spike | Priority | Target platforms | Blocks |
|---|---|---|---|---|
| SP-01 | rusqlite + SQLCipher + FTS5 | Critical | Linux, Windows | Phase 2 |
| SP-02 | SQLx + PostgreSQL and authorization boundaries | High | Linux initially; CI portability review | Phase 2, 8 |
| SP-03 | Local client IPC: UDS and named pipes | Critical | Linux, Windows | Phase 1 |
| SP-04 | Supervised worker protocol and lifecycle | Critical | Linux, Windows | Phase 1, 4 |
| SP-05 | Native worker sandbox conformance | Critical | Linux, Windows | Phase 1, 6, 7 |
| SP-06 | Wasmtime capability sandbox | High | Linux, Windows | Phase 7 |
| SP-07 | llama.cpp / ONNX Runtime hardware matrix | Critical | Supported x86-64-v3 systems | Phase 3 |
| SP-08 | Qt 6/QML packaging and client connectivity | High | Linux, Windows | Phase 5 |
| SP-09 | GRUB + Btrfs snapshot + Secure Boot recovery chain | Critical | Representative UEFI hardware/VMs | Phase 9 |
| SP-10 | Calamares install, partitioning and LUKS2 flows | High | Representative UEFI hardware/VMs | Phase 9 |
| SP-11 | Build, signing, SBOM and release provenance pipeline | High | CI; Linux and Windows artifacts | Phase 0, 9, 10 |

## 3. Spike acceptance criteria

### SP-01 — rusqlite + SQLCipher + FTS5

**Question:** Can the exact selected Rust/SQLite dependency configuration build and operate encrypted personal databases on Linux and Windows?

**Prototype:**
- Minimal Rust program using rusqlite with bundled SQLCipher and FTS5 enabled.
- Create, migrate, query and close an encrypted database.
- Exercise FTS5 search.
- Exercise backup, restore, WAL and checkpoint behaviour on a local filesystem.
- Inspect linked SQLite/SQLCipher build and runtime versions.

**Pass criteria:**
- Clean reproducible builds on supported Linux and Windows toolchains.
- Runtime confirms SQLCipher is active; the application fails closed if encryption support is absent.
- Correct key opens the database; wrong key and modified ciphertext fail safely.
- FTS5 works in the same encrypted build.
- Backup/restore preserves encrypted database integrity and searchable content.
- WAL/checkpoint behaviour is understood and tested; WAL is never used over a network filesystem.
- Extension loading is disabled unless a separately reviewed use case requires it.
- Dependency versions and feature flags are pinned and documented.

**Fail / escalation:** Any plaintext fallback, platform-specific missing encryption, unreliable FTS5 integration or unrecoverable backup behaviour blocks the selected configuration.

### SP-02 — SQLx + PostgreSQL

**Question:** Can the shared-server persistence candidate support migrations and enforce the intended defence-in-depth isolation?

**Prototype:**
- Minimal SQLx/PostgreSQL service with versioned migrations.
- Simulate multiple users and workspaces.
- Apply row-level security policies and application-level authorization checks.
- Test connection pooling and transaction context handling.

**Pass criteria:**
- Migrations run cleanly from an empty database and from the previous test schema.
- Cross-user and cross-workspace reads/writes are denied at the application layer.
- RLS provides an additional denial boundary when a query is accidentally missing an expected filter.
- Connection-pool reuse cannot leak identity/session context between requests.
- Application authorization remains mandatory even when RLS is enabled.
- Backup/restore and schema-upgrade procedures are documented.

**Fail / escalation:** Any cross-tenant data exposure, pooled-context leakage or reliance on RLS as the sole authorization mechanism blocks approval.

### SP-03 — Local client IPC

**Question:** Can local desktop clients authenticate to the core using platform-native IPC with explicit peer checks?

**Linux prototype:**
- Unix-domain socket owned by the intended per-user service.
- Restrictive directory/socket permissions.
- Peer credential verification and short-lived scoped client sessions.

**Windows prototype:**
- Named pipe with explicit ACLs.
- Client identity verification and short-lived scoped client sessions.

**Pass criteria:**
- Authorized same-user client can establish a session.
- Unauthorized OS user cannot connect or impersonate an authorized client.
- Socket/pipe replacement, stale endpoints and service restart are handled safely.
- Every operation is still authorized by the core; transport identity alone is not sufficient.
- No unauthenticated remote listener is created.
- Linux and Windows negative tests run independently and pass.

**Fail / escalation:** If peer identity cannot be reliably bound to a session, select a stronger OS-supported transport/authentication arrangement before Phase 1.

### SP-04 — Supervised worker protocol and lifecycle

**Question:** Can the core supervise Python workers over a bounded, versioned JSON protocol?

**Prototype:**
- Framed request/response messages over supervised stdio.
- Correlation IDs, protocol negotiation, schemas, deadlines, cancellation and structured errors.
- stdout reserved for protocol; stderr reserved for diagnostics.
- Large payloads represented by scoped temporary-file references.
- Test worker crash, hang, malformed output and restart.

**Pass criteria:**
- Invalid, oversized, malformed or out-of-order messages are rejected safely.
- Worker cannot choose or alter trusted user/workspace identity.
- Deadlines and cancellation terminate or isolate the relevant worker.
- Worker crash does not crash the core.
- A timeout does not trigger blind repetition of a potentially completed non-idempotent action.
- stdout diagnostics cannot corrupt protocol framing.
- Temporary references are scoped, expire and are cleaned up.

### SP-05 — Native worker sandbox conformance

**Question:** Can native plugin/automation workers be meaningfully restricted on each supported OS?

**Linux prototype:**
- Bubblewrap namespaces, minimal explicit mounts, Landlock, seccomp, dropped capabilities, no-new-privileges and resource limits.
- No host D-Bus/session sockets by default.
- Test denied filesystem, process, network and privilege operations.

**Windows prototype:**
- AppContainer where compatible, restricted tokens, explicit ACLs, job objects/resource limits and network restrictions.
- Test denied filesystem, process, network and privilege operations.

**Pass criteria:**
- Each sandbox has a documented threat boundary and tested policy.
- Unauthorized host paths, processes, sockets and network destinations are inaccessible.
- Resource limits are enforced and worker termination is reliable.
- Missing mandatory isolation capabilities cause fail-closed behaviour or a documented stronger fallback.
- Process separation is never treated as proof of sandboxing.
- Independent Linux and Windows conformance reports exist.

**Fail / escalation:** Any worker path that silently runs unrestricted when isolation is unavailable blocks untrusted worker execution.

### SP-06 — Wasmtime capability sandbox

**Question:** Can Wasm plugins use explicit host capabilities without ambient access?

**Prototype:**
- Minimal component/plugin interface with explicit host calls.
- No ambient filesystem or network access.
- Explicit preopens/capabilities, memory limits, fuel/time limits and broker-mediated privileged calls.

**Pass criteria:**
- Plugin cannot access filesystem/network except through granted capabilities.
- Resource exhaustion is bounded and the instance can be terminated.
- Plugin cannot mint identity, grant permissions or bypass the broker.
- Capability revocation and plugin shutdown behave predictably.
- ABI/contract version mismatch is rejected with an actionable error.

### SP-07 — Inference hardware/backend matrix

**Question:** Which llama.cpp and ONNX Runtime combinations are actually supportable on the initial CPU/GPU baseline?

**Prototype:**
- CPU-only inference baseline.
- NVIDIA CUDA path on the RTX 3050 4 GB development system.
- AMD and Intel paths evaluated on available representative hardware.
- Hybrid-graphics selection and CPU fallback.
- Model load/unload, VRAM accounting and Gaming Mode release tests.

**Pass criteria:**
- CPU fallback works without a supported GPU.
- Tested backend, driver and runtime versions are recorded.
- Unsupported combinations fail clearly and fall back safely.
- Memory pressure and model unloading do not corrupt active tasks.
- Gaming Mode stops assistant GPU inference and releases assistant allocations as designed.
- Published compatibility claims are limited to tested combinations.

**Fail / escalation:** Do not claim support for a vendor/backend combination without reproducible hardware evidence.

### SP-08 — Qt 6/QML packaging and client connectivity

**Question:** Can one Qt 6/QML client codebase package and communicate with the core on Linux and Windows?

**Prototype:**
- Minimal QML window and thin C++ transport/view-model layer.
- Versioned API request/response and WebSocket event stream.
- Linux and Windows package smoke tests.
- Keyboard navigation and basic screen-reader checks.

**Pass criteria:**
- Clean build and launch on both supported desktop platforms.
- Client authenticates to the core using the selected local transport.
- UI has no direct database access and does not make authorization decisions.
- Core restart and connection loss are handled without UI crashes or false success.
- Keyboard navigation and basic accessibility checks pass.

### SP-09 — GRUB, Btrfs snapshots and Secure Boot

**Question:** Can the intended boot and recovery chain survive normal updates and recover from failures without weakening Secure Boot?

**Prototype:**
- UEFI test systems/VMs with Btrfs and ext4 coverage where practical.
- GRUB boot entries and Btrfs snapshot integration.
- Signed boot-chain prototype, key enrollment and rotation procedure.
- Kernel/initramfs and bootloader update tests.
- Failed-update and recovery drills.

**Pass criteria:**
- Normal signed boot succeeds with Secure Boot enabled on the declared test configurations.
- Snapshot boot entries select matching kernel/initramfs content.
- Failed update retains a known-good recovery route.
- Signature verification/lockdown is not silently disabled.
- Existing firmware keys are not silently replaced.
- Key enrollment, revocation/rotation and recovery are documented and tested.
- Claims are limited to the configurations that pass end-to-end tests.

**Fail / escalation:** Secure Boot must remain an unadvertised validation target until the complete installation, update and recovery chain passes.

### SP-10 — Calamares, partitioning and LUKS2

**Question:** Can the installer safely support guided and advanced installation paths?

**Prototype:**
- Calamares configuration and project-owned modules where required.
- UEFI/GPT guided Btrfs path.
- ext4 alternative.
- Optional LUKS2 system encryption.
- Advanced/manual partitioning path.
- Destructive-action confirmation and preflight validation.

**Pass criteria:**
- Install to disposable test disks/VMs without modifying unintended disks.
- Partition summary and destructive-operation confirmation are explicit.
- Preflight catches unsupported or inconsistent layouts.
- Btrfs and ext4 installations boot and recover.
- LUKS2 unlock and recovery procedure is documented.
- Installer failure leaves a clear, non-misleading result.

### SP-11 — Build, signing, SBOM and provenance

**Question:** Can CI produce traceable Linux and Windows artifacts without exposing production signing secrets?

**Prototype:**
- Linux and Windows build workflows.
- Formatting, lint, test, dependency and secret scanning.
- SPDX SBOM generation.
- Artifact checksums and provenance metadata.
- Separate test signing from production release-key custody.

**Pass criteria:**
- Clean CI builds are reproducible to the documented level.
- Failed required checks block artifact promotion.
- SBOMs correspond to the actual produced artifacts.
- No signing secret is stored in source control or exposed to untrusted pull-request jobs.
- Release artifacts have verifiable provenance and documented promotion steps.
- Licence scanning reports exceptions for human review rather than silently accepting incompatible dependencies.

## 4. Required report template

Each spike must produce a report containing:

1. Spike ID and question.
2. Date, operator and clean environment details.
3. OS, CPU architecture, toolchain and dependency versions.
4. Exact source revision and commands to reproduce.
5. Test cases and expected outcomes.
6. Actual results and logs with secrets removed.
7. Pass/fail result for each acceptance criterion.
8. Known limitations and unresolved risks.
9. Decision: accept, revise, replace or defer.
10. Follow-up ADR or implementation-specification changes, if needed.

## 5. Recommended execution order

1. SP-01 encrypted SQLite/FTS5.
2. SP-03 local IPC on Linux and Windows.
3. SP-04 supervised worker protocol.
4. SP-05 native sandbox conformance.
5. SP-02 PostgreSQL isolation.
6. SP-06 Wasmtime capabilities.
7. SP-07 inference hardware/backend matrix.
8. SP-08 Qt packaging and connectivity.
9. SP-09 GRUB/Secure Boot/recovery.
10. SP-10 Calamares installation and LUKS2.
11. SP-11 CI, signing, SBOM and provenance integration.

The order prioritizes the technical risks that could invalidate core architecture assumptions. Some experiments can run in parallel once their environments are ready, but security-sensitive results must be reviewed before dependent production work begins.

## 6. Phase 0 exit gate

Phase 0 is ready to close only when:

- [ ] Requirements traceability matrix has been reviewed and ownership/test plans refined.
- [ ] Repository boundaries and code-bootstrap governance are settled.
- [ ] Linux and Windows development toolchains and CI design are documented.
- [ ] Critical spikes SP-01, SP-03, SP-04 and SP-05 have recorded pass/fail results.
- [ ] Remaining high-priority spikes have results or explicit risk acceptance and a scheduled resolution.
- [ ] No unresolved finding requires weakening approved privacy, authorization or isolation requirements.
- [ ] Phase 1 component specifications and acceptance tests are ready.
- [ ] User approves moving from engineering preparation into implementation.

**Important:** Completing Phase 0 planning and prototypes does not automatically authorize broad implementation. Phase 1 begins only after its entry gate is reviewed.
