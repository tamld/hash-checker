# Backlog (Post-Rust MVP)

## Features
- Rust GUI enhancements: history panel, multi-file queue, theming support.
- Directory verification with manifest export/import (JSON, CSV).
- Cloud source integration (OneDrive, Google Drive) via plugin architecture.
- Batch comparison reports with diff-friendly output.
- API/SDK binding (Python/Node) consuming Rust core as library.

## Improvements
- Performance tuning for large files (async IO, multi-thread hashing).
- Configurable logging/telemetry options (structured logs, opt-in metrics).
- Internationalization support for GUI text.
- Accessibility enhancements (screen-reader labels, focus management).

## Security Hardening
- Supply chain monitoring (`cargo audit` automated, SBOM generation).
- Binary signing + notarization (macOS, Windows).
- Secure update channel (auto-update with checksum validation).
- Threat modeling review and penetration testing after MVP.

## CI & Release Automation
- GitHub Actions matrix for lint/test/build per platform.
- Automated GUI smoke test (Playwright or headless).
- Release workflow with artifact packaging and optional signing.

## Release & QA
- Automated changelog generation and release notes.
- Signed artefacts (codesign/notarize, signtool).
- Beta feedback loop & manual QA checklist.
