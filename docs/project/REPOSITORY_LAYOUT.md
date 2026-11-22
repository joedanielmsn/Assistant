# Local Assistant 2.0 — Proposed Repository Layout

**Status:** Phase 0 engineering proposal; not yet an implementation authorization  
**Date:** 2026-11-22  
**Governing decision:** ADR-022  
**Architecture source of truth:** `AssistantV2` planning repository

## 1. Repository boundaries

The project should use two independently built and released code repositories:

1. **Platform monorepo** — the assistant core, desktop client, workers, plugin SDK, shared contracts, tests and platform packaging.
2. **Linux distribution repository** — CachyOS-derived distribution configuration, installer, package definitions, desktop integration, repository metadata and release tooling.

The repositories share versioned interfaces and released platform artifacts. The distribution repository must not vendor or fork the platform's core implementation as its normal integration method.

The existing `joedanielmsn/AssistantV2` repository remains the architecture and planning source of truth during Phase 0. This proposal does not silently convert it into a code repository or create either code repository. Repository creation and code bootstrap remain explicit execution steps after Phase 0 approval.

## 2. Platform monorepo — proposed top-level structure

```text
assistant-platform/
├── Cargo.toml
├── Cargo.lock
├── rust-toolchain.toml
├── .cargo/
│   └── config.toml
├── CMakeLists.txt
├── CMakePresets.json
├── pyproject.toml
├── uv.lock
├── README.md
├── CONTRIBUTING.md
├── SECURITY.md
├── CODE_OF_CONDUCT.md
├── LICENSE                 # added only after the licence decision
├── docs/
│   ├── architecture/
│   ├── decisions/
│   ├── security/
│   ├── protocols/
│   ├── operations/
│   └── development/
├── contracts/
│   ├── api/
│   ├── worker/
│   ├── plugin/
│   ├── events/
│   └── schemas/
├── core/
│   ├── crates/
│   │   ├── assistant-core/
│   │   ├── identity/
│   │   ├── authorization/
│   │   ├── permission-broker/
│   │   ├── resource-manager/
│   │   ├── model-manager/
│   │   ├── task-engine/
│   │   ├── memory/
│   │   ├── knowledge/
│   │   ├── persistence/
│   │   ├── event-system/
│   │   ├── audit/
│   │   └── platform-abstraction/
│   └── binaries/
│       └── assistantd/
├── desktop/
│   ├── app/
│   ├── qml/
│   ├── cpp/
│   ├── assets/
│   ├── platform/
│   │   ├── linux/
│   │   └── windows/
│   └── tests/
├── workers/
│   ├── common/
│   ├── specialist-runtime/
│   ├── inference-adapters/
│   ├── voice/
│   ├── browser-automation/
│   └── desktop-automation/
├── plugins/
│   ├── sdk/
│   ├── official/
│   ├── examples/
│   └── test-fixtures/
├── platform/
│   ├── linux/
│   │   ├── systemd/
│   │   ├── sandbox/
│   │   ├── packaging/
│   │   └── privileged-helper/
│   └── windows/
│       ├── lifecycle/
│       ├── sandbox/
│       ├── packaging/
│       └── privileged-helper/
├── tools/
│   ├── codegen/
│   ├── schema-validation/
│   ├── dev-environment/
│   ├── release/
│   └── diagnostics/
├── tests/
│   ├── integration/
│   ├── security/
│   ├── isolation/
│   ├── compatibility/
│   ├── performance/
│   └── fixtures/
├── packaging/
│   ├── linux/
│   └── windows/
├── ci/
│   ├── workflows/
│   ├── scripts/
│   └── policies/
└── third_party/
    └── README.md
```

This is a logical starting layout, not a requirement to create every directory on day one. Directories should appear when their first owned code or contract is introduced.

## 3. Linux distribution repository — proposed structure

```text
assistant-os/
├── README.md
├── docs/
│   ├── architecture/
│   ├── installation/
│   ├── recovery/
│   ├── security/
│   └── release/
├── branding/
│   ├── artwork/
│   ├── icons/
│   ├── wallpapers/
│   └── identity/
├── packages/
│   ├── base/
│   ├── desktop/
│   ├── assistant-integration/
│   ├── installer/
│   └── distro-tools/
├── installer/
│   ├── calamares/
│   ├── partitioning/
│   └── presets/
├── desktop/
│   ├── plasma/
│   ├── defaults/
│   ├── settings/
│   └── utilities/
├── boot/
│   ├── grub/
│   ├── secure-boot/
│   └── recovery/
├── repositories/
│   ├── testing/
│   └── stable/
├── build/
│   ├── image/
│   ├── package/
│   └── signing/
├── hardware/
│   ├── profiles/
│   └── compatibility/
├── tests/
│   ├── installation/
│   ├── upgrade/
│   ├── recovery/
│   ├── secure-boot/
│   └── hardware/
└── ci/
    └── workflows/
```

The distribution repository owns OS integration and release composition. It consumes signed, versioned platform packages and must be able to pin a known-good platform release.

## 4. Dependency and ownership rules

| Area | Owner | May depend on | Must not own |
|---|---|---|---|
| Rust core | Platform monorepo | Shared contracts, platform abstractions | UI rendering, distro branding |
| Desktop UI | Platform monorepo | Versioned client API, UI assets | Direct database access, authorization decisions |
| Python workers | Platform monorepo | Versioned worker contracts, approved libraries | Permission policy, trusted identity issuance |
| Plugin SDK/runtime | Platform monorepo | Versioned plugin contracts and capability host API | Ambient host privileges |
| OS adapters/helpers | Platform monorepo | Narrow platform interfaces and broker protocol | General-purpose root/admin shell |
| Linux distribution | Distribution repository | Signed platform packages, documented integration APIs | Forked core implementation as routine integration |
| Build/release | Each repository independently | Pinned inputs, signed artifacts, provenance | Implicit trust in unverified upstream artifacts |

## 5. Cross-repository compatibility

- Publish versioned API, worker and plugin contracts from the platform repository.
- Version platform packages independently from the distribution release.
- The distribution pins explicit platform package versions and tested dependency sets.
- Breaking contract changes require a compatibility plan and migration notes.
- CI should test the distribution against its selected platform release candidate.
- Platform releases must not assume that the distribution repository is present.
- Distribution builds must not require access to private developer machines or undocumented local files.
- Shared schemas should have one canonical source; generated copies must be reproducible and checked for drift.

## 6. Build-system boundaries

| Component | Initial build tooling |
|---|---|
| Rust core and Rust tools | Cargo workspace |
| Qt/QML desktop | CMake + Ninja |
| Python workers | uv with committed lockfile |
| API and worker contracts | Versioned schemas with validation/code generation |
| Platform CI | GitHub Actions initially |
| Linux distribution packages | Arch/CachyOS-compatible package build tooling |
| Distribution installer and images | Calamares configuration plus reproducible image/package scripts |

Avoid introducing a second build orchestrator until concrete cross-language build requirements justify it. CI should call each component's native build system and assemble verified artifacts.

## 7. Repository hygiene and security

- Keep secrets, signing keys, credentials, local databases, model weights and user data out of Git.
- Store only small, redistributable test fixtures; large model assets are fetched by verified, pinned manifests.
- Pin dependencies and commit lockfiles.
- Generate SBOMs for release artifacts.
- Separate ordinary CI signing from production release-key custody.
- Use protected review for privileged helpers, authorization, cryptography, sandbox profiles, update code and release workflows.
- Require tests for schema compatibility and migration changes.
- Keep generated files reproducible and identify their source schema/tool version.
- Do not add a licence file or claim a final licence until the project-wide licence decision is made.

## 8. Suggested implementation order for the repository itself

1. Establish the platform repository and baseline documentation.
2. Add Rust workspace and toolchain pinning.
3. Add minimal Qt/CMake and Python/uv workspace scaffolding.
4. Add contract/schema validation tooling.
5. Add Linux and Windows CI with formatting, linting and placeholder test jobs.
6. Add security scanning, secret scanning and SBOM generation.
7. Add only the component directories needed for Phase 1.
8. Establish the separate distribution repository when Phase 9 work is scheduled; maintain its design and interface specifications in Phase 0.

## 9. Open implementation details

These can be selected during engineering preparation without changing the approved product direction:

- Exact Rust crate boundaries after dependency prototypes.
- Exact schema language and code-generation tool after API/worker contract spikes.
- Exact Linux package formats and Windows installer technology.
- GitHub Actions runner sizing and any self-hosted hardware runners.
- Artifact retention, cache policy and release provenance configuration.
- Exact repository names and whether the platform monorepo should be created as a new repository or established from the existing planning repository.

The last item is a repository-governance choice and should be explicitly settled before code bootstrap. Until then, this document defines the target layout without creating or moving repositories.
