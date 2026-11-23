# Local Assistant 2.0

A locally hosted, privacy-focused AI desktop assistant.

## Development Status

Initial development — Phase 1: First working model-backed prototype.

## Architecture

- Core: Rust
- Desktop: Qt 6 and QML
- Initial inference backend: Ollama
- Local IPC: QLocalSocket with JSON-RPC 2.0 over NDJSON

## Project Principles

- Independent implementation.
- Local-first architecture.
- Provider-neutral inference interface.
- Rust core owns application state and orchestration.
- Streaming and cancellation from the beginning.
