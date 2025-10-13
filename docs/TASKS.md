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
- [x] Perform threat modelling and review path/canonicalisation handling (see `docs/security/THREAT_MODEL.md`).
- [x] Integrate `cargo audit` and `cargo deny` into CI (gating).
- [x] Publish signature verification guidance for end users (checksum + signature instructions in `docs/security/VERIFICATION_GUIDE.md`).
- [ ] Automate Windows signing with SignPath Foundation (GitHub integration) and document the pipeline configuration (CI job `windows_sign` plus `docs/security/CI_SIGNING.md` added 2025-10-13); awaiting secrets before enabling in production.
- [ ] Keep macOS unsigned; maintain Gatekeeper bypass documentation and verify smoke coverage for every release.
- [ ] Maintain a runbook for credential management (rotation, recovery, revocation).
- [ ] Record the Vagrant validation path for signed artefacts and map checksums to releases (log roots documented in `docs/OPERATIONS.md`).
- [ ] Run the monthly dependency refresh checklist (`make deps-refresh`, `cargo audit`, `cargo deny`) and attach reports to the tracking issue/PR.
- [ ] Update builder toolchains (Docker images, `cargo-packager`, `rustup` components) in sync with the dependency refresh.
- [ ] Maintain the Vagrant validation playbook (document box versions, run cadence, log archive path) and execute it quarterly.

## Phase 5 – Stretch & Ecosystem
- Core features:
  - [ ] Directory hashing & manifest export/import.
  - [ ] Batch comparison reports (JSON/CSV).
  - [ ] Plugin interface for additional runtimes (Node, WASI, ...).
- Observability & performance:
  - [ ] Add optional structured logging with opt-in telemetry toggle.
  - [ ] Create regression benchmarks (criterion) covering large file hashing.
- Distribution automation:
  - [ ] Integrate reproducible build check (`cargo dist build --dry-run`) into CI.
  - [ ] Produce macOS universal DMG (Intel + Apple Silicon) via multi-target build in CI.

  - [ ] Scaffold winget/homebrew manifest generation with validation job.

## Maintenance & Ops
- [ ] Keep README/docs free of personal machine paths; prefer relative paths or environment variables.
- [ ] Maintain fixture/hash samples for regression QA.
- [ ] Retire the legacy Python implementation once the Rust release is production ready.
- [ ] Plan dependency migrations away from GTK3 bindings and `instant` when replacements stabilise.
- [ ] Monitor the local CI workflow (`make ci-linux-local`) and platform fallback logs; adjust scripts as tooling evolves.
- [ ] Schedule monthly dependency updates (`cargo update`, review yanked crates) and capture results in PR notes.
- [ ] Add scheduled maintenance run for `cargo-dist` upgrades and document checksum asset deduplication in release workflow notes (added 2025-10-13).

## Phase 6 – Governance & Automation
- [ ] Roll out SignPath secrets/variables and confirm signed artefacts in release workflow.
- [ ] Document credential lifecycle (issuance, rotation cadence, emergency revoke playbook).
- [ ] Create scheduled workflows for `make deps-refresh` (monthly) and Vagrant smoke with signing verification (quarterly).
- [ ] Archive release/signing logs under `logs/release-history/<tag>/`.
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
