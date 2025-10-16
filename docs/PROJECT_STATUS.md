# Project Status Overview

_Last updated: 2025-10-16_

This page summarises the current state. Refer to [`docs/PLAN.md`](PLAN.md) for
the roadmap and [`docs/TASKS.md`](TASKS.md) for near-term work; this overview
only highlights the key signals and where to dig deeper.

## Snapshot
- GUI polish landed on `main` (theme presets, prefixed hash copy); QA log for the refreshed screenshots is stored under `logs/qa/theme-copy-verification-20251015.md`, and assets were updated again on 2025-10-16.
- Release automation now builds and verifies the macOS universal DMG inside `release.yml`; the local script remains available for manual runs.
- Linux CI job now runs on pushes and pull requests again; keep using `workflow_dispatch` inputs for targeted reruns when necessary.
- SignPath onboarding remains blocked on OSS subscription; track progress in `.agents/project_state.yml` and `docs/security/SIGNPATH_CHECKLIST.md`.
- Vagrant smoke validation still requires a VMware-capable host; follow `docs/vagrant/VALIDATION_PLAYBOOK.md` when the hardware window opens.

## Maintenance cadence
- Monthly: dependency refresh (`deps-refresh.yml`) and cargo-dist maintenance.
- Quarterly: Vagrant smoke reminder + log archival (`docs/vagrant/VALIDATION_PLAYBOOK.md`).

## Looking ahead
- Near-term backlog comes directly from `docs/TASKS.md` (“Ready backlog”).
- Longer-term stretch goals remain in `docs/PLAN.md` §Phase 5.
