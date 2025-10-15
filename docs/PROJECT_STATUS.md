# Project Status Overview

_Last updated: 2025-10-15_

This page highlights the high-level state; detailed task tracking now lives in
[`docs/TASKS.md`](TASKS.md) and the roadmap in [`docs/PLAN.md`](PLAN.md).

## In-flight
- **Release polishing** – Theme picker + copy/paste UX (PR #7/#9) pending merge; run QA then tag the next release.
- **macOS universal DMG** – Local script builds a universal DMG into `/tmp`; wire it into `release.yml` prior to release.
- **Screenshot refresh** – Follow the list in `docs/GUI_SCREENSHOT.md` once the palette locks.
- **SignPath onboarding** – Awaiting OSS credentials/secrets (see `docs/security/SIGNPATH_CHECKLIST.md`).

## Maintenance cadence
- Monthly: dependency refresh (`deps-refresh.yml`) and cargo-dist maintenance.
- Quarterly: Vagrant smoke reminder + log archival (`docs/vagrant/VALIDATION_PLAYBOOK.md`).
- Internal release checklist recorded after the withdrawn v0.1.3 tag to prevent empty releases.

## Upcoming priorities (ordered)
1. Structured logging / telemetry toggle.
2. Criterion regression benchmarks.
3. Directory hashing & manifest export.
4. Batch comparison reports for CI.
5. GTK3 ➜ GTK4/egui-native migration (watch `docs/DEPENDENCY_MIGRATION.md`).

For a comprehensive backlog, consult `docs/PLAN.md` §Phase 5.
