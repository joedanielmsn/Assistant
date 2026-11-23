# Local Assistant 2.0 — Desktop Product and UI Plan

Last updated: 2026-11-23

This document records the agreed near-term product direction and desktop
implementation plan. It supplements the Master Plan, Action Plan, Project
Plan, Architecture, Development Status, and Integration Catalog.

## 1. Immediate objective

Deliver a polished, usable Linux desktop application before broadening the
feature set. The current priority is the end-to-end desktop experience:
navigation, workspace selection, conversations, streaming chat, tool
activity, settings, and reliable startup/build behaviour.

Do not delay a working product to build every planned subsystem. Complete
the essential user-facing workflows first, then expand agent capabilities,
screen awareness, voice, memory, and integrations in deliberate milestones.

## 2. Agreed desktop visual direction

The desktop should take strong visual inspiration from the **liquid-glass
appearance of Hermes Desktop for Windows**, while retaining Local Assistant's
own identity, terminology, interaction model, and implementation.

Visual principles:

- Layered translucent surfaces with visible depth.
- Soft background blur where the platform compositor supports it.
- Glass-like panels with subtle luminous edges and restrained gradients.
- Floating or lightly separated navigation and inspector surfaces.
- A polished translucent composer and carefully differentiated message
  surfaces.
- Soft shadows, fine highlights, gentle separators, and smooth transitions.
- Restrained violet/blue accents; avoid excessive neon, glow, or decorative
  effects.
- Strong text contrast and legibility remain more important than visual
  effects.
- Avoid a stack of nested opaque cards; use spacing and hierarchy to keep
  the interface calm and readable.

### KDE/Linux implementation

The current target is CachyOS with KDE Plasma/KWin. Use compositor blur when
available, but never make it a hard dependency. The UI must remain visually
coherent using translucent QML surfaces, gradients, highlights, and shadows
when blur is unavailable or disabled.

The desktop should adapt to window size and avoid forcing side panels onto
narrow layouts.

### Information architecture

Keep the conversation as the primary working surface:

- **Navigation:** Chat, Projects/workspaces, Agents, Tools, and Settings,
  with recent conversations and the selected workspace.
- **Main conversation:** message transcript, streaming response, Markdown
  and code presentation, and a prominent composer with send/stop behaviour.
- **Context inspector:** task/tool activity first; files and previews should
  become functional when their underlying data workflows are available.
- **Task continuity:** opening settings, inspecting activity, or switching
  workspace should not unexpectedly discard the active conversation or
  steal transcript focus.

Do not present controls as functional when their backend action is not
implemented. Prefer a smaller number of working destinations over inert
navigation.

## 3. Voice and screen overlay direction

Voice and screen-awareness modes should use a distinct, restrained
futuristic visual language rather than copying the desktop layout or using
a cliché Jarvis hologram aesthetic.

Desired direction:

- Compact floating translucent panel or pill.
- Clear Listening, Thinking, and Speaking states.
- Subtle waveform or audio-level visualisation.
- Small, explicit screen-context/capture status indicator.
- Immediate pause/stop and privacy controls.
- Minimal motion, no giant holograms, spinning rings, or excessive neon.
- Always communicate when microphone or screen capture is active.

The overlay is a later focused implementation. Tonight's desktop sprint
establishes its visual direction but does not attempt to finish voice,
microphone, or screen-capture infrastructure.

## 4. Approved application architecture

The agreed initial implementation remains:

- Rust owns authoritative application state, orchestration, tools,
  permissions, and task lifecycle.
- Qt 6, C++, and QML provide the initial desktop client.
- JSON-RPC 2.0 over NDJSON remains the application protocol.
- Linux uses the existing Unix-domain socket transport and peer
  authentication.
- Ollama remains the current inference provider.
- The current model remains `assistant-qwen:latest` (Qwen 3.5 2B Q4_K_M).
- Default context remains 16K, configurable from 4K to 16K.
- The desktop remains a client of the Rust core, not the authority for
  permissions or agent execution.

The user has explicitly agreed that this architecture is not immutable.
A frontend or foundational technology may be replaced if a demonstrated
benefit justifies the migration. Do not migrate merely for novelty or
because another framework makes a visual effect easier. Present major
alternatives, trade-offs, licensing, performance, privacy, and migration
cost before seeking approval for a major replacement.

For the immediate sprint, retain Rust + Qt/QML. It is already integrated
with the working core and can deliver the desired visual direction without
a disruptive rewrite. Reassess only if practical limitations appear.

## 5. Current implementation baseline

The local working tree has progressed beyond some older committed planning
status descriptions. The latest reported focused validation includes:

- Workspace-aware request context and tool execution.
- Workspace selection from the desktop and per-request propagation.
- Workspace-constrained listing, reading, project search, and code
  inspection with sensitive-path and traversal protections.
- Tree-sitter-based structural inspection for the initially selected
  language set.
- Agent-specific tool allowlists, planner filtering, and independent
  orchestrator enforcement.
- Keyword-based specialist routing with General fallback.
- General, Research, Coding, and System agent definitions.
- Streaming chat, cancellation, persistent conversations, and tool
  activity notifications.
- Focused parser and planner tests passed; latest reported full backend
  test run: 95 passed, 0 failed.
- The user tested the current desktop workflow and reported that it
  "looks good".

These are reported local-working-tree results, not a claim that every
change has already been committed to this branch. Do not overwrite local
uncommitted files or assume the local branch is synchronized with GitHub.

## 6. Tonight's desktop completion sprint

### Goal

Turn the current prototype into a visually coherent, usable desktop
application with a liquid-glass-inspired appearance.

### Priority order

1. Inspect the current local QML and backend API; preserve workspace,
   conversation, settings, streaming, cancellation, and tool-event wiring.
2. Refine the main application shell: window background, navigation,
   top-level status, content spacing, and glass surfaces.
3. Polish the conversation view: empty state, message hierarchy, Markdown
   and code readability, streaming state, and composer.
4. Make workspace selection and current workspace visible and dependable.
5. Add a useful task/activity inspector using the existing tool-event data.
6. Make the layout adapt to narrower windows; hide or collapse secondary
   panes instead of allowing overflow.
7. Polish essential settings, loading, disconnected, and error states.
8. Build and launch the app; fix only meaningful regressions and QML errors.
9. Record a Git checkpoint after the user confirms the result.

### Scope boundaries for tonight

- Do not rewrite the Rust core.
- Do not migrate to Electron/React or another frontend tonight.
- Do not build the complete voice or screen-awareness engine tonight.
- Do not add large dependencies solely for blur or animation.
- Do not spend the sprint on broad test, formatting, or Clippy campaigns.
- Do not replace working features with visual mockups.
- Keep existing local uncommitted changes and backups intact.

### Completion criteria

- Application builds and launches on the current CachyOS/KDE environment.
- User can start/select conversations and send messages.
- Streaming, stop/cancel, and connection status remain functional.
- Workspace selection remains functional and is visible.
- Existing tool activity appears meaningfully in the UI.
- Core controls and settings remain reachable.
- Glass styling is legible, responsive, and coherent with or without
  compositor blur.
- No major existing workflow is lost.

## 7. Next development session: feature expansion

After the desktop experience is stable, continue with:

1. Complete the reusable Tool System and adapters.
2. Expand the Agent Coordinator beyond current specialist routing.
3. Add delegated task contracts, bounded worker lifecycle, result
   aggregation, failure handling, and permission preservation.
4. Finish execution supervision, timeouts, resource limits, and isolation.
5. Expand Coding Agent workflows using ripgrep and Tree-sitter.
6. Integrate MCP only through the native Tool Registry and permission
   broker.
7. Add research providers with explicit network and data-transmission
   controls.
8. Build user-controlled memory and personalization.
9. Implement the Screen Awareness Engine using replaceable KDE/Linux
   providers and visible privacy controls.
10. Evaluate voice components and build the separate compact voice overlay.
11. Complete model management, settings, packaging, and installer.
12. Stabilize Linux before starting the Windows desktop product.

The AI-integrated OS exploration remains a later product, after Linux and
Windows desktop products.

## 8. Reuse-first and approval rules

Continue to prefer mature open-source components when they provide the
required capability without compromising UX, privacy, security,
performance, licensing, or maintainability. Integrate them behind Local
Assistant-owned interfaces and preserve applicable license notices.

The following approaches were explicitly approved:

- ripgrep: installed executable through a controlled, bounded adapter.
- MCP: adapter into the native Tool Registry; native permissions remain
  authoritative.
- Tree-sitter: native Rust parser/grammar integration for code structure.

Approval of an approach does not automatically approve unrelated runtime
dependencies, distribution changes, or broader architecture changes.

Major architecture, framework, dependency, licensing, platform, hardware,
cloud/data-transmission, and product-direction changes require a concise
comparison of options and explicit user approval. Routine implementation
within approved decisions can proceed without repeated confirmation.

## 9. Working style and delivery

- Work in coherent batches to reduce unnecessary back-and-forth.
- Prefer direct, copy-paste terminal commands that write files.
- Inspect the actual local project state before changing it.
- Preserve local modifications and backups.
- Use connected GitHub access to inspect committed repository files.
- Ask for local command output only when it is needed to understand
  uncommitted state or validate a local build.
- Focused validation is preferred over repeatedly running every test.
- Do not push or overwrite local changes without a clear checkpoint plan.
