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
- [ ] Batch comparison reports API for CI.
- [ ] Automate reproducible build smoke (`cargo dist build --dry-run`) and Debian installer verification in CI.
- [ ] GTK4/`instant` migration spike (due 2025-11-15): audit GTK3 usage, evaluate alternatives, open tracking issue.
- [x] Build GUI manifest deep-tree harness + golden artefacts (see docs/GUI_MANIFEST_TEST_PLAN.md).
- [x] Adjust manifest table layout + control widths to satisfy GUI_MANIFEST_TEST_PLAN assumptions (A3/A4).

## Deferred / blocked
- [ ] SignPath onboarding (awaiting OSS credentials and secrets).
- [ ] Capture Vagrant smoke log – Pending VMware-capable host; run the playbook and archive logs under `logs/release-history/<tag>/vagrant/` once hardware is available.

> When a task leaves this list, update `docs/PLAN.md` and reference commit IDs or PRs for traceability.
