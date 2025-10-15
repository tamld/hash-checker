# Task Tracker

This file lists near-term tasks for the upcoming release. The long-term roadmap is
maintained in [`docs/PLAN.md`](PLAN.md); historical summaries live in
[`docs/PROJECT_STATUS.md`](PROJECT_STATUS.md).

> Scope: keep this list focused on actionable work for the current cycle. Longer-term context stays in `docs/PLAN.md`; summary updates belong in `docs/PROJECT_STATUS.md`.

## Current release focus
- _None at the moment._ All items from the October 2025 polishing cycle have landed; pick work from the backlog below.

## Ready backlog (pull next)
- [x] Criterion-based regression benchmarks for large files (see Criterion benches).
- [ ] Directory hashing + manifest export/import workflows.
- [ ] Batch comparison reports API for CI.

## Deferred / blocked
- [ ] SignPath onboarding (awaiting OSS credentials and secrets).
- [ ] Capture Vagrant smoke log – Pending VMware-capable host; run the playbook and archive logs under `logs/release-history/<tag>/vagrant/` once hardware is available.
- [ ] GTK3 ➜ GTK4/egui-native migration (watch `docs/DEPENDENCY_MIGRATION.md`).

> When a task leaves this list, update `docs/PLAN.md` and reference commit IDs or PRs for traceability.
