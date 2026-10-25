# Backlog

## Table of Contents
- [Features](#features)
- [Improvements](#improvements)
- [CI & Release Automation](#ci--release-automation)
- [Security Hardening](#security-hardening)

## Features
- History panel for recent hashes and validation results.
- Cloud integration (OneDrive, Google Drive) through the plugin architecture.
- SDK bindings (Node/other runtimes) consuming the Rust core library.
- Optional file/folder encryption module built atop the existing hash workflows.

## Improvements
- Multi-threaded directory hashing across large file trees.
- Internationalisation support for GUI text.
- Enhanced accessibility (screen reader labels, focus management).

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
