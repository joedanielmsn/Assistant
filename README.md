# Local Assistant 2.0

A local-first assistant platform built around a trusted Rust core, supervised Python AI workers, and a Qt 6/QML desktop client.

## First working prototype

The desktop chat UI connects to the per-user Rust core over its private Unix socket. The core derives the peer UID, authorizes `agent.chat` through its central permission broker, and supervises a short-lived Python worker for each request. The worker calls the local Ollama HTTP API and returns a text response.

The first agent is conversational only. It does not execute shell commands, modify files, browse the web, or invoke plugins. Model output is untrusted text, not permission to perform actions.

## Run on CachyOS

Requirements: x86-64-v3-capable CachyOS/Linux, Rust toolchain, CMake, Ninja, Qt 6 Quick/QML/Quick Controls, Python 3, curl, and a running local Ollama server.

```bash
./run.sh
```

Useful commands: `./run.sh build`, `./run.sh status`, `./run.sh stop`, and `./run.sh clean`.

Default model: `qwen3.5:2b-q4_K_M`. Override with `ASSISTANT_MODEL`. Default Ollama endpoint: `http://127.0.0.1:11434`; override with `OLLAMA_HOST`.

The launcher does not take over Ollama service management. Start your existing Ollama server separately if it is not already running.

## Architecture

- `core/assistantd/`: Rust/Tokio authoritative core, local IPC and permission broker.
- `workers/agent/`: Python conversational worker and Ollama adapter.
- `desktop/`: Qt 6 + QML desktop client.
- `run.sh`: one-command CachyOS development launcher.
- `docs/first-working-prototype.md`: setup, scope and limitations.
- `plan/` and `phase-0/`: authoritative architecture and engineering records.

The approved long-term inference direction remains a replaceable backend architecture with llama.cpp as the initial target. Ollama is used as a development adapter against the already-available local runtime.

## Project constraints

- Personal per-OS-user deployment first; shared multi-user server architecture remains a future profile.
- Private-by-default data and explicit user/workspace authorization context.
- All privileged operations pass through the central permission broker.
- Models and workers are untrusted requesters, never authorities.
- No telemetry, cloud inference or remote listener is enabled by this prototype.
- The old `~/local-assistant` project is reference-only; this repository is a fresh implementation.
- Licensing remains deferred until release planning.
