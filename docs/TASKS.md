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
- [ ] Capture refreshed screenshots/fixtures for README and docs. (Checklist: `docs/GUI_SCREENSHOT.md`; pending access to a GUI-capable workstation.)

## Phase 3 – Distribution & Releases
- [x] Integrate a GUI automation job (headless GUI tests) into CI and gate merges on it.
- [x] Ship the Windows GA release: portable ZIP + NSIS installer, validated in CI and release workflows.
- [ ] Ship the macOS GA release as a universal (Intel + Apple Silicon) DMG. Local script (`make macos-dmg-universal`) now builds and verifies the universal DMG; integrate it into the release workflow/re-signing path before closing.
- [x] Ship the Linux GA release: produce Debian `.deb`, AppImage, and Arch `pacman` packages in CI/release workflows.
- [x] Embed branded icon assets across all installers.
- [x] Add `cargo-dist` automation for release manifests.
- [x] Schedule a nightly Debian packaging cron job.
- [x] Integrate optional Vagrant headless smoke tests (guarded by `VAGRANT_ENABLED`) for Windows/Linux packaging scripts.

## Phase 4 – Security & Compliance
- [x] Perform threat modelling and review path/canonicalisation handling (see `docs/security/THREAT_MODEL.md`).
- [x] Integrate `cargo audit` and `cargo deny` into CI (gating).
- [x] Publish signature verification guidance for end users (checksum + signature instructions in `docs/security/VERIFICATION_GUIDE.md`).
- [ ] Automate Windows signing with SignPath Foundation (GitHub integration) and document the pipeline configuration (CI job `windows_sign` plus `docs/security/CI_SIGNING.md` added 2025-10-13); awaiting secrets before enabling in production.
- [x] Keep macOS unsigned; maintain Gatekeeper bypass documentation and verify smoke coverage for every release (README Gatekeeper section updated 2025-10-13).
- [x] Maintain a runbook for credential management (rotation, recovery, revocation). See `docs/security/CREDENTIAL_RUNBOOK.md`.
- [x] Record the Vagrant validation path for signed artefacts and map checksums to releases (log roots documented in `docs/vagrant/VALIDATION_PLAYBOOK.md`).
- [x] Run the monthly dependency refresh checklist (`make deps-refresh`, `cargo audit`, `cargo deny`) – automated via `.github/workflows/deps-refresh.yml` (logs stored in workflow artefacts).
- [x] Update builder toolchains (Docker images, `cargo-packager`, `rustup` components) alongside deps refresh (see `scripts/deps-refresh.sh`).
- [x] Maintain the Vagrant validation playbook (document box versions, log archive template) and execute it quarterly (reminder via `vagrant-smoke-reminder.yml`).

## Phase 5 – Stretch & Ecosystem
- Core features:
  - [ ] Directory hashing & manifest export/import.
  - [ ] Batch comparison reports (JSON/CSV).
  - [ ] Plugin interface for additional runtimes (Node, WASI, ...).
- Observability & performance:
  - [ ] Add optional structured logging with opt-in telemetry toggle.
  - [ ] Create regression benchmarks (criterion) covering large file hashing.
- Distribution automation:
  - [x] Integrate reproducible build check (`cargo dist build --dry-run`) into CI (ci.yml linux job).
  - [ ] Produce macOS universal DMG (Intel + Apple Silicon) via multi-target build in CI.

  - [x] Scaffold winget/homebrew manifest generation with validation job (see `scripts/generate_manifests.sh`).

## Maintenance & Ops
- [x] Keep README/docs free of personal machine paths (enforced via `rg "/Users/"` sanity check).
- [x] Maintain fixture/hash samples for regression QA (see `docs/maintenance/QA_FIXTURES.md`).
- [x] Retire the legacy Python implementation once the Rust release is production ready (confirmed Rust-only; see `docs/maintenance/LEGACY_CLEANUP.md`).
- [ ] Plan dependency migrations away from GTK3 bindings and `instant` when replacements stabilise.
- [x] Align crate version numbers with release tags (v0.1.2).
- [x] Monitor the local CI workflow (`make ci-linux-local`) and fallback logs; procedure documented in `docs/maintenance/LEGACY_CLEANUP.md`.
- [x] Schedule monthly dependency updates (`cargo update`, review yanked crates) – automated by `.github/workflows/deps-refresh.yml` (artefact log).
- [x] Add scheduled maintenance run for `cargo-dist` upgrades (`.github/workflows/cargo-dist-maintenance.yml`) and keep checksum deduplication guidance in release documentation.

## Phase 6 – Governance & Automation
- [ ] Roll out SignPath secrets/variables and confirm signed artefacts in release workflow (blocked on OSS onboarding).
- [x] Document credential lifecycle (issuance, rotation cadence, emergency revoke playbook) – see `docs/security/CREDENTIAL_RUNBOOK.md`.
- [x] Create scheduled workflows for `make deps-refresh` (monthly) and Vagrant smoke with signing verification (quarterly).
  - [x] Monthly deps-refresh workflow in place (`deps-refresh.yml`).
  - [x] Quarterly reminder: `.github/workflows/vagrant-smoke-reminder.yml` opens an issue for manual execution.
- [x] Archive release/signing logs under `logs/release-history/<tag>/` (see operations guide + release log template).
- [ ] Update SECURITY.md, issue/PR templates, and CODEOWNERS to enforce reviews on release & security artefacts.
- [ ] Track SignPath OSS onboarding (pending OSS org + test certificate).
- [ ] Prepare GitHub workflow updates to consume SignPath subscription/test certificate once provisioned.
- [ ] Draft release checklist updates covering SignPath test + production certificate rollout.

## Issue Planning Summary (2025-10-08)
- GUI automation in CI: top priority, target completion by 2025-10-22.
- `cargo-dist` + release notes: follow GUI automation; update `docs/OPERATIONS.md` once the flow is proven.
- Nightly Debian packaging: enable after Windows/macOS releases stabilise; archive logs and artefacts.
- RHEL/Arch packaging: develop on dedicated branches, merge only with green smoke and Vagrant runs.
- Vagrant headless coverage: required for Windows and Linux in CI and release workflows; store smoke logs.
- Signing automation: prepare SignPath Foundation integration for Windows; macOS remains unsigned with clear Gatekeeper bypass guidance.
- Data hygiene: re-verify the repository for sensitive paths before every public release.
