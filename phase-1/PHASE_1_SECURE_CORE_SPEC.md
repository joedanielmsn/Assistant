# Phase 1 — Secure Local Core: Implementation Specification

**Status:** Approved implementation baseline for the first working prototype  
**Date:** 2026-11-22  
**Target:** CachyOS Linux, x86-64-v3 minimum  
**Repository:** `joedanielmsn/AssistantV2`  
**Governing records:** `plan/ARCHITECTURE_DECISIONS.md`, `plan/IMPLEMENTATION_ROADMAP.md`, `plan/SECURITY_FOUNDATION.md`, `plan/CONTRACTS_AND_VERSIONING.md`, and `phase-0/`.

## 1. Objective

Build a small, runnable local core that establishes the trust boundaries required by the full assistant. The first prototype is infrastructure, not yet a conversational agent. It must demonstrate that a local client can communicate with a per-user core, that the core derives the caller's OS identity rather than trusting request fields, and that every exposed operation is authorized by the central broker.

The first milestone must be buildable and inspectable on the user's CachyOS development machine. Keep dependencies and runtime behavior small. Add modules only when they have a concrete responsibility.

## 2. Scope

### Included in the first prototype

1. Rust workspace and pinned stable toolchain.
2. One core daemon binary, `assistantd`.
3. Per-user Linux lifecycle; no root/system-wide daemon.
4. Local Unix-domain socket under a private runtime directory.
5. Versioned, bounded newline-delimited JSON request/response protocol.
6. Peer UID obtained from the operating system; never accept caller-supplied UID as identity.
7. Typed principal, operation, resource and authorization decision structures.
8. One central deny-by-default permission broker.
9. A minimal safe operation set: health/status read only.
10. Structured local audit events that avoid private content and secrets.
11. Graceful shutdown, bounded request handling and stale-socket safety.
12. Unit and integration tests for the security-critical paths.
13. CachyOS build/run instructions and a minimal CI workflow.

### Explicitly excluded from the first prototype

- Model loading, inference and model-generated tool execution.
- Python workers, plugins, Wasmtime execution and browser/desktop automation.
- Arbitrary shell, process execution, filesystem mutation or privileged OS operations.
- Root helper, system-wide service, bootloader, installer, partitioning or OS update operations.
- PostgreSQL or production personal-data persistence.
- Qt/QML desktop UI.
- Windows, other Linux distributions and ARM64 support.
- Cloud API, telemetry or remote listener.

These are later milestones. Their absence is intentional, not an unfinished security bypass.

## 3. Trust model

### Trusted components

- The running core process and its compiled policy.
- The OS kernel's peer-credential result for the local Unix socket.
- The core's own configuration and private runtime directory.

### Untrusted inputs

- Every client request and JSON field.
- All model output, worker output, plugin content and retrieved documents when those are introduced.
- Client-supplied user IDs, workspace IDs, role names, permission claims and resource paths.
- Socket contents and any pre-existing filesystem object at the socket path until validated.

### Identity rules

- The daemon runs as the logged-in OS user, without elevated privileges.
- The core obtains peer UID from the accepted Unix-domain connection.
- A request body cannot choose or override principal identity.
- Matching OS UID is only a transport identity signal; it does not make a client a policy authority.
- Every operation is independently authorized by the central broker.
- Initial prototype exposes only non-sensitive read-only health/status operations.

### Authorization rules

- Deny by default.
- Broker input includes the trusted principal, operation identifier, resource scope and applicable policy context.
- No handler performs a privileged or sensitive action before broker approval.
- No model, client, worker or plugin can grant itself permissions.
- Missing or malformed identity/context is denied.
- The first prototype has no privileged operation to authorize; it proves the broker boundary using safe read-only operations.

## 4. Local IPC and protocol

- Use Linux Unix-domain sockets only.
- Socket directory must be private to the current OS user (mode 0700); socket must be owner-only (mode 0600).
- Resolve the runtime directory from a validated `XDG_RUNTIME_DIR`; fail with an actionable error if unavailable or unsuitable. Do not silently fall back to a shared temporary directory.
- Validate socket path ownership/type before cleanup. Never unlink a regular file or an endpoint not owned by the current user.
- Use a bounded newline-delimited JSON protocol with a documented maximum frame size.
- Require protocol version and request identifier.
- Reject malformed JSON, unknown message types, unsupported versions and oversized frames.
- Responses use a stable envelope with request ID, success/result or structured error.
- stdout is not used for daemon protocol; diagnostics go to stderr/journal with sensitive data minimized.
- Do not create a TCP listener or expose remote access.

The initial API should contain only:
- `core.health.read`: returns process health and protocol version.
- `core.status.read`: returns a minimal status summary without private content.

No client-provided path or arbitrary operation string may be executed.

## 5. Initial module boundaries

Start with a single Rust package/workspace if that keeps the first commit simple. Keep module boundaries explicit so extraction into crates can happen when there is a real dependency/ownership reason.

Suggested internal modules:
- `config`: validated runtime paths and bounded server settings.
- `identity`: principal constructed from OS peer credentials.
- `protocol`: request/response types, versioning and frame limits.
- `authorization`: policy inputs, decision type and deny-by-default broker.
- `audit`: structured allow/deny event representation with redaction.
- `ipc`: Unix listener, peer credentials, socket lifecycle and request loop.
- `operations`: safe read-only health/status handlers.
- `main`: argument parsing, startup and graceful shutdown.

Do not create placeholder crates for future features. Extract modules into separate crates when ownership or build boundaries justify it.

## 6. Runtime and failure behavior

- One per-user daemon instance; no privileged service.
- Bound concurrent connections and per-connection request size.
- Bound request processing time and avoid unbounded queues.
- Graceful shutdown stops accepting new connections and closes active sessions safely.
- A malformed request receives a structured error when safe; otherwise close the connection.
- Socket setup failure, missing runtime directory, permission mismatch or peer-credential failure prevents startup/operation.
- Existing non-socket files are never deleted during startup.
- No automatic privilege escalation or unrestricted fallback.
- Logs include event/request IDs and outcome codes, not request payloads, secrets or private document content.

## 7. Verification and acceptance

The first prototype is accepted when all of the following are demonstrated on CachyOS:

1. Pinned toolchain builds the workspace from a clean checkout.
2. `cargo fmt --check`, `cargo clippy` and `cargo test` pass.
3. Daemon starts as an ordinary user and binds only to its private Unix socket.
4. Authorized same-UID client can request health/status.
5. Caller-supplied identity fields are ignored or rejected; they cannot change the principal.
6. Broker allows only the explicit read-only operations and denies unknown operations.
7. Malformed, unsupported-version and oversized frames are rejected.
8. A regular file at the socket path is preserved; stale owned sockets are handled safely.
9. Socket permissions and peer credentials are verified.
10. Daemon shutdown and restart work without leaving a live listener or deleting unrelated files.
11. No TCP listener, arbitrary shell, privileged helper or worker execution is present.
12. Documentation accurately describes what was and was not tested.

Security-critical authorization and socket lifecycle behavior require focused negative tests. Avoid broad benchmark campaigns at this stage.

## 8. Milestones

### M1 — Runnable core skeleton
Workspace, toolchain, daemon command, configuration validation, health check, formatting/lint/test commands.

### M2 — Secure local IPC
Private runtime directory, Unix socket, peer UID, bounded protocol, safe lifecycle and integration tests.

### M3 — Permission broker and audit path
Typed policy request/decision, deny-by-default behavior, health/status operations routed through broker, structured audit events and negative tests.

### M4 — Prototype review
Run the documented checks on CachyOS, inspect dependencies and threat boundaries, update traceability and roadmap. Only then plan worker integration.

## 9. Worker integration gate

No untrusted worker is launched until:
- Trusted execution context is created by the core and cannot be supplied by the worker.
- Broker authorization is enforced for every operation.
- The complete mandatory Linux sandbox is established atomically.
- Worker cgroup ownership, descendant cleanup and resource budgets are implemented and verified.
- Timeout/cancellation semantics prevent blind repetition of uncertain non-idempotent operations.
- The worker protocol has bounded framing and explicit compatibility rules.

## 10. Initial repository implementation

The initial implementation bootstrap is in `core/assistantd/` with a root Cargo workspace. It currently provides a Linux-only daemon/client prototype, bounded JSON-line IPC, OS-derived peer UID, a minimal deny-by-default operation allowlist, and health/status handlers. It is an early implementation baseline, not yet validated by a local CachyOS build. The repository does not yet contain a generated `Cargo.lock`; generate it and commit it after the first successful local dependency resolution and build.

## 11. Change control

This specification implements existing approved architecture directions. Any change that weakens privacy, identity provenance, deny-by-default authorization, broker mediation or sandbox fail-closed behavior requires explicit owner approval and a new decision record. Routine module naming and implementation details may be selected by the implementer if they preserve these requirements.
