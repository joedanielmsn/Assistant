# Local Assistant 2.0 — Data Classification and Threat Register

**Status:** Phase 0 security-planning baseline  
**Date:** 2026-11-22  
**Authority:** ADR-003, ADR-004, ADR-009, ADR-010, ADR-017, ADR-024, ADR-025, ADR-034 and ADR-035  
**Important:** This register describes required controls and tests. It is not evidence that the controls have already been implemented or independently certified.

## 1. Classification model

Classification describes sensitivity; ownership scope describes who is allowed to access an item. Both must be evaluated. A low-sensitivity item in a private user scope is still not automatically shareable.

| Class | Examples | Default handling |
|---|---|---|
| C0 — Public | Public documentation, intentionally public plugin metadata, public release notes | May be disclosed publicly after normal review |
| C1 — Internal | Non-secret operational configuration, non-sensitive component metadata, internal diagnostics without personal content | Local/project access only; disclose only where intended |
| C2 — Private | Conversations, personal preferences, personal memory, project documents, workspace content | Encrypted at rest where stored; owner/workspace authorization required; no automatic cloud disclosure |
| C3 — Sensitive | Screen-derived content, voice-derived content, private credentials metadata, sensitive project material, security audit details | Narrow purpose and scope; explicit capability/policy; minimized retention; stricter cloud restrictions |
| C4 — Secret material | Passwords, API tokens, session credentials, encryption keys, recovery keys, signing keys | Never expose to models, plugins or ordinary logs; protected secret/key stores; narrowly scoped access and rotation/recovery controls |

### Classification rules

- Assign a classification when data enters the system or is derived from existing data.
- Derived summaries, embeddings, indexes, caches and exports inherit the source's classification and ownership restrictions unless a reviewed transformation demonstrably changes them.
- When classification is unknown, treat the data as the more restrictive plausible class until resolved.
- Classification never grants access. Authorization must independently evaluate subject, resource, scope, action, component and conditions.
- User/workspace ownership and sensitivity are separate dimensions.
- A workspace share is an explicit authorization change, not a classification downgrade.
- Screen, voice and credential content are excluded from cloud requests by default and require separate explicit authorization.
- Logs and diagnostics must avoid C2–C4 content unless a narrowly justified, user-visible diagnostic export explicitly includes a redacted item.

## 2. Data handling matrix

| Data category | At rest | In transit | Model/worker access | Logs/diagnostics | Cloud |
|---|---|---|---|---|---|
| Public | Integrity-protected as appropriate | TLS for remote transport | May be used when task-relevant | Minimal metadata allowed | Allowed by configured policy |
| Internal | Access-controlled | Authenticated transport | Need-to-know only | Minimize | Only under configured provider/scope policy |
| Private | Encrypted storage and scoped keys | Authenticated encrypted transport | Only authorized task context | No content by default | Local-only by default; scope policy and user authorization required |
| Sensitive | Encrypted, minimized and purpose-bound | Authenticated encrypted transport | Explicit capability and bounded purpose | Excluded by default; preview/redact before export | Blocked by default; separate explicit authorization |
| Secret material | Dedicated protected secret/key storage | Never sent unless a protocol specifically requires protected secret handling | Not available to models/plugins; narrow broker/runtime access only | Never log | Never include in inference prompts or diagnostic bundles |

## 3. Scope and lifecycle requirements

Every persisted or transient object must carry, directly or through trusted metadata:

- Owner identity, where applicable.
- Workspace identity, where applicable.
- Installation/user/workspace/task-session scope.
- Classification and provenance.
- Creation/update time and retention policy.
- Authorization context for access and sharing.
- Derived-from references where practical.
- Deletion, revocation and expiry state where applicable.

Background tasks retain the initiating owner's identity, scope and authorization context. Delayed actions revalidate permissions when required. Deletion and revocation must account for derived indexes, caches, summaries, temporary files and scheduled work, with backup retention limitations documented.

## 4. Threat register

| ID | Threat / abuse case | Primary assets and boundary | Required controls | Verification evidence | ADRs |
|---|---|---|---|---|---|
| TH-001 | Malicious plugin attempts filesystem, process, network or credential access beyond its grant | Plugin worker ↔ host OS/core | Capability declarations, Wasmtime or OS-enforced sandbox, broker mediation, least privilege, revocation | Sandbox conformance and negative capability tests | 004, 014, 024, 029 |
| TH-002 | Compromised specialist worker forges identity or requests privileged operations directly | Worker ↔ core/broker/helper | Core-issued trusted context, structured protocol, broker-only privileged path, worker isolation | Forged-context, direct-helper and broker-bypass tests | 004, 005, 017, 021, 024 |
| TH-003 | Prompt injection in web pages, files, documents or tool output attempts to override policy | Untrusted content ↔ model/orchestrator | Treat retrieved content as data, separate instructions/policy, schema validation, deterministic broker enforcement | Adversarial prompt-injection tests; prove no authorization changes | 009, 011, 012, 024 |
| TH-004 | User accesses another user's or workspace's private data | User/client ↔ storage/API/worker/cache | Trusted identity/scope propagation, per-resource authorization, scoped keys, RLS defence in depth | Cross-user/workspace tests across APIs, caches, indexes, backups and workers | 002, 003, 004, 025, 032 |
| TH-005 | Server administrator or maintenance process accesses private user plaintext | Admin boundary ↔ private data | Separate admin/data roles, envelope encryption, scoped keys, audited break-glass policy if ever introduced | Admin-role negative tests; inspect ordinary admin APIs and storage access | 002, 003, 025 |
| TH-006 | Local process impersonates a desktop client or connects to the core socket/pipe | Local OS process ↔ IPC endpoint | Restrictive socket permissions/pipe ACLs, peer identity, scoped client sessions, per-operation authorization | Unauthorized-user and endpoint-substitution tests on Linux/Windows | 007, 030 |
| TH-007 | Privileged helper is abused as arbitrary root/admin execution | Core/broker ↔ privileged helper ↔ OS | Fixed structured operations, validated targets, narrow helper identity, no arbitrary shell, fail closed | Fuzzing, malformed-target tests, operation allowlist review | 004, 009, 017, 024 |
| TH-008 | Model or worker consumes unbounded CPU, RAM, GPU, VRAM or disk | Worker/model ↔ resource manager/host | Fixed safety limits, adaptive scheduling, concurrency caps, timeouts and termination | Resource-exhaustion tests and recovery benchmarks | 005, 006, 008, 011, 024 |
| TH-009 | Worker crash or timeout causes duplicate external side effects | Orchestrator ↔ worker/tool/external service | Durable task state, idempotency keys, action status reconciliation, bounded retry policy | Fault-injection tests around uncertain completion | 011, 013 |
| TH-010 | Stolen password, token or key exposes private data | Credential/key store ↔ core/storage/provider | Argon2id, envelope encryption, secret isolation, key rotation/revocation, protected OS storage | Wrong-key, tamper, revocation and recovery tests | 025, 034 |
| TH-011 | Screen/audio content is captured or retained without user awareness | Desktop capture/voice ↔ core/storage/cloud | Persistent indicators, pause/disable, exclusions, transient defaults, separate retention/cloud permissions | Capture-state, retention and cloud-block tests | 010, 034, 035 |
| TH-012 | Cloud provider receives private or sensitive data contrary to scope policy | Core/model router ↔ provider | Local-only default, sensitivity labels, provider allowlist, explicit scope authorization, disclosure | Network capture and policy tests for restricted content categories | 034, 035 |
| TH-013 | Malicious or compromised update/package/plugin replaces trusted code | Build/registry/update ↔ installed system | Pinned dependencies, signatures, provenance, SBOM, staged promotion, rollback and key custody | Signature-tampering, provenance and rollback tests | 014, 022, 028, 029 |
| TH-014 | Secure Boot or system update leaves device unbootable or weakens verification | Installer/update ↔ boot chain/firmware | Signed chain, preserved firmware keys, preflight, snapshots, known-good recovery, no silent disablement | Full install/update/failure/recovery test matrix | 016, 028, 031, 033 |
| TH-015 | Logs, crash reports or diagnostics disclose private content or secrets | Core/workers ↔ local logs/export | Data minimization, local-only diagnostics, redaction, explicit preview/export, no auto-upload | Log scanning and diagnostic-bundle inspection tests | 024, 025, 035 |
| TH-016 | Sync server, revoked device or stale key exposes private/workspace data | Device ↔ self-hosted sync ↔ other devices | End-to-end encryption, per-scope keys, device enrollment/revocation, key rotation, conflict-safe state | Protocol review, revoked-device and stale-key tests | 015, 025, 026 |
| TH-017 | Database, cache or index is copied or restored outside its intended owner/scope | Storage/backup ↔ application | Encryption, scoped keys, authorization-aware restore, lifecycle metadata and integrity checks | Restore under wrong identity, tamper and cross-scope tests | 003, 012, 025, 032 |
| TH-018 | Dependency, build runner or signing credential is compromised | Source/CI ↔ release artifacts | Lockfiles, protected workflows, isolated signing, SBOM, provenance, review and staged releases | CI secret-exposure tests and artifact provenance verification | 022, 028, 029 |
| TH-019 | Remote API is exposed unintentionally or accepts unauthenticated requests | Core API ↔ network | Remote disabled by default, authenticated sessions, TLS for configured remote transport, rate/resource limits | Port/listener scan and unauthenticated request tests | 007, 030, 034 |
| TH-020 | Telemetry or diagnostic SDK silently exports usage or content | Application ↔ external service | No telemetry policy, dependency review, egress review, explicit user-export diagnostics only | Dependency scan and controlled network egress tests | 023, 034, 035 |

## 5. Minimum security review checklist

For every new feature or integration, reviewers should answer:

1. What data does it read, derive, write or transmit?
2. What classification and ownership scope apply?
3. Which trusted identity and workspace context authorize it?
4. Does it cross a process, OS, network, storage or provider boundary?
5. Can untrusted input influence a tool call, path, query, command or privileged operation?
6. Does every privileged operation pass through the central broker?
7. What happens when identity, policy, sandboxing or storage is unavailable?
8. Can it leak data through logs, caches, indexes, crash output or diagnostics?
9. Does it add network access, telemetry, a hosted dependency or provider disclosure?
10. How are cancellation, revocation, deletion, backup and recovery handled?
11. What negative tests prove prohibited behaviour is denied?
12. What new abuse case or trust boundary must be added to this register?

## 6. Review cadence

- Review this register when a feature adds or changes a trust boundary.
- Revisit it before enabling plugins, computer interaction, cloud inference, sync and privileged OS integration.
- Run a full threat-model review before beta/public release.
- Track each finding to a test, mitigation, owner and closure evidence.
- Do not mark a threat as resolved solely because a design document describes a control; require implementation and test evidence.

## 7. Phase 0 completion criteria

- [ ] Confirm classification definitions and default handling.
- [ ] Add data labels and ownership metadata to contract designs.
- [ ] Map every high-risk threat to at least one test plan.
- [ ] Define how test evidence and unresolved findings are recorded.
- [ ] Review the register against all 35 ADRs before Phase 1.
