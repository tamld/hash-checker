# Project Status Overview

_Last updated: 2026-10-25_

This page summarises the current state. Refer to [`docs/PLAN.md`](PLAN.md) for
the roadmap and [`docs/TASKS.md`](TASKS.md) for near-term work; this overview
only highlights the key signals and where to dig deeper.

## Snapshot
- Release `v0.2.1` (2026-10-25) ships the 4-phase SDD overhaul: formal `specs/` (`PRD.md`, `SRS.md`, `FSM.md`, `AC.md`), `.g8s/feature-map.yml`, cooperative hashing/manifest cancellation, real-time GUI progress bars with **Cancel**, 8-algorithm prefix auto-detection, and manifest path-traversal hardening.
- GTK4-native dialog backend shipped behind the `gtk4-native` feature flag (Issue #39 / PR #40) with `gtk4` 0.10.1 / `glib` 0.21.3 security upgrade (Issues #43/#47) and `rand` 0.9.4 upgrade.
- Unified `hash-checker.dmg` macOS universal DMG packaging and `scripts/gui-snapshot-harness.sh` (`make gui-snapshot`) are active on `main` (Issues #33/#48).
- All 3 Offer Bundle CI gates (`ci_spec_code_sync.sh`, `ci_structure_sync.sh`, `ci_link_integrity.sh`) and 54/54 Rust unit/integration tests pass with zero warnings.
- SignPath onboarding remains optional pending OSS certificate provisioning; see [`docs/security/SIGNPATH_CHECKLIST.md`](security/SIGNPATH_CHECKLIST.md).

## Maintenance cadence
- Monthly: dependency refresh (`deps-refresh.yml`) and cargo-dist maintenance.
- Quarterly: Vagrant smoke reminder + log archival (`docs/vagrant/VALIDATION_PLAYBOOK.md`).

## Looking ahead
- Near-term backlog comes directly from `docs/TASKS.md` (“Ready backlog”).
- Longer-term stretch goals remain in `docs/PLAN.md` §Phase 5.
