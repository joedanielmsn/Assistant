# Local Assistant 2.0 — First Working Prototype

**Status:** Working local prototype; not production/security acceptance.  
**Last updated:** 23 November 2026

## Current implementation

On the user's CachyOS/KDE Plasma environment, the current local build provides:
- Qt 6/QML desktop chat connected to the Rust `assistantd` core.
- Local Ollama inference through a Rust provider abstraction.
- Streaming chat responses and cancellation support in the request path.
- SQLite persistence for conversations, messages and context-length settings.
- Conversation list, selection, rename and delete operations in the desktop client.
- Core-owned history loading and context assembly.
- Separate validated task context, persisted per conversation.
- Background JSON task analysis that can retain, update or clear task context.

Latest reported verification: 11 Rust unit tests pass; `cargo check` and `cargo build` complete without warnings. The user reports that a live task-context smoke test works.

## Model and context

| Setting | Current value |
|---|---|
| Model | `assistant-qwen:latest` (Qwen 3.5 2B Q4_K_M) |
| Context default | 16,384 tokens |
| Context range | 4,096–16,384 tokens |
| Slider increment | 1,024 tokens |
| Inference backend | Ollama, behind provider abstraction |

The user reports successful 12K and 16K configured-context runs with 100% GPU placement on an RTX 3050 4GB. This verifies placement and successful requests, not full-context stress behaviour.

## Run and development

Use the current local launcher/build instructions in the repository and keep Ollama running separately. Do not assume commands or paths from the older reference implementation apply to this fresh build.

## Explicit limitations

The following are still required by the approved architecture and are not claimed as implemented in the current local build:
- Integrated OS-derived peer identity and trusted client enrollment.
- Central deny-by-default permission broker and structured audit path.
- Full versioned JSON-RPC 2.0 contract, bounded NDJSON framing and reconnect-safe behaviour.
- Strict aggregate context budgeting and race-safe task-context updates.
- Typed tool registry, strict argument validation and broker authorization.
- Supervised worker lifecycle, fail-closed Bubblewrap/Landlock sandboxing and resource limits.
- Screen Awareness Engine, Assistant Mode, visible capture state and exclusions.
- Complete settings/model manager, adaptive profile and user-controlled long-term memory.
- Linux packaging, update and uninstall experience.
- Windows implementation and future OS integration.

The earlier Phase 0 spike results are research evidence only. They do not prove production integration in this codebase. The previous rushed implementation is reference-only.
