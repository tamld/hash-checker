# Project Status Overview

_Last updated: 2025-10-15_

This page summarises the current state. Refer to [`docs/PLAN.md`](PLAN.md) for
the roadmap and [`docs/TASKS.md`](TASKS.md) for near-term work; this overview
only highlights the key signals and where to dig deeper.

## Snapshot
- Feature polishing: PR #7 (`feature/gui-themes`) and PR #9 (`feature/gui-copy-prefix`) have complete QA logs; pending review/merge. See `docs/TASKS.md` and `logs/qa/` for evidence.
- macOS universal DMG: local script still produces the artefact; CI automation remains in `docs/PLAN.md` §Phase 3.
- Vagrant smoke: manual VMware/VirtualBox validation remains outstanding for the next release; see `docs/vagrant/VALIDATION_PLAYBOOK.md`.
- Documentation: README/CHANGELOG and the screenshot set (`docs/assets/`) were refreshed on 2025-10-15; release assets for v0.1.4 were pruned to installers plus a consolidated SHA256SUMS.
- SignPath onboarding: still blocked on OSS subscription; status tracked in `.agents/project_state.yml` and `docs/security/SIGNPATH_CHECKLIST.md`.

## Maintenance cadence
- Monthly: dependency refresh (`deps-refresh.yml`) and cargo-dist maintenance.
- Quarterly: Vagrant smoke reminder + log archival (`docs/vagrant/VALIDATION_PLAYBOOK.md`).

## Looking ahead
- Near-term backlog comes directly from `docs/TASKS.md` (“Ready backlog”).
- Longer-term stretch goals remain in `docs/PLAN.md` §Phase 5.
