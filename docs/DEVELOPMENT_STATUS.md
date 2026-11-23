# Local Assistant 2.0 — Development Status

Last updated: 2026-11-23

## Current milestone

The project has a working Linux desktop prototype and an expanding local
agent/tool pipeline. The immediate product priority is completing a polished,
usable desktop experience with the agreed liquid-glass visual direction.
Broad feature expansion follows after this desktop checkpoint.

See [DESKTOP_PRODUCT_AND_UI_PLAN.md](DESKTOP_PRODUCT_AND_UI_PLAN.md) for the
agreed desktop design, tonight's sprint, and next-session roadmap.

## Implemented foundations

### Core and inference

- Rust daemon and provider abstraction.
- Ollama inference integration.
- Selected model: `assistant-qwen:latest` (Qwen 3.5 2B Q4_K_M).
- 16K default context, configurable from 4K to 16K.
- Context manager with mandatory-message protection and history budgeting.
- Planner prompt budgeting.
- SQLite conversation and task-context persistence.

### IPC and desktop

- JSON-RPC 2.0 over NDJSON.
- Linux Unix-domain socket transport.
- Local peer-credential authentication.
- Streaming generation notifications.
- Task cancellation infrastructure.
- Qt 6/QML desktop and C++ IPC bridge.
- Conversation management.
- Tool activity events and cancellation controls.
- Desktop workspace selection, persistence, and per-request workspace
  propagation (implemented in the current local working tree).

### Agents and tools

- Agent registry with General, Research, Coding, and System definitions.
- Shared runtime tool registry.
- Tool metadata and JSON argument schemas.
- Strict recursive argument validation.
- Capability checks before execution.
- Structured tool results and execution logging.
- Planner decisions between direct responses and tool execution.
- Multi-step planning using previous tool results.
- Duplicate tool-call protection and a four-call per-turn limit.
- Bounded tool output and cancellation checks between execution stages.
- Per-agent tool allowlists, planner filtering, and orchestrator enforcement.
- Automatic keyword-based specialist routing with General fallback.

### Built-in tools and code intelligence

- Calculator.
- `system_info`: read-only system diagnostics.
- `workspace_list`: bounded workspace directory listing.
- `workspace_read`: bounded UTF-8 text-file reading.
- `project_search`: workspace-constrained project search using the
  approved ripgrep executable adapter (implementation in progress/current
  local work).
- `code_inspect`: bounded syntax inspection using Tree-sitter.
- Workspace path canonicalization, traversal protection, symlink checks,
  sensitive-path restrictions, and bounded reads/listings.

## Latest reported local validation

- Tree-sitter parser focused tests: 4 passed.
- Planner focused tests: 4 passed.
- Latest reported full backend test run: 95 passed, 0 failed.
- `git diff --check` passed.
- User tested the current desktop workflow and reported that it looked good.

These results describe the latest reported local working tree. They do not
mean every local change has been committed to the remote branch.

## Current development environment

- CachyOS / KDE Plasma.
- Rust core.
- Qt 6 desktop.
- Ollama inference.
- AMD CPU, 16 GB DDR5 RAM, NVIDIA RTX 3050 4 GB.
- Primary model: `assistant-qwen:latest`.
- Default context: 16,384 tokens.

## Immediate desktop priorities

1. Refine the main desktop shell using layered liquid-glass surfaces,
   translucent panels, subtle highlights, and restrained accents.
2. Preserve the working chat, streaming, cancellation, conversation,
   settings, workspace, and tool-event flows.
3. Add a useful activity inspector and responsive panel behaviour.
4. Polish empty, loading, disconnected, and error states.
5. Build and launch on CachyOS/KDE, then checkpoint after user review.

## Following milestones

- Complete ripgrep-backed project search and safe code workflows.
- Expand the Agent Coordinator and delegated specialist lifecycle.
- Add worker supervision, timeouts, resource limits, and isolation.
- Integrate MCP through the native Tool Registry.
- Add research providers with explicit network/privacy boundaries.
- Implement user-controlled memory and personalization.
- Build the Screen Awareness Engine with KDE/Linux providers.
- Add the separate compact voice/screen overlay.
- Complete model management, packaging, and Linux installer.
- Begin Windows desktop work only after Linux product completion.

## Known limitations and deferred work

- Full specialist delegation and worker supervision remain future work.
- Tool handlers and some planning calls are not yet fully interruptible.
- IPC reconnect and lifecycle recovery need further hardening.
- User memory and adaptive profile are not implemented.
- Screen Awareness Engine and voice overlay are not implemented.
- Windows transport and platform providers are not implemented.
- Encryption remains intentionally deferred.
