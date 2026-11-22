# Contributing to Local Assistant 2.0

This repository is the planning, architecture and governance source of truth for Local Assistant 2.0. Production implementation has not been authorized yet.

## Branch policy

- `main` is the stable, reviewed branch.
- Do not commit routine work directly to `main`.
- Create a short-lived branch for each focused change:
  - `docs/<topic>` — documentation and planning
  - `adr/<topic>` — architecture decision records
  - `spikes/<id>-<topic>` — technical spike evidence
  - `feat/<topic>` — approved implementation work
  - `fix/<topic>` — corrections
  - `chore/<topic>` — repository maintenance
- Branch from the latest `main`; keep changes focused and avoid mixing unrelated work.
- Open a pull request into `main`. Use a draft PR while work or evidence is incomplete.
- Do not merge a PR with unresolved security, privacy, data-ownership or architecture concerns.
- Prefer squash merging to keep the stable branch history readable. Delete merged short-lived branches.

## Pull request expectations

Every PR should explain:
1. What changed.
2. Why it changed and which requirement, ADR, issue or spike it relates to.
3. Evidence: tests, command output, research sources or explicit reason tests were not applicable.
4. Security, privacy, compatibility and migration implications.
5. Remaining limitations and follow-up work.

For technical spikes, keep disposable prototype code outside production paths. Record exact environment, toolchain/dependency versions, reproducible commands, results, limitations and acceptance status in the spike report. A local Linux result must not be represented as Windows validation.

## Architecture decisions

- Record consequential decisions in `ARCHITECTURE_DECISIONS.md` as numbered ADRs.
- Each ADR should state context, decision, alternatives/trade-offs and consequences.
- Do not silently rewrite an accepted ADR. Supersede it with a new ADR and link both.
- Distinguish approved decisions, implementation defaults, proposals and deferred questions.
- Update the masterplan and traceability matrix when requirements or architecture change.

## Documentation hygiene

- Keep the roadmap, research status, spike register and traceability matrix consistent.
- Link related documents rather than copying large sections into multiple files.
- Date evidence and preserve failed/partial results.
- Never include credentials, keys, personal data, private logs or other secrets in commits or PRs.
- Do not claim a test, CI check, review or platform validation passed unless its result is available and verified.

## Reviews and merging

This project may currently have a single active maintainer. A pull request is still required to create a reviewable change record, but a self-approval is not a meaningful independent review. For consequential security or architecture changes, keep the PR open for explicit owner review and record any accepted risk. Enable automated checks and branch protection as the repository gains suitable CI and collaborators.

## Implementation boundary

Planning and disposable spikes do not authorize production implementation. Phase 1 begins only after the Phase 0 exit criteria and explicit owner approval are satisfied.
