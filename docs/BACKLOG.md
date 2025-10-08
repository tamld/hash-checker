# Backlog

## Table of Contents
- [Features](#features)
- [Improvements](#improvements)
- [CI & Release Automation](#ci--release-automation)
- [Security Hardening](#security-hardening)

## Features
- History panel for recent hashes and validation results.
- Batch and multi-file queue support with progress reporting.
- Directory hashing with manifest export/import (JSON/CSV).
- Cloud integration (OneDrive, Google Drive) through the plugin architecture.
- SDK bindings (Node/other runtimes) consuming the Rust core library.

## Improvements
- Performance tuning for large files (async IO, multi-thread hashing).
- Structured logging/telemetry (opt-in) for troubleshooting.
- Internationalisation support for GUI text.
- Enhanced accessibility (screen reader labels, focus management).
- Export results as JSON/CSV reports.

## CI & Release Automation
- Extend CI to include GUI regression tests.
- Automated GitHub Release publishing with installers and changelog generation.
- Binary signing/notarisation workflow once credentials are provisioned.
- Periodic smoke verification of packaged installers.

## Security Hardening
- Supply-chain monitoring (`cargo audit`, `cargo deny`).
- Signed artefacts (codesign/notarise, signtool, package signing).
- Secure update channel (auto-update with checksum validation).
- Threat modelling review & periodic penetration testing.

Items above feed into `docs/PLAN.md` and `docs/TASKS.md` as they are prioritised.
