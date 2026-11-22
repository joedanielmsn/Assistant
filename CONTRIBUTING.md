# Contributing to Local Assistant 2.0

## Repository workflow
The project owner develops directly on `main`. This is a deliberate single-maintainer workflow; a separate implementation repository and mandatory pull requests are not used.

1. Pull the latest `main` before starting work.
2. Keep each change focused and make small, descriptive commits.
3. Run relevant formatting, lint, build and tests before committing.
4. Keep `main` buildable; do not commit known-broken intermediate states.
5. Never force-push or rewrite shared history. Correct mistakes with a new commit or a documented revert.
6. Keep disposable technical-spike code outside production source directories. Commit reproducible reports and retained fixtures only after review.
7. External contributors may use pull requests if the project later accepts outside contributions.

## Architecture decisions
- The authoritative project plan and approved ADRs are under [plan/](./plan/README.md).
- Record consequential decisions as numbered ADRs.
- Do not silently rewrite an accepted ADR. Supersede it with a new ADR and link both.
- Distinguish approved decisions, implementation defaults, proposals and deferred questions.
- Update the roadmap, traceability matrix and relevant specifications when requirements or architecture change.

## Engineering and security
- The Rust core remains authoritative for identity, authorization and privileged operations.
- Every privileged action must pass through the central permission broker.
- Never weaken privacy, data ownership, sandboxing or fail-closed requirements to make a test pass.
- Changes to authorization, cryptography, sandboxing, privileged helpers, signing, update verification or data isolation require focused negative tests and a documented security checklist.
- Do not include credentials, keys, private user data, local databases or model weights in commits.
- Pin dependencies and commit applicable lockfiles.
- Do not claim a test, CI check or platform validation passed without verified evidence.
- Record exact commands, versions, results and limitations for technical spikes.

## Implementation boundary
Production implementation begins only after the Phase 0 exit criteria and explicit owner approval are satisfied. Planning and disposable prototypes do not independently authorize broad production implementation.
