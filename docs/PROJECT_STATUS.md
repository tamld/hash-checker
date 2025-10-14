# Project Status Overview

_Last updated: 2025-10-14_

## In Progress / Pending
- **SignPath OSS onboarding**: awaiting response to provision OSS subscription, test certificate, and secrets (`docs/security/SIGNPATH_CHECKLIST.md`).
- **GUI screenshots**: capture `gui-high-contrast.png` per `docs/GUI_SCREENSHOT.md` once a GUI-enabled workstation is available.
- **macOS universal DMG**: release `v0.1.2` ships an arm64-only DMG; local automation (`make macos-dmg-universal`) now produces a universal build. Integrate it into `release.yml` before publishing the next tag (`docs/PLAN.md`, `docs/TASKS.md`).
- **Version sync**: crate manifests now declare `0.1.4`; ensure the release tag matches.

## Automation & Maintenance
- Monthly dependency refresh (`deps-refresh.yml`) and cargo-dist maintenance (`cargo-dist-maintenance.yml`) run automatically; review artefact logs on failure.
- Quarterly Vagrant smoke reminder (`vagrant-smoke-reminder.yml`) opens an issue for manual VM validation; follow `docs/vagrant/VALIDATION_PLAYBOOK.md`.
- Credential/runbook references: `docs/security/CREDENTIAL_RUNBOOK.md`, `docs/security/CI_SIGNING.md`.

## Stretch Goals (Phase 5)
- Directory hashing manifests (TXT/CSV/JSON).
- Batch comparison reporting for CI/QA.
- Plugin interface for custom algorithms/SDK bindings.
- Structured telemetry/logging (see `docs/security/TELEMETRY_NOTES.md`).
- Regression benchmarks (criterion) to guard performance.

## Strategic Items
- GTK3 migration path (evaluate GTK4/libadwaita or wgpu/tao stack).
- Alternative distribution channels (winget/Homebrew manifests) ready via `scripts/generate_manifests.sh` when needed.

## Recent Updates
- Maintenance workflows tightened (`deps-refresh` enhancements, Vagrant reminder).
- SignPath onboarding checklist added.
- Vagrant playbook clarified with manual/automation rationale.
