# Task Tracker

This file lists near-term tasks for the upcoming release. The long-term roadmap is
maintained in [`docs/PLAN.md`](PLAN.md); historical summaries live in
[`docs/PROJECT_STATUS.md`](PROJECT_STATUS.md).

## Current release focus
- [ ] **Merge theme & copy UX updates** – Review and merge PR #7 (`feature/gui-themes`) and PR #9 (`feature/gui-copy-prefix`) once QA passes.
- [ ] **Tweak Soft Light palette** – Validate the new colours on real displays, capture feedback, and apply follow-up adjustments if needed.
- [ ] **Refresh screenshots** – Capture the six frames enumerated in `docs/GUI_SCREENSHOT.md` after palette sign-off.
- [ ] **Integrate macOS DMG flow into CI** – Move the `/tmp` universal DMG build into `release.yml` so tags publish the universal artefact automatically.

## Ready backlog (pull next)
- [ ] Structured logging / telemetry toggle for long-running jobs.
- [ ] Criterion-based regression benchmarks for large files.
- [ ] Directory hashing + manifest export/import workflows.
- [ ] Batch comparison reports API for CI.

## Deferred / blocked
- [ ] SignPath onboarding (awaiting OSS credentials and secrets).
- [ ] GTK3 ➜ GTK4/egui-native migration (watch `docs/DEPENDENCY_MIGRATION.md`).

> When a task leaves this list, update `docs/PLAN.md` and reference commit IDs or PRs for traceability.
