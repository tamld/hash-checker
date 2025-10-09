# Task Tracker

## Table of Contents
- [Phase 2 – GUI Experience](#phase-2--gui-experience)
- [Phase 3 – Distribution & Releases](#phase-3--distribution--releases)
- [Phase 4 – Security & Compliance](#phase-4--security--compliance)
- [Phase 5 – Stretch & Ecosystem](#phase-5--stretch--ecosystem)
- [Maintenance & Ops](#maintenance--ops)
- [Issue Planning Summary (2025-10-08)](#issue-planning-summary-2025-10-08)

## Phase 2 – GUI Experience
- [x] Scaffold `hash-checker-gui` with egui/eframe and wire it to the Rust core.
- [x] Complete core GUI flow (picker, algorithm dropdown, result panel).
- [x] Add accessibility toggles (high contrast, keyboard hints, clipboard).
- [x] Hook the `--smoke-test` mode into CI/Vagrant.
- [x] Automate GUI regression via headless `cargo test`.
- [ ] Capture refreshed screenshots/fixtures for README and docs.

## Phase 3 – Distribution & Releases
- [x] Integrate a GUI automation job (headless GUI tests) into CI and gate merges on it.
- [x] Ship the Windows GA release: portable ZIP + NSIS installer, validated in CI and release workflows.
- [x] Ship the macOS GA release: universal, unsigned DMG with documented Gatekeeper bypass steps.
- [x] Ship the Linux GA release: produce Debian `.deb`, AppImage, and Arch `pacman` packages in CI/release workflows.
- [x] Embed branded icon assets across all installers.
- [x] Add `cargo-dist` automation for release manifests.
- [x] Schedule a nightly Debian packaging cron job.
- [x] Integrate optional Vagrant headless smoke tests (guarded by `VAGRANT_ENABLED`) for Windows/Linux packaging scripts.

## Phase 4 – Security & Compliance
- [ ] Perform threat modelling and review path/canonicalisation handling.
- [x] Integrate `cargo audit` and `cargo deny` into CI (gating).
- [ ] Publish checksum/signature verification guidance for end users.
- [ ] Automate Windows signing with SignPath Foundation (GitHub integration) and record the pipeline configuration.
- [ ] Keep macOS unsigned; maintain Gatekeeper bypass documentation and verify smoke coverage for every release.
- [ ] Maintain a runbook for credential management (rotation, recovery, revocation).
- [ ] Record the Vagrant validation path for signed artefacts and map checksums to releases.

## Phase 5 – Stretch & Ecosystem
- [ ] Directory hashing & manifest export/import.
- [ ] Batch comparison reports (JSON/CSV).
- [ ] Plugin interface for additional runtimes (Node, WASI, ...).

## Maintenance & Ops
- [ ] Keep README/docs free of personal machine paths; prefer relative paths or environment variables.
- [ ] Maintain fixture/hash samples for regression QA.
- [ ] Retire the legacy Python implementation once the Rust release is production ready.
- [ ] Plan dependency migrations away from GTK3 bindings and `instant` when replacements stabilise.
- [ ] Monitor the local CI workflow (`make ci-linux-local`) and platform fallback logs; adjust scripts as tooling evolves.

## Issue Planning Summary (2025-10-08)
- GUI automation in CI: top priority, target completion by 2025-10-22.
- `cargo-dist` + release notes: follow GUI automation; update `docs/OPERATIONS.md` once the flow is proven.
- Nightly Debian packaging: enable after Windows/macOS releases stabilise; archive logs and artefacts.
- RHEL/Arch packaging: develop on dedicated branches, merge only with green smoke and Vagrant runs.
- Vagrant headless coverage: required for Windows and Linux in CI and release workflows; store smoke logs.
- Signing automation: prepare SignPath Foundation integration for Windows; macOS remains unsigned with clear Gatekeeper bypass guidance.
- Data hygiene: re-verify the repository for sensitive paths before every public release.
