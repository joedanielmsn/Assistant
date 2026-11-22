# Local Assistant 2.0 — Phase 0 Technical Spike Plan

**Status:** Transitioned to Phase 1 with deferred acceptance items; see [consolidated Phase 0 results](../phase-0/SPIKE_RESULTS.md) and [exit review](../phase-0/EXIT_REVIEW.md).  
**Date:** 2026-11-22  
**Purpose:** Resolve high-risk compatibility and security assumptions before production implementation.  
**Rule:** A spike is disposable validation code, not a production feature. Record environment, versions, commands, results, limitations and reproducibility instructions.

## 1. Execution rules

- Run prototypes in disposable workspaces; record validated reports directly on `main`. Do not place disposable prototype code in production modules.
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

### SP-01 execution report — Linux prototype (2026-11-22)

**Result:** The consolidated Linux regression suite passed. Overall SP-01 remains **in progress** pending Windows validation, retained/committed spike-fixture review and final dependency review.

**Environment**
- OS: CachyOS
- Architecture: x86_64 (the host was not independently checked for the full x86-64-v3 feature set during this spike)
- Rust/Cargo: 1.100.0
- Build tools observed: pkg-config 3.0.11, CMake 4.5.1, Ninja 1.13.2
- rusqlite: 0.43.0
- libsqlite3-sys: 0.40.0, locally patched copy
- SQLCipher source/runtime: 4.20.0 (runtime reported `4.20.0 community`)
- Bundled SQLite source version reported by the dependency: 3.54.1
- SQLCipher feature: `bundled-sqlcipher-vendored-openssl`; backup API feature enabled
- System SQLite pkg-config version: 3.54.0; system SQLCipher pkg-config metadata was not detected. The successful build used bundled SQLCipher and did not rely on system SQLCipher.

**Prototype and observed tests**
1. Built a disposable Rust project under `/tmp/assistantv2-sp01-SSgSG0`; no production repository or existing assistant project was modified.
2. Confirmed `PRAGMA cipher_version` returned SQLCipher 4.20.0.
3. Created an encrypted database and FTS5 virtual table; inserted content and successfully searched for a unique token.
4. Closed and reopened the encrypted database with the correct key; the FTS5 search result remained available.
5. Opened the database with an incorrect key; the database was rejected.
6. Created an encrypted online backup, reopened it with the correct key and verified both FTS5 search and checkpoint-table data. An incorrect backup key was rejected.
7. Searched the closed database and backup bytes for a unique plaintext marker; the marker was absent. Earlier targeted WAL marker check also passed.
8. Confirmed WAL mode. A PASSIVE checkpoint observed 22 frames and 22 checkpointed; a subsequent TRUNCATE checkpoint reported no remaining frames. All 20 test rows remained readable.
9. Modified an encrypted database byte at offset 2000 without touching the salt. SQLCipher logged an HMAC verification failure and the subsequent query was rejected.
10. Patched the disposable local `libsqlite3-sys 0.40.0` build script to replace `-DSQLITE_ENABLE_LOAD_EXTENSION=1` with `-DSQLITE_OMIT_LOAD_EXTENSION=1`. Cargo resolved the patched local crate. A clean build (`cargo clean`, then `env -u CFLAGS cargo run --locked --offline`) passed without relying on a manually supplied CFLAGS variable.
11. Verified `PRAGMA compile_options` reported `OMIT_LOAD_EXTENSION`, did not report `ENABLE_LOAD_EXTENSION`, and the SQL `load_extension()` function was unavailable. The compiled `libsqlcipher.a` archive contained neither `sqlite3_load_extension` nor `sqlite3_enable_load_extension`.
12. The consolidated regression executable completed with all tests passing.

**Acceptance status**
- [x] Linux bundled build and runtime SQLCipher detection.
- [x] Correct-key access and wrong-key rejection.
- [x] FTS5 in the same encrypted build.
- [x] Encrypted online backup and restore, including FTS5 content and wrong-key rejection.
- [x] WAL mode and PASSIVE/TRUNCATE checkpoint behaviour on local storage.
- [x] Modified-ciphertext rejection with SQLCipher HMAC failure observed.
- [x] Explicit verification that extension loading is disabled in the patched Linux build.
- [x] Clean locked offline Linux rebuild without manually supplied CFLAGS.
- [ ] Windows clean build and runtime validation.
- [ ] Retained/committed spike fixture and final pinned-lockfile/dependency review.

**Limitations and interpretation**
- The prototype and local crate patch remain disposable files outside the repository. The local patch has not yet been reviewed as a retained project dependency or tested on Windows.
- The plaintext-marker checks are targeted smoke tests, not a general proof that no sensitive data can ever appear in temporary files, logs, journals or process memory.
- Wrong-key and modified-ciphertext rejection were demonstrated for these test cases; broader corruption-recovery and crash/interruption behaviour remain untested.
- The backup test used SQLCipher-keyed source and destination connections and verified restoration; it is not a complete backup/recovery assurance case.
- No Windows result is available, so the cross-platform acceptance question remains open.
- The temporary prototype uses fixed test keys only; this is not a production key-management design.

**Decision:** Retain the approved rusqlite + bundled SQLCipher + FTS5 direction provisionally. Retain the local `libsqlite3-sys` build-script patch as the current candidate for disabling extension loading. Do not mark SP-01 fully complete or treat this Linux smoke test as production readiness until the patch and fixture are reviewed in the repository and Windows validation passes.


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

### SP-03 execution report — Linux prototype (2026-11-22)

**Result:** All executed Linux prototype test groups passed. SP-03 remains **in progress** overall because this was a disposable prototype, production security review and Windows named-pipe validation remain outstanding.

**Environment**
- OS: CachyOS, kernel 7.3.5-2-cachyos, x86_64.
- Rust/Cargo: 1.100.0.
- Tokio: 1.54.1; source was cached. The prototype disabled Tokio default features and proc macros to permit an offline build.
- util-linux `setpriv`: 2.42.4.
- A `nobody` account exists (UID/GID 65534).
- The normal user shell could not switch UID with `setpriv` (`setresuid failed: Operation not permitted`). Cross-UID tests were therefore run with the disposable test server elevated; the connecting client ran as UID 65534.
- All prototype code remained in a temporary directory under `/tmp/assistantv2-sp03-ARbhF9`; no production code was added.

**Executed test groups and results**

1. **Socket access and peer identity**
   - Per-user private socket denied a client running under a different UID.
   - On a deliberately accessible test socket, Linux `SO_PEERCRED` reported connecting UID 65534.
   - Server compared peer UID with its expected UID and rejected the different UID.
   - Result: all three reported checks passed.

2. **Socket lifecycle**
   - A second listener could not replace an active socket.
   - Cleanup refused to remove an active socket.
   - After the listener exited, the stale socket was detected and removed.
   - A new listener successfully rebound the path.
   - Cleanup rejected a regular file at the socket path and preserved its contents.
   - Result: all five checks passed.

3. **Session mechanics**
   - Same-UID client completed a handshake and received a random 256-bit token generated from `/dev/urandom`.
   - Session allowed a `read` operation and denied an out-of-scope `write`.
   - Explicit revocation immediately denied a subsequent request.
   - An expired session was denied.
   - Result: all four checks passed.

4. **Protocol robustness**
   - Valid request received the expected response.
   - Malformed request received an error response.
   - Oversized request exceeding the 1 KiB prototype limit caused connection closure; test accepted EOF or Linux `ConnectionReset`.
   - Server remained available after abrupt client disconnect.
   - Sixteen concurrent clients received valid responses.
   - Server restarted and accepted a new connection.
   - Result: all six checks passed.

**Acceptance status — Linux prototype evidence**
- [x] Restrictive per-user socket access demonstrated.
- [x] Linux peer credentials retrieved and a different UID rejected.
- [x] Active-socket replacement/removal protections demonstrated.
- [x] Stale endpoint cleanup and subsequent rebind demonstrated.
- [x] Non-socket path preserved by cleanup helper.
- [x] Short-lived session mechanics, scope checks, revocation and expiry demonstrated.
- [x] Malformed and oversized request handling demonstrated.
- [x] Concurrent client and abrupt-disconnect handling demonstrated.
- [x] Restart/reconnection demonstrated.
- [ ] Production-grade client enrollment/authentication bound to a client identity, beyond OS UID.
- [ ] Production protocol framing, comprehensive parser fuzzing, rate limits and resource-exhaustion review.
- [ ] Race-resistant socket-directory ownership and startup coordination reviewed and implemented.
- [ ] Integration with the central permission broker and trusted user/workspace authorization context.
- [ ] Independent Windows named-pipe ACL and client-identity validation.

**Limitations and interpretation**
- These were separate disposable demonstrations, not one integrated IPC implementation. In particular, the simplified protocol robustness harness did not combine peer-credential checks, session authentication and message-size handling into a single server.
- The session registry was in-memory. The handshake issued a session to any process with the same UID that could connect; it did not establish a distinct enrolled desktop-client identity. A same-UID plugin or worker must not become trusted merely because it shares the OS account.
- The session's `read` scope was a prototype allowlist, not the central permission broker. Production authorization must also evaluate user, workspace, resource, component, action and policy conditions on every operation.
- The stale-socket helper checked socket type, owner, device and inode and rechecked before removal, but a metadata-check-then-unlink sequence alone is not a complete defence against path races. Production must anchor operations in a private, trusted runtime directory and coordinate concurrent startup.
- The cross-UID run required elevation because the user's ordinary process lacked permission to switch UID. The successful test demonstrates Linux behavior in the controlled harness, not that the normal development account can impersonate arbitrary UIDs.
- The protocol test used a 1 KiB limit and a simple newline-delimited test command. This is not a final API framing or message-size decision.
- No network listener was created by these local prototypes; a full application network-exposure audit remains future work.

**Decision:** Retain Unix-domain sockets with restrictive per-user runtime-directory permissions and `SO_PEERCRED` as the Linux local transport direction, consistent with ADR-007 and ADR-030. Treat OS peer identity as an input to client authentication, not as application authorization. Do not mark SP-03 fully complete until the integrated implementation, broker enforcement, race-resistant lifecycle, and Windows named-pipe validation pass.

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

### SP-04 execution report — integrated Linux prototype (2026-11-22)

**Result:** The integrated disposable Linux supervisor harness completed all reported checks successfully. SP-04 remains **in progress**; this is prototype evidence, not production acceptance. Production integration, trusted-context enforcement, cooperative cancellation semantics, scoped temporary-file references, OS-enforced sandboxing and Windows validation remain open.

**Environment**
- OS: CachyOS Linux, x86_64; kernel 7.3.5-2-cachyos (environment recorded during the SP-03 work on the same development system).
- Rust/Cargo: 1.100.0.
- Tokio: 1.54.1.
- serde_json: 1.0.154.
- Python: 3.14.10.
- Cached Rust dependencies allowed the prototype to build and run with `cargo run --offline`.
- All source files were created in a disposable directory under `/tmp/assistantv2-sp04-2DMoaZ`; no production repository source was modified.

**Integrated harness and observed results**

The integrated Rust/Tokio harness launched Python workers with stdin/stdout/stderr pipes, used newline-delimited JSON, checked protocol version 1 and bounded individual frames to 64 KiB. It ran the lifecycle/protocol group followed by a concurrency/diagnostics group.

1. Worker startup and protocol-version negotiation passed.
2. Five sequential correlated request/response exchanges on one worker passed.
3. Structured worker error response was surfaced and checked.
4. Mismatched response correlation ID was rejected.
5. Graceful worker shutdown and process reaping passed.
6. Incompatible protocol version was rejected.
7. Malformed JSON output was rejected.
8. Oversized worker output was rejected.
9. Unexpected worker exit (exit code 23) was detected and reaped.
10. A slow worker exceeded the response deadline; the pending read was abandoned and the worker process terminated/reaped.
11. A replacement worker started, negotiated the protocol and handled a request.
12. Five concurrent load jobs obeyed a two-worker semaphore limit.
13. Each load worker emitted 1 MiB to stderr; the supervisor drained the stream without blocking while retaining no more than 4 KiB per worker.
14. All concurrent worker processes were reaped.

**Acceptance status — Linux prototype evidence**
- [x] Versioned protocol handshake and incompatible-version rejection.
- [x] Structured JSON requests/responses and correlation-ID checking.
- [x] Structured worker error response handling.
- [x] Malformed and oversized worker output rejection.
- [x] stdout protocol and stderr diagnostic streams separated.
- [x] Worker crash detection and process reaping.
- [x] Response deadline and forced cancellation/termination path.
- [x] Replacement worker startup after failure.
- [x] Demonstrated bounded concurrent worker count in a semaphore-based harness.
- [x] Continuous stderr draining with bounded retained sample per worker.
- [ ] End-to-end trusted user/workspace context creation, propagation and enforcement across the actual core-to-worker request pipeline.
- [ ] Cooperative cancellation message/acknowledgement semantics and safe handling of work that may have completed before cancellation.
- [ ] Out-of-order and multiplexed concurrent request handling on a single worker.
- [ ] Scoped temporary-file reference creation, expiry, access checks and cleanup for large payloads.
- [ ] Aggregate CPU, memory, process-count and diagnostic-storage budgets integrated with the central resource manager.
- [ ] Production worker restart/backoff policy, health monitoring and shutdown supervision.
- [ ] OS-enforced Linux worker sandbox integration and conformance testing.
- [ ] Windows process supervision and equivalent protocol/lifecycle validation.

**Limitations and interpretation**
- The harness is a disposable demonstration, not a reusable production supervisor library or integrated core service.
- The concurrency cap was demonstrated with a Tokio semaphore around short-lived load workers. It does not yet establish production scheduling, fairness, admission control or machine-wide resource budgeting.
- The stderr test retained at most 4 KiB per worker while draining 1 MiB, but aggregate retention still scales with the number of concurrently active workers. Production must impose an aggregate logging/storage budget and safe redaction.
- Cancellation currently means abandoning a pending read and terminating the worker. It does not demonstrate cooperative cancellation or determine whether a non-idempotent operation completed before termination.
- Correlation was tested with sequential requests and a deliberately mismatched ID. Multiplexed/out-of-order response routing on one worker was not implemented.
- The prototype did not implement a full trusted execution-context envelope. The earlier Stage 1 demonstration treated a worker-supplied identity claim as untrusted response data, but that is not proof of end-to-end identity propagation or authorization enforcement.
- Large-file/scoped temporary references were not implemented.
- Process separation and pipe supervision do not constitute an OS-enforced sandbox.
- No Windows result is available.

**Decision:** Retain Rust/Tokio supervision with separate Python worker processes and a versioned JSON-over-stdio protocol as the approved implementation direction. Keep stdout protocol-only and stderr diagnostics-only. Treat worker output as untrusted, keep identity and authorization authority in the core, enforce bounded frames and deadlines, and require process/resource limits. Do not mark SP-04 fully complete until the outstanding trusted-context, cancellation, multiplexing, temporary-reference, resource-budget, sandbox and Windows criteria are addressed.

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

### SP-05 execution report — Linux prototype (2026-11-22)

**Result:** The Linux prototype demonstrated layered sandbox enforcement and actual cgroup resource enforcement. SP-05 remains **in progress**: these are disposable conformance experiments, not a production sandbox implementation; network-denial/escape coverage, production supervisor integration and Windows validation remain outstanding.

**Environment**
- OS: CachyOS Linux, kernel 7.3.5-2-cachyos, x86_64.
- Bubblewrap: 0.13.1.
- Python: 3.14.10.
- Landlock ABI: 11 (x86-64 syscall interface).
- cgroup v2 mounted at `/sys/fs/cgroup`.
- User systemd manager service had `Delegate=yes`; CPU, cpuset, IO, memory and PID controllers were available at the delegated user-service level.
- Tests ran as the ordinary user in disposable Bubblewrap/systemd scopes; no global cgroup configuration was changed.

**Executed test groups and results**

1. **Bubblewrap runtime and namespace/filesystem isolation**
   - Minimal merged-/usr sandbox layout executed `/usr/bin/true` and Python successfully.
   - User namespace mapped namespace UID/GID 0 to the invoking host UID/GID 1000; the process retained the unprivileged host identity.
   - Effective, permitted, inheritable and bounding capability sets were empty in the worker.
   - `NoNewPrivs: 1` was observed.
   - Network, mount and PID namespace identities differed from the host.
   - Host `/home` and `/var` were not mounted in the tested sandbox; `/tmp` was private and writable; `/usr` was read-only and write attempts returned `EROFS`.

2. **Landlock filesystem enforcement**
   - Kernel reported Landlock ABI 11.
   - Ruleset creation, filesystem rule insertion and self-restriction succeeded.
   - Permitted reads succeeded; unauthorized write and file-creation attempts were denied with `EACCES`.
   - In the integrated worker run, Landlock remained active alongside Bubblewrap, seccomp and process limits.

3. **Seccomp enforcement**
   - A disposable BPF filter installed successfully.
   - The test allowed `getpid` and denied `getppid` with `EPERM`.
   - The integrated worker repeated the blocked-`getppid` check after installing Landlock restrictions.

4. **Per-process resource limits**
   - `RLIMIT_NOFILE` was set to 64.
   - `RLIMIT_FSIZE` was set to 4096 bytes; an oversized write was rejected with `EFBIG`, and the resulting file did not exceed the limit.
   - These controls were also checked in the integrated worker.

5. **Delegated cgroup creation and controller propagation**
   - A transient user systemd scope with `Delegate=yes` successfully created a child cgroup.
   - After moving the test shell into the child, the scope enabled `cpu`, `memory` and `pids` for descendants.
   - The child exposed writable `cpu.max`, `memory.max` and `pids.max` interfaces.
   - The initial inspection showed no child controllers before the parent explicitly enabled propagation; the corrected test confirmed propagation worked. No global hierarchy settings were changed.

6. **Integrated sandbox run**
   - One Python worker ran inside Bubblewrap with isolated namespaces, explicit read-only system mounts and private temporary storage.
   - Landlock denied unauthorized file creation while allowing the required system-file read.
   - Seccomp denied the selected syscall.
   - `NoNewPrivs`, dropped effective capabilities and the two RLIMIT settings were verified in that same worker.
   - All integrated worker assertions passed.

7. **Actual cgroup resource enforcement**
   - A disposable child cgroup was configured with `cpu.max = 50000 100000` (50% of one CPU), `memory.max = 536870912` (512 MiB), `memory.high = 67108864` (64 MiB), and `pids.max = 16`.
   - PID test attempted to create more than the allowed number of child processes; creation was constrained and `pids.events` recorded a `max` event.
   - CPU workload increased the `nr_throttled` counter by 21, demonstrating quota throttling.
   - A workload touching 128 MiB increased the `memory.events` `high` counter by 217, demonstrating memory pressure handling at `memory.high`.
   - All three enforcement checks passed. The `memory.max` hard limit was configured but was not deliberately reached; no OOM-kill test was performed.

**Acceptance status — Linux prototype evidence**
- [x] Bubblewrap launches a worker with isolated user, mount, PID and network namespaces.
- [x] Tested host paths are excluded or mounted read-only as intended.
- [x] Worker retains unprivileged host identity with no effective capabilities.
- [x] No-new-privileges is active.
- [x] Landlock filesystem denial is enforced.
- [x] Seccomp syscall denial is enforced.
- [x] Per-process file-descriptor and file-size limits are enforced.
- [x] Delegated cgroup child creation and CPU/memory/PID controller propagation work without root.
- [x] CPU quota throttling, PID ceiling and memory.high pressure events were observed.
- [x] Loopback host-listener denial from a Bubblewrap worker in an unshared network namespace demonstrated (2026-11-22); this does not prove all outbound network or socket exposure paths.
- [ ] Broader outbound network and host-socket/D-Bus exposure tests.
- [ ] Broader filesystem/process escape and adversarial syscall coverage.
- [ ] Deliberate `memory.max` hard-limit/OOM behaviour and recovery validation.
- [x] Disposable process-group test terminated a worker and its child process (2026-11-22); this does not prove cgroup descendant cleanup.
- [ ] Worker termination, descendant cleanup and supervisor restart integration in the production worker manager.
- [ ] Reviewed production seccomp profiles and per-worker Landlock policies.
- [ ] Fail-closed behavior when a required sandbox primitive is unavailable.
- [ ] Independent Windows AppContainer/restricted-token/job-object conformance validation.

**Limitations and interpretation**
- The experiments used disposable Python and shell harnesses, not production worker code or a reusable sandbox policy engine.
- Namespace identity checks demonstrate namespace separation, but do not alone prove all outbound network paths are blocked. Explicit network denial and socket exposure tests remain open.
- Landlock and seccomp rules were deliberately small proof-of-enforcement policies. They are not approved production allowlists.
- The integrated run verified cgroup limits were configured for its worker. Actual CPU, memory-pressure and PID enforcement were demonstrated in a separate disposable cgroup test.
- The `memory.high` event proves reclaim/throttling pressure was triggered; it is not evidence that the `memory.max` hard ceiling or OOM recovery behavior was tested.
- A follow-up loopback test verified that a host listener reachable from the host was unreachable from a Python worker launched with Bubblewrap `--unshare-net`; broader outbound/socket exposure was not tested.
- A disposable process-group test terminated both a worker and its child; production cgroup cleanup/restart lifecycle and adversarial escape tests remain outstanding.
- No production cleanup/restart lifecycle or adversarial escape test was performed.
- Windows has not yet been tested, consistent with the Linux-first execution order.

**Decision:** Retain the ADR-024 Linux direction: Bubblewrap namespaces, explicit minimal mounts, Landlock, seccomp, dropped capabilities, no-new-privileges, per-process RLIMITs and delegated cgroup v2 resource controls. Run the core under a delegated per-user systemd service and create subordinate worker cgroups; do not modify the global cgroup hierarchy. Untrusted worker execution must fail closed if mandatory isolation cannot be established. Do not mark SP-05 fully complete until the remaining Linux integration/negative tests and later Windows conformance are complete.

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

**Execution report:** [SP-06 Linux Wasmtime prototype](./SP-06-WASMTIME-LINUX.md).

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
