# Hash Checker

[![CI](https://github.com/tamld/hash-checker/actions/workflows/ci.yml/badge.svg)](https://github.com/tamld/hash-checker/actions/workflows/ci.yml)
![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)
![Rust](https://img.shields.io/badge/Rust-1.88%2B-orange?logo=rust)

> Cross-platform integrity checker with a shared Rust core powering CLI and egui desktop apps.

![Hash Checker Slate theme](docs/assets/gui-main.png)

Hash Checker delivers reproducible workflows for validating file hashes on Windows, macOS, and Linux. The project emphasises clean builds (Docker/Vagrant helpers) and an accessible GUI that mirrors the CLI feature set.

## Highlights
- Supports SHA-2, SHA-1, MD5, and BLAKE2 families with automatic algorithm detection from hash prefixes.
- Shared Rust core exposes both command-line and egui desktop interfaces.
- Batch comparison reports for CI: feed JSON/CSV definitions to `hash-checker batch` and export structured results.
- Linux builds now rely on the XDG desktop portal backend—no more GTK runtime dependency for file dialogs.
- Experimental GTK4-native dialog mode (Linux) is available via the `gtk4-native` feature flag; see `docs/OPERATIONS.md` for prerequisites.
- Directory manifest export/verify with JSON, CSV, and TXT outputs.
- Container-first `make` targets keep builds reproducible across platforms, while host builds remain available.
- Accessibility extras: keyboard shortcuts, clipboard integration, and Slate/High Contrast themes.

## Getting Started
- Clone: `git clone https://github.com/tamld/hash-checker.git && cd hash-checker` (agents may work from `local-scripts/hash-checker/` inside the workspace layout).
- Container workflow: `make rust-test`, `make rust-gui-build`, `make rust-gui-smoke`, and `make cleanup-packaging`. See `docs/OPERATIONS.md` for the full command matrix.
- Host build: `cargo build --release --manifest-path rust/hash-checker/Cargo.toml` plus the GUI variant (`rust/hash-checker-gui/`) or the equivalent `make` targets documented in `docs/OPERATIONS.md`.
- Linux GUI runs via the XDG desktop portal. Install `xdg-desktop-portal` (and a backend such as `xdg-desktop-portal-gtk`) if your distro does not enable it by default.
- Local gate: run `make ci-linux-local` before pushing to exercise fmt, clippy, tests, and Docker workflows.

## Documentation
- `docs/OPERATIONS.md` – quick start, build/packaging commands, CI modes, Gatekeeper notes.
- `docs/PLAN.md` – roadmap and phase status.
- `docs/TASKS.md` – near-term work queue.
- `docs/BACKLOG.md` – longer-term ideas.
- `docs/security/VERIFICATION_GUIDE.md` – checksum, GPG, and Authenticode verification.
- `docs/GUI_SCREENSHOT.md` – screenshot checklist and gallery.

## Verify & Support
- Stable releases live on the [GitHub Releases page](https://github.com/tamld/hash-checker/releases). Installer and binary packages remain unsigned until SignPath onboarding completes.
- Follow `docs/security/VERIFICATION_GUIDE.md` to validate downloads; macOS Gatekeeper bypass commands are mirrored in `docs/OPERATIONS.md`.
- File bugs or feature requests through [GitHub Issues](https://github.com/tamld/hash-checker/issues) and reference the relevant documentation or `.agents/` notes for evidence.

## Status
- `main` is gated by `ci.yml` (fmt, clippy, unit tests, Docker builds, GUI smoke). Packaging jobs run via `release.yml` when preparing artefacts.
- Internal automation checklists and lessons live under the git-ignored `.agents/` workspace; sync them after every session.
