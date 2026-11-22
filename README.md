# Local Assistant 2.0

A local-first AI assistant platform, universal desktop experience, and eventually an AI-first CachyOS-derived operating system.

## Repository organization

- [`plan/`](./plan/README.md) — master plan, approved architecture decisions, roadmap, detailed spike definitions, traceability and engineering/security policies.
- [`phase-0/`](./phase-0/README.md) — consolidated Phase 0 evidence, decisions, deferred risks and transition review.
- [`phase-1/`](./phase-1/PHASE_1_SECURE_CORE_SPEC.md) — secure local core implementation specification.
- Repository root — platform implementation, Rust workspace, desktop client, workers, contracts, tests and build configuration as they are introduced.
- Separate distribution repository (future) — CachyOS-derived operating-system composition and release tooling, planned for the OS integration phase.

## Current status

Architecture discovery is sufficiently complete to begin the first secure local core prototype. Phase 0 evidence and remaining limitations are documented; this does not mean all original cross-platform or production acceptance criteria have passed.

The first prototype is a per-user CachyOS Linux core with a private Unix socket, OS-derived peer identity, deny-by-default operation broker and safe read-only health/status operations. Model execution, untrusted workers, plugins, privileged OS actions and the desktop UI are intentionally out of scope until their security boundaries are implemented.

## Development workflow

The project owner develops directly on `main` using focused commits. Keep `main` buildable, run relevant checks before committing, and never rewrite shared history. See [CONTRIBUTING.md](./CONTRIBUTING.md).

Start with the [project plan](./plan/README.md), [Phase 0 record](./phase-0/README.md) and [Phase 1 secure-core specification](./phase-1/PHASE_1_SECURE_CORE_SPEC.md) before changing architecture, security boundaries, data ownership or implementation contracts.
