# Task Tracker

This file lists near-term tasks for the upcoming release. The long-term roadmap is
maintained in [`docs/PLAN.md`](PLAN.md); historical summaries live in
[`docs/PROJECT_STATUS.md`](PROJECT_STATUS.md).

> Scope: keep this list focused on actionable work for the current cycle. Longer-term context stays in `docs/PLAN.md`; summary updates belong in `docs/PROJECT_STATUS.md`.

## Current release focus
- _None at the moment._ All items from the October 2025 polishing cycle have landed; pick work from the backlog below.

## Ready backlog (pull next)
- [x] Criterion-based regression benchmarks for large files (see Criterion benches).
- [x] Directory hashing + manifest export/import workflows (CLI support shipped in issue #20; GUI follow-up backlogged).
- [x] Batch comparison reports API for CI. (Issue #21)
- [x] Automate reproducible build smoke (`dist plan`) and Debian installer verification in CI. (Issue #22)
- [x] GTK4/`instant` migration spike (due 2025-11-15): audit GTK3 usage, evaluate alternatives, open tracking issue. (Issue #23)
- [x] Build GUI manifest deep-tree harness + golden artefacts (see docs/GUI_MANIFEST_TEST_PLAN.md).
- [x] Adjust manifest table layout + control widths to satisfy GUI_MANIFEST_TEST_PLAN assumptions (A3/A4).
- [x] Adopt rfd xdg-portal backend to drop GTK3 dependency (Issue #34).
- [x] Upgrade eframe/egui stack to remove `instant` and bump MSRV (Issue #35).
- [ ] Rename macOS DMG artefact to `hash-checker.dmg` (Issue #48; branch `feature/dmg-rename-issue48-a1`).
- [ ] Automate GUI snapshot harness & telemetry logs (Issue #33).
- [ ] Formalise dependency refresh workflow (cargo outdated/audit reporting).
- [x] Implement GTK4-native dialog backend behind feature flag (Issue #39 / PR #40; evidence: logs/qa/gtk4-20251022-ci.md).
- [x] Track gtk4/glib upgrade for RUSTSEC-2024-0429 (Issue #43). _Resolved via PR pending review (`fix/glib-upgrade-issue47`, CI run 18743487218)._
- [x] Bump glib crate to >=0.20.0 (Dependabot #47). _Resolved together with Issue #43 (same PR)._

## Deferred / blocked
- [ ] SignPath onboarding (awaiting OSS credentials and secrets).
- [ ] Capture Vagrant smoke log – Pending VMware-capable host; run the playbook and archive logs under `logs/release-history/<tag>/vagrant/` once hardware is available.

> When a task leaves this list, update `docs/PLAN.md` and reference commit IDs or PRs for traceability.
