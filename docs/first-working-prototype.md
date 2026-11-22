# First working model-backed desktop prototype

**Target:** CachyOS Linux, x86-64-v3  
**Baseline:** `58fc14222cd482d31ef3665e8f9e46efd3f2bc09`

## Integrated request path

The desktop client connects to `$XDG_RUNTIME_DIR/assistant-v2/core.sock`. Each chat request carries a bounded conversation history and the `agent.chat` operation.

The Rust core derives the peer UID, validates the request envelope and message history, authorizes the operation through the central permission broker, starts the Python worker, enforces a 180-second deadline, reaps the child, validates the worker response and returns a structured response to the UI.

The Python worker adds a fixed system instruction and calls the local Ollama `/api/chat` endpoint. Model output is treated as untrusted conversational text.

## Start

Run from the repository root in a CachyOS desktop session:

```bash
./run.sh
```

The launcher checks dependencies and offers to install missing CachyOS packages (`base-devel`, `cmake`, `ninja`, `qt6-base`, `qt6-declarative`, `python`, `curl`). It does not silently install packages.

It expects the existing Ollama server to be running. It does not enable, disable or replace an Ollama systemd service. If the model is not cached, it asks before downloading.

Other commands: `./run.sh build`, `./run.sh status`, `./run.sh stop`, `./run.sh clean`.

## Configuration

| Variable | Default |
|---|---|
| `ASSISTANT_MODEL` | `qwen3.5:2b-q4_K_M` |
| `OLLAMA_HOST` | `http://127.0.0.1:11434` |
| `ASSISTANT_CONTEXT_LENGTH` | `8192` |

Context is constrained to 4096–16384 tokens in 1024-token steps. The 8192 default is provisional. Persistent Settings UI and resource-aware memory validation remain to be implemented.

Daemon diagnostics are written to `.run/assistantd.log`. Generated files under `build/` and `.run/` are ignored by Git.

## Explicit prototype limits

- Conversational only: no shell, filesystem mutation, browser, plugin, automation or privileged action.
- Model output cannot authorize operations.
- The Python worker is started per request; a persistent worker pool is a later optimization.
- Conversation history is in GUI memory only; durable storage and long-term memory are not implemented.
- Ollama is a development adapter. The approved long-term initial inference direction is llama.cpp behind a replaceable backend interface.
- This is a personal per-user CachyOS prototype, not a shared server.
- Same-UID client enrollment is not implemented; peer credentials are transport identity, not proof that every same-user process is a trusted desktop client.
- Qt packaging, accessibility, resource-pressure validation and Windows build are not claimed as complete.
- Do not commit credentials, private conversations, local databases, model weights or user data.

## Next milestones

1. Smoke-test the complete GUI → Rust broker → Python worker → Ollama round trip.
2. Fix concrete build/runtime issues.
3. Add persistent conversation storage and Settings.
4. Introduce the llama.cpp backend.
5. Add specialist tools only after authorization and sandbox paths are implemented.
