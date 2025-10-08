# Development Plan

## Table of Contents
- [Phase 0: Foundations (Complete)](#phase-0-foundations-complete)
- [Phase 1: Core CLI (Complete)](#phase-1-core-cli-complete)
- [Phase 2: GUI Experience (In Progress)](#phase-2-gui-experience-in-progress)
- [Phase 3: Distribution & Releases (In Progress)](#phase-3-distribution--releases-in-progress)
- [Phase 4: Security & Compliance (Planned)](#phase-4-security--compliance-planned)
- [Phase 5: Stretch Improvements (Future)](#phase-5-stretch-improvements-future)
- [Phase 6: Ecosystem Enhancements (Future)](#phase-6-ecosystem-enhancements-future)

## Phase 0: Foundations (Complete)
- [x] Consolidate documentation under `docs/` and align project guardrails.
- [x] Establish Rust as the single implementation language and retire legacy components.
- [x] Set up reproducible tooling (Docker, Vagrant) for cross-platform work.

## Phase 1: Core CLI (Complete)
- [x] Scaffold the Rust workspace (`hash-checker`, `hash-checker-gui`).
- [x] Implement hashing primitives (SHA2 family, MD5, BLAKE2) with auto-detect support.
- [x] Deliver CLI UX parity with robust exit codes and logging.
- [x] Cover core functionality with unit/integration tests (`assert_cmd`, fixture hashing).

## Phase 2: GUI Experience (In Progress)
- [x] Select egui/eframe for the desktop interface and wire it to the Rust core.
- [x] Provide accessibility features (keyboard navigation, high-contrast toggle, clipboard copy).
- [ ] Automate GUI smoke validation (Playwright or similar) beyond the current manual/Vagrant flow.
- [ ] Capture refreshed UI assets (screenshots, short demo) for documentation and release notes.

## Phase 3: Distribution & Releases (In Progress)
- [x] Run fmt/clippy/test across Linux/macOS/Windows in GitHub Actions.
- [x] Produce `.dmg` and `.deb` installers via `cargo-packager` and keep Windows portable archives in CI.
- [ ] Publish release automation that bundles installers + portable artefacts to GitHub Releases with changelog notes.
- [ ] Document offline install paths and update the README/installer guide accordingly.

## Phase 4: Security & Compliance (Planned)
- [ ] Integrate supply-chain scanning (`cargo audit`, `cargo deny`) into CI gating.
- [ ] Define checksum signing/notarisation strategy (macOS notarisation, Windows codesign) and document manual bypass instructions until keys are provisioned.
- [ ] Draft a threat model covering file-handling, checksum validation, and update channels.

## Phase 5: Stretch Improvements (Future)
- [ ] Directory hashing with manifest export/import for large batch jobs.
- [ ] Rich comparison reports (JSON/CSV) for automation pipelines.
- [ ] Extensibility hooks to allow plug-in hashing algorithms or remote storage integrations.

## Phase 6: Ecosystem Enhancements (Future)
- [ ] SDK/binding strategy for other runtimes (e.g. Node, WASI) built on the Rust core library.
- [ ] Optional telemetry/diagnostics with explicit opt-in and privacy guardrails.
- [ ] Community templates (CI snippets, packaging recipes) to accelerate downstream adoption.
