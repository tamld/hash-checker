# Operations Guide

## Release Checklist
1. Ensure CI (`ci.yml`) is green on the target commit.
2. Confirm the Release Readiness checklist in `docs/PLAN.md` is satisfied.
3. Prepare release notes including:
   - Semantic version (e.g. `v0.4.0`).
   - Summary of changes / primary purpose for the release.
   - Known issues or manual steps (Gatekeeper bypass, etc.).
4. Tag the commit (`git tag vX.Y.Z && git push origin vX.Y.Z`).
5. After the automated workflow publishes artefacts:
   - Edit the GitHub Release description with the prepared notes.
   - Verify `.dmg`, `.deb`, and Windows `.zip` artefacts download and launch successfully.
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

### Vagrant headless validation
- Use `make rust-gui-smoke` (Vagrant) to launch Windows and Linux smoke environments headlessly; capture logs to `artifacts/vagrant-smoke-<os>.log`.
- Integrate the same target into CI/release workflows so that pre-release artefacts are validated in near-production environments.
- Ensure Vagrant boxes remain up to date (document base box versions and update cadence here when they change).

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
- **Canonical runs (merge/release):** Pushes and pull requests must execute the full matrix (Linux, macOS, Windows). Branch protection and release workflows depend on all three jobs passing before artefacts are published or tags are created.
- **Release gating:** The publish workflow remains blocked unless the preceding CI run (with every platform enabled) completes successfully. Never release artefacts produced exclusively by debug-only runs.
