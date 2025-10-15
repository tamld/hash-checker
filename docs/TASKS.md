# Task Tracker

This file lists near-term tasks for the upcoming release. The long-term roadmap is
maintained in [`docs/PLAN.md`](PLAN.md); historical summaries live in
[`docs/PROJECT_STATUS.md`](PROJECT_STATUS.md).

> Scope: keep this list focused on actionable work for the current cycle. Longer-term context stays in `docs/PLAN.md`; summary updates belong in `docs/PROJECT_STATUS.md`.

-## Current release focus
- [ ] **Merge theme & copy UX updates** – PR #7 (`feature/gui-themes`) and PR #9 (`feature/gui-copy-prefix`) have full QA coverage; awaiting review/merge alongside the refreshed screenshots.
- [x] **Tweak Soft Light palette** – Palette accepted as of 2025-10-15; capture future adjustments if additional feedback arrives.
- [x] **Refresh screenshots** – Updated set stored in `docs/assets/` with QA evidence in `logs/qa/theme-copy-verification-20251015.md`.
- [ ] **Integrate macOS DMG flow into CI** – Move the `/tmp` universal DMG build into `release.yml` so tags publish the universal artefact automatically.
- [x] **Add unsupported-prefix warning docs** – Update README/CHANGELOG to mention the new behaviour (Issue #10, completed 2025-10-15).

### QA checklist – theme & copy UX
- [x] Capture container test evidence (`docker run … cargo test`) and attach logs/screenshots to PR #9. (See `logs/local-test/docker-rust-test-20251015.log`.)
- [x] Run GUI regression tests (`cargo test --manifest-path rust/hash-checker-gui/Cargo.toml`) and confirm clipboard prefix flow manually. (See `logs/local-test/gui-tests-20251015.log`.)
- [x] Validate theme presets on real displays (Soft Light glare, Slate contrast) and note observations in `logs/qa/`. (See `logs/qa/theme-copy-verification-20251015.md`.)
- [x] Refresh screenshots following `docs/GUI_SCREENSHOT.md` after palette sign-off. (Assets uploaded 2025-10-15.)

## Ready backlog (pull next)
- [ ] Structured logging / telemetry toggle for long-running jobs.
- [ ] Criterion-based regression benchmarks for large files.
- [ ] Directory hashing + manifest export/import workflows.
- [ ] Batch comparison reports API for CI.

## Deferred / blocked
- [ ] SignPath onboarding (awaiting OSS credentials and secrets).
- [ ] GTK3 ➜ GTK4/egui-native migration (watch `docs/DEPENDENCY_MIGRATION.md`).

> When a task leaves this list, update `docs/PLAN.md` and reference commit IDs or PRs for traceability.
