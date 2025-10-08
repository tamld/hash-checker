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
- [ ] Automate GUI regression via Playwright (or equivalent headless harness).
- [ ] Capture fresh UI screenshots and short clip for README/docs.

## Phase 3 – Distribution & Releases
- [x] Run fmt/clippy/test in the GitHub Actions matrix (Linux/macOS/Windows).
- [x] Generate `.dmg`/`.deb` installers via `cargo-packager` and keep the Windows portable ZIP.
- [ ] Automate GitHub Release publication with installers, portable artefacts, checksums, and templated notes.
- [ ] Document offline installation steps and update the installer guidance.
- [ ] Add smoke verification for produced installers (CLI + GUI launch).

## Phase 4 – Security & Compliance
- [ ] Script checksum/codesigning workflow (macOS notarisation, Windows codesign, Linux package signing) once credentials are available.
- [ ] Integrate `cargo audit`/`cargo deny` into CI and gate merges on critical advisories.
- [ ] Draft the security hardening roadmap and threat model deliverables.

## Phase 5 – Stretch & Ecosystem
- [ ] Create richer sample fixtures for manual QA and documentation.
- [ ] Design directory hashing + manifest export/import.
- [ ] Plan SDK/binding story for other runtimes (e.g. Node, WASI) on top of the Rust core.
