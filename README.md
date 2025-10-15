# Hash Checker

[![CI](https://github.com/tamld/hash-checker/actions/workflows/ci.yml/badge.svg)](https://github.com/tamld/hash-checker/actions/workflows/ci.yml)
![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)
![Rust](https://img.shields.io/badge/Rust-1.83%2B-orange?logo=rust)
![Platforms](https://img.shields.io/badge/platforms-Windows%20%7C%20macOS%20%7C%20Linux-4c1)

> Cross-platform integrity checker with a shared Rust core powering CLI and egui desktop apps.

## Table of Contents
- [Overview](#overview)
- [Feature Highlights](#feature-highlights)
- [Clone & Workspace Setup](#clone--workspace-setup)
- [Quick Start (Containerised)](#quick-start-containerised)
- [Manual Host Build](#manual-host-build)
- [Installer Builds](#installer-builds)
- [Verify Downloads](#verify-downloads)
- [Release & Changelog](#release--changelog)
- [Temporary Artefacts](#temporary-artefacts)
- [Continuous Integration](#continuous-integration)
- [Project Documents](#project-documents)

## Overview
Hash Checker delivers reproducible, container-friendly workflows for validating file hashes across Windows, macOS, and Linux. The project is fully Rust-based and uses Docker/Vagrant helpers to keep host systems clean.

## GUI Preview
![Hash Checker Slate theme](docs/assets/gui-main.png)
*Slate theme (default) with copy-to-clipboard shortcut and status banner.*

Key interface states:

![Algorithm dropdown auto-selected from prefix](docs/assets/gui-algorithm.png)
![Successful verification state](docs/assets/gui-match.png)
![Verification mismatch warning](docs/assets/gui-mismatch.png)
![High Contrast mode](docs/assets/gui-high-contrast.png)
![Theme selector (Slate, Soft Light, High Contrast)](docs/assets/gui-theme-slate.png)

> Screenshot update workflow is documented in `docs/GUI_SCREENSHOT.md`.

## Feature Highlights
- Multiple algorithms: SHA-2 family, SHA-1, MD5, BLAKE2.
- Automatic algorithm detection based on digest length.
- Command-line and egui desktop interfaces sharing the same Rust core.
- Container-first workflows (Docker/Vagrant) to avoid host pollution.
- Built-in clipboard workflow (copy computed hashes) and accessibility toggles for the GUI.
- Theme picker for the GUI (Soft Light, Slate, High Contrast Dark).
- Warns when pasted hashes use unsupported prefixes so users can correct them before verification.
- Copy/paste aware hash comparison: copy buttons include algorithm prefixes and pasting prefixed values auto-selects the algorithm.

## Clone & Workspace Setup
```bash
# Clone the public repository
git clone https://github.com/tamld/hash-checker.git
cd hash-checker

# (Optional) work inside the assistant workspace layout
cd local-scripts/hash-checker
```
> Note: all scripts/Make targets assume you are inside the repository folder (`hash-checker/` or `local-scripts/hash-checker/`). Update your own remote/fork if you are contributing from a fork.

## Quick Start (Containerised)
Prerequisites: Docker (build/test) and Vagrant + VMware Fusion (optional, for headless GUI smoke tests).

| Command | Purpose |
| --- | --- |
| `make rust-test` | Run Rust CLI unit/integration tests inside Docker |
| `make rust-build` | Build Rust CLI release binary in Docker |
| `make rust-gui-build` | Build Rust GUI release binary in Docker (installs GTK) |
| `make rust-gui-smoke` | Launch Vagrant VM and run `cargo run -- --smoke-test` |
| `make rust-build-temp` | Build CLI+GUI in Docker and copy artefacts to `/tmp/hash-checker-build` |
| `make clean` | Remove build artefacts and prune Docker volumes |
| `make cleanup-packaging` | Remove packaging staging directories (`dist/linux`, `target/packager`, `/tmp/hash-checker-*`) |

> Tip: keep build outputs out of the repo workspace—set `CARGO_TARGET_DIR` to an OS-specific temp path (for example `/tmp/hash-checker-target` on Linux/macOS or `%TEMP%\hash-checker-target` on Windows) before running Docker/host builds so sync tools do not lock intermediate files, and delete that temp directory or run `make clean` once finished.

## Manual Host Build
Prefer building locally? With Rust installed:

```bash
# CLI
cargo build --release --manifest-path rust/hash-checker/Cargo.toml

# GUI (ensure system GTK deps on Linux/macOS)
cargo build --release --manifest-path rust/hash-checker-gui/Cargo.toml
cargo run --release --manifest-path rust/hash-checker-gui/Cargo.toml -- --smoke-test
```

Equivalent Make targets:
```bash
make rust-build-host
make rust-gui-build-host
make rust-gui-smoke-host
```

## CLI Logging Modes
- Theo mặc định, CLI chỉ in kết quả kiểm tra. Khi cần theo dõi tiến trình hoặc tích hợp hệ thống log, bật structured logging qua `--log-format`:

```bash
hash-checker path/to/file.txt EXPECTED_HASH --log-format text   # log dạng text
hash-checker path/to/file.txt EXPECTED_HASH --log-format json   # log dạng JSON
```

- Log được ghi ra `stderr`, còn `stdout` giữ nguyên thông báo kết quả nên các script/CI cũ không bị ảnh hưởng.

## Benchmark
- Sử dụng Criterion để theo dõi hiệu năng hashing:

```bash
cargo bench --manifest-path rust/hash-checker/Cargo.toml
```

- Benchmarks tạo dữ liệu 1/10/50 MB và đo SHA-256, SHA-512, BLAKE2s, BLAKE2b. Kết quả nằm trong `target/criterion/` (có HTML, CSV) để so sánh giữa các commit.

## Installer Builds
- Install the packager once per machine: `cargo install cargo-packager@0.11.7 --locked`.
- From `rust/hash-checker-gui/`, run `cargo packager --release --formats dmg` (macOS) or `cargo packager --release --formats deb appimage pacman` (Linux) to produce native installers.
- Windows CI publishes both the portable `hash-checker-windows-portable.zip` archive and an NSIS installer executable.
- Windows packaging consumes the multi-resolution icon at `docs/assets/icon-hash-checker.ico` so the executable/installer display the Hash Checker branding.
- macOS artefacts are currently unsigned; users can Control-click → **Open** or clear quarantine with `xattr -d com.apple.quarantine "/Applications/Hash Checker.app"`.

## Verify Downloads
- Each release ships a `SHA256SUMS` file; download it alongside the installer/binary.
- On macOS/Linux run `shasum -a 256 <artefact>` and compare with the recorded digest (`grep <artefact> SHA256SUMS`).
- On Windows run `Get-FileHash <artefact> -Algorithm SHA256` or use the CLI binary and the recorded digest.
- Full command examples for every platform (including upcoming signature validation) live in `docs/security/VERIFICATION_GUIDE.md`.
- macOS build ships a universal (Intel + Apple Silicon) DMG; see Gatekeeper notes below for unsigned installs.
- Current status: macOS DMG is unsigned and Windows artefacts remain unsigned while SignPath onboarding is pending. Always verify with the guide above. Hash inputs may include prefixes like `sha256:<digest>`; the app auto-detects the algorithm and updates the dropdown automatically.

### macOS Gatekeeper (Unsigned Build)

Gatekeeper warns when launching an unsigned DMG/app. Follow these steps to run the current release:

1. Download and mount the DMG from the Hash Checker release page.
2. In Finder, right-click `Hash Checker.app` and choose **Open**.
3. When the warning dialog appears, click **Open** again to confirm.

To remove the quarantine flag manually (e.g., after copying to `/Applications`):

```bash
xattr -d com.apple.quarantine "/Applications/Hash Checker.app"
```

This guidance will be updated once SignPath signing or notarisation is available.

## Release & Changelog
- Latest release: **[v0.1.4](https://github.com/tamld/hash-checker/releases/tag/v0.1.4)** – ships the first universal macOS DMG, aligns crate versions, and publishes packaging evidence (logs and checksums).
- Previous releases are listed on the [GitHub Releases](https://github.com/tamld/hash-checker/releases) page. Follow the checklist in `docs/OPERATIONS.md` for release readiness.
- A structured changelog will be introduced in future releases; for now, consult the release notes for key highlights and verification steps.

## Temporary Artefacts
Need binaries quickly without touching the repo tree?

```bash
make rust-build-temp
ls /tmp/hash-checker-build
```
Produces `hash-checker`, `hash-checker-gui`, and `SHA256SUMS` in `/tmp/hash-checker-build`.

Platform-specific packages can also be generated into temp directories:

```bash
make rust-gui-dmg-temp        # -> /tmp/hash-checker-gui/*.dmg (macOS)
make rust-linux-deb-temp      # -> /tmp/hash-checker-deb/*.deb (Linux build from mac host via cargo-packager)
make rust-windows-zip-temp    # -> ${TMPDIR:-/tmp}/hash-checker-win/hash-checker-windows-portable.zip (use %TEMP%\hash-checker-win on native Windows)
```

## Continuous Integration
- `.github/workflows/ci.yml` runs macOS and Windows on every push/PR. Linux is opt-in and should be triggered manually via `gh workflow run` once the local gate passes.
- Each job now focuses on formatting, linting, unit/smoke tests. Packaging of installers happens only in the release workflow (tags or manual dispatch).
- Docker helpers ensure build outputs remain accessible between steps.
- Before pushing, run `make ci-linux-local` to execute fmt/clippy/tests inside a Docker container and store logs under `logs/`.
- Execute `make deps-refresh` during the monthly maintenance cadence to update dependencies and log security scans.
- Store any sensitive planning docs under `docs/private/` (ignored by git) so the public repo remains clean.

## Project Documents
- Roadmap overview: `docs/PLAN.md`
- Active task tracker: `docs/TASKS.md`
- Backlog & long-term ideas: `docs/BACKLOG.md`
- Security strategy: `docs/SECURITY_ROADMAP.md`
- Threat model summary: `docs/security/THREAT_MODEL.md`
- Download verification (checksum, GPG, Authenticode): `docs/security/VERIFICATION_GUIDE.md`
- CI signing integration (SignPath + GPG): `docs/security/CI_SIGNING.md`
- Release operations & runner strategy: `docs/OPERATIONS.md`
- Windows signing playbook: `docs/SIGNING.md`
- Dependency migration plan: `docs/DEPENDENCY_MIGRATION.md`
- GUI architecture notes: `docs/GUI_DECISION.md`, `docs/GUI_MVP_DESIGN.md`
- Repo guardrails: `docs/PROJECT_STATUS.md` (public overview). Internal automation notes live in the git-ignored `.agents/` workspace.
- Contributor behaviour: `CODE_OF_CONDUCT.md`
