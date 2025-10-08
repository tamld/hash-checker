# Task Tracker

## Table of Contents
- [Phase 2 – GUI Experience](#phase-2--gui-experience)
- [Phase 3 – Distribution & Releases](#phase-3--distribution--releases)
- [Phase 4 – Security & Compliance](#phase-4--security--compliance)
- [Phase 5 – Stretch & Ecosystem](#phase-5--stretch--ecosystem)

## Phase 2 – GUI Experience
- [x] Scaffold `hash-checker-gui` crate with egui/eframe.
- [x] Wire GUI to the shared Rust core.
- [x] Implement file picker, algorithm dropdown, and comparison messaging.
- [x] Add accessibility toggles (contrast theme, keyboard hints, clipboard copy).
- [x] Integrate GUI smoke test into the Vagrant pipeline.
- [x] Automate GUI regression via headless harness (`cargo test` launches `hash-checker-gui --smoke-test`).
- [x] Capture refreshed UI screenshots for README/docs (`docs/assets/gui-*.png`).

## Phase 3 – Distribution & Releases
- [x] Run fmt/clippy/test in the GitHub Actions matrix (Linux/macOS/Windows).
- [x] Generate `.dmg`/`.deb` installers via `cargo-packager` and keep the Windows portable ZIP.
- [x] Automate GitHub Release publication with installers, portable artefacts, checksums, and templated notes (release workflow).
- [x] Include application icon assets (PNG 1024/512/256) in packaging configuration.
- [x] Add smoke verification for produced installers (CLI + GUI launch in release workflow).
- [ ] Add `cargo-dist` release notes automation (optional backlog).
- [ ] Add nightly cron job that runs `cargo packager --release --formats deb` to catch regressions early.
- [x] Replace deprecated GitHub Actions (e.g. `actions-rs/toolchain@v1`) with supported toolchain setup that uses environment files.
- [ ] Prepare dependency migration PRs to drop GTK3 bindings and `instant` once upstream replacements are validated; abandon PRs if tests fail.
- [x] Add Make/script cleanup targets to purge `dist/`, `rust/*/target/packager`, and `/tmp/hash-checker-*` artefacts after successful packaging runs (with opt-out switch) and log the rule in docs/OPERATIONS.md.

## Phase 4 – Security & Compliance
- [ ] Script checksum/codesigning workflow (macOS notarisation, Windows codesign, Linux package signing) once credentials are available, capturing current macOS/Windows technical constraints as part of the implementation notes.
- [x] Integrate `cargo audit`/`cargo deny` into CI and gate merges on critical advisories.
- [ ] Draft the security hardening roadmap and threat model deliverables.

## Phase 5 – Stretch & Ecosystem
- [ ] Create richer sample fixtures for manual QA and documentation.
- [ ] Design directory hashing + manifest export/import.
- [ ] Plan SDK/binding story for other runtimes (e.g. Node, WASI) on top of the Rust core.

## Issue Planning Summary (2025-10-08)
- GUI automation in CI - se su dung Playwright/headless harness trong GitHub Actions, muc tieu hoan thanh truoc 2025-10-22 theo roadmap tai `docs/PLAN.md` muc 2.
- `cargo-dist` release notes automation - thu nghiem `cargo dist init` tren nhanh rieng, cap nhat `docs/OPERATIONS.md`, muc tieu ngay sau khi GUI automation on dinh.
- Nightly Debian packaging - tao cron workflow chay `cargo packager --formats deb`, phu thuoc vao viec on dinh icon va cleanup script, du kien sau 2025-10-22.
- Thay the workflow `actions-rs/toolchain@v1` - DA HOAN THANH 2025-10-08: thay bang `dtolnay/rust-toolchain@stable` de loai bo canh bao `set-output`.
- Di tru phu thuoc GTK3/`instant` - khao sat thay the, mo PR rieng kem smoke tests; bo neu gap hoi quy, len lich dau thang 11.
- Script ky so da nen tang - thu thap chung chi/macOS notarisation truoc, ghi chu han che ky thuat hien tai va bo sung workflow khi credential san sang.
- Bo sung security roadmap & threat model - mo phien lam viec bao mat sau khi ha tang ky so hoan tat, cap nhat `docs/SECURITY_ROADMAP.md`.
- Mo rong fixture/thu vien - thu thap mau QA va thiet ke manifest/directory hashing sau khi stabilise phat hanh, se duoc uu tien trong sprint tiep theo.
