# Local Assistant 2.0 — Action Plan

Last updated: 2026-11-23

This document describes the practical implementation sequence. The Master
Plan defines the long-term product direction; this document determines
what to implement next.

## Open-Source Integration Research

A living technology inventory and evaluation checklist is maintained in
[docs/INTEGRATION_CATALOG.md](INTEGRATION_CATALOG.md). Consult it before
starting a new subsystem and update it when new useful projects are found,
evaluated, adopted, rejected, or superseded.

The catalog is deliberately broad: inference, agent orchestration, tool
protocols, coding intelligence, memory/search, document extraction, OCR,
voice, screen awareness, Linux desktop services, security, packaging, OS
infrastructure, and developer tooling. Listing a candidate does not approve
its adoption.

## Development Rules

- Reuse mature open-source components when they meet our requirements.
- Do not introduce major dependencies without evaluating them.
- Ask the user to approve major architectural and dependency choices.
- Keep external integrations behind replaceable interfaces.
- Preserve local-first privacy and capability boundaries.
- Work in coherent implementation batches.
- Keep the application buildable and runnable.
- Update documentation and Git checkpoints after meaningful milestones.

# Milestone 1 — Workspace-Aware Tool Execution

Status: NEXT

Goal: Make tools operate on the workspace selected for the current request
rather than relying on a daemon-startup environment variable.

Tasks:

1. Inspect the existing workspace implementation and security boundaries.
2. Fix sensitive-directory handling in workspace listing.
3. Ensure directory limits also limit enumeration work.
4. Review symlink handling and path canonicalization.
5. Add workspace selection to the desktop.
6. Add workspace information to the chat IPC request.
7. Propagate workspace through RequestContext.
8. Make workspace available through the agent/tool execution context.
9. Remove the unused workspace-builder warning by completing propagation.
10. Preserve safe defaults when no workspace is selected.

Acceptance criteria:

- Workspace selection works from the desktop.
- The selected workspace reaches tool handlers for that request.
- A request cannot escape its selected workspace.
- Sensitive locations are consistently protected.
- Listing and reading have bounded resource usage.
- Existing calculator and system tools continue working.
- Tests cover traversal, symlinks, sensitive paths, limits, and missing
  workspace configuration.

# Milestone 2 — Reusable Tool System Expansion

Goal: Make the tool system useful for real project work.

Tasks:

1. Standardize execution context, results, errors, and logging.
2. Add project directory exploration.
3. Add bounded file search.
4. Add code and text inspection.
5. Add file metadata inspection.
6. Evaluate external tool integration through MCP.
7. Define permission levels for read-only and modifying tools.
8. Design safe file modification and approval workflows.
9. Add timeout and resource-limit conventions.
10. Keep arbitrary shell execution outside the default tool surface.

Approved integration decisions (2026-11-23):

- **ripgrep:** use the installed `rg` executable through a controlled binary
  adapter. Do not embed or reimplement its search engine.
- **MCP:** integrate through an adapter into our native Tool Registry. The
  native registry remains authoritative for permissions, execution context,
  logging, and result normalization.
- **Tree-sitter:** include structural parsing in the Coding Agent after
  basic project search is working. Use it for syntax-aware code navigation,
  not as a replacement for ordinary text search.

Implementation requirements:

1. Detect and validate the `rg` executable and expose a clear unavailable
   error if it is missing.
2. Invoke it without a shell, using argument arrays and a controlled
   environment.
3. Restrict search roots to the request's authorized workspace.
4. Bound execution time, result count, output size, and concurrency.
5. Respect ignore files by default and provide explicit, permission-checked
   options when broader search is needed.
6. Normalize matches into stable structured results with paths, line
   numbers, and snippets.
7. Add Tree-sitter as a separate code-intelligence adapter with supported
   language/parser coverage documented.
8. Evaluate parser distribution and grammar licensing before shipping
   language grammars.
9. Add MCP client/server transport support only through a permission-aware
   adapter; external servers must not bypass the native Tool Registry.

The exact Rust crates, MCP transport options, and Tree-sitter grammar
distribution strategy remain implementation research items; no additional
major framework has been approved.

Acceptance criteria:

- Built-in and external tools use a consistent registry interface.
- Arguments are validated before execution.
- Capabilities are checked centrally.
- Tool output and execution time are bounded.
- Errors are structured and useful to the agent.
- Sensitive operations require explicit authorization.

# Milestone 3 — Agent Coordinator

Goal: Turn the current planner and agent definitions into an actual
multi-agent system.

Tasks:

1. Define coordinator responsibilities and task contracts.
2. Classify incoming tasks.
3. Select General, Research, Coding, or System specialists.
4. Route work through a shared execution interface.
5. Preserve the original authenticated principal.
6. Enforce specialist capability boundaries.
7. Track delegated task IDs and lifecycle.
8. Aggregate specialist results.
9. Handle failures and bounded retries.
10. Prevent uncontrolled delegation loops.

Reuse investigation:

- Compare a custom lightweight Rust coordinator with established
  orchestration projects.
- Evaluate LangGraph and Rust-native alternatives.
- Evaluate whether a framework adds enough value to justify its runtime,
  complexity, and integration cost.

Approval gate:

The user must approve a major orchestration framework or architecture
choice before adoption.

Acceptance criteria:

- A request can be routed to an appropriate specialist.
- Specialists receive bounded task contracts.
- Results return to the coordinator.
- Delegation cannot expand permissions.
- Failures do not create uncontrolled retries.
- The desktop can display meaningful task activity.

# Milestone 4 — Specialist Workflows

Goal: Give each specialist useful, bounded capabilities.

General:
- Coordinate ordinary requests.
- Delegate when appropriate.
- Produce final user-facing responses.

Coding:
- Inspect project structure.
- Search and read code.
- Explain code and identify relevant files.
- Propose changes.
- Use explicitly authorized modification workflows.

Research:
- Gather information through approved local or external providers.
- Compare sources.
- Produce structured findings.
- Respect external data and network permissions.

System:
- Provide system information.
- Inspect approved system state.
- Perform only explicitly authorized system actions.

Acceptance criteria:

- Each specialist has a clear responsibility.
- Tool access is capability-limited.
- The coordinator can combine specialist results.
- No specialist can grant itself new permissions.

# Milestone 5 — Execution Supervision and Reliability

Goal: Make longer-running tasks safe and manageable.

Tasks:

- Worker lifecycle management.
- Cancellation and timeout propagation.
- Bounded concurrency.
- Resource limits.
- Failure isolation.
- Controlled retries.
- Reliable IPC reconnection.
- Application startup and shutdown recovery.

Reuse investigation:

Evaluate existing process supervision and asynchronous runtime
capabilities before writing custom infrastructure.

Acceptance criteria:

- Tasks have observable lifecycle states.
- Cancellation reaches supported operations.
- Failed workers do not corrupt conversations.
- Resource use is bounded.
- Desktop state recovers after daemon or connection failures.

# Milestone 6 — Memory and Personalization

Goal: Provide user-controlled continuity beyond individual conversations.

Tasks:

- Separate conversation history from durable memory.
- Define memory records and provenance.
- Support user review and correction.
- Support deletion and retention controls.
- Store adaptive interaction preferences separately.
- Add confidence and evidence tracking.
- Introduce versioned skills and evaluations.

Reuse investigation:

Evaluate existing local storage, search, and embedding components.
Do not introduce a vector database unless actual retrieval requirements
justify it.

Acceptance criteria:

- Memory is inspectable and user-controlled.
- Conversation deletion and memory deletion have clear semantics.
- Personalization cannot override safety or permissions.
- Skills are evaluated before activation.

# Milestone 7 — Screen Awareness Engine

Goal: Provide useful desktop context during active Assistant Mode.

Tasks:

1. Define the capture-provider interface.
2. Evaluate XDG Desktop Portal and PipeWire.
3. Integrate KDE/KWin context.
4. Evaluate AT-SPI accessibility information.
5. Integrate OCR only where useful.
6. Build visible activation and pause controls.
7. Add app/window exclusions.
8. Add sensitive-content protections.
9. Optimize event-driven context updates.
10. Keep a replaceable Windows provider boundary.

Acceptance criteria:

- Capture is visibly active.
- Pause and disable work immediately.
- No capture occurs while inactive or paused.
- Exclusions are enforced.
- Processing remains local by default.
- Resource usage is reasonable.

Approval gate:

Any major capture framework, always-on service, or cloud processing
decision requires user approval.

# Milestone 8 — Linux Product Completion

Tasks:

- Complete settings and model management.
- Complete workspace and tool controls.
- Complete memory and privacy controls.
- Complete Screen Awareness integration.
- Complete agent activity presentation.
- Improve IPC and lifecycle reliability.
- Perform focused integration and failure-path testing.
- Review dependencies and licenses.
- Build Linux packaging and installer.
- Write installation and troubleshooting documentation.

Acceptance criteria:

- A new user can install and run the product.
- Core workflows work without development tools.
- Errors are understandable.
- Privacy controls are accessible.
- The product remains usable on the target hardware.

# Milestone 9 — Windows Desktop

Begin only after Linux product completion.

Tasks:

- Select Windows IPC transport.
- Implement Windows capture and accessibility providers.
- Validate shared Rust core compatibility.
- Implement Windows packaging.
- Test feature parity and performance.

Major platform dependencies and architectural changes require user approval.

# Milestone 10 — AI-Integrated OS Exploration

Begin only after both desktop products are complete.

Evaluate integration into a CachyOS-derived OS while preserving the
independent desktop assistant product.

# Immediate Work Queue

The next implementation batch should be:

1. Review and harden workspace listing and reading.
2. Propagate selected workspace through desktop, IPC, request context,
   agent context, and tool handlers.
3. Add focused security and integration tests.
4. Validate the complete Rust and desktop builds.
5. Commit a coherent checkpoint.
6. Begin the reusable tool expansion using the approved ripgrep binary
   adapter.
7. Add Tree-sitter structural parsing after basic search is working.
8. Implement MCP interoperability through a native-registry adapter.

Do not begin a major agent framework integration before the relevant
alternatives have been researched and presented for approval.

# Completion Tracking

Update this action plan when a milestone changes state:

- Planned
- In progress
- Blocked
- Complete

Record significant decisions, chosen dependencies, rejected alternatives,
and reasons in the architecture documentation or a decision record.
