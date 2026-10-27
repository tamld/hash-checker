# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Planned
- SignPath signing integration for Windows artefacts.

## [0.2.1] - 2026-10-28

### Added & Changed
- Upgrade `.g8s/feature-map.yml` to Spec-Kit v2.0 with `inherits_spec_kit: g8s/offer/profiles/utility@v0.21`.
- Add 3-attempt retry to `cargo audit` in CI and scaffold `AGENTS.md`.

## [0.2.0] - 2026-10-25
### Added
- Cooperative cancellation (`HashError::Cancelled`, `compute_hash_with_options`, `generate_manifest_with_options`, `verify_manifest_with_options`) and real-time progress callbacks across CLI and GUI (`specs/PRD.md`, `specs/SRS.md`, `specs/FSM.md`, `specs/AC.md`).
- Live progress bars and **Cancel** buttons in `hash-checker-gui` for both Single File and Folder Manifest tabs.
- Full 8-algorithm prefix auto-detection (`sha256`, `sha512`, `sha3-256`, `sha3-512`, `blake2b`, `blake2s`, `sha1`, `md5`) in `hash-checker-gui` on paste and before calculation.
- Declarative `.g8s/feature-map.yml`, GUI snapshot harness (`scripts/gui-snapshot-harness.sh`, `make gui-snapshot`), and telemetry validator (`scripts/validate_telemetry.py`).
### Changed
- Aligned `ThemePreset::default()` and `HashCheckerApp::default()` to `ThemePreset::SoftLight`.
- Replaced `EitherWriter` boilerplate with `Box<dyn Write>` in CLI output handling and pre-allocated path normalisation buffers.
- Standardised macOS DMG artefact name to `hash-checker.dmg` across local scripts and release workflow (Issue #48).
### Security
- Enforced strict relative path normalisation (`normalize_manifest_rel_path`) rejecting `..`, `.`, and root/prefix components during manifest verification.
- Upgraded `rand` to `0.9.4` in `hash-checker-gui/Cargo.lock` (Dependabot alert #6) and bumped optional GTK4 backend to `gtk4` 0.10.1 / `glib` 0.21.3 (RUSTSEC-2024-0429).

## [0.1.7] - 2025-10-22
### Experimental
- Added optional `gtk4-native` feature (Linux) to begin wiring GTK4 file dialogs behind a feature flag.
### Changed
- Documented the GTK4 migration plan and updated CI to smoke test the feature flag on Linux alongside the existing portal flow.

## [0.1.5] - 2025-10-17
### Added
- Theme picker with Soft Light, Slate, and High Contrast Dark presets.
- Hash copy button now includes the algorithm prefix (e.g. `SHA256:abcdef…`).
- Pasting a prefixed hash auto-selects the matching algorithm.
- Documentation for the GUI warning that surfaces when an unsupported hash prefix is pasted.
- Internal release checklist recorded to prevent empty/tag-only releases.
- Structured logging toggle for CLI runs via `--log-format` (text/json).
- Criterion benchmarks for hashing (1/10/50 MB files across SHA-2/BLAKE2 families).
- Guidance for using the `skip-linux-ci` label so documentation-only PRs can bypass the Linux job safely.

### Changed
- Release workflow trims GitHub assets to installers/portable packages and a single SHA256SUMS manifest.
- Soft Light palette softened to reduce glare; Slate palette tuned for better contrast.
- README Feature Highlights updated for theme/copy improvements.
- Refreshed GUI screenshots (Slate default, algorithm dropdown, match/mismatch, high contrast) in README and docs.
- CLI terminal detection now relies on `std::io::IsTerminal`, removing the unmaintained `atty` dependency and clearing `cargo audit` warnings.
- Release script now copies the universal macOS DMG into `dist/macos-universal/`, restoring the architecture verification step in CI.

## [0.1.4] - 2025-10-14
### Added
- macOS universal DMG build via local script.
- Release notes include packaging logs and checksums.

### Changed
- README “Release & Changelog” section created.

## [0.1.3] - 2025-10-14
### Removed
- Release/tag withdrawn due to incomplete artefacts and missing notes.

## [0.1.2] - 2025-10-14
### Added
- Initial Rust GUI release, Windows/macOS/Linux packaging.
