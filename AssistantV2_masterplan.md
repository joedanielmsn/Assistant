# LOCAL ASSISTANT 2.0 — MASTER PROJECT PLAN

**Version:** 1.0  
**Last updated:** 22 November 2026  
**Status:** Requirements discovery completed; architecture discovery in progress. Deployment profiles, multi-user direction, data ownership, central permission-broker policy, modular-core/selective-worker architecture and core/client communication and adaptive resource-management direction approved; implementation technologies remain unselected.

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
- Concrete IPC path/ACL configuration, authentication/session implementation and remote-access integration.
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


# 26. APPROVED ARCHITECTURE: CORE/CLIENT COMMUNICATION, IPC AND REMOTE ACCESS

Approved on 22 November 2026. This establishes the communication architecture and security boundaries, not the concrete server library or authentication implementation.

## 26.1 One authoritative core and common client API

The platform uses one authoritative core as its control and security boundary. Desktop interfaces, CLI clients, browser interfaces, companion surfaces and future clients communicate with the core through a versioned, language-neutral API.

Clients must not access private stores, privileged tools or protected resources directly. The core establishes trusted identity and workspace context, evaluates authorization and routes approved operations.

## 26.2 Communication model

- Use a versioned HTTP-compatible API for structured request/response operations.
- Use a WebSocket event channel for streaming responses, task progress, state changes and notifications.
- Keep the API contract independent of a particular programming language or transport implementation.
- Support explicit API version negotiation and capability discovery so clients can handle incompatible versions and unsupported features safely.
- Internal worker communication may use a specialised transport if measured requirements justify it, without weakening identity or authorization boundaries.

## 26.3 Platform-specific local IPC

- **Linux:** Unix domain sockets, with OS peer-identity checks and restrictive socket permissions.
- **Windows:** named pipes, with explicit access-control configuration and client identity checks.
- Local IPC is not implicitly trusted. A successful local connection is not blanket permission to access data or perform actions; establish an authenticated client session and authorize every operation.

## 26.4 Remote access and authentication

- Remote access is disabled by default.
- Users may explicitly enable LAN access or secure remote access through a user-configured VPN or tunnel.
- Shared-server deployments use built-in accounts with passkey support, with optional external identity-provider integration.
- Authentication establishes identity; it does not itself grant resource or action permissions.
- Remote endpoints must be securely authenticated. Unauthenticated development endpoints must never be exposed as remote services.

## 26.5 Worker and plugin identity

Worker and plugin identities are constrained by the core. They cannot impersonate users, self-assert arbitrary workspace context or bypass the central permission broker. Every operation retains trusted user, workspace and component context as applicable.

## 26.6 Still open

- Concrete HTTP server/library, WebSocket implementation and API schema format.
- Socket/pipe paths, access-control details and platform-specific lifecycle.
- Passkey implementation, external identity-provider protocols and session/token format and lifecycle.
- LAN exposure controls and VPN/tunnel integration.
- API compatibility/version-support policy.
- Whether any internal worker protocol should differ from the client API.


# 27. APPROVED ARCHITECTURE: ADAPTIVE RESOURCE MANAGEMENT AND WORKLOAD SCHEDULING

Approved on 22 November 2026. This establishes resource policy and scheduling direction, not final budgets, algorithms or implementation technologies.

## 27.1 Scheduling strategy

Use a hybrid strategy: enforce fixed safety boundaries while dynamically scheduling workloads according to priority, current resource pressure and available capacity. The resource manager coordinates CPU, RAM, GPU and VRAM rather than treating them as independent systems.

## 27.2 User-facing performance modes

Provide four built-in modes:
- **Normal:** balanced everyday operation.
- **Performance:** favour responsiveness and active user-requested work while respecting system safety.
- **Gaming:** follow the aggressive game-priority behaviour in Section 25.
- **Battery/Quiet:** reduce background activity and favour lower power/noise where the platform exposes suitable controls.

Automatic adaptation should complement these modes without silently overriding explicit user preferences or safety limits.

## 27.3 Model lifecycle

Retain frequently used models when resources permit. Unload idle models when memory pressure rises, and support configurable idle timeouts. Avoid duplicate model instances where runtimes can be shared. Model lifecycle decisions should consider memory pressure, expected reuse, workload priority and the cost of reloading.

## 27.4 Multi-user fairness

Use weighted fair scheduling per user, individual concurrency limits and priority for interactive tasks. Prevent one user's workloads from monopolizing shared resources while avoiding indefinite starvation of lower-priority work.

## 27.5 Resource-management requirements

- Monitor CPU, RAM, GPU and VRAM availability and pressure.
- Use adaptive scheduling within fixed safety limits.
- Prioritize foreground responsiveness and active user-requested tasks.
- Reduce, pause or defer nonessential indexing, maintenance, downloads and automations under pressure.
- Apply platform-native resource controls through platform adapters.
- Keep OS-specific mechanisms separate from the core scheduling policy.
- Preserve the approved Gaming Mode requirements in Section 25.
- Validate resource behaviour on constrained hardware and under simultaneous foreground workloads.

## 27.6 Still open

- Exact CPU/RAM/GPU/VRAM safety limits and mode-specific budgets.
- Scheduling weights, queue algorithms and adaptation thresholds.
- Model idle timeouts and GPU preemption/unload behaviour.
- Power and thermal policy.
- Linux and Windows resource-control APIs and concrete implementation.
- Hardware-specific performance and fairness validation.


# 28. APPROVED ARCHITECTURE: COMPUTER INTERACTION AND TOOL EXECUTION

Approved on 22 November 2026. This establishes the interaction strategy and execution safeguards, not specific tool libraries or sandbox technologies.

## 28.1 Layered interaction strategy

Prefer the most reliable available interface in this order:

1. Native application APIs and structured integrations.
2. Operating-system accessibility interfaces.
3. Browser automation using semantic controls and locators.
4. Visual screen understanding with keyboard and pointer interaction as a fallback.

This avoids making fragile coordinate-based automation the default.

## 28.2 Common tool interface

Use a common internal tool contract with adapters for external integration protocols such as MCP. MCP is supported, but it is not required to be the sole internal tool interface.

Tool contracts should include validated input/output schemas, capability declarations, timeouts, cancellation, execution identifiers and auditable outcomes.

## 28.3 Host access and permission enforcement

The user selected broad host access instead of a fixed folder allowlist. Broad access is not ambient or unrestricted access.

- Every file, process, terminal and system operation must be evaluated by the central permission broker.
- Every operation carries trusted user, workspace, component, target-resource and action context.
- User-wide grants remain bounded by ownership, workspace membership, resource restrictions and other applicable policies.
- Terminal requests must be structured, validated and attributable; previews and confirmation follow the approved risk policy.
- Execute sensitive or untrusted work in isolated, restartable workers with OS-enforced restrictions and resource limits where appropriate.
- Validate paths and targets at execution time.
- Distinguish read-only operations from modifications and execution.
- Treat tool output, web content and file contents as untrusted data, never as instructions that can override platform policy.

## 28.4 Browser identity

Use a separate assistant-managed browser profile by default. Connecting to the user's existing browser profile is a separate, explicitly authorized capability.

## 28.5 Recovery

Use operation-level undo where supported and checkpoints for consequential or bulk changes. The UI and action history must clearly identify operations that cannot be reliably undone.

## 28.6 Still open

- Tool schema and capability-manifest details.
- MCP adapter design.
- Terminal command policy and approval thresholds.
- Worker sandbox and OS-enforcement technologies.
- Linux/Windows accessibility and browser automation libraries.
- Concrete undo and checkpoint mechanisms.

---

# 29. APPROVED ARCHITECTURE: DESKTOP EXPERIENCE, AWARENESS AND VOICE

Approved on 22 November 2026. This establishes user-facing behaviour and privacy requirements, not a UI framework or speech technology selection.

## 29.1 Unified desktop application

Provide one primary desktop application with optional dashboard, tray controls, notifications and floating companion surfaces. These remain clients/views of the authoritative core, not independent authorities or permission systems.

## 29.2 Continuous desktop awareness

The user selected continuous screen awareness while the assistant is running, with a user-configurable opt-out.

Continuous awareness is not permission for covert recording. Mandatory safeguards:

- Display a persistent, unmistakable indicator whenever screen content is captured or analysed.
- Provide immediate pause and disable controls.
- Support application/window exclusions and configurable capture scope.
- Default to transient processing; do not silently retain screen recordings or captured frames.
- Require separate explicit permission and retention choices for screenshot storage, persistent extraction or sharing.
- Respect operating-system capture permissions and privacy controls.
- Prefer local processing where practical; disclose and obtain permission before sending screen content to cloud services.
- Respect protected OS surfaces such as lock screens and secure desktops.
- Make application metadata awareness and screen-content awareness separately configurable.

## 29.3 Voice

Push-to-talk is the default. Wake-word activation is optional and must be explicitly enabled. Provide visible microphone/listening status, immediate mute and disable controls, local-first processing where practical, and no default permanent recording or retention of raw audio.

## 29.4 Accessibility and notifications

Support keyboard navigation, clear focus handling and screen-reader accessibility. Notifications must be configurable, respect quiet hours and avoid excessive interruptions.

## 29.5 Still open

- Desktop/UI framework and application shell.
- Tray, overlay and notification implementation.
- Screen-capture adapters, indicator design and platform privacy integration.
- Wake-word engine and speech recognition/synthesis technologies.
- Audio and screen-content retention controls and detailed settings.



# 30. APPROVED ARCHITECTURE: INTELLIGENCE AND ORCHESTRATION

Approved on 22 November 2026. This establishes orchestration behaviour and safety boundaries, not concrete models, libraries or algorithms.

## 30.1 Central orchestrator

Use a central, stateful orchestrator with bounded adaptive execution. It owns the overall task plan, execution state, specialist delegation, recovery and final result. Specialist agents are on-demand bounded workers and do not independently control the assistant.

## 30.2 Planning and execution

- Classify requests, plan work, select models/tools/specialists, coordinate execution, verify outcomes and report completion.
- Plans may adapt to new information within explicit step, time, token and resource limits.
- Sequential execution is the default. Parallelize independent work only when materially useful and within resource-manager limits.
- Specialist contracts define scoped inputs, authorized context, structured outputs, evidence, status and errors.
- Model outputs can propose plans and tool calls, but deterministic schemas, validation, authorization and the central permission broker control execution. Models are never security authorities.
- Route models based on capability, complexity, latency, privacy/cloud policy and available resources. Preserve approved model lifecycle and Gaming Mode behaviour.
- Persist task state and checkpoints for interruption and safe resumption. Retries must account for operations that may have completed despite timeouts; do not blindly repeat non-idempotent actions.
- Support cancellation, bounded retries, worker-failure isolation and useful failure reporting.
- Evaluate task completion, tool selection and argument correctness, unnecessary calls, verification, recovery, latency and resource consumption.
- Preserve trusted user/workspace identity and authorization context across delegation and tool execution.

## 30.3 Still open

Model-routing algorithms, planner implementation, task-state schema, retry/idempotency mechanisms, specialist taxonomy, evaluation datasets/benchmarks, model/runtime choices and orchestration libraries.


# 31. APPROVED ARCHITECTURE: MEMORY AND KNOWLEDGE

Approved on 22 November 2026. This establishes information layers, governance and retrieval boundaries, not concrete databases or retrieval technologies.

## 31.1 Layered information model

Keep memory distinct from source knowledge repositories:
- **Working memory:** temporary current-conversation and active-task context.
- **Episodic memory:** useful summaries of past interactions and experiences.
- **Semantic memory:** confirmed facts, preferences and relationships.
- **Procedural memory:** reusable workflows and methods.
- **Knowledge repositories:** source documents, code, project files and reference material, indexed separately from personal memory.

## 31.2 Memory governance

- Conversation history does not automatically become permanent memory.
- Use review-based writes by default: identify useful candidates and ask the user to approve persistent saving. Separately approved automatic rules may be added later.
- Preserve provenance, confidence, timestamps and temporal validity where applicable.
- Reconcile or version conflicting facts and corrections rather than silently overwriting established information.

## 31.3 Project knowledge and retrieval

- Index user-selected project locations and monitor only those approved locations for changes. Never silently expand monitoring scope.
- Original source documents remain authoritative; indexes, embeddings and summaries are derived and rebuildable.
- Apply trusted user/workspace authorization before retrieved content is exposed to a model. Keep results, metadata, caches and derived summaries within their authorized boundaries.
- Treat retrieved documents, memories and tool outputs as untrusted data, not instructions or sources of authorization.
- Propagate updates, corrections, revocation and deletion to derived indexes, caches and summaries, subject to documented backup retention.
- Use keyword, semantic and reranking methods according to measured quality improvements and their latency/resource costs.
- Construct bounded, task-specific context from only relevant authorized information.

## 31.4 Still open

Memory schemas, database and vector/index technologies, embedding and reranking models, retrieval algorithms, conflict-resolution implementation, retention periods, deletion guarantees, project watcher mechanisms and review interface.



# 32. APPROVED ARCHITECTURE: AUTOMATION AND PROACTIVE WORKFLOWS

Approved on 22 November 2026. This establishes workflow behaviour and governance, not a scheduler or workflow-engine selection.

## 32.1 Event-driven durable workflows

Use event-driven automation integrated with the central orchestrator, resource manager and permission broker. Prefer explicit events, user requests and scheduled triggers over continuous polling or unnecessary model inference. Persist workflow state, scheduled waits, checkpoints and approval pauses so workflows can survive restarts.

## 32.2 Ownership, safety and resource governance

- Every automation has a trusted owner and user/workspace scope. Shared workflows require explicit workspace ownership and policy.
- Background execution retains initiating identity, scope and authorization context; revalidate permissions before delayed or consequential actions when appropriate.
- Bound execution time, steps, retries, concurrency and resource consumption.
- Provide previews, approval checkpoints, pause, cancellation, execution history and actionable failure reporting.
- Use idempotency keys, deduplication or equivalent safeguards for retried external operations. Do not promise exactly-once effects where external systems cannot guarantee them.
- Defer nonessential automation during Gaming Mode and resource pressure.
- Learned workflows may be proposed for reuse, but persistent saving and activation remain subject to memory-governance and user-review rules.

## 32.3 Still open

Workflow definition format, scheduler, event transport, persistence schema, trigger catalogue, retry policy details and automation UI.


# 33. APPROVED ARCHITECTURE: PLUGIN AND INTEGRATION ECOSYSTEM

Approved on 22 November 2026. This establishes ecosystem openness and security requirements, not a registry, package format or sandbox technology.

## 33.1 Open ecosystem and trust

Provide a public community registry and support third-party and locally developed extensions. Plugins declare identity, publisher, version, dependencies, capabilities, network needs, resource requirements and requested permissions through a manifest.

Support publisher verification, package integrity checks, signatures and provenance tracking. Clearly distinguish verified, community and locally developed/unverified extensions; verification is not a guarantee that software is harmless.

## 33.2 Execution and permission enforcement

- Plugin installation does not grant blanket access. Every privileged operation is evaluated by the central permission broker under user grants and all applicable ownership/workspace boundaries.
- Run plugins in restricted, isolated and restartable workers with OS-enforced controls appropriate to their capabilities; process separation alone is not a sandbox.
- Use the common internal tool contract with adapters for external protocols such as MCP. MCP is supported but is not the sole internal plugin/tool architecture.
- Scope integration credentials to the relevant service, user/workspace and capability. Protect secrets from unrelated plugins and workers.
- Review material permission, dependency and capability changes during updates; preserve version/provenance and provide rollback where feasible.
- Disabling or uninstalling a plugin revokes its credentials and grants and cancels or detaches scheduled tasks and event subscriptions.
- Treat plugin output and package content as untrusted data, not instructions or sources of authority.

## 33.3 Still open

Manifest schema, registry governance, signing infrastructure, package format, sandbox technologies, update/rollback mechanics, plugin SDK, compatibility policy and moderation operations.


# 34. APPROVED ARCHITECTURE: SYNCHRONIZATION, BACKUP AND PORTABILITY

Approved on 22 November 2026. This establishes local-first behaviour and data-protection boundaries, not concrete sync or cryptographic protocols.

## 34.1 Synchronization

- Keep the platform fully useful locally and offline. Sync is optional.
- Support both an optional managed synchronization service and self-hosted synchronization using the same interoperable architecture.
- Encrypt private data on authorized devices before transmission so the sync service does not need plaintext access.
- Keep private user data and explicitly shared workspace data under distinct authorization and key boundaries. Workspace sharing requires explicit membership and controlled key distribution.
- Support device enrollment, device revocation and documented recovery procedures for lost devices or keys.
- Automatically merge safe structured changes where reliable; preserve version history and surface meaningful conflicts for user resolution. Avoid timestamp-only last-write-wins when it could silently discard meaningful edits.
- Preserve ownership, provenance, versioning and authorization metadata throughout synchronization.

## 34.2 Backup and recovery

- Provide encrypted, versioned backups with configurable retention and multiple destinations, including optional offline/offsite copies.
- Protect backup integrity and deletion controls; verify backup integrity and regularly test restoration.
- Support selective restoration where feasible and report conflicts or unrecoverable data.
- Distinguish live-data deletion from eventual expiration in retained backups and document limitations honestly.

## 34.3 Export and import

- Provide user-readable exports separately from complete backup/migration archives.
- Define documented, versioned formats for conversations, memory, projects, settings, automation definitions, workspace data and relevant attachments/metadata.
- Support full migration and selective import, with reports for unsupported or omitted data.
- Imported plugins and executable automation remain inactive and untrusted until reviewed and explicitly authorized. Importing data never silently grants permissions or activates code.

## 34.4 Still open

Sync protocol, managed-service operations, cryptographic protocols and key recovery design, backup formats and retention periods, export schemas, migration tooling, conflict-resolution algorithms and deletion guarantees.


# 35. APPROVED ARCHITECTURE: AI-FIRST CACHYOS-DERIVED LINUX DISTRIBUTION

Approved on 22 November 2026. This establishes the distribution's foundation, product identity, desktop direction, AI integration philosophy and update strategy. It does not select every package, installer component or implementation technology.

## 35.1 Foundation and independent identity

Build the distribution on CachyOS as its technical foundation, selectively reusing useful upstream infrastructure, packages, kernel work, hardware support and installation tooling. The result must be an independently designed and maintained distribution, not a simple CachyOS reskin.

Create an independent name and brand, visual language, desktop defaults, settings experience, system utilities and workflows. Maintain our own repositories, build and testing pipeline, release identity, documentation and distribution-level quality control. CachyOS updates are upstream inputs, not automatic approvals for our releases.

## 35.2 Desktop experience

Use KDE Plasma as the initial desktop foundation, with a substantially redesigned and integrated experience. Keep the architecture modular enough to replace or extend desktop components later. The goal is an original, cohesive desktop product without taking on the full cost of developing a new desktop environment from scratch at the outset.

## 35.3 AI-first integration

Assistant integration is a first-class part of the operating system experience from the beginning. The distribution is deliberately AI-first; users who do not want an AI-focused operating system can choose CachyOS itself.

Integrate assistant capabilities into desktop and system workflows rather than treating the assistant as an unrelated optional application. AI services may be paused or disabled without breaking ordinary operating-system and desktop functionality.

System integration does not grant unrestricted administrative authority. Assistant actions remain subject to trusted identity, the central permission broker and resource manager. A per-user assistant service remains the default; privileged system components must be narrowly scoped to necessary OS integration.

## 35.4 Update and release strategy

Use a curated rolling-release model. Maintain our own testing channel and promote validated updates to our stable channel. Do not automatically ship every CachyOS upstream update without distribution-level validation.

Prefer upstream components and minimise long-lived custom patches where practical. Maintain our own package repositories, build pipeline, release process and quality gates.

## 35.5 Distribution requirements

- Developer productivity and gaming are first-class use cases.
- Laptop hardware compatibility, graphics support and power management require deliberate validation.
- Installation, recovery and system rollback are core product capabilities.
- The OS remains useful without AI inference, cloud access or a hosted account.
- No mandatory telemetry or cloud services.
- Hardware optimisations must not make the distribution unusable on ordinary supported hardware.

## 35.6 Still open

Distribution name and branding, exact CachyOS components to adopt or replace, repository/build infrastructure, package promotion and testing policy, desktop-shell modifications, kernel variants, installer configuration, filesystem/partitioning defaults, Secure Boot, hardware profiles, recovery tooling, update/rollback mechanics and release support policy.


# 36. APPROVED ARCHITECTURE: PRIVILEGED SYSTEM OPERATIONS

Approved on 22 November 2026. The central permission broker is the authoritative authorization and approval layer for assistant-initiated privileged system operations. A second interactive Polkit/UAC authorization prompt is not required for every operation.

- The ordinary assistant core and model workers do not run with broad root/administrator privileges.
- Privileged execution is delegated to a narrowly scoped OS helper exposing only predefined, structured operations with validated arguments and targets.
- The helper must not expose arbitrary shell execution, unrestricted administrator/root access or a general-purpose privilege escalation interface.
- Plugins, specialists, automations and models cannot invoke the helper directly or bypass the central broker.
- OS-enforced process isolation, access controls and least-privilege boundaries remain mandatory. Only the duplicate interactive authorization layer is omitted.
- If trusted identity, broker policy evaluation or authorization is unavailable or ambiguous, the operation is denied.
- The broker records authorization decisions and execution outcomes under the applicable audit and data-minimization policies.

The exact helper architecture, operation catalogue, platform-specific implementation, privilege lifecycle, IPC protocol and audit storage remain open for implementation design.

# 37. APPROVED PLATFORM SCOPE: X86-64 ONLY

The initial supported hardware architecture for Local Assistant 2.0 and its associated desktop, server and AI Linux distribution deliverables is **x86-64 (AMD64) only**.

- Official builds, installation media, packaging, testing and support target x86-64 systems.
- ARM64 and other CPU architectures are outside the initial support commitment.
- Keep architecture-dependent implementation details behind portable interfaces where practical, without treating future portability as a current support promise.
- CPU architecture support is separate from GPU and device compatibility; the x86-64 target does not guarantee support for every graphics device or peripheral.
- The minimum CPU instruction-set baseline, processor generations and GPU/accelerator compatibility matrix remain to be researched.

See ADR-018 in [ARCHITECTURE_DECISIONS.md](./ARCHITECTURE_DECISIONS.md).


# 38. APPROVED CPU BASELINE: X86-64-V3

The initial supported x86-64 platform requires **x86-64-v3** CPU instruction-set compatibility.

- x86-64-v3 is the minimum baseline for official builds and supported systems.
- x86-64-v4 may be used for optional optimized builds or runtime-dispatched code on compatible processors; it is not mandatory.
- Older x86-64 processors that do not meet the v3 baseline are outside the initial support commitment.
- Provide compatibility checks and clear diagnostics before installing or running architecture-specific packages where practical.
- Keep architecture-specific optimizations separated from portable source code so future targets can be evaluated independently.

This is a CPU instruction-set requirement, not a guarantee of support for every GPU, peripheral or laptop configuration. See ADR-019 in [ARCHITECTURE_DECISIONS.md](./ARCHITECTURE_DECISIONS.md).


# 39. APPROVED DESKTOP APPLICATION FRAMEWORK: QT 6 + QML

Use **Qt 6 with QML/Qt Quick** for the primary cross-platform desktop application.

- Share the desktop application codebase across Linux and Windows where practical.
- Keep the desktop app as a client of the authoritative assistant core; identity, authorization, task state and resource policy remain core responsibilities.
- Use Qt/QML for the primary desktop experience, with optional dashboard, tray controls, notifications and floating companion surfaces.
- Integrate naturally with the KDE Plasma foundation while retaining a viable Windows experience.
- Keep OS-specific integrations behind platform adapters.
- Preserve accessibility, keyboard navigation, screen-reader support and visible privacy/awareness indicators.

Exact Qt modules, QML architecture, build system, packaging and platform-specific UI integration remain implementation-planning decisions. See ADR-020 in [ARCHITECTURE_DECISIONS.md](./ARCHITECTURE_DECISIONS.md).

## 39.1 Proposed implementation stack — not yet approved as a complete stack

The following are current technology candidates for evaluation, not final commitments:

| Area | Candidate |
|---|---|
| Authoritative core | Rust + Tokio |
| AI/specialist workers | Python |
| LLM inference | llama.cpp |
| Specialized inference | ONNX Runtime |
| Personal desktop persistence | SQLite |
| Shared server persistence | PostgreSQL |
| Initial local full-text search | SQLite FTS5 |
| Desktop application | Qt 6 + QML (approved) |
| API and streaming | Versioned HTTP-compatible API + WebSockets |
| Initial event/workflow engine | Internal typed event system + durable database state |

The overall stack must be validated for cross-platform support, security boundaries, hardware/resource efficiency, maintainability and ecosystem maturity before the remaining candidates are approved.

## 40. Approved initial technology architecture — 22 November 2026

The initial core/worker/runtime technology direction is approved and recorded in ADR-021.

| Layer | Approved direction |
|---|---|
| Authoritative core | Rust + Tokio |
| AI/specialist workers | Separate supervised Python processes |
| Desktop UI | Qt 6 + QML (ADR-020) |
| Local LLM inference | llama.cpp |
| Specialized inference | ONNX Runtime, with per-platform/provider validation |
| Personal desktop database | SQLite |
| Shared-server database | PostgreSQL |
| Initial local full-text search | SQLite FTS5 |
| Rust database access | SQLx as the initial candidate, subject to validation |
| Client API | Versioned HTTP-compatible API + WebSockets |
| Worker IPC | Versioned JSON over supervised local IPC |
| Portable plugin sandbox | WebAssembly with Wasmtime |
| Native integration isolation | OS-restricted workers; Bubblewrap/kernel controls on Linux and AppContainer where compatible on Windows |
| Workflows/events | Internal typed event system and durable database-backed workflow engine |

### Architecture boundaries

- Rust owns authoritative state, orchestration, identity, authorization, permission enforcement and resource policy.
- Python workers provide specialist capabilities only; they cannot establish identity, authorize actions or bypass the broker.
- Worker failures must remain isolated from the core.
- All inference backends sit behind a common internal contract and are managed centrally.
- Plugin sandboxing and native-worker isolation are separate execution paths; process separation alone is not sufficient.
- All privileged operations remain subject to the central permission broker.

These are approved technology directions, not completed implementation specifications. Exact versions, schemas, process lifecycle, sandbox profiles, GPU compatibility, database design, encryption, packaging, build/release engineering and licensing still require detailed work and validation.

Licensing remains intentionally deferred for architecture finalisation and public-release review.

## 41. Approved repository, build and release architecture — 22 November 2026

ADR-022 establishes the initial build and repository direction.

### Repository structure

- **Platform monorepo:** Rust core, Qt 6/QML desktop, Python workers, official plugins/SDK, shared interfaces, platform packaging and developer documentation.
- **Separate Linux distribution repository:** CachyOS-derived image/installer configuration, distribution-specific kernel/desktop changes, package definitions, repository configuration and release channels.

The platform and OS share versioned components and interfaces but can be built, tested and released independently.

### Build and CI direction

| Component | Initial tooling |
|---|---|
| Rust | Cargo workspace |
| Qt 6/QML | CMake + Ninja |
| Python workers | uv with locked environments |
| Linux distribution | Arch/CachyOS-compatible package tooling and PKGBUILDs |
| CI/CD | GitHub Actions, including Linux/Windows matrices and separate hardware validation |

Use fast pull-request checks, full platform integration builds, controlled GPU validation and reviewed/tagged release builds. Ordinary CI must not depend on GPU availability.

### Packaging and release direction

- Native Linux packages are the primary desktop/distribution integration path.
- The distribution maintains its own signed repositories and controlled testing/stable promotion.
- Windows packages include the required Qt deployment dependencies and separately managed core/worker components where practical.
- Python workers use pinned, isolated environments.
- Platform and distribution release cadences remain independent.
- Updates require compatibility checks, staged rollout, failure reporting and documented recovery/rollback.
- Release artifacts should be traceable to reviewed source revisions and dependency/toolchain inputs.

Exact directory layout, workflow definitions, package formats, signing operations, installer choice, versioning and rollback mechanics remain detailed design tasks.
