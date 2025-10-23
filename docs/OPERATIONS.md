# Operations Guide

> Multi-agent workflow: this project adopts the global `policy/multi_agent_delivery`. Use CARE specs under `specs/`, keep `.agents/branch_progress.yml` current, run `.agents/scripts/validate_handoff.sh` before handoffs, and record entries in `.agents/metrics_log.yml` after each PR.

## Developer Quick Reference
> Minimum supported Rust version (MSRV): **1.88.0**


### Clone & Workspace Layout
```bash
# Clone the public repository
git clone https://github.com/tamld/hash-checker.git
cd hash-checker

# (Optional) work inside the assistant workspace layout
cd local-scripts/hash-checker
```

> Most scripts/Make targets expect you to run them from the repository root (`hash-checker/` or `local-scripts/hash-checker/`). Update the remote when working from a fork.

### Container Quick Start
Prerequisites: Docker for build/test; Vagrant + VMware Fusion (optional) for headless GUI smoke tests.

| Command | Purpose |
| --- | --- |
| `make rust-test` | Run Rust CLI unit/integration tests inside Docker |
| `make rust-build` | Build Rust CLI release binary in Docker |
| `make rust-gui-build` | Build Rust GUI release binary in Docker (ensures XDG desktop portal dependencies) |
| `make rust-gui-smoke` | Launch Vagrant VM and run `cargo run -- --smoke-test` |
| `make rust-build-temp` | Build CLI + GUI in Docker and copy artefacts to `/tmp/hash-checker-build` |
| `make clean` | Remove build artefacts and prune Docker volumes |
| `make cleanup-packaging` | Remove packaging staging directories (`dist/linux`, `target/packager`, `/tmp/hash-checker-*`) |

> Recommendation: export `CARGO_TARGET_DIR` to an OS-specific temp path (for example `/tmp/hash-checker-target` or `%TEMP%\hash-checker-target`) so sync tools do not lock intermediate files. Clean up the directory or run `make clean` after finishing.

### Host Builds (Rust Installed)
```bash
# CLI
cargo build --release --manifest-path rust/hash-checker/Cargo.toml

# GUI (ensure XDG desktop portal on Linux; install pkg-config on macOS)
cargo build --release --manifest-path rust/hash-checker-gui/Cargo.toml
cargo run --release --manifest-path rust/hash-checker-gui/Cargo.toml -- --smoke-test

# Equivalent Make targets
make rust-build-host
make rust-gui-build-host
make rust-gui-smoke-host

#### GTK4-native (tùy chọn, Linux)
- Tính năng đang được thử nghiệm qua feature `gtk4-native`. Vì môi trường local (macOS) không có GTK4, hãy bật/tắt và kiểm tra trên runner Linux (GitHub Actions) hoặc VM dành riêng.
- Cài đặt gói cần thiết trước khi build:
  ```bash
  sudo apt-get update
  sudo apt-get install libgtk-4-dev libadwaita-1-dev libglib2.0-dev
  cargo check --manifest-path rust/hash-checker-gui/Cargo.toml --features gtk4-native
  ```
- Khi chạy trong CI, thêm bước cài gói trên runner Ubuntu và dùng `cargo run --release --manifest-path rust/hash-checker-gui/Cargo.toml --features gtk4-native -- --smoke-test`.
- Flatpak và môi trường sandbox vẫn phải sử dụng portal (feature mặc định). Snapshots/logs GTK4 cần lưu dưới `logs/qa/gtk4-<date>.md`.
```

### CLI Logging & Manifests
- By default the CLI prints only the verification outcome. Use `--log-format text|json` when you need structured progress information; logs write to `stderr` so `stdout` stays script-friendly.
- Export directory manifests with `hash-checker manifest export <path> -o <file> -r` (JSON default). Verify with `hash-checker manifest verify <file>`.
- Helpful flags: `--format csv|txt`, `--algorithm <algo>`, `--root <path>` (when verifying from a different base directory), `--report-limit <n>` to cap mismatch summaries.

### Batch Comparison Reports
- Define expected hashes in JSON or CSV and feed them to the batch command:

  ```json
  [
    { "path": "dist/hash-checker", "expected": "sha256:<digest>" },
    { "path": "README.md", "expected": "1097…", "algorithm": "md5" }
  ]
  ```

- Run the CLI and capture a structured report:

  ```bash
  hash-checker batch --input hashes.json --output report.json --output-format json
  hash-checker batch --input hashes.csv --input-format csv --output report.csv --output-format csv
  ```

- Exit codes: `0` (all matched), `3` (mismatched/missing entries), `2` (errors such as unsupported algorithms or I/O failures).
- Reports include a summary block plus `entries[]` with `status` (`match`, `mismatch`, `missing`, `error`) and the computed hash when applicable, making CI assertions straightforward.

### Distribution Automation
- Workflow **Distribution Dry Run** (`.github/workflows/dist-validation.yml`) runs weekly (Mon 06:00 UTC) and on demand:
  - Job 1 installs `cargo-dist@0.30.0`, runs `dist plan --output-format json`, captures `dist-manifest.json`, và lưu cả hai artefact để review.
  - Job 2 installs Debian deps, invokes `scripts/debian-smoke.sh`, and uploads the generated `.deb` plus CLI smoke logs.
- Reproduce locally:

  ```bash
  sudo apt-get install libasound2-dev xdg-desktop-portal xvfb
  ./scripts/debian-smoke.sh
  ```

  The script builds the `.deb`, installs it (using `sudo` when available), and runs `hash-checker --version` plus `hash-checker-gui -- --smoke-test` (through `xvfb` when available). Logs are written under `logs/cli-snapshots/`.

- Build toàn bộ matrix artefact trên macOS (yêu cầu Zig, `cargo-xwin`, `cargo-zigbuild`, `rust-src` và ưu tiên `~/.cargo/bin` trong `PATH`):

  ```bash
  brew install zig
  cargo install cargo-zigbuild@0.18.2 --locked
  cargo install cargo-xwin@0.17.0 --locked
  rustup component add rust-src
  PATH="$HOME/.cargo/bin:$PATH" dist build --artifacts=local
  ```

  Lệnh trên tạo đủ bộ `tar.xz`/`zip` cho macOS (x86_64 + arm64), Linux và Windows dưới `target/distrib/`. Nếu dùng `brew` cargo (không phải rustup), preferring `~/.cargo/bin` như trên là bắt buộc để tránh thiếu `rust-std` cross-target.

### Benchmarks
Run Criterion benchmarks to track hashing performance:
```bash
cargo bench --manifest-path rust/hash-checker/Cargo.toml
```
Benchmarks generate 1/10/50 MB samples and record SHA-256/512, BLAKE2s/B results in `target/criterion/` (HTML + CSV).

### Installer Builds
- Install `cargo-packager` once: `cargo install cargo-packager@0.11.7 --locked`.
- From `rust/hash-checker-gui/` run `cargo packager --release --formats dmg` (macOS) or `cargo packager --release --formats deb appimage pacman` (Linux).
- Windows CI publishes both the portable ZIP and NSIS installer. Packaging uses `docs/assets/icon-hash-checker.ico` for application branding.
- macOS artefacts are currently unsigned; see Gatekeeper notes below or in the README.

### Temporary Artefacts & Cleanup
Use `make rust-build-temp` to stage CLI + GUI binaries under `/tmp/hash-checker-build`. Platform-specific helpers:
```bash
make rust-gui-dmg-temp        # -> /tmp/hash-checker-gui/*.dmg (macOS)
make rust-linux-deb-temp      # -> /tmp/hash-checker-deb/*.deb (Linux build from mac host via cargo-packager)
make rust-windows-zip-temp    # -> ${TMPDIR:-/tmp}/hash-checker-win/hash-checker-windows-portable.zip (%TEMP% on Windows)
```
Follow up with `make cleanup-packaging` to remove staging artefacts.

### Verify Downloads
- Each release publishes `SHA256SUMS`; download it alongside the artefact and validate with `shasum -a 256`, `Get-FileHash`, or the CLI.
- For full platform instructions (including GPG and Authenticode verification) follow `docs/security/VERIFICATION_GUIDE.md`.
- macOS builds remain unsigned; Gatekeeper bypass commands are documented below until notarisation is available.

#### macOS Gatekeeper (Unsigned Builds)
1. Mount the DMG from the release page.
2. In Finder, right-click `Hash Checker.app` and choose **Open**.
3. When the warning dialog appears, click **Open** again to confirm.

Clear the quarantine flag manually after copying to `/Applications`:
```bash
xattr -d com.apple.quarantine "/Applications/Hash Checker.app"
```

### Local CI Gate
- Run `make ci-linux-local` before pushing to execute fmt, clippy, tests, and Docker workflows.
- Monthly maintenance: `make deps-refresh` updates dependencies, runs `cargo outdated --workspace --exit-code 1 --root-deps-only` (report) plus `cargo audit`/`cargo deny`, and refreshes cached toolchains. Capture logs under `logs/` and link the report in the monthly maintenance ticket.
- See **CI Modes (2025-10-19)** for the mapping between local checks and GitHub Actions.

### Documentation Index
- Roadmap: `docs/PLAN.md`
- Tasks: `docs/TASKS.md`
- Backlog: `docs/BACKLOG.md`
- Security roadmap: `docs/SECURITY_ROADMAP.md`
- Threat model: `docs/security/THREAT_MODEL.md`
- Verification guide: `docs/security/VERIFICATION_GUIDE.md`
- Signing/credential playbooks: `docs/SIGNING.md`, `docs/security/CREDENTIAL_RUNBOOK.md`
- GUI design: `docs/GUI_DECISION.md`, `docs/GUI_MVP_DESIGN.md`
- Release operations: this file (`docs/OPERATIONS.md`)
- Contributor conduct: `CODE_OF_CONDUCT.md`

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
5. If any secrets/certificates change, append an entry using `docs/security/CREDENTIAL_RUNBOOK.md` (onboarding, rotation, incident).
6. After the automated workflow publishes artefacts:
   - Edit the GitHub Release description with the prepared notes.
   - Verify `.dmg`, `.deb`, and Windows artefacts (portable/installer) run successfully; follow `docs/vagrant/VALIDATION_PLAYBOOK.md` to capture smoke-test logs.
   - For manual Vagrant validation, export `VAGRANT_DEFAULT_PROVIDER=vmware_fusion` (or the provider you have installed), run through the playbook, and archive the resulting logs under `logs/release-history/<tag>/vagrant/`.
   - Attach **only** the curated deliverables to the release: `hash-checker-gui-setup.exe`, `hash-checker-windows-portable.zip`, `Hash.Checker.dmg`, `hash-checker-gui_<version>_amd64.deb`, `hash-checker-gui_<version>_x86_64.AppImage`, `hash-checker-gui_<version>_x86_64.tar.gz`, and the consolidated `SHA256SUMS` file. Do not upload build logs or per-platform checksum files; keep those in workflow artefacts if you need them for triage.
   - `release.yml` produces a consolidated `SHA256SUMS` file (and `SHA256SUMS.sig` when signing secrets are present); validate the checksum/signature pair before attaching them to the release.
   - Until SignPath is live, explicitly call out in the release notes that Windows artefacts are unsigned and link to `docs/security/VERIFICATION_GUIDE.md`.
   - When the workflow definition changes, run `gh workflow run release.yml --ref <branch-or-tag>` to verify the pipeline in GitHub Actions before tagging; record the dispatch run ID in the corresponding issue/PR for traceability.
7. Update `docs/PLAN.md` / `docs/TASKS.md` if new follow-up work is identified.

Keep this document with the repo to standardise release expectations.

## CI Modes (2025-10-19)
- **Push / Pull Request**: `ci.yml` runs fmt, clippy, unit tests, Docker tests, and GUI smoke. No packaging steps are executed; this keeps the quick feedback loop green.
- **Workflow Dispatch (manual trigger)**: same set of checks as above. As of 2025-10-19, `cargo-dist` validation is *not* executed here; packaging remains in the release workflow to avoid redundant failures.
- **Release workflow (`release.yml`)**: runs the full packaging matrix (cargo-packager, installers, AppImage, etc.) and should be used when preparing artefacts for publication.

> Note: `cargo-dist` was removed from the manual CI run after repeated failures caused by missing target definitions. Packaging is validated in `release.yml`; use that workflow to exercise installers before tagging.

## Build Diagnostics (2025-10-08)

### Successful checks
- `cargo fmt --check` for CLI and GUI crates.
- `cargo clippy --all-targets -- -D warnings` on both crates.
- `cargo test` (CLI crate) and `make rust-test` inside Docker.
- `cargo run --release -- --smoke-test` for the GUI crate.
- `cargo packager --release --formats deb` inside `rust:1.88` Docker after icon fix.

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
- **macOS**: target a universal DMG (Intel/ARM). Use `make macos-dmg-universal` (scripted via `scripts/macos-universal-dmg.sh`) to build both slices, combine with `lipo`, and generate the DMG locally. Keep Gatekeeper guidance until signing is available.
- **Linux**: start with Debian `.deb`; record follow-up tasks for RHEL (`.rpm`) and Arch (`pkg.tar.zst`). Each package must include SHA256SUMS and release notes entries.
- **Build from source**: keep README instructions accurate (`cargo build --release` for CLI/GUI) and verify parity with packaged builds each release cycle.

### Release automation
- Routine CI jobs no longer build installers; packaging is confined to the release workflow to preserve runner minutes.
- `release.yml` installs `cargo-dist@0.30.0` (verified 2025-10-16); keep the version in sync with `.github/workflows/ci.yml` and note any upstream regressions in the changelog.

### Signing pipeline (release.yml)
- The optional `windows_sign` job submits executables/installers to SignPath and uploads the signed variants (`windows-*-signed`). If SignPath secrets/variables are absent, the workflow falls back to unsigned artefacts.
- The `publish` job consolidates signed/unsigned artefacts, generates `release-final/SHA256SUMS`, and signs it when `GPG_PRIVATE_KEY` and `GPG_PASSPHRASE` are available.
- Configure secrets/variables as described in `docs/security/CI_SIGNING.md`. Run a `workflow_dispatch` test before shipping a real release.
- Release notes should include the GPG fingerprint and clarify whether Windows artefacts were signed via SignPath or fell back to unsigned binaries.
- Always run the Linux job (`run_linux=true`) when triggering `release.yml` to keep parity with `ci.yml`; record the run ID in the release issue when dispatching manual validations.
- macOS builds publish a universal (Intel + Apple Silicon) DMG; retain `scripts/macos-universal-dmg.sh` for local smoke tests or incident response.

### Runner & environment strategy
- **Linux runner**: primary automation host. Execute `make ci-linux-local` and other checks inside Docker images (e.g. `rust:1.88`) so the host OS stays clean and reproducible.
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
- Pinned GitHub Actions to Rust 1.88.0 via `dtolnay/rust-toolchain@master` to keep CI in lockstep with the MSRV.
- Removed legacy GTK installation steps; Linux now depends on the XDG desktop portal and macOS only installs `pkg-config` when missing.

### CI run modes (2025-10-19)

| Mode | Trigger | Purpose | Steps | Notes |
| --- | --- | --- | --- | --- |
| Push / PR | Automatic on push & pull request | Fast feedback cycle | fmt, clippy, unit tests, GUI tests, bench, Docker smoke | Packaging bỏ qua để giữ runtime < 10 phút; chỉ dùng nhãn `skip-linux-ci` cho PR doc-only |
| Dispatch – fast | `workflow_dispatch` (defaults: `run_packaging=false`) | Rerun failed jobs quickly | Same set as Push / PR | Dùng khi cần rerun thủ công mà không rehearsal packaging |
| Dispatch – packaging rehearsal | `workflow_dispatch` with `run_packaging=true` | Dress rehearsal trước khi tag release | Push / PR steps + `dist plan`, `dist manifest` | Ghi lại run URL + dist manifest vào issue release |
| Release workflow | Tag push hoặc manual dispatch `release.yml` | Build & publish artefacts | Full packaging + signing | Nguồn artefact chính thức để phát hành |

- **Debug runs (job-specific):** Khi một job matrix fail, chạy `gh workflow run CI --field run_linux=true --field run_macos=false --field run_windows=false` để lặp nhanh. Các run này không đủ điều kiện merge/release.
- **Canonical runs (merge/release):** Trước khi merge nhánh chuẩn bị release, đảm bảo ít nhất một run “Dispatch – packaging rehearsal” xanh để `cargo-dist` được rehearsal.

#### Sử dụng nhãn `skip-linux-ci` an toàn
1. **Phạm vi tệp được phép:** chỉ áp dụng cho PR chỉnh sửa tài liệu hoặc metadata thuần văn bản (ví dụ `README.md`, `docs/**/*.md`, `CHANGELOG.md`). Nếu đụng tới `rust/**`, `scripts/**`, workflow `.yml`, hay bất kỳ mã nguồn/build script nào, *không* được gắn nhãn.
2. **Tự kiểm tra thay đổi:** dùng `git diff --stat` hoặc `gh pr view --files` để chắc chắn các đường dẫn đều thuộc danh sách an toàn. Reviewer phải xác nhận lại trước khi merge.
3. **Gắn/thu hồi nhãn:** thêm nhãn `skip-linux-ci` vào PR để bỏ qua job. Nếu cần chạy lại Linux CI (ví dụ trước khi merge), chỉ việc xoá nhãn rồi dùng `gh workflow run CI --field run_linux=true`.
4. **Bổ sung bằng chứng (khuyến nghị):** với PR tài liệu nhạy cảm (hướng dẫn build, vận hành), chạy `make ci-linux-local` và đính kèm log để có chứng cứ dù job Linux bị skip.
- **Release gating:** The publish workflow remains blocked unless the preceding CI run (with every platform enabled) completes successfully. Never release artefacts produced exclusively by debug-only runs.

### Dependency hygiene
- Treat yanked-crate warnings as blocking. If `cargo install` or local builds mention a yanked version, update immediately using `cargo update -p <crate>` or `cargo upgrade` and refresh `Cargo.lock`.
- Run a dependency refresh at least once per month (`cargo update`, `cargo audit`, `cargo deny`) and capture the findings in the PR description.
- Record major dependency upgrades (GTK, egui, packaging toolchains) in `docs/PLAN.md` and `docs/TASKS.md` so future releases track the migration status.
- Execute `make deps-refresh` during the monthly cycle to automate these commands and capture log output under `logs/deps-refresh-<date>.log`.
- Review Docker base images, `cargo-packager`, and other builder toolchains as part of the same cycle; update script definitions if newer versions are adopted.
- Keep sensitive planning notes under `docs/private/` (gitignored) so public releases contain only sanitized documentation.

### Local Linux CI workflow
- Run `make ci-linux-local` before committing or pushing. This command launches `scripts/ci-linux-local.sh`, which spins up a Docker container (`rust:1.88`) and executes `cargo fmt`, `cargo clippy`, and `cargo test` for both CLI and GUI crates.
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


## Signing & Credential Notes
- Windows signing: see `docs/SIGNING.md` once SignPath is enabled.
- Credential management: follow `docs/security/CREDENTIAL_RUNBOOK.md`.
- macOS builds are unsigned; follow the Gatekeeper section in the README.


## Distribution Manifests
- Generate templates with `scripts/generate_manifests.sh <version> <download-base-url>`.
- Fill in the actual checksums (`logs/release-history/<tag>/SHA256SUMS`).
- Submit PRs to the appropriate tap/package feed after manual validation.


## Maintenance Automation
- Workflow `.github/workflows/cargo-dist-maintenance.yml` runs monthly to verify the latest `cargo-dist` via `cargo install --locked`.
- Workflow `.github/workflows/deps-refresh.yml` executes `make deps-refresh`, runs `cargo outdated` (report mode) + `cargo audit`/`cargo deny`, refreshes the toolchain (`rustup update`, `docker pull`, `cargo-packager`), and stores logs in the workflow artefact for traceability.
- Workflow `.github/workflows/vagrant-smoke-reminder.yml` triggers every quarter (and on demand) to open a reminder issue for manual Vagrant smoke testing.
- If any workflow fails, open an issue, attach the artefact (`cargo-dist-install-log` or `deps-refresh-log`), fix the pipeline, and update maintenance notes.


## Vagrant Smoke Log
- Follow `docs/vagrant/VALIDATION_PLAYBOOK.md` when running smoke tests.
- After each run, copy transcripts/logs into `logs/release-history/<tag>/` and record the summary using `docs/vagrant/RELEASE_LOG_TEMPLATE.md`.
