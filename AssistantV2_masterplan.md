# LOCAL ASSISTANT 2.0 — MASTER PROJECT PLAN

**Version:** 1.0  
**Last updated:** 22 November 2026  
**Status:** Requirements discovery completed; architecture discovery in progress. Deployment profiles, multi-user direction, data ownership, central permission-broker policy and modular-core/selective-worker architecture approved; implementation technologies remain unselected.

> This document is the central source of truth for the Local Assistant 2.0 project. Upload it at the beginning of future conversations to restore project context. Update it whenever major decisions change.

---

# 1. PROJECT VISION

Build a free, open-source, modular, hardware-efficient AI computing platform that can become an intelligent part of the user's computing environment.

This is much more than a chatbot. The long-term vision combines three major products.

## 1.1 Local Assistant Platform

The intelligence layer responsible for:

- AI models and inference.
- Model selection and routing.
- Main agent and specialist capabilities.
- Memory and knowledge.
- Project intelligence.
- Tool execution.
- Permissions and security.
- Integrations and plugins.
- Automation and workflows.

## 1.2 Universal AI Desktop

The user-facing experience:

- Chat interface.
- Real-time voice assistant.
- Floating desktop companion.
- Command-centre dashboard.
- Visual desktop understanding.
- Application interaction.
- Notifications and proactive assistance.
- Configurable settings and themes.

## 1.3 AI-powered Arch Linux distribution

An eventual lightweight Arch-based operating system featuring:

- Assistant integration throughout the desktop.
- AI-powered system management.
- Intelligent resource management.
- Developer-friendly environment.
- Gaming support.
- Configurable installation profiles.
- Hardware detection and automatic configuration.
- Simple graphical installer.

These three products should be separate but interoperable.

The assistant must also work on ordinary Linux and Windows installations without requiring users to install the custom operating system.

**Important:** The existing `~/local-assistant` repository is reference material only. It must not dictate the new architecture or technology choices.

---

# 2. USER'S GOALS AND PRINCIPLES

The following are established requirements.

1. The project must always be free and open source.
2. Local-first operation, with optional cloud services.
3. Linux and Windows are the initial platform priorities.
4. Start development on CachyOS / Arch Linux.
5. macOS, mobile clients and headless/server deployments may follow.
6. The operating system must be lightweight.
7. Excellent support for programming and development tools.
8. Excellent Windows gaming support through Steam, Proton and Lutris.
9. Highly configurable interface, voice, monitoring and automation.
10. Avoid stereotypical sci-fi/JARVIS aesthetics.
11. Feature-rich, but mindful of hardware limitations.
12. Multiple users should eventually be supported.
13. The platform should be extensible by the open-source community.
14. The assistant should be proactive while remaining under user control.

Features should be evaluated according to their usefulness, performance impact, maintenance burden and complexity.

---

# 3. HARDWARE AND DEVELOPMENT ENVIRONMENT

Current development machine:

| Component | Specification |
|---|---|
| Laptop | ASUS TUF A15 |
| GPU | NVIDIA RTX 3050, 4 GB VRAM |
| RAM | 16 GB DDR5 |
| CPU | AMD Ryzen 7 170 (user-reported model) |
| CPU threads | 16 |
| Reported maximum clock | 4.83 GHz |
| Development OS | CachyOS / Arch Linux |

This is the initial practical development target, not the maximum hardware the finished platform should support.

The exact CPU identification should be verified before final hardware-specific tuning.

## Hardware design principles

- Avoid unnecessarily large models.
- Use task-specific smaller models where appropriate.
- Dynamically manage GPU and CPU workloads.
- Avoid loading multiple large models unnecessarily.
- Unload unused models.
- Support CPU inference and GPU offloading.
- Keep background processes lightweight.
- Adapt to available RAM and VRAM.
- Avoid interfering with gaming and other demanding applications.

The system must remain useful even when the GPU is occupied.

---

# 4. CENTRAL AGENT ARCHITECTURE

## 4.1 One main orchestrator

The system will use ONE central main agent to manage intelligence and coordinate specialist capabilities.

The main agent will:

1. Understand the user's request.
2. Determine the required capabilities.
3. Plan the task.
4. Select suitable tools and specialist agents.
5. Request permissions when necessary.
6. Delegate work.
7. Collect specialist results.
8. Verify outcomes.
9. Report the result to the user.

## 4.2 Specialist agents

Specialists will be task-specific capabilities invoked by the central orchestrator.

Potential specialists:

- Coding and software engineering.
- Research and browsing.
- Documents and files.
- System administration.
- Project management.
- Knowledge and learning.
- Desktop interaction.
- Workflow automation.
- Scheduling and organisation.

They should not all run continuously.

## 4.3 Why one central orchestrator?

The user prefers one main agent because it should reduce unnecessary resource usage and improve efficiency.

This is a sensible direction.

A single orchestrator can reduce:

- Duplicate planning.
- Unnecessary agent coordination.
- Multiple simultaneous model loads.
- Uncontrolled background activity.
- Repeated context processing.

However, a single orchestrator does not automatically guarantee lower inference costs.

Efficiency will depend on:

- Model size.
- Context length.
- Number of concurrent requests.
- Specialist implementation.
- Runtime sharing.
- Hardware scheduling.

Therefore, the intended design is:

**One central orchestrator + on-demand specialist agents + shared model runtimes where practical + bounded parallelism.**

Sequential execution should be the default. Parallel execution should only be used when it materially improves the task.

---

# 5. CORE CAPABILITIES

## 5.1 Conversational intelligence

- ChatGPT-style interface.
- Persistent conversations.
- Local model inference.
- Optional cloud model providers.
- Automatic model selection based on task.
- Manual model overrides.
- Hardware-aware model routing.
- Offline operation.
- Configurable privacy and cloud-sharing policies.

## 5.2 Memory and knowledge

The assistant should support:

- Short-term conversational context.
- Long-term memory.
- Structured knowledge.
- Document indexing and retrieval.
- Project-specific memory.
- User preferences.
- Past actions and workflows.
- Automatic organisation.
- User-controlled memory review and management.

It must distinguish between confirmed facts, assumptions and unknown information.

Users should be able to inspect, edit, export and delete their stored information.

## 5.3 Project intelligence

The assistant should maintain structured understanding of multiple projects.

Each project can contain:

- Goals.
- Tasks.
- Milestones.
- Documents.
- Source code.
- Decisions.
- Conversation history.
- Known issues.
- Progress reports.
- Suggested improvements.

It should monitor selected projects and proactively report meaningful changes.

## 5.4 Desktop awareness

The assistant should support:

- Inspecting the active application on request.
- Configurable periodic desktop captures.
- Explicitly enabled visual monitoring.
- Per-application permissions.
- Visible monitoring indicators.
- Pause and resume controls.
- Privacy and retention settings.

No hidden screen, microphone or camera recording.

## 5.5 Computer interaction

With user-configured permissions, the assistant should eventually be able to:

- Read and modify files.
- Use terminals.
- Interact with code editors.
- Browse websites.
- Control desktop applications.
- Manage approved processes.
- Change approved system settings.
- Install or configure software.
- Execute multi-step tasks.
- Verify that actions succeeded.

Support multiple interaction methods:

1. Application APIs.
2. Accessibility interfaces.
3. Browser automation.
4. Visual screen understanding.
5. Keyboard and mouse interaction.
6. Command-line tools.

The assistant should choose the most reliable method available.

## 5.6 Voice

- Natural real-time voice.
- Configurable speech recognition.
- Configurable speech synthesis.
- Voice selection.
- Interruptions.
- Context-aware conversation.
- Optional wake word.
- Push-to-talk.
- Visible microphone status.

Voice functionality must be independently configurable and disableable.

## 5.7 Proactive assistance

The assistant should provide:

- Desktop notifications.
- Floating desktop companion.
- Configurable notification frequency.
- Quiet hours.
- Project suggestions.
- Workflow suggestions.
- Event-based notifications.
- Important task reminders.

The user should control how frequently the assistant interrupts.

## 5.8 Integrations

Create a unified integration framework supporting:

- Native plugins.
- APIs.
- Webhooks.
- MCP.
- External tool servers.
- Browser extensions.
- Native desktop integrations.
- Application-specific connectors.

The plugin framework should support:

- Language-independent interfaces where practical.
- Python and JavaScript/TypeScript implementations.
- Plugin manifests.
- Permission declarations.
- Dependency management.
- Version compatibility.
- Sandboxing.
- Signing and trust information.
- Community discovery.
- Easy installation and removal.

Plugins must not silently gain privileges.

---

# 6. AUTOMATION AND WORKFLOW LEARNING

The assistant should support:

- Individual automated actions.
- Multi-step tasks.
- Reusable workflows.
- Scheduled tasks.
- Event-triggered workflows.
- Configurable autonomous automation.

## Workflow learning

The user explicitly chose BOTH:

1. Learn from approved observation and suggest workflows.
2. Provide explicit workflow recording that the user can start and stop.

The assistant should:

- Observe approved interactions.
- Recognise repetitive activities.
- Suggest possible workflows.
- Allow the user to inspect and edit them.
- Save workflows only after review.
- Require appropriate permission before activating automation.

It must never silently convert observations into active automation.

---

# 7. CENTRAL PERMISSION BROKER

This is one of the most important architectural components.

Every agent, plugin, integration and automation must request privileged operations through one central permission broker.

No agent or plugin may bypass it or grant itself additional permissions.

## Permission evaluation

The broker should evaluate:

- Requesting component.
- Requested capability.
- Target application or resource.
- User-defined policies.
- Risk level.
- Reversibility.
- Scope and duration.
- Approval requirements.
- Resource limits.

## Required functionality

- Per-tool permissions.
- Per-plugin permissions.
- Per-application permissions.
- Per-folder permissions.
- Per-project permissions.
- Read-only and write/execute permissions.
- Temporary permissions.
- Expiring grants.
- Configurable autonomy.
- Approval prompts.
- Action previews.
- Audit logs.
- Cancellation.
- Emergency stop.

## Autonomy policy

The assistant should have configurable autonomy levels.

Default approach:

- Harmless reversible actions may run automatically when enabled.
- Ordinary modifications follow the user's configured permissions.
- Consequential or destructive actions require approval.
- Security-sensitive operations receive additional scrutiny.
- Users can enable ask-before-every-action mode.
- Permissions must be understandable and revocable.

---

# 8. AI RESOURCE MANAGER

This is a foundational component.

It should manage CPU, RAM, GPU, VRAM and AI workloads across the platform.

## Responsibilities

- Monitor resource availability.
- Manage model loading and unloading.
- Avoid duplicate model instances.
- Coordinate inference requests.
- Limit concurrent model workloads.
- Queue tasks.
- Defer unnecessary background work.
- Support CPU and GPU inference.
- Manage GPU offloading.
- Provide hardware profiles.
- Adapt to gaming, coding, idle and power modes.

## Example behaviour

During gaming:

- Reduce background AI activity.
- Pause nonessential monitoring.
- Avoid unnecessary model loading.
- Preserve GPU resources for games.

During coding:

- Prioritise coding-related assistance.
- Make project context readily available.
- Allow relevant background work.

When GPU resources are unavailable:

- Use an appropriate CPU strategy.
- Offload where supported.
- Queue or defer work.

The resource manager should integrate with the existing gaming-mode workflow where appropriate:

`~/.local/bin/assistant-mode gaming; %command%; ~/.local/bin/assistant-mode gpu`

The future implementation may improve or replace this workflow.

---

# 9. EVENT-DRIVEN ARCHITECTURE

Prefer event-driven execution rather than permanently running expensive agents.

Examples:

- File changes → notify relevant project context.
- Build failure → invoke coding specialist.
- Scheduled task due → trigger automation.
- Development application opens → provide relevant project context.
- GPU becomes available → resume queued AI work.

Lightweight event detection should be preferred over continuous large-model inference.

This is essential for performance on constrained hardware.

---

# 10. DASHBOARD AND INTERFACE

The assistant should offer several configurable interfaces:

1. Chat workspace.
2. Command-centre dashboard.
3. Floating desktop companion.
4. Project workspaces.
5. Voice controls.
6. Settings and permission centre.

The dashboard may display:

- Active tasks.
- Active specialist capabilities.
- Model usage.
- CPU and RAM usage.
- GPU and VRAM usage.
- Current permissions.
- Pending approvals.
- Recent actions.
- Project progress.
- Background automations.
- Integration health.

The dashboard should be optional and modular.

Visual design should be clean, modern and configurable, not excessively futuristic.

---

# 11. VISUAL AUTOMATION ENGINE

The assistant should be capable of interacting with applications even when they lack good APIs.

It should support:

- Visual screen understanding.
- Accessibility interfaces.
- Browser automation.
- Keyboard and mouse interaction.
- Application APIs.
- CLI tools.

It should select the most reliable available method.

All interactions must respect the permission broker.

The assistant should verify results rather than assuming that an action succeeded.

---

# 12. MISTAKE HANDLING AND RECOVERY

The user chose:

**Automatic undo wherever possible, and checkpoints for larger operations.**

Required features:

- Automatic undo for supported reversible actions.
- Checkpoints before consequential changes.
- Rollback where possible.
- Preview of destructive changes.
- Action history.
- Verification of results.
- Cancellation of long-running operations.
- Emergency stop.

Also provide:

- Configuration backups.
- Memory backups.
- Project exports.
- Workflow exports.
- Plugin version rollback.
- Restore after failed updates.
- Migration between machines.

---

# 13. DATA STORAGE AND SYNCHRONISATION

Support multiple configurations:

- Local-only operation.
- Manual backup and restore.
- Encrypted device-to-device sync.
- Self-hosted server.
- Optional cloud synchronisation.

The user should control what data is synchronised and where it is stored.

Memory, project information and configuration should be portable.

---

# 14. PLUGIN ECOSYSTEM

The project should eventually support a community plugin ecosystem.

Requirements:

- Plugin SDK.
- Language-independent interfaces.
- Manifests.
- Permission declarations.
- Dependency management.
- Versioning.
- Compatibility checks.
- Sandboxing.
- Publisher verification.
- Signing and trust information.
- Community repository.
- Easy installation, updating and removal.

Plugins should have restricted permissions by default.

---

# 15. AI-POWERED ARCH LINUX DISTRIBUTION

Long-term goal: create a lightweight Arch-based AI operating system.

## Priorities

- Lightweight default installation.
- Clean, modern desktop.
- Configurable appearance.
- Assistant integrated into desktop workflows.
- Developer-friendly environment.
- Excellent support for VS Code and other development applications.
- Gaming support through Steam, Proton and Lutris.
- Gaming-aware resource management.
- Hardware detection.
- Modular installation profiles.
- Simple graphical installer.
- Backup and recovery.
- Ability to disable the assistant without breaking the desktop.

The OS should be a good operating system first. AI should improve the experience rather than make ordinary computing slower.

Use established Arch tooling where appropriate instead of reinventing the entire installation process.

---

# 16. LICENSING

**Intentionally deferred.**

The user wants the project to remain free and open source but does not want to decide licensing yet.

IMPORTANT REMINDER:

Revisit licensing during architecture finalisation and before public release.

Evaluate appropriate licences for:

- Main application.
- Desktop components.
- Backend services.
- Plugins and SDK.
- Installer.
- Documentation.
- Distribution components.

Potential candidates include GPL, AGPL and Apache/MIT-style licensing.

Also examine third-party dependency licences and model-weight licences.

Open-source application code does not automatically make third-party models or cloud services open source.

---

# 17. TECHNOLOGY SELECTION

No technology stack has been finalised.

All technologies must be re-evaluated from scratch.

Do not automatically retain:

- FastAPI.
- Tauri.
- React.
- TypeScript.
- Rust.
- LangGraph.
- Ollama.
- SQLite.
- ChromaDB.
- Whisper.cpp.
- Piper.

These are historical experiments or previous suggestions, not binding decisions.

Compare current technologies for:

- Desktop application framework.
- UI framework.
- Backend architecture.
- Inter-process communication.
- Agent orchestration.
- Local inference.
- Model formats.
- Hardware acceleration.
- Model routing.
- Speech recognition.
- Speech synthesis.
- Memory and knowledge storage.
- Search and retrieval.
- Browser automation.
- Desktop automation.
- Accessibility interfaces.
- Plugin execution and sandboxing.
- Event systems.
- Scheduling.
- Security.
- Encrypted synchronisation.
- Linux integration.
- Windows support.
- Packaging and updates.
- Testing and diagnostics.

## Evaluation criteria

1. Performance on RTX 3050 4 GB / 16 GB RAM.
2. CPU and memory overhead.
3. Reliability.
4. Cross-platform support.
5. Security.
6. Extensibility.
7. Maintainability.
8. Offline capability.
9. Developer experience.
10. Long-term sustainability.

Prefer the simplest architecture that satisfies the requirements.

A modular monolith may be a good starting point, with clearly defined boundaries that allow components to become separate services when justified.

---

# 18. DEVELOPMENT ROADMAP

These are proposed stages. Detailed milestones and acceptance tests must be created after architecture decisions.

## Phase 0 — Discovery and architecture

- Consolidate requirements.
- Audit old repository for lessons.
- Verify hardware.
- Research technologies.
- Design permission architecture.
- Design resource manager.
- Define data boundaries.
- Define module boundaries.
- Design testing strategy.

## Phase 1 — Platform foundation

- Application shell.
- Configuration.
- Core service lifecycle.
- Event system.
- Logging.
- Diagnostics.
- Permission broker.
- Resource manager foundations.
- Local model runtime abstraction.
- Basic chat.

## Phase 2 — Intelligence and memory

- Main orchestrator.
- On-demand specialist capabilities.
- Model routing.
- Conversation memory.
- Structured memory.
- Document retrieval.
- Project workspaces.

## Phase 3 — Computer interaction

- Tool framework.
- File management.
- Terminal tools.
- Browser integration.
- Accessibility interfaces.
- Visual interaction.
- Permission prompts.
- Action previews.
- Verification.
- Undo and checkpoints.

## Phase 4 — Desktop experience

- Desktop UI.
- Dashboard.
- Notifications.
- Floating companion.
- Voice.
- Application awareness.
- Monitoring controls.

## Phase 5 — Automation and ecosystem

- Workflow recording.
- Approved-observation learning.
- Scheduled workflows.
- Event-triggered workflows.
- Plugin SDK.
- Plugin sandboxing.
- Versioning.
- Integration management.

## Phase 6 — Sync and portability

- Encrypted synchronisation.
- Self-hosted mode.
- Backup and restore.
- Data migration.
- Windows support.
- Cross-platform packaging.

## Phase 7 — AI Linux distribution

- Base system strategy.
- Desktop selection.
- Installer.
- Hardware profiles.
- Developer profile.
- Gaming profile.
- Assistant integration.
- Update and recovery system.
- ISO testing.

---

# 19. ACCEPTANCE TESTS

Each development milestone must include measurable tests.

At minimum, verify:

- Assistant starts and stops reliably.
- Local-only operation works.
- Cloud routing respects configured policies.
- No agent or plugin can bypass the permission broker.
- Permissions are visible and revocable.
- Destructive operations require configured approval.
- Action logs identify requesting components.
- Cancellation works.
- Undo works where supported.
- Checkpoints can be restored.
- Resource limits prevent uncontrolled model concurrency.
- Gaming mode reduces nonessential AI workloads.
- Models unload and release resources.
- Memory can be inspected, edited and exported.
- Project memory is appropriately separated.
- Visual monitoring is visible and pausable.
- No hidden screen, camera or microphone recording occurs.
- Plugins cannot silently elevate permissions.
- Workflows require review before activation.
- Backups can be restored.
- Optional modules can be disabled without breaking core functionality.
- The platform remains responsive on baseline hardware.

---

# 20. FUTURE CONVERSATION INSTRUCTIONS

When this document is uploaded into a new conversation:

1. Read the entire document before suggesting changes.
2. Treat it as the current source of truth.
3. Do not ask the user to repeat established requirements.
4. Do not assume the previous technology stack is retained.
5. Do not start coding prematurely.
6. Continue with technology research and architecture comparisons.
7. Explain trade-offs and recommend a clear direction.
8. Ask only questions that materially affect important decisions.
9. Update the plan when new decisions are made.
10. Keep the project modular and implementation realistic.
11. Prefer incremental, testable implementation over giant scripts.
12. Preserve the central orchestrator, permission broker and resource manager as foundational design requirements.
13. Remind the user about licensing at architecture finalisation and release planning.

---

# 21. COMPACT PROJECT SUMMARY

**Project:** Local Assistant 2.0

**Goal:** Free, open-source, local-first AI computing platform, eventually integrated into a lightweight AI-powered Arch distribution.

**Initial OS:** CachyOS / Arch Linux.

**Hardware:** ASUS TUF A15, RTX 3050 4 GB, 16 GB DDR5, AMD Ryzen 7 170 (user-reported), 16 threads, up to 4.83 GHz.

**Platforms:** Linux and Windows first; macOS and other clients later.

**Agent design:** One central orchestrator delegates to on-demand specialist agents. Shared model runtimes where practical, sequential execution by default and bounded parallelism.

**Foundations:** Central permission broker, AI resource manager, event-driven architecture, modular plugin framework, memory and project intelligence, undo and checkpoints.

**Interaction:** Chat, configurable voice, visible desktop awareness, visual automation, notifications, floating companion and dashboard.

**Autonomy:** Configurable. Harmless actions can run automatically; consequential operations follow approval policies.

**Workflow learning:** Approved observation plus explicit recording, with user review before activation.

**Operating system:** Lightweight, developer-friendly, gaming-friendly, highly configurable and not JARVIS-themed.

**Licensing:** Deferred intentionally. Remind the user during architecture finalisation and release planning.

**Technology:** Not selected. Research and compare everything from scratch.

**Existing repository:** Reference only, not the foundation.

**Next step:** Continue Stage 1A architecture research with core/client communication, IPC and lifecycle contracts. Do not start coding yet.

---



# 22. APPROVED ARCHITECTURE: DEPLOYMENT, MULTI-USER AND DATA OWNERSHIP

This section records architecture directions explicitly approved during discovery on 22 November 2026. These decisions define requirements and boundaries; they do not select implementation technologies.

## 22.1 Deployment profiles

The platform should support four deployment profiles over shared core capabilities:

1. **Desktop service:** the core starts at user login and remains available when the main UI window closes. The UI connects independently. Background work runs only when permitted.
2. **Standalone desktop:** the core starts with the GUI and the associated core stops when the application closes.
3. **Headless/server:** no GUI is required; authenticated local or remote clients can connect, subject to explicit authorization.
4. **Optional system-integrated mode:** deeper OS integration and approved system-level events may be supported later. Privileged actions remain brokered. This is not a requirement for the initial release.

These are deployment profiles, not separate products or unrelated codebases. The same core capabilities should be reused.

Platform lifecycle direction:
- Linux desktop operation should default to a per-user background service, not a privileged system-wide daemon.
- Windows desktop operation should use per-user background processes and a separate interactive desktop process; avoid placing the whole assistant in a privileged Windows service.
- Headless operation should be supported.
- Local IPC is not automatically trusted; clients must be authenticated and authorized.
- Inference runtimes and specialist workers may have independent lifecycles.
- Background workloads remain subject to user permissions and resource management.

Exact service managers, IPC transports, authentication mechanisms and process implementations remain undecided.

## 22.2 Multi-user architecture

The approved direction is to support both:
- **Personal per-OS-user desktop installations:** each OS user has an independent assistant core and private conversations, memory, settings, permissions and background tasks.
- **Shared multi-user server deployments:** one managed core serves multiple authenticated users with strict data separation, per-user permissions, workspace membership and fair resource allocation.

A shared server may host multiple workspaces, and a user may belong to more than one workspace. Private data remains private unless explicitly shared.

Every operation must carry an authorization context identifying the user, workspace where applicable, acting component, requested resource and action. Background tasks retain the initiating owner's identity, scope and authorization; they do not gain broader rights by running on a shared worker.

Server administration must be separated from unrestricted access to users' private data. Resource scheduling must be bounded machine-wide and fair across users.

Initial delivery should prioritize the personal per-user core while designing identity, workspace and authorization interfaces for later shared-server operation. Multiple simulated users should be tested early.

Database isolation, authentication technology, encryption and scheduler implementation remain undecided.

## 22.3 Data ownership and persistence boundaries

Information ownership is organized into four scopes. These are logical ownership boundaries and do not require four separate databases.

### Installation scope
Owns machine/server-level configuration, installed components and runtimes, detected hardware capabilities, machine-wide resource limits and server security configuration.

### User scope
Owns private conversations, personal memory, preferences, personal knowledge and documents, personal credentials, permissions and personal automation. User data is private by default and is not automatically shared.

### Workspace scope
Owns information deliberately shared within a workspace, such as shared projects, documents, workspace knowledge, shared integrations, tasks and workspace policies. Access is limited to authorized members and roles.

### Task/session scope
Owns temporary execution state, temporary files and results, agent context, approval state and execution history associated with a particular task or session. Temporary context is isolated and subject to expiry/cleanup.

### Sharing and memory rules
- Personal memory and workspace knowledge are distinct.
- Personal information is never promoted to shared knowledge without explicit authorization.
- Using information in a response is not the same as saving it permanently.
- Conversation details do not automatically become long-term memory.
- An agent working on a shared project may access only the personal context and project/workspace context it is authorized to use, not a user's entire private memory.
- Sharing is explicit, scoped and revocable.

### Authorization context
Authorization decisions must consider the authenticated subject, resource, ownership scope, acting component, requested action and relevant conditions. Simple workspace roles (such as owner, administrator and member) may be combined with contextual attributes and policies.

The trusted core establishes and validates identity and workspace context. Agents, plugins and workers cannot self-assert a user identity or choose an arbitrary workspace context.

### Background work and shared compute
Queued and delayed tasks retain trusted owner and workspace context, permissions, resource/operation scope, expiry and cancellation state. Authorization should be rechecked when a delayed task executes if membership or permissions may have changed.

A shared inference runtime may serve multiple users, but conversation history, memory, retrieval results, caches, summaries, tool outputs and approval state must remain correctly scoped. Shared compute does not imply shared data or model context.

### Lifecycle, export and deletion
- Conversation persistence follows user-configured retention and deletion rules.
- Long-term memory is user-controlled, reviewable and editable.
- Project knowledge follows project ownership and lifecycle.
- Temporary task context is removed after completion or expiry according to policy.
- Background task state is retained only as needed for execution and configured history.
- Audit records are access-restricted and should minimize sensitive content.
- Caches and retrieval artifacts are scoped and expire.
- Users need data export and deletion controls; projects need archive/deletion controls; workspace membership revocation must remove access.
- Deletion and revocation must account for derived indexes, caches and delayed jobs.
- Backup deletion and expiry must be documented honestly; deleting a live record does not necessarily erase every retained backup immediately.

### Storage isolation direction
Evaluate storage isolation according to sensitivity, threat model and operational requirements. Separate per-user or per-workspace stores can provide stronger boundaries with additional overhead; shared stores can be more efficient but require consistently enforced access policies. A hybrid approach is the current architectural direction to evaluate further. No database, schema, vector store or encryption/key-management technology is selected.

### Approved principles
1. Use installation, user, workspace and task/session ownership scopes.
2. Keep personal data private by default.
3. Require explicit sharing.
4. Separate personal memory from shared project/workspace knowledge.
5. Do not automatically promote temporary context into permanent memory.
6. Authorize every resource operation.
7. Establish trusted identity and workspace context in the core.
8. Preserve owner authorization across background tasks.
9. Keep user contexts isolated even when compute is shared.
10. Choose storage isolation according to sensitivity and operational needs.
11. Provide user-controlled export, deletion and retention.
12. Account for derived data and delayed work when deleting or revoking access.
13. Separate server administration from unrestricted access to private user data.

## 22.4 Still open

The following remain under research and must not be treated as approved implementation choices:
- Database and object-storage technologies.
- Separate databases, schemas, row-level isolation or other concrete tenancy mechanisms.
- Authentication and session technology.
- Encryption and key management.
- Exact retention periods and backup lifecycle.
- IPC transport and remote-client protocol.
- Concrete resource scheduler and fairness algorithm.
- Exact service/process topology and platform-specific implementation.

# 23. APPROVED ARCHITECTURE: CENTRAL PERMISSION BROKER AND AUTHORIZATION

Approved on 22 November 2026. This records policy and security requirements, not a selected policy-engine or implementation technology.

## 23.1 Authorization and permission policy

All privileged actions initiated by agents, plugins, integrations, tools and automations must pass through one central permission broker. No component may bypass the broker, grant itself permissions or self-assert an authorization identity.

The approved policy combines:

- **Hybrid RBAC + ABAC:** roles establish baseline access; contextual attributes and policy conditions determine whether a specific action is authorized.
- **Risk-based approval:** low-risk actions may run automatically when the applicable policy permits them; sensitive, consequential or destructive actions require confirmation.
- **User-wide permission scope:** a grant belongs to the individual user who issued it and may apply across that user's authorized contexts. It is not installation-wide and never transfers to another user.
- **Configurable lifetime:** grants may be permanent, session-scoped, task-scoped or time-limited, with expiry and revocation.

## 23.2 Mandatory security boundaries

- Every privileged operation is evaluated using trusted identity and context: requesting component, user, workspace where applicable, target resource, action, risk and applicable policy conditions.
- User-wide grants do not override data ownership, workspace membership, resource-level restrictions or other security boundaries.
- A user's grant never authorizes access to another user's private data or grants privileges to another user.
- More restrictive applicable resource, workspace, application or policy restrictions take precedence over a broader user-wide grant.
- Deny access by default when identity, scope or authorization is unclear.
- Background and delayed tasks retain their initiating user's identity, scope and authorization context; revalidate permissions when appropriate before execution.
- Permission records must be understandable, revocable and subject to expiry.
- Consequential actions receive an appropriate preview before execution.
- Provide audit history, individual action/task cancellation and an emergency stop.
- Restrict access to audit records and minimize unnecessary sensitive content in them.

Risk-based autonomy remains configurable. Users may choose stricter ask-before-every-action behaviour.

## 23.3 Still open

The policy engine, permission storage format, authentication/session technology, risk-classification implementation, approval UI, audit storage and concrete enforcement implementation remain undecided.


**END OF MASTER PLAN**

# 24. APPROVED ARCHITECTURE: MODULAR CORE AND SELECTIVELY ISOLATED WORKERS

Approved on 22 November 2026. This establishes the architectural direction and process-separation criteria, not a final process topology or technology selection.

## 24.1 Overall architecture

Use a **modular core with selectively isolated processes**. Avoid both a single monolithic process for every workload and a broad microservices architecture that adds unnecessary operational and resource overhead.

The main core owns orchestration, authorization, resource policy and shared platform state. Keep components as core modules unless a concrete lifecycle, security, reliability or runtime requirement justifies process separation.

## 24.2 Process-separation criteria

A component should become a separate process when at least one of these applies:

1. It needs an independent lifecycle or resource-management boundary.
2. It handles untrusted code or sensitive operations requiring stronger isolation.
3. Its failure must not crash or destabilize the main core.
4. It requires a specialised runtime, hardware access or dependency environment.

Otherwise, keep it inside the core as a module.

Process separation alone is not a security sandbox. Untrusted workers require actual OS-enforced restrictions and controlled interfaces.

## 24.3 Initial process-boundary direction

The following is a starting baseline; individual boundaries may be refined as workloads and platform constraints are researched.

| Component | Initial direction |
|---|---|
| Main orchestrator | Main core |
| Permission broker | Main core |
| Resource manager | Main core |
| Memory and knowledge | Main core initially |
| Event system | Main core initially |
| Desktop UI | Separate client process |
| Inference runtime | Independent process |
| Specialist agents | On-demand, bounded worker pool |
| Plugins | Restricted isolated processes |
| Browser/desktop automation | Isolated workers |
| Voice processing | On-demand worker |

## 24.4 Operational constraints

- Worker failures must not bring down the main core; workers should be restartable where appropriate.
- Specialist workers are on-demand and bounded rather than permanently spawned without need.
- Share inference runtimes where practical and avoid unnecessary duplicate model instances.
- Worker execution remains subject to the central permission broker and resource manager.
- The exact process topology and platform-specific lifecycle remain open for further design.

## 24.5 Still open

- Exact process topology and module boundaries.
- IPC transport and worker communication protocol.
- Sandbox and OS-enforcement technologies.
- Service managers and platform-specific lifecycle implementation.
- Worker restart, health-check and failure-recovery policy.


# 25. APPROVED ARCHITECTURE: GAMING-MODE RESOURCE PROTECTION

Approved on 22 November 2026. Gaming Mode prioritizes the game while retaining an optional minimal CPU-only assistant.

## 25.1 Approved behaviour

- Avoid assistant GPU inference and release assistant GPU/model allocations when necessary, keeping the GPU available to the game.
- Optionally run a very small CPU-only model, initially targeting approximately 0.4B–0.8B parameters. This is a target range, not a final model selection.
- Retain basic web search and selected lightweight tools where practical.
- Enforce strict, adaptive CPU and RAM budgets to minimize competition with the game.
- Pause or defer nonessential indexing, maintenance, model downloads and background automations.
- Restore normal resource scheduling and model availability when Gaming Mode ends.

## 25.2 Reliability and tool-use requirements

A tiny model may be unreliable at choosing tools or producing structured calls. Use deterministic tool routing, strict input/output validation and the central permission broker; never treat the model as an authorization authority.

Resource budgets must adapt to actual machine and game load rather than assume one fixed safe allowance. Mode transitions must be recoverable, including when a game exits unexpectedly.

## 25.3 Still open

- Exact CPU and RAM budgets.
- CPU model/runtime and final model selection.
- Lightweight tool allowlist and web-search implementation.
- Game detection and mode-transition implementation.
- Hardware-specific validation and performance thresholds.
