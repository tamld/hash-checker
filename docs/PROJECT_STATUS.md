# Project Status Overview

_Last updated: 2025-10-23_

This page summarises the current state. Refer to [`docs/PLAN.md`](PLAN.md) for
the roadmap and [`docs/TASKS.md`](TASKS.md) for near-term work; this overview
only highlights the key signals and where to dig deeper.

## Snapshot
- Multi-agent delivery workflow merged (PR #46); `.agents/AGENTS.yml` and supporting runbooks now reflect cross-agent guardrails.
- GTK4-native dialog backend shipped behind the `gtk4-native` feature flag (Issue #39 / PR #40). CI evidence captured in `logs/qa/gtk4-20251022-ci.md`.
- Release `v0.1.7` (2025-10-22) is live, superseding the earlier 0.1.5 notes. Changelog captured via PR #42.
- Security follow-up (Issues #43/#47): branch `fix/glib-upgrade-issue47` bumps the GTK4 stack to 0.10.1 (glib 0.21.3). Validation log: `logs/qa/glib-upgrade-20251023.md`; CI run 18743487218 passed (portal + gtk4-native smoke).
- DMG artefact rename is in progress on branch `feature/dmg-rename-issue48-a1`, targeting a unified `hash-checker.dmg` across scripts/workflows (Issue #48).
- SignPath onboarding remains blocked on OSS subscription; track progress in `.agents/project_state.yml` and `docs/security/SIGNPATH_CHECKLIST.md`.
- Vagrant smoke validation still requires a VMware-capable host; follow `docs/vagrant/VALIDATION_PLAYBOOK.md` when the hardware window opens.

## Maintenance cadence
- Monthly: dependency refresh (`deps-refresh.yml`) and cargo-dist maintenance.
- Quarterly: Vagrant smoke reminder + log archival (`docs/vagrant/VALIDATION_PLAYBOOK.md`).

## Looking ahead
- Near-term backlog comes directly from `docs/TASKS.md` (“Ready backlog”).
- Longer-term stretch goals remain in `docs/PLAN.md` §Phase 5.
