# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Planned
- SignPath signing integration for Windows artefacts.
- Light-theme palette follow-up adjustments based on tester feedback.

## [0.1.6] - 2025-10-22
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
