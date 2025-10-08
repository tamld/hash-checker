# Hash Checker

[![CI](https://github.com/tamld/hash-checker/actions/workflows/ci.yml/badge.svg)](https://github.com/tamld/hash-checker/actions/workflows/ci.yml)
![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)
![Rust](https://img.shields.io/badge/Rust-1.72%2B-orange?logo=rust)
![Platforms](https://img.shields.io/badge/platforms-Windows%20%7C%20macOS%20%7C%20Linux-4c1)

> Cross-platform integrity checker with a shared Rust core powering CLI and egui desktop apps.

## Table of Contents
- [Overview](#overview)
- [Feature Highlights](#feature-highlights)
- [Quick Start (Containerised)](#quick-start-containerised)
- [Manual Host Build](#manual-host-build)
- [Installer Builds](#installer-builds)
- [Temporary Artefacts](#temporary-artefacts)
- [Continuous Integration](#continuous-integration)
- [Project Documents](#project-documents)

## Overview
Hash Checker delivers reproducible, container-friendly workflows for validating file hashes across Windows, macOS, and Linux. The project is fully Rust-based and uses Docker/Vagrant helpers to keep host systems clean.

## Feature Highlights
- Multiple algorithms: SHA-2 family, SHA-1, MD5, BLAKE2.
- Automatic algorithm detection based on digest length.
- Command-line and egui desktop interfaces sharing the same Rust core.
- Container-first workflows (Docker/Vagrant) to avoid host pollution.

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

## Installer Builds
- Install the packager once per machine: `cargo install cargo-packager@0.11.7 --locked`.
- From `rust/hash-checker-gui/`, run `cargo packager --release --formats dmg` (macOS) or `cargo packager --release --formats deb` (Linux) to produce native installers.
- Windows CI publishes a portable `hash-checker-windows-portable.zip` archive.
- macOS artefacts are currently unsigned; users can Control-click → **Open** or clear quarantine with `xattr -d com.apple.quarantine "/Applications/Hash Checker.app"`.

## Temporary Artefacts
Need binaries quickly without touching the repo tree?

```bash
make rust-build-temp
ls /tmp/hash-checker-build
```
Produces `hash-checker`, `hash-checker-gui`, and `SHA256SUMS` in `/tmp/hash-checker-build`.

## Continuous Integration
- `.github/workflows/ci.yml` runs sequentially: Linux → macOS → Windows.
- Each job performs formatting, linting, tests, release builds, and GUI smoke tests, then publishes installers with checksums.
- Docker helpers ensure build outputs remain accessible between steps.

## Project Documents
- `docs/PLAN.md` – development roadmap.
- `docs/TASKS.md` – actionable task list per phase.
- `docs/BACKLOG.md` – backlog and long-term improvements.
- `docs/SECURITY_ROADMAP.md` – staged security work.
- `docs/GUI_DECISION.md` / `docs/GUI_MVP_DESIGN.md` – GUI architecture notes.
- `.agent/AGENTS.md` – operational guidelines for assistants.
