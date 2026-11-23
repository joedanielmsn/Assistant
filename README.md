# Local Assistant 2.0

A local-first desktop assistant built around a Rust core, provider-neutral inference integration, a Qt 6/QML client, persistent conversations and a future governed agent/tool architecture.

## Current local prototype (23 November 2026)

The CachyOS implementation has a working Qt desktop chat connected to the Rust `assistantd` core and local Ollama. Streaming chat works. The core owns conversation history and context assembly, with SQLite persistence for conversations, messages and context settings. The desktop supports listing, selecting, renaming and deleting conversations. A background task analyzer uses structured JSON generation to maintain a separate task context; the user has reported a successful live smoke test.

Latest reported verification: 11 Rust unit tests pass; `cargo check` and `cargo build` complete without warnings.

This is an early working prototype, not a security-complete or production release. The central permission broker, fully integrated trusted peer identity, production worker supervision/sandboxing, governed tool execution, Screen Awareness Engine, Assistant Mode and Linux packaging remain outstanding. Do not infer that earlier disposable technical spikes are already integrated into this fresh implementation.

## Architecture direction

- `core/assistantd/`: authoritative Rust core for configuration, conversation state, context assembly, inference coordination, persistence and orchestration.
- `desktop/`: Qt 6 + QML desktop client communicating with the core over local IPC.
- Ollama is the current first inference backend behind a provider abstraction. The internal interface remains provider-neutral; llama.cpp can be evaluated later, not as a prerequisite for the current working prototype.
- The planned local client contract is QLocalSocket + JSON-RPC 2.0 over bounded NDJSON framing, with request IDs, streaming, cancellation and structured errors. Verify and complete this contract before treating it as accepted.
- The planned security architecture includes trusted OS-derived identity, a central deny-by-default permission broker, audit events and supervised, sandboxed workers. These are not yet claimed as complete.
- Screen awareness is a dedicated Assistant Mode capability, default-on while Assistant Mode is active, with visible status, pause/disable and exclusions.

## Model and context configuration

Current selected model: `assistant-qwen:latest` (Qwen 3.5 2B Q4_K_M).

Current context default: 16,384 tokens. Planned configurable range: 4,096–16,384 tokens in 1,024-token increments, persisted and resource-validated. Local tests reported successful 12K and 16K configurations with 100% GPU placement on an NVIDIA RTX 3050 4GB; this is not a full-context stress-test or production performance claim.

## Product sequence

1. Complete the Linux desktop product on CachyOS.
2. Rebuild platform-specific integration for Windows after Linux is complete.
3. Deliver OS-appropriate Linux and Windows installers.
4. Only then begin the separate AI-integrated CachyOS-derived product, initially using upstream repositories.

The desktop assistant remains independently installable. No telemetry or automatic diagnostic uploads. Encryption and final licensing decisions remain deferred to the planned release stage.
