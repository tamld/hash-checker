# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- Theme picker with Soft Light, Slate, and High Contrast Dark presets.
- Hash copy button now includes the algorithm prefix (e.g. `SHA256:abcdef…`).
- Pasting a prefixed hash auto-selects the matching algorithm.
- Internal release checklist recorded to prevent empty/tag-only releases.

### Changed
- Soft Light palette softened to reduce glare; Slate palette tuned for better contrast.
- README Feature Highlights updated for theme/copy improvements.

### Planned
- SignPath signing integration for Windows artefacts.
- Light-theme palette follow-up adjustments based on tester feedback.

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

