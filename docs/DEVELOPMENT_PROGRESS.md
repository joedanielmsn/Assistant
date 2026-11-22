# Local Assistant 2.0 — Development Progress Log

**Last updated:** 2026-11-22  
**Repository:** `joedanielmsn/AssistantV2`  
**Branch:** `main`  
**Current HEAD at time of writing:** `ab49b50ad45f84dbbf434464e16f97386d10bc62`  
**Next session:** 2026-11-23

This file is the hand-off record for the next development session. It summarizes the decisions and implementation progress that matter for continuing the project without repeating discovery work.

## 1. Current state at a glance

The repository now contains the first integrated, model-backed desktop prototype implementation:

- Rust/Tokio `assistantd` core and local Unix-socket IPC.
- Central permission broker with a narrow `agent.chat` operation.
- Python one-request conversational worker using the existing local Ollama HTTP API.
- Qt 6/QML desktop chat application.
- Root-level `run.sh` launcher for dependency checks, build, core startup, and GUI launch.
- Setup/scope documentation.

**Important status distinction:** the implementation has been committed, but the new full GUI → core → worker → model → GUI path has not yet been smoke-tested on the development laptop. Do not describe the GUI prototype as runtime-verified until that round trip succeeds.

## 2. Repository and development workflow

- Repository: `joedanielmsn/AssistantV2`.
- Main implementation branch: `main`.
- Work directly on `main` using focused commits; do not force-push or rewrite shared history.
- Planning documents live under `plan/`; Phase 0 records live under `phase-0/`; implementation lives at repository root.
- The old `~/local-assistant` project is reference-only. This is a fresh independent implementation.
- Development target: CachyOS Linux, x86-64-v3.
- Current shell is Fish. Bash scripts can be run explicitly using Bash; project launcher is `./run.sh`.

## 3. Approved architecture and product decisions

### Core authority and security

- Rust core is the trusted authority for identity, authorization, orchestration, and privileged-operation decisions.
- One central permission broker is the only authorization path for privileged/sensitive operations.
- Hybrid RBAC + ABAC is the approved policy direction.
- Models, Python workers, and future plugins are untrusted requesters. They cannot create identity, grant permissions, or authorize themselves.
- Authorization is deny-by-default. Missing trusted identity or policy context means deny.
- Personal per-OS-user desktop installation is the first deployment profile. Preserve a path to future shared multi-user server deployments.
- Data is private by default and operations must carry trusted user/workspace/task scope.
- Same-UID peer credentials are transport identity, not proof that every process belonging to that user is a trusted GUI client. Client enrollment remains an open security milestone.
- No shell execution, unrestricted filesystem access, browser automation, plugins, or privileged tools are enabled in the current chat prototype.

### Platform and hardware

- Initial target is CachyOS only; broader Linux support is later. Windows validation is deferred.
- Minimum CPU baseline: x86-64-v3. x86-64-v4 may be an optional optimization. Original x86-64-only and ARM64 are outside the initial baseline.
- Development laptop: ASUS TUF A15, AMD Ryzen 7 170 (8 cores / 16 threads), NVIDIA RTX 3050 Mobile 4 GB, 16 GB DDR5.
- Ollama is currently used as a development inference adapter. The longer-term initial inference target remains llama.cpp behind a replaceable backend interface.
- Do not change the user's Ollama systemd/service management without explicit approval.

### Model and context

- Provisional primary: `qwen3.5:2b-q4_K_M`.
- Provisional fallback: `qwen3.5:0.8b-q8_0`.
- Context setting: 4K–16K in 1K steps; 8K provisional default.
- Runtime must validate resource availability and fail safely rather than silently changing the selected context.
- Current worker reads `ASSISTANT_CONTEXT_LENGTH`, defaults to 8192, and validates the range/step. Persistent Settings UI and actual resource-aware memory validation are not implemented yet.

### Persistence and later capabilities

- Personal persistence direction: encrypted SQLite/SQLCipher + FTS5.
- Shared-server persistence direction: PostgreSQL + SQLx.
- Existing prototype research is not production approval for key management, database roles, or tenant authentication.
- Worker sandbox direction: Bubblewrap + Landlock + seccomp + no-new-privileges + dropped capabilities + RLIMITs + delegated cgroup v2; fail closed when mandatory controls are missing.
- Wasmtime is the approved direction for future capability-limited plugins, with no ambient filesystem/network authority.
- Licensing is deliberately deferred until release planning; revisit then.

## 4. Work completed

### Phase 0 technical discovery

The repository contains the Phase 0 research and transition records, including spike results, decisions/risks, traceability, and exit review. Discovery produced Linux prototype evidence across local persistence, IPC, worker supervision, sandbox primitives, PostgreSQL isolation, Wasmtime capabilities, and local inference.

Phase 0 was closed as a **controlled transition**, not as a claim that every spike or production acceptance criterion passed. Outstanding items remain explicit in `phase-0/EXIT_REVIEW.md` and `phase-0/DECISIONS_AND_RISKS.md`.

### Secure Rust core foundation

- Created a clean Rust workspace with `core/assistantd`.
- Implemented a Tokio daemon using a private per-user XDG runtime directory and Unix-domain socket.
- Socket/runtime permissions are owner-only; peer credentials are checked.
- Implemented bounded newline-delimited JSON IPC, request validation, protocol version checks, connection deadlines and concurrency bounds.
- Added a central permission broker with only safe core health/status reads in the initial daemon milestone.
- Added structured authorization audit messages and careful socket cleanup.
- Implemented daemon health/status client commands and graceful shutdown behavior.
- Added six Rust unit tests covering broker policy, request validation, and frame parsing.
- Added a process-level IPC integration test covering runtime/socket permissions, health/status requests, denied operation, unsupported version, malformed JSON, duplicate daemon protection, oversized frame handling, and graceful socket cleanup.
- The six unit tests and one IPC integration test passed. Clippy with warnings denied, formatting check, and diff whitespace check were also reported passing after the integration test fixes.

### First model-backed desktop implementation

Committed in the recent prototype work:

- `workers/agent/worker.py`: validates a bounded chat history, adds a fixed system prompt, calls local Ollama `/api/chat`, disables streaming/thinking for this initial path, and returns one JSON response. Worker stdout is reserved for protocol output; errors go to stderr.
- Rust core: added the `agent.chat` operation, permission-broker authorization, Python worker supervision, deadline/response validation, and a single in-flight model-request semaphore.
- `desktop/`: Qt 6/QML desktop chat interface and C++ `ChatController`, communicating with the core via the private local socket.
- GUI response handling buffers data until a newline so fragmented Unix-socket reads do not get mistaken for complete JSON.
- Conversation history currently lives only in GUI memory and is bounded before sending to the worker.
- `run.sh`: master CachyOS development launcher with dependency checks, explicit install prompt, Ollama endpoint/model checks, optional model download prompt, build, daemon start, status/stop/clean commands, and GUI launch.
- `docs/first-working-prototype.md` documents the current prototype path, configuration, and limitations.

The launcher does not silently take over Ollama service management. The default model is `qwen3.5:2b-q4_K_M`; default Ollama endpoint is `http://127.0.0.1:11434`; default context is 8192.

## 5. Current Git state

Latest known commit:

```
ab49b50 fix: handle fragmented desktop IPC responses
```

Recent prototype-related commits include:
- `58fc142` — initial model-backed desktop prototype files.
- `427ca89` — launcher variable expansion fix.
- `7db5f1e` — bound concurrent model requests and GUI message sizes.
- `786e1a0` — safer launcher runtime-directory validation.
- `ab49b50` — handle fragmented desktop IPC responses.

The authoritative branch head should always be confirmed with `git pull --ff-only` and `git status -sb` at the start of a new session.

## 6. Known gaps and unverified items

1. Full desktop GUI build and launch have not yet been confirmed on the laptop after the latest commits.
2. The first real model round trip through the complete stack is not yet confirmed.
3. Qt packaging, accessibility and broader desktop integration are not complete.
4. Chat history is not persisted; closing the GUI loses the conversation.
5. Settings UI and persistent configuration are not implemented.
6. Context resource validation is not implemented; the worker only validates the selected numeric range.
7. Ollama is temporary; llama.cpp backend integration remains future work.
8. The worker is a conversational-only prototype, not yet running under the full approved production sandbox.
9. Same-UID GUI client enrollment is not implemented.
10. No plugin, browser, filesystem, shell, automation, voice, or OS-control capabilities are enabled.
11. Shared multi-user server deployment is not implemented.
12. Production key management, database authorization, cgroup descendant cleanup, boot recovery, installer, CI/release signing, Windows, and other-distribution support remain future gates.

## 7. Testing policy for the next implementation sessions

The user explicitly asked to prioritize getting the first working model and GUI running rather than spending early development time repeatedly running broad test suites.

Until the dedicated debugging/stabilization stage:
- Run only essential build/run smoke checks needed to make forward progress.
- When a concrete failure appears, inspect and fix that failure.
- Do not routinely run the full test suite, Clippy, formatting checks, or broad validation after every small change.
- Preserve existing tests and defer comprehensive regression/security validation to the dedicated stabilization stage.
- Never claim a component works merely because it compiles or has been committed.

This is a prioritization decision, not a waiver of security requirements or a claim that deferred tests passed.
