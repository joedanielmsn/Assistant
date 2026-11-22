# Phase 0 — Consolidated Technical Spike Results

**Date:** 2026-11-22  
**Environment focus:** CachyOS Linux, x86_64  
**Interpretation:** Prototype evidence only; not production acceptance.

This report consolidates the available Phase 0 evidence. Where a test was deferred, incomplete or based on a user-reported result without retained logs, that limitation is stated explicitly.

## Summary

| Spike | Area | Evidence status | Current disposition |
|---|---|---|---|
| SP-01 | rusqlite + SQLCipher + FTS5 | Linux prototype passed its recorded regression checks | Retain direction provisionally; Windows and dependency-fixture review deferred |
| SP-02 | SQLx + PostgreSQL authorization | Linux prototype substantially passed | Retain PostgreSQL/SQLx candidate; carry trusted-identity and DB-role review into implementation |
| SP-03 | Local IPC | Linux UDS prototype checks passed | Retain UDS direction; production enrollment, broker integration and Windows named pipes remain |
| SP-04 | Worker protocol/supervision | Integrated Linux disposable harness passed reported checks | Retain Rust/Tokio + Python worker direction; production lifecycle and identity integration remain |
| SP-05 | Native worker sandbox | Linux layered-enforcement evidence plus additional network/process-group checks | Retain Bubblewrap/Landlock/seccomp/cgroup direction; production integration and fail-closed work remain |
| SP-06 | Wasmtime | Minimal Linux Core-Wasm prototype passed (user-reported) | Retain embedded Wasmtime candidate; Component Model/WIT and broker integration remain |
| SP-07 | Model inference/GPU | Linux NVIDIA, CPU fallback, tool calling and context checks passed | Qwen 3.5 candidates retained provisionally; no production reliability claim |
| SP-08 | Qt 6/QML client | No execution report available | Deferred; choose/validate UI implementation during its delivery phase |
| SP-09 | GRUB/Btrfs/Secure Boot recovery | Preliminary audit only; no demonstrated end-to-end rollback | Deferred; mandatory before OS-level boot/update integration |
| SP-10 | Calamares/LUKS2 installer | No execution report available | Deferred to OS distribution phase |
| SP-11 | CI/signing/SBOM/provenance | No execution report available | CI baseline to be established during implementation; release-signing work later |

---

## SP-01 — Encrypted personal database

**Environment:** CachyOS x86_64; Rust/Cargo 1.100.0; rusqlite 0.43.0; libsqlite3-sys 0.40.0; SQLCipher 4.20.0; bundled SQLite source 3.54.1.

**Demonstrated:**
- Bundled SQLCipher runtime detected through `PRAGMA cipher_version`.
- Encrypted database creation, close and correct-key reopen.
- Wrong-key rejection.
- FTS5 virtual table and search in the encrypted build.
- Encrypted online backup and restore; wrong backup key rejected.
- Targeted plaintext-marker absence checks in database/WAL.
- WAL PASSIVE/TRUNCATE checkpoint behavior; 22 frames checkpointed and data remained readable.
- Modified ciphertext rejected with an HMAC verification failure.
- Disposable local `libsqlite3-sys` build-script patch disabled extension loading; compile options and archive symbols were inspected.
- Clean locked offline Linux rebuild passed.

**Limitations:** The local crate patch and disposable fixture are not yet reviewed as retained project dependencies. No Windows build, general crash/corruption recovery assurance, or production key-management design was validated. Targeted plaintext-marker checks are not proof that all sensitive data is absent from every temporary file or process surface.

**Disposition:** Retain rusqlite + bundled SQLCipher + FTS5 provisionally. Production work must fail closed if encryption is unavailable, keep extension loading disabled, and define a reviewed key hierarchy. Windows validation is deferred under the current CachyOS-first product scope.

## SP-02 — Shared-server PostgreSQL persistence

**Environment:** CachyOS; Rust/Cargo 1.100.0; SQLx 0.8.6; Tokio 1.54.1; PostgreSQL 17.12 in a disposable rootless Podman container.

**Demonstrated:**
- Fresh database migrations and sequential schema upgrade.
- PostgreSQL RLS isolation between simulated tenants.
- Missing tenant context returned no rows.
- Transaction-local tenant context did not leak when a pooled connection was reused.
- Cross-tenant insert rejected.
- 200 concurrent tenant-isolation tasks passed using an eight-connection pool.
- Restricted application role was confirmed not to be superuser and not to have BYPASSRLS.
- Application authorization derived tenant scope from a membership lookup using a trusted authenticated-principal input; cross-workspace, missing-membership and mismatched-worker-claim cases were rejected.
- Backup and restore preserved data, migrations, RLS policy and RLS table flags.

**Important security finding:** A PostgreSQL custom session variable used as RLS context can be set by the application role. It is not proof of caller identity. The trusted core must authenticate the principal and derive workspace/tenant scope; worker-supplied identity claims must never authorize access. RLS is defense in depth, not the authentication boundary. The prototype membership table itself did not have RLS enabled.

**Disposition:** Retain PostgreSQL 17 + SQLx as the shared-server persistence candidate. Carry database-role design, membership-data protection and trusted identity propagation into implementation review. Windows portability is deferred.

## SP-03 — Local client IPC

**Environment:** CachyOS; Rust/Cargo 1.100.0; Tokio 1.54.1; util-linux setpriv 2.42.4.

**Demonstrated:** Restrictive per-user Unix-domain socket access, Linux `SO_PEERCRED` retrieval and different-UID rejection, active-socket replacement/removal protection, stale-socket cleanup/rebind, non-socket path preservation, short-lived session mechanics, scope checks, revocation/expiry, malformed/oversized request handling, abrupt disconnect handling, 16 concurrent clients and restart/reconnection.

**Limitations:** These were separate disposable demonstrations, not one integrated production server. Same-UID processes could obtain a prototype session; there was no enrolled desktop-client identity. The scope list was not the central permission broker. The stale-socket metadata check is not a complete path-race defense. Windows named-pipe validation was not run.

**Disposition:** Retain Unix-domain sockets in a private per-user runtime directory, with peer credentials as one authentication input. Every operation still requires core authorization. Do not trust a process or plugin solely because it shares the OS UID.

## SP-04 — Supervised worker protocol

**Environment:** CachyOS x86_64; Rust/Cargo 1.100.0; Tokio 1.54.1; serde_json 1.0.154; Python 3.14.10.

**Demonstrated in an integrated disposable Rust/Tokio harness:** Version negotiation, correlated JSON request/response, structured errors, mismatched-correlation rejection, malformed and oversized output rejection, worker crash detection/reaping, deadline-triggered termination, replacement worker startup, bounded concurrency via a two-worker semaphore, continuous stderr draining with bounded retained samples, and process reaping.

**Limitations:** No end-to-end trusted user/workspace context envelope, cooperative cancellation protocol, multiplexed out-of-order requests, scoped temporary-file references, aggregate resource budgets, production restart/backoff policy, OS sandbox integration or Windows validation. Terminating a worker after a timeout does not prove a non-idempotent operation did not already complete.

**Disposition:** Retain Rust/Tokio supervision with separate Python worker processes and versioned JSON-over-stdio. stdout is protocol-only; stderr is diagnostics-only. Identity and authorization remain core-owned.

## SP-05 — Native worker sandbox

**Environment:** CachyOS; kernel 7.3.5-2-cachyos; Bubblewrap 0.13.1; Python 3.14.10; Landlock ABI 11; cgroup v2.

**Existing Linux prototype evidence:**
- Bubblewrap user, mount, PID and network namespace setup.
- Unprivileged host identity, empty effective capabilities and `NoNewPrivs: 1`.
- Read-only system mounts, excluded host paths and private writable temporary storage.
- Landlock filesystem denials and seccomp syscall denial.
- RLIMIT_NOFILE and RLIMIT_FSIZE enforcement.
- Delegated cgroup creation/controller propagation.
- CPU quota throttling, PID ceiling and `memory.high` pressure events.

**Additional checks completed on 2026-11-22:**
- A host loopback listener was reachable from the host.
- A Python process launched by Bubblewrap with `--unshare-net` and a read-only root filesystem could not connect to that host loopback listener; the connection attempt returned `ConnectionRefusedError`. This is a valid loopback-isolation result, not proof of every possible network or socket boundary.
- A disposable worker and its child process were placed in one process group. Sending TERM to the group resulted in both processes being reported terminated. This demonstrates process-group behavior, not cgroup descendant cleanup or production supervisor integration.

**Not demonstrated:** Broader filesystem/process escape resistance, host D-Bus/socket exposure audit, deliberate `memory.max` OOM behavior/recovery, production cgroup lifecycle cleanup, supervisor restart integration, production seccomp/Landlock policy review, fail-closed behavior when primitives are unavailable, or Windows AppContainer/restricted-token validation.

**Disposition:** Retain Bubblewrap + explicit mounts + Landlock + seccomp + dropped capabilities + no-new-privileges + per-process limits + delegated cgroup v2. Mandatory isolation must fail closed. Do not run untrusted workers unrestricted if sandbox setup fails.

## SP-06 — Wasmtime capability sandbox

**Environment:** CachyOS x86_64; Rust/Cargo 1.100.0; Wasmtime 51.0.0.

**User-reported prototype checks:** Explicit registered host function call succeeded; memory growth beyond a 128 KiB store limit was rejected; undeclared host import was rejected; unprovided WASI import was rejected; infinite-loop module trapped on fuel exhaustion.

**Limitations:** Exact logs were not retained. This was Core WebAssembly, not Component Model/WIT validation. No production permission-broker integration, identity propagation, revocation, lifecycle/cancellation, adversarial module validation or Windows test.

**Disposition:** Retain embedded Wasmtime as a candidate. Keep host capabilities explicit and minimal, no ambient WASI by default, and route privileged host calls through the central permission broker.

## SP-07 — Local inference and model behavior

**Environment:** CachyOS; AMD Ryzen 7 170 (8 cores/16 threads); NVIDIA RTX 3050 Mobile 4 GB; 16 GB DDR5; Ollama 0.40.17; NVIDIA driver 620.73.04.

**Demonstrated:**
- Qwen 3.5 2B Q4_K_M ran fully on the NVIDIA GPU at 4K, 8K and 16K configured context.
- Structured tool calls were returned through the Ollama chat API, and a manually completed mock tool round trip returned the correct result.
- `think:false` suppressed the model's default thinking output.
- CPU-only inference worked for the 2B Q4_K_M and 0.8B Q8_0 candidates with GPU visibility disabled on a separate temporary Ollama server.
- Both 2B and 0.8B candidates passed the same five basic deterministic mock tool-selection/argument/clarification/round-trip checks.

**Provisional model direction:** Qwen 3.5 2B Q4_K_M is the primary candidate; Qwen 3.5 0.8B Q8_0 is the lightweight fallback candidate. These are not final production selections.

**Context setting requirement approved:** Settings → Model → Context length slider from 4K to 16K in 1K increments; provisional default 8K; persist across restarts; validate memory before loading; reject unsafe settings with a clear explanation and safe alternatives rather than silently changing the selection.

**Limitations:** The 8K/16K tests verified configured context and successful short-prompt inference, not a fully populated context stress test. Five mock tool checks do not establish production tool reliability. No rigorous latency benchmark or broad backend matrix was completed. Further SP-07 testing is intentionally deferred unless implementation exposes a concrete issue.

## SP-08 — Qt 6/QML packaging and client connectivity

No execution report is available. Qt packaging, API connectivity, accessibility and platform packaging remain unvalidated. This is not a blocker for starting the Rust core foundation; the UI remains a later client milestone.

## SP-09 — Boot, snapshots and recovery

A preliminary audit exists, but it does not demonstrate an actual failed-update rollback or complete boot recovery chain. No claim of Secure Boot end-to-end support is approved. This work remains a hard gate before OS-level bootloader, update or recovery integration.

## SP-10 — Installer, partitioning and LUKS2

No execution report is available. Defer to the later OS/distribution phase. No destructive installer or disk-partitioning work is authorized by this report.

## SP-11 — CI, signing, SBOM and provenance

No execution report is available. Establish a minimal safe CI/build/test baseline as part of the core repository bootstrap. Production signing, artifact provenance and release SBOM gates remain later release-engineering requirements.

---

## Overall conclusion

The Linux prototypes have provided enough evidence to begin the secure core foundation without continuing broad Phase 0 experimentation. The results support provisional architecture choices, not claims of production readiness or cross-platform support. Remaining limitations are tracked in [Decisions and risks](./DECISIONS_AND_RISKS.md) and must become implementation gates where they protect trust boundaries.