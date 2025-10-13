# Operations Guide

## Release Checklist
1. Ensure CI (`ci.yml`) is green on the target commit.
2. Confirm the Release Readiness checklist in `docs/PLAN.md` is satisfied.
3. Prepare release notes including:
   - Semantic version (e.g. `v0.4.0`).
   - Summary of changes / primary purpose for the release.
   - Known issues or manual steps (e.g. Gatekeeper bypass, unsigned artefacts when signing is unavailable).
   - Link to `docs/security/VERIFICATION_GUIDE.md` so end users can validate downloads.
   - Signing status: published GPG fingerprint and the outcome of the `windows_sign` (SignPath) job when enabled.
4. Tag the commit (`git tag vX.Y.Z && git push origin vX.Y.Z`).
5. After the automated workflow publishes artefacts:
   - Edit the GitHub Release description with the prepared notes.
   - Verify `.dmg`, `.deb`, and Windows artefacts (portable/installer) execute successfully; when signed, confirm Authenticode/SmartScreen trust.
   - `release.yml` produces `SHA256SUMS` + `SHA256SUMS.sig`; validate the GPG signature and attach both files to the release.
   - Until SignPath is live, explicitly call out in the release notes that Windows artefacts are unsigned and link to `docs/security/VERIFICATION_GUIDE.md`.
6. Update `docs/PLAN.md` / `docs/TASKS.md` if new follow-up work is identified.

Keep this document with the repo to standardise release expectations.

## Build Diagnostics (2025-10-08)

### Successful checks
- `cargo fmt --check` for CLI and GUI crates.
- `cargo clippy --all-targets -- -D warnings` on both crates.
- `cargo test` (CLI crate) and `make rust-test` inside Docker.
- `cargo run --release -- --smoke-test` for the GUI crate.
- `cargo packager --release --formats deb` inside `rust:1.83` Docker after icon fix.

### Common failure modes
- `cargo packager` rejects legacy keys such as `[package.metadata.packager.macos].icons` and `[package.metadata.packager.windows].icon-path`; consolidate under the root `icons` array.
- Debian packaging fails with `Invalid PNG signature` if `docs/assets/icon-hash-checker-*.png` contains JPEG data—re-export icons as real PNGs (`sips -s format png …`).

### Cleanup rule
- Packaging targets run `scripts/cleanup-packaging.sh` by default, removing staging artefacts (`dist/linux`, `rust/hash-checker-gui/target/packager`, `/tmp/hash-checker-*`) once validation completes.
- To retain artefacts for debugging, set `KEEP_PACKAGING=1` before invoking `make dist-linux` or `make cleanup-packaging`.
- Manual cleanup is available via `make cleanup-packaging`.
- `make clean` now calls `scripts/clean.sh`, which handles cross-platform deletion (Rust `target/`, `dist/`, `/tmp/hash-checker-*` plus `${TMPDIR}`/`%TEMP%` mirrors) and prunes Docker volumes unless `CLEAN_DOCKER=0`.

### Platform release targets
- **Windows**: publish both the portable ZIP and NSIS installer. Keep them separate, attach SHA256SUMS, and ensure Vagrant smoke + CI validation passes before tagging.
- **macOS**: build a universal DMG (Intel/ARM). Do not auto-install—deliver the `.dmg` artefact and document the manual drag-and-drop into `/Applications`.
- **Linux**: start with Debian `.deb`; record follow-up tasks for RHEL (`.rpm`) and Arch (`pkg.tar.zst`). Each package must include SHA256SUMS and release notes entries.
- **Build from source**: keep README instructions accurate (`cargo build --release` for CLI/GUI) and verify parity with packaged builds each release cycle.

### Release automation
- Generate a `cargo dist manifest` in CI/release pipelines to provide structured release metadata alongside artefacts.
- Store the resulting manifest under `release-artifacts/dist-manifest.json` for auditing and downstream tooling.
- Routine CI jobs no longer build installers; packaging is confined to the release workflow to preserve runner minutes.

### Signing pipeline (release.yml)
- The optional `windows_sign` job submits executables/installers to SignPath and uploads the signed variants (`windows-*-signed`). If SignPath secrets/variables are absent, the workflow falls back to unsigned artefacts.
- The `publish` job consolidates signed/unsigned artefacts, generates `release-final/SHA256SUMS`, and signs it when `GPG_PRIVATE_KEY` and `GPG_PASSPHRASE` are available.
- Configure secrets/variables as described in `docs/security/CI_SIGNING.md`. Run a `workflow_dispatch` test before shipping a real release.
- Release notes should include the GPG fingerprint and clarify whether Windows artefacts were signed via SignPath or fell back to unsigned binaries.
- Always run the Linux job (default `run_linux=true`) when triggering `release.yml` to keep parity with `ci.yml`.

### Runner & environment strategy
- **Linux runner**: primary automation host. Execute `make ci-linux-local` and other checks inside Docker images (e.g. `rust:1.83`) so the host OS stays clean and reproducible.
- **macOS runner**: dedicated macOS environment (GitHub-hosted or self-managed). Install build deps via Homebrew inside each job; run GUI smoke tests natively.
- **Windows runner**: required for native GUI smoke, NSIS packaging, and SignPath submissions. Cross-compiling from Linux is acceptable for CLI builds but still validate on Windows.
- **Isolation**: each job defines its own `CARGO_TARGET_DIR`/temp directories and archives logs under `logs/` to avoid cross-contamination.
- **Local parity**: when testing locally, prefer Vagrant (on macOS) or WSL2 (on Windows) to mimic CI setups without polluting the developer host.

### Vagrant headless validation
- Use `make rust-gui-smoke` (Vagrant) to launch Windows and Linux smoke environments headlessly; capture logs to `artifacts/vagrant-smoke-<os>.log`.
- Integrate the same target into CI/release workflows so that pre-release artefacts are validated in near-production environments.
- Ensure Vagrant boxes remain up to date (document base box versions and update cadence here when they change).
- Follow the quarterly playbook: refresh base boxes, run `make rust-gui-smoke`, and archive logs under `artifacts/vagrant/<year>-Q<quarter>/`; reference the results in the monthly dependency refresh ticket.
- When releases introduce signing artifacts, pair each log bundle with the tag (e.g. `artifacts/vagrant/v0.4.1/`) and record checksum verification evidence in the release notes.

### Signing automation roadmap
- Windows: integrate SignPath Foundation (public repository, GitHub Actions uploads `.zip`/`.exe`, retrieves signed artefacts before publishing).
- macOS: keep the release unsigned to avoid extra cost; maintain Gatekeeper bypass guidance in README/docs and validate with smoke tests.
- Release workflow: separate the build → (Windows) upload/sign/download → publish steps; for macOS, ship the unsigned `.dmg` and remind users of manual installation.
- Secrets: store `SIGNPATH_ORG`, `SIGNPATH_PROJECT`, and API tokens in GitHub Actions; review their validity regularly.

### CI warning mitigation (2025-10-08)
- Replaced `actions-rs/toolchain@v1` with `dtolnay/rust-toolchain@stable` across workflows to remove the deprecated `set-output` warning.
- Added `brew list` guards before installing `gtk+3`/`pkg-config` so macOS logs no longer emit “pkgconf already installed” noise.

### CI run modes
- **Debug runs (job-specific):** When a matrix job fails, re-run only the affected job via `workflow_dispatch` or GitHub CLI, for example:
  ```bash
  gh workflow run CI \
    --field run_linux=true \
    --field run_macos=false \
    --field run_windows=false
  ```
  Use these targeted runs to iterate quickly; they are not considered valid for releases or merges.
- **Canonical runs (merge/release):** Pushes and pull requests execute macOS and Windows by default. Linux is manual-only—dispatch the job above when cloud validation is required. Branch protection and release workflows depend on the platforms that remain enabled plus any manual Linux run you trigger.
- **Release gating:** The publish workflow remains blocked unless the preceding CI run (with every platform enabled) completes successfully. Never release artefacts produced exclusively by debug-only runs.

### Dependency hygiene
- Treat yanked-crate warnings as blocking. If `cargo install` or local builds mention a yanked version, update immediately using `cargo update -p <crate>` or `cargo upgrade` and refresh `Cargo.lock`.
- Run a dependency refresh at least once per month (`cargo update`, `cargo audit`, `cargo deny`) and capture the findings in the PR description.
- Record major dependency upgrades (GTK, egui, packaging toolchains) in `docs/PLAN.md` and `docs/TASKS.md` so future releases track the migration status.
- Execute `make deps-refresh` during the monthly cycle to automate these commands and capture log output under `logs/deps-refresh-<date>.log`.
- Review Docker base images, `cargo-packager`, and other builder toolchains as part of the same cycle; update script definitions if newer versions are adopted.
- Keep sensitive planning notes under `docs/private/` (gitignored) so public releases contain only sanitized documentation.

### Local Linux CI workflow
- Run `make ci-linux-local` before committing or pushing. This command launches `scripts/ci-linux-local.sh`, which spins up a Docker container (`rust:1.83`) and executes `cargo fmt`, `cargo clippy`, and `cargo test` for both CLI and GUI crates.
- Logs are written to `logs/ci-linux-<timestamp>.log`. Keep the most recent log until the change lands on `main` so troubleshooting has an audit trail.
- Environment overrides:
  - `CI_LINUX_IMAGE` – alternative container name/tag.
  - `CI_LINUX_LOG_DIR` – custom directory for logs.

### Platform fallback policy
- macOS & Windows jobs on GitHub Actions may fail due to platform-specific tooling. If either platform fails **twice in a row** for the same change:
  1. Stop rerunning the cloud job.
  2. Switch to the local workflow:
     - **macOS:** run `make rust-gui-build-host`, `make rust-gui-smoke-host`, and `make rust-gui-dmg-temp`. Capture the console log and attach it to the PR/issue.
     - **Windows:** run the equivalent PowerShell sequence (`cargo fmt`, `cargo clippy`, `cargo test`, `cargo run -- --smoke-test`, packaging via `cargo packager`/NSIS). Save the transcript.
  3. Push again only after the local run succeeds or the regression is resolved.
- Document any fallback run in the PR description or commit message so reviewers know the cloud job was intentionally bypassed.
