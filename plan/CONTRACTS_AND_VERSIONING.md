# Local Assistant 2.0 — Contracts and Versioning Policy

**Status:** Phase 0 interface policy proposal  
**Date:** 2026-11-22  
**Authority:** ADR-007, ADR-011, ADR-013, ADR-014, ADR-021 and ADR-030  
**Purpose:** Establish compatibility rules before client, worker and plugin implementations diverge.

## 1. Contract families

The project has four distinct contract families:

1. **Client API:** desktop, CLI, browser and future clients communicate with the authoritative core through a versioned HTTP-compatible API and WebSocket event stream.
2. **Worker protocol:** the core supervises Python and other worker processes through a versioned structured protocol over local supervised IPC.
3. **Plugin contract:** plugins declare identity, version, capabilities, permissions, dependencies and runtime requirements through a validated manifest and capability-limited host interface.
4. **Internal event contract:** core components exchange typed events; durable workflow events are persisted where required.

Contracts are independent of internal Rust module boundaries, UI implementation details and database schemas.

## 2. Canonical schema ownership

- Store canonical API, worker, plugin and event schemas under the platform monorepo's `contracts/` directory.
- Each schema has one authoritative source file and a documented owner.
- Generated types/clients are derived artifacts; never hand-edit generated output.
- CI validates schemas and checks generated output for drift.
- Every contract declares its own version and compatibility policy.
- Schema validation is deterministic and must not depend on an LLM.
- Unknown security-sensitive fields, malformed identities and unsupported capability requests are rejected rather than guessed.

## 3. Client API policy

**Initial direction:** HTTP-compatible structured API, OpenAPI 3.1 documentation and WebSockets for streaming/events.

Proposed conventions:

- Version the public client API explicitly, initially using a major version in the URL path, such as `/api/v1/`.
- Use JSON request and response bodies with stable, documented field names.
- Use consistent structured error envelopes with machine-readable codes and safe user-facing messages.
- Include request/correlation identifiers for tracing across core and worker boundaries.
- Use explicit pagination, limits and deadlines for potentially large or long-running operations.
- Long-running work returns a task identifier and streams progress/state events; connection loss must not silently duplicate an action.
- Authentication establishes the client identity; the core authorizes every operation independently.
- WebSocket connections authenticate and bind to a scoped client session; each subscription is checked for authorization.
- Version WebSocket event protocols independently, for example with a negotiated `assistant.events.v1` subprotocol.
- Never expose database schema or internal file paths as accidental API contracts.
- Remote access remains disabled by default and requires explicit configuration and secure transport.

### Compatibility rules

- Adding an optional response field is normally backward-compatible.
- Adding a required request field, changing field meaning, removing a field or changing authorization semantics is breaking.
- Enum expansion must be handled deliberately; clients must not assume unknown values are safe.
- Security restrictions may become more restrictive in a minor release when required to close a vulnerability; document the impact.
- Breaking API changes require a new major version and a migration/deprecation plan.
- Keep the previous supported API major available during a documented transition where practical; do not preserve unsafe behaviour for compatibility.

## 4. Worker protocol policy

**Approved architecture:** versioned JSON messages over supervised local IPC, initially using process stdio. Exact framing is a technical-spike decision.

Every message exchange must support:

- Protocol version negotiation.
- Message type and schema validation.
- Unique request/correlation identifier.
- Trusted execution-context reference established by the core, never self-asserted by a worker.
- Explicit operation and bounded payload.
- Deadline and cancellation propagation.
- Structured success, failure and retryability status.
- Bounded message sizes and rate/resource limits.
- Explicit handling of unknown message types and fields.
- stdout reserved for protocol; stderr for diagnostics.
- Scoped temporary references for large payloads rather than unbounded inline messages.

Workers may propose results and tool requests. The core validates the contract and authorization before acting. A worker cannot create users, change workspace membership, grant permissions or invoke privileged helpers directly.

Protocol framing, handshake details, maximum message sizes and restart semantics must be finalized by SP-04 before implementation.

## 5. Plugin manifest policy

A plugin manifest must declare, at minimum:

- Stable plugin identifier and display metadata.
- Publisher identity and package version.
- Manifest schema version.
- Supported assistant/plugin API versions.
- Runtime type and entry point.
- Requested capabilities and permission categories.
- Network destinations or network capability needs, if any.
- Filesystem/resource preopens, if applicable.
- CPU, memory, execution-time and concurrency requirements.
- Dependencies and minimum compatible versions.
- Package integrity/signature/provenance metadata.
- Update and migration metadata where needed.

Rules:

- Manifest declarations are requests, not grants.
- Installation does not grant access.
- The broker and sandbox enforce actual capabilities at execution time.
- Publisher verification/signatures establish provenance, not safety.
- Permission/capability changes on update require renewed review.
- Unknown required capabilities cause installation or activation to fail safely.
- Plugin output and package content remain untrusted data.
- Disable/uninstall revokes credentials/grants and removes scheduled work/subscriptions.

The exact JSON Schema dialect and Wasm component metadata format are selected after SP-06 and SDK prototyping.

## 6. Internal event envelope

Internal event contracts should include:

- Event type and schema version.
- Event identifier.
- Occurrence and recording timestamps where relevant.
- Trusted owner/workspace/task context where applicable.
- Producer component identity.
- Correlation/causation identifiers.
- Payload classification and scope metadata.
- Idempotency/deduplication key for events that can trigger external effects.
- Traceable outcome or processing state for durable workflow events.

Events are not authorization tokens. Consumers must revalidate identity, scope and permissions before accessing resources or executing actions. Avoid embedding private content when a scoped reference is sufficient.

## 7. Error and retry semantics

- Distinguish validation, authentication, authorization, conflict, rate/resource limit, timeout, cancellation, unavailable dependency and internal errors.
- Do not return secrets, encryption details or sensitive internal paths in errors.
- Mark retryability explicitly; do not infer it from a generic server error.
- A timeout after an external action may mean the action completed. Reconcile status before retrying non-idempotent operations.
- Cancellation is best-effort for external systems; report uncertain completion honestly.
- Preserve correlation IDs across client, core, worker and workflow boundaries.

## 8. Schema evolution and deprecation

- Every contract change includes compatibility analysis and tests.
- Prefer additive changes and optional fields.
- Never reuse a removed field name with a different meaning.
- Deprecate before removal where feasible; publish removal versions and migration instructions.
- Keep fixtures for previous supported versions.
- Test both producer-new/consumer-old and producer-old/consumer-new combinations where compatibility is claimed.
- Persisted workflow/event schemas require migration and replay compatibility tests.
- Database migrations are separate from API contract versions.

## 9. Security and privacy requirements

- Contracts must not carry raw passwords, recovery keys or encryption keys unless a narrowly defined protected key-management protocol requires it.
- Credentials are never returned to models or plugins.
- Sensitive payloads are minimized and labelled.
- Authorization context is established by the core and cannot be overridden by request body fields.
- Retrieved documents, plugin output and worker results are untrusted data, not policy instructions.
- Remote API and event transport require authentication and secure transport.
- Diagnostic logging records identifiers/status where possible, not private payloads.

## 10. Phase 0 decisions still requiring prototype validation

- Exact schema language/tooling and code generation.
- Worker IPC framing and handshake.
- API authentication/session details and token lifetime.
- WebSocket reconnection and event replay semantics.
- Plugin manifest schema dialect and Wasm component interface details.
- Contract compatibility window and release cadence.

These details can be selected as engineering defaults after the relevant spikes; none changes the approved product direction.
