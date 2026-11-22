# Local Assistant 2.0

A local-first AI assistant platform, universal desktop experience, and eventually an AI-first Linux distribution.

## Repository organization
- [`plan/`](./plan/README.md) — master plan, ADRs, roadmap, technical spike evidence, traceability, engineering and security specifications.
- Repository root — platform implementation, Rust workspace, Qt desktop application, Python workers, contracts, tests and build configuration as they are introduced.
- Separate distribution repository (future) — CachyOS-derived operating-system composition and release tooling, planned for Phase 9.

## Current status
Architecture direction is consolidated through ADR-035. Phase 0 engineering preparation and technical validation are in progress. Production implementation has not started; Phase 1 begins after the Phase 0 exit gate and explicit owner approval.

## Development workflow
The project owner develops directly on `main` using focused commits. Run relevant checks before committing, preserve a buildable main branch, and never rewrite shared history. See [CONTRIBUTING.md](./CONTRIBUTING.md).

Start with the [project plan index](./plan/README.md) before changing architecture, security boundaries, data ownership or implementation contracts.
