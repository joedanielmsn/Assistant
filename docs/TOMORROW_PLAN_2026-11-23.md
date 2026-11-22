# Local Assistant 2.0 — Plan for 2026-11-23

**Starting point:** `ab49b50ad45f84dbbf434464e16f97386d10bc62` (confirm latest `main` before work)  
**Primary objective:** Get the first real local-model conversation working in the desktop GUI.

## Definition of success for tomorrow

From a normal CachyOS desktop session, launch `./run.sh`, see the Qt chat window, send a message, receive a response from the configured local Qwen model, and see that response appear in the GUI.

The complete request path must be:

```text
Qt/QML GUI
   │ private Unix socket
   ▼
Rust assistantd
   │ central permission broker authorizes agent.chat
   ▼
supervised Python worker
   │ local HTTP request
   ▼
Ollama + Qwen 3.5
   │ model response
   ▼
Python worker → Rust core → GUI
```

## Session sequence

### 1. Resume cleanly

Run:

```bash
cd ~/AssistantV2
git pull --ff-only
git status -sb
git log -1 --oneline
```

Confirm the working tree is clean before starting. Do not reset or overwrite local work if it is not clean; inspect it first.

### 2. Confirm the local prerequisites

- Run `./run.sh help` to see launcher commands.
- Confirm the existing Ollama server is reachable at `http://127.0.0.1:11434`.
- Confirm `qwen3.5:2b-q4_K_M` is present; if not, decide whether to pull it when prompted.
- Confirm the normal desktop session exposes `XDG_RUNTIME_DIR`.
- Let the launcher identify missing CachyOS build dependencies. It asks before package installation.
- Do not change Ollama service enablement or management.

### 3. First integrated launch

Run:

```bash
./run.sh
```

Allow it to build the Rust core and Qt application, start `assistantd`, and open the GUI.

If the build or launch fails:
- Capture the first meaningful error, not just the final summary.
- Inspect the relevant source and exact command output.
- Make the smallest targeted fix.
- Re-run only the failed build/run step.
- Do not launch a broad suite of unrelated checks.

Useful diagnostics:

```bash
tail -n 100 .run/assistantd.log
./run.sh status
```

### 4. Validate the first conversation manually

In the GUI:
1. Send a short greeting or simple factual question.
2. Confirm the busy/thinking state appears.
3. Confirm the assistant response arrives and is displayed.
4. Send a follow-up to confirm recent conversation history reaches the model.
5. Confirm a new conversation clears the in-memory chat history.
6. Close the GUI and use `./run.sh stop` if the daemon remains running.

Record what actually worked and any remaining defects. A successful build alone is not the milestone; the model response must make it back into the GUI.

### 5. Fix only blockers found in the smoke test

Prioritize issues in this order:
1. Build/configuration errors.
2. Core startup/socket lifecycle errors.
3. Permission-broker or request-envelope mismatch.
4. Python worker startup/protocol errors.
5. Ollama endpoint/model/context errors.
6. Qt socket handling, response parsing, and UI state errors.

Keep the current prototype conversational-only. Do not add shell execution, filesystem tools, browser access, plugins, or privileged actions as a shortcut.

### 6. After the first round trip: resume the implementation roadmap

Once the model-backed GUI works, continue with the next smallest product foundations:

1. **Persistent application configuration:** establish a typed configuration source for model name, Ollama endpoint/backend selection, and context length. Keep defaults aligned with the approved decisions.
2. **Settings UI:** add the agreed 4K–16K context slider in 1K steps, defaulting provisionally to 8K. Persist the selection and validate resource availability before model load; never silently lower the selected value.
3. **Conversation persistence:** begin the personal local persistence path using the approved encrypted SQLite/SQLCipher + FTS5 direction. Do not treat prototype test keys as production key management.
4. **Inference abstraction:** separate the model provider contract from the worker/chat orchestration so Ollama can be replaced by the planned llama.cpp backend without rewriting the GUI or permission broker.

Choose the first of these based on what the successful smoke test reveals. Do not start all four at once.

## Explicitly out of scope for tomorrow

- Comprehensive test-suite, Clippy, rustfmt, or broad validation runs unless a concrete defect requires one.
- Production sandbox enablement before its mandatory isolation and lifecycle requirements are integrated.
- Shell, filesystem mutation, browser automation, plugins, or autonomous OS actions.
- Shared multi-user server deployment.
- Windows or other Linux distribution support.
- Installer, bootloader, system update, or recovery integration.
- Licensing decisions; those remain deferred until release planning.

## End-of-session checkpoint

Before stopping:
- Commit focused, working changes to `main`.
- Update `docs/DEVELOPMENT_PROGRESS.md` with what was actually completed.
- Update this plan only if priorities materially changed.
- Record the exact last commit and the next concrete command/action.
- Do not label unverified behavior as complete.

## First command tomorrow

```bash
cd ~/AssistantV2 && git pull --ff-only && git status -sb && ./run.sh
```
