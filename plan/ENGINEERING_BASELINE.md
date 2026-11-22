# Local Assistant 2.0 — Engineering Baseline

**Status:** Phase 0 proposal; toolchain versions and CI workflows are not yet bootstrapped  
**Date:** 2026-11-22  
**Applies to:** Platform monorepo initially; distribution repository adopts the relevant shared policies independently.

## 1. Engineering principles

- Security boundaries are explicit, testable and fail closed.
- The Rust core is authoritative for identity, authorization, task state and resource policy.
- UI, models, workers and plugins are untrusted requesters, not policy authorities.
- Prefer small modules with explicit interfaces over broad shared mutable state.
- Keep platform-specific code behind narrow adapters.
- Keep domain logic independent of database drivers and UI frameworks.
- Prefer deterministic validation and policy enforcement over model-generated decisions.
- Avoid unnecessary dependencies, background services and network access.
- Every feature includes tests, documentation and failure behaviour.
- No telemetry, automatic diagnostic uploads or hidden hosted-service dependency.

## 2. Main-branch development workflow

**Project decision:** The owner develops directly on `main` in this single-maintainer repository. A separate implementation repository and mandatory pull-request workflow are not used.

- Keep `main` buildable and coherent.
- Make small, focused, atomic commits with descriptive messages.
- Run relevant formatting, lint, build and test checks before committing.
- Record test commands, outcomes, limitations and related ADR/requirement references.
- Never force-push or rewrite shared history; use corrective commits or a documented revert.
- Keep disposable spike implementations outside production source directories.
- Security-sensitive changes require focused negative tests and a documented security checklist.
- Changes to authorization, cryptography, sandboxing, privileged helpers, signing, update verification or data isolation must remain narrowly scoped and receive extra scrutiny.
- Generated files must identify their source and be reproducible.
- External contributors may use pull requests if the project later accepts outside contributions.

Direct commits do not lower testing or security requirements. Automated CI should run on every main-branch commit. Independent review remains desirable for high-risk security changes and is required before public stable releases when a qualified reviewer is available.

## 3. Coding standards

### Rust core

- Use stable Rust unless a documented feature requires otherwise.
- Pin the toolchain in `rust-toolchain.toml`.
- Use workspace-level dependency versions and commit `Cargo.lock` for application binaries.
- Run `cargo fmt --check`, `cargo clippy` with warnings treated as errors for project code, and workspace tests.
- Avoid `unsafe`; every justified use requires a small isolated boundary, safety comment and focused tests.
- Do not block Tokio executor threads with synchronous database or filesystem work; use bounded blocking execution where required.
- Use typed errors and structured diagnostics; do not leak secrets or private content into logs.
- Avoid panics on untrusted input and operational failure paths.
- Keep authorization checks explicit and close to resource access.

### Qt 6 / QML desktop

- Use CMake + Ninja and Qt 6 with QML/Qt Quick Controls.
- Keep QML focused on presentation and interaction.
- Keep transport and view-model responsibilities in the thin C++ layer.
- Do not access databases directly from the UI.
- Do not implement security policy or trust client-side permission state.
- Keep Linux and Windows differences in platform adapters.
- Require keyboard navigation and accessibility checks for user-facing controls.

### Python workers

- Manage each worker environment through uv with committed lockfiles.
- Keep worker contracts versioned and schema-validated.
- Use type hints and deterministic input validation at process boundaries.
- Keep stdout reserved for protocol messages and stderr for diagnostics.
- Workers must not store or invent trusted identity, permissions or credentials.
- Keep dependencies minimal and isolate optional heavy ML dependencies by worker/runtime where practical.

### Schemas and contracts

- Treat API, worker and plugin schemas as public compatibility boundaries within the project.
- Version contracts independently from internal implementation details.
- Reject unknown or invalid security-sensitive fields rather than guessing.
- Document compatibility and deprecation rules.
- Generate clients/types from canonical schemas where practical; test generated output for drift.
- Never accept model output as validated authority without deterministic schema and policy checks.

## 4. Dependency and supply-chain policy

- Pin direct dependencies and commit language-specific lockfiles.
- Review new dependencies for maintenance, licence, security history, platform support and transitive footprint.
- Prefer maintained upstream packages over copied or vendored code.
- Keep vendoring exceptional, documented and reproducible.
- Run vulnerability and licence scans on a regular schedule and in release workflows.
- Produce SPDX-compatible SBOMs for release artifacts.
- Record native libraries and runtime components, not only language-level packages.
- Verify hashes/signatures for downloaded models, plugins and release artifacts.
- Do not bundle model weights or datasets until their distribution rights are reviewed.
- Keep production signing keys outside source control and untrusted CI jobs.
- Do not add a final licence file until the project-wide licence decision is made.

## 5. Test strategy

| Test layer | Purpose | Required examples |
|---|---|---|
| Unit | Validate isolated deterministic logic | Policy evaluation, schemas, resource calculations, key metadata |
| Integration | Validate component interactions | Core/API, persistence, worker protocol, model manager |
| Security negative | Prove prohibited actions are denied | Broker bypass, cross-user access, path escape, invalid credentials |
| Sandbox conformance | Prove OS restrictions are enforced | Filesystem, process, network and privilege denial |
| Failure/recovery | Validate safe behaviour under faults | Crash, timeout, cancellation, restart, interrupted migration |
| Compatibility | Validate supported platform combinations | Linux/Windows builds, drivers, GPU runtimes, CPU baseline |
| Performance/resource | Validate bounded behaviour | Memory pressure, concurrency, model lifecycle, Gaming Mode |
| UI/accessibility | Validate user interaction | Keyboard navigation, screen reader, scaling and focus |

Tests must not rely on real user data, production credentials or live external services unless a separately controlled integration test explicitly requires them.

## 6. CI pipeline proposal

### Commit and CI checks

1. Validate formatting and source hygiene.
2. Build Rust workspace and run unit tests.
3. Build Qt/CMake targets on supported Linux and Windows runners.
4. Validate Python environments and worker contract tests.
5. Validate API, worker and plugin schemas.
6. Run static analysis and linting.
7. Run secret scanning and dependency vulnerability checks.
8. Run relevant integration tests.
9. Generate test SBOMs or dependency inventories where supported.
10. Publish logs and test summaries without uploading private source data to unrelated services.

### Main-branch checks

- Run the required checks against every commit pushed to `main`.
- Build versioned development artifacts.
- Generate SBOM and provenance metadata.
- Run extended integration and migration tests.
- Keep GPU/hardware tests in a separate controlled workflow; ordinary CI must not depend on GPU availability.

### Release workflow

- Require an explicitly versioned release candidate.
- Run the full supported platform matrix and required security tests.
- Generate release SBOMs and provenance.
- Sign artifacts through protected release infrastructure.
- Promote artifacts through controlled testing/stable channels.
- Publish checksums, compatibility notes, known limitations and recovery instructions.
- Retain previous known-good artifacts for rollback.

## 7. CI and environment separation

- Pull-request CI must not receive production signing secrets.
- Fork/untrusted pull requests run with read-only or no secrets.
- GPU tests use dedicated hardware runners and publish exact device, driver and runtime details.
- Secure Boot tests use disposable test keys and test hardware/VMs, never production key material.
- Integration tests use disposable databases and test identities.
- CI logs must not contain credentials, private content, encryption keys or raw screen/audio data.
- No automatic telemetry or remote crash collection is introduced by CI tooling.

## 8. Toolchain version policy

Exact versions will be pinned at code bootstrap after the Phase 0 compatibility spikes. The policy is:

- Rust: current stable toolchain validated by the workspace.
- CMake and Ninja: supported stable versions pinned in CI images/documentation.
- Qt: one selected Qt 6 minor line for the initial desktop release, pinned consistently across supported platforms.
- Python: one supported CPython minor line for workers, pinned by project tooling.
- uv: pinned version in development and CI setup.
- PostgreSQL: one supported major version for development and CI, with migration tests.
- Native SQLCipher/SQLite: exact tested versions/features pinned and recorded.
- llama.cpp and ONNX Runtime: exact tested revisions/releases per backend matrix.
- Linux packaging tools: versions matching the selected CachyOS/Arch build environment.

Do not automatically adopt a new major/minor toolchain release without CI validation and review of native dependency compatibility.

## 9. Definition of done

A change is complete only when all applicable items are satisfied:

- [ ] Scope and user-visible behaviour are documented.
- [ ] Authorization and data ownership are enforced by the core.
- [ ] Failure, timeout and cancellation behaviour are defined.
- [ ] Tests cover normal and prohibited paths.
- [ ] Logs avoid secrets and unnecessary private content.
- [ ] Platform-specific behaviour is isolated and tested.
- [ ] Dependency and licence implications are reviewed.
- [ ] Documentation and recovery guidance are updated.
- [ ] CI passes.
- [ ] Any new trust boundary is added to the threat model.

## 10. Decisions intentionally left for bootstrap

These are implementation setup choices, not unresolved product-direction questions:

- Exact toolchain patch/minor versions after compatibility spikes.
- Exact CI runner images and caching configuration.
- Whether hardware CI uses a self-hosted runner or controlled manual release gate.
- Exact schema/code-generation tooling.
- Exact Windows installer and Linux package formats.
- Branch-protection settings available under the selected GitHub repository configuration.

The engineering baseline is reviewed before code bootstrap and updated through focused commits directly on `main`.
