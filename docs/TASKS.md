# Task Tracker

This file lists near-term tasks for the upcoming release. The long-term roadmap is
maintained in [`docs/PLAN.md`](PLAN.md); historical summaries live in
[`docs/PROJECT_STATUS.md`](PROJECT_STATUS.md).

> Scope: keep this list focused on actionable work for the current cycle. Longer-term context stays in `docs/PLAN.md`; summary updates belong in `docs/PROJECT_STATUS.md`.

## Current release focus
- [x] **Merge theme & copy UX updates** – PR #7 (`feature/gui-themes`) và PR #9 (`feature/gui-copy-prefix`) đã được gộp vào `main` (2025-10-15).
- [x] **Integrate macOS DMG flow into CI** – Universal DMG được build trực tiếp trong `release.yml` kể từ PR #11.
- [x] **Tweak Soft Light palette** – Palette hiện tại được chấp nhận; ghi chú ở `logs/qa/theme-copy-verification-20251015.md`.
- [x] **Refresh screenshots** – Bộ ảnh mới nằm trong `docs/assets/` với log QA tương ứng.
- [x] **Add unsupported-prefix warning docs** – README/CHANGELOG phản ánh hành vi mới (Issue #10).
- [ ] **Capture Vagrant smoke log** – Chạy `vagrant up` và lưu log vào `logs/release-history/<tag>/vagrant/` trước bản phát hành kế tiếp.

### QA checklist – theme & copy UX
- [x] Capture container test evidence (`docker run … cargo test`) and attach logs/screenshots to PR #9. (See `logs/local-test/docker-rust-test-20251015.log`.)
- [x] Run GUI regression tests (`cargo test --manifest-path rust/hash-checker-gui/Cargo.toml`) and confirm clipboard prefix flow manually. (See `logs/local-test/gui-tests-20251015.log`.)
- [x] Validate theme presets on real displays (Soft Light glare, Slate contrast) and note observations in `logs/qa/`. (See `logs/qa/theme-copy-verification-20251015.md`.)
- [x] Refresh screenshots following `docs/GUI_SCREENSHOT.md` after palette sign-off. (Assets uploaded 2025-10-15.)

## Ready backlog (pull next)
- [ ] Criterion-based regression benchmarks for large files.
- [ ] Directory hashing + manifest export/import workflows.
- [ ] Batch comparison reports API for CI.

## Deferred / blocked
- [ ] SignPath onboarding (awaiting OSS credentials and secrets).
- [ ] GTK3 ➜ GTK4/egui-native migration (watch `docs/DEPENDENCY_MIGRATION.md`).

> When a task leaves this list, update `docs/PLAN.md` and reference commit IDs or PRs for traceability.
