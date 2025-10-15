# Development Plan

> Canonical roadmap lives in this document. `docs/TASKS.md` now only lists the
> near-term scope derived from the plan.
> High-level status snapshots are kept in `docs/PROJECT_STATUS.md` to avoid duplicating checklists.

## Table of Contents
- [Phase 0 – Preparation](#phase-0--preparation)
- [Phase 1 – Rust Core MVP](#phase-1--rust-core-mvp)
- [Phase 2 – GUI MVP](#phase-2--gui-mvp)
- [Phase 3 – Tooling & Distribution](#phase-3--tooling--distribution)
- [Phase 4 – Security Hardening](#phase-4--security-hardening)
- [Phase 5 – Stretch Improvements](#phase-5--stretch-improvements)
- [Lessons Learned & Guardrails](#lessons-learned--guardrails)

## Phase 0 – Preparation
- [x] Consolidate documentation under `docs/` and sanitise internal paths.
- [x] Finalise Rust migration decision and communicate scope.
- [x] Freeze Python implementation except for critical fixes.

## Phase 1 – Rust Core MVP
- [x] Integrate Docker-based testing pipeline (Python & Rust).
- [x] Configure Vagrant headless VM for GUI smoke tests.
- [x] Scaffold Rust workspace (`cargo new hash-checker`).
- [x] Port hashing primitives with auto-detect support.
- [x] Implement CLI with arg parity + exit codes.
- [x] Add unit/integration tests (`assert_cmd`).
- [x] Provide build instructions for all platforms.

## Phase 2 – GUI MVP
- [x] Select egui/eframe (see `docs/GUI_DECISION.md`).
- [x] Implement minimal GUI flow (file select, algorithm, result panel).
- [x] Add accessibility essentials (high contrast, keyboard hints, clipboard).
- [x] Expose `--smoke-test` mode for automation.
- [x] Add automated GUI regression tests (headless `cargo test`).
- [x] Capture refreshed screenshots/fixtures for README and docs. (Updated 2025-10-15; see `docs/assets/` and `logs/qa/theme-copy-verification-20251015.md`.)

## Phase 3 – Tooling & Distribution
- [x] Set up GitHub Actions matrix (Linux/macOS/Windows) running fmt, clippy, tests, Docker builds and packaging.
- [x] Integrate GUI automation into CI.
- [x] Deliver the Windows GA release (portable ZIP + NSIS installer) via CI and release workflows.
- [ ] Deliver the macOS GA release as a universal DMG. Local automation (`make macos-dmg-universal`) now produces a universal DMG; wire it into `release.yml` before marking complete.
- [x] Deliver the Linux GA release (Debian `.deb`, AppImage, Arch `pacman`).
- [x] Document the build-from-source pathway and keep parity with packaged artefacts.
- [x] Integrate optional Vagrant headless smoke tests for Windows and Linux packaging jobs.
- [x] Add `cargo-dist` (or an equivalent tool) to automate release notes and distribution.

### Closure roadmap (target: 2025-10-22)
1. (Completed) Stabilised Linux packaging + nightly Debian job.
2. (Completed) Gate merges on GUI automation job.
3. (Completed) `cargo dist manifest` automation in `docs/OPERATIONS.md`.
4. (Completed) Modernise GitHub Actions toolchain.
5. (Completed) Signing/notarisation runbook (`docs/SIGNING.md`).
6. (Completed) Dependency migration roadmap (`docs/DEPENDENCY_MIGRATION.md`).
7. (Completed) Cleanup targets (`make cleanup-packaging`).

## Phase 4 – Security Hardening
- [x] Threat modelling (see `docs/security/THREAT_MODEL.md`).
- [x] Integrate `cargo audit` and `cargo deny` into CI.
- [x] Publish verification guide for end users (`docs/security/VERIFICATION_GUIDE.md`).
- [ ] Automate Windows signing via SignPath (pending secrets).
- [ ] Ship macOS universal DMG in release workflow (once CI integration complete).
- [x] Record Vagrant validation steps (`docs/vagrant/VALIDATION_PLAYBOOK.md`).
- [x] Monthly dependency refresh cadence (`deps-refresh.yml`).
- [x] Maintain builder toolchains (`scripts/deps-refresh.sh`).
- [x] Document fallback automation (`make deps-refresh`).
- [x] Create Vagrant validation playbook.

## Phase 5 – Stretch Improvements
- Core feature growth:
  - [ ] Directory hashing & manifest export/import.
  - [ ] Batch comparison reports (JSON/CSV).
  - [ ] Plugin interface for custom algorithms / SDK bindings.
- Observability & performance:
  - [x] Structured logging/telemetry toggle for CLI runs (added `--log-format`, Oct 2025).
  - [ ] Regression benchmarks (criterion) for large files.
- Distribution & automation:
  - [ ] Automate reproducible builds (`cargo dist build --dry-run`) in CI.
  - [ ] Add winget/homebrew manifests.

### Implementation order (easy → hard)
1. Structured logging / telemetry toggle (completed Oct 2025).
2. Criterion regression benchmarks.
3. Directory hashing & manifest export workflows.
4. Batch comparison reports API.
5. Plugin interface for additional runtimes.

### Migration roadmap
- Track GTK3 → GTK4/egui-native in `docs/DEPENDENCY_MIGRATION.md`.
- Replace `instant` crate once stable alternatives land.

## Lessons Learned & Guardrails
- Allocate large IO buffers on heap (avoid Windows stack overflow, ref Oct 2025).
- Manage temp resources via `TempDir`/`TempPath` in tests for portability.
- After CI failure, record RCA instead of repeating same attempt.
- Keep README/public docs polished.
- Treat every release tag as production-ready: update changelog, verify artefacts, tick release checklist before tagging (lesson from withdrawn `v0.1.3`).

Progress is tracked via `docs/TASKS.md`, with backlog items in `docs/BACKLOG.md`.

## Phase 6 – Governance & Automation (new)
- [ ] Finalise SignPath rollout (secrets, verification docs).
- [ ] Publish credential/runbook documentation (rotations, emergency).
- [ ] Maintenance cadence: monthly deps-refresh, quarterly Vagrant smoke.
- [ ] Enforce branch protection for release workflows.
- [ ] Define contributor onboarding (SECURITY.md update, templates).
## Release readiness checklist
- [x] GUI regression automation wired to CI.
- [x] Installer smoke jobs in CI.
- [x] End-user checksum instructions included.
- [x] Supply-chain scanning (cargo audit/deny) gating.
- [ ] Refresh GUI screenshots for latest theme/UX.
- [ ] Produce ICO/ICNS bundles alongside signed distributions.
