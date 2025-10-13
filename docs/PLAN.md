# Development Plan

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
- [x] Add accessibility essentials (high contrast, keyboard shortcut).
- [x] Expose `--smoke-test` mode for automation.
- [x] Add automated GUI regression tests (Playwright or similar).
- [x] Capture screenshots / fixture hashes for documentation.

## Phase 3 – Tooling & Distribution
- [x] Set up GitHub Actions matrix (Linux/macOS/Windows) running fmt, clippy, tests, Docker builds and packaging.
- [x] Integrate GUI automation into CI.
- [x] Deliver the Windows GA release (portable ZIP + NSIS installer) via CI and release workflows.
- [ ] Deliver the macOS GA release as a universal DMG (Intel + Apple Silicon). Current arm64 build is in place; add x86_64 support and keep it unsigned with Gatekeeper guidance.
- [x] Deliver the Linux GA release (Debian `.deb`, AppImage, Arch `pacman`) alongside continuous smoke coverage.
- [x] Document the build-from-source pathway and keep parity with packaged artefacts.
- [x] Integrate optional Vagrant headless smoke tests for Windows and Linux packaging jobs.
- [x] Add `cargo-dist` (or an equivalent tool) to automate release notes and distribution.

### Closure roadmap (target: 2025-10-22)
1. (Completed 2025-10-08) Stabilised Linux packaging in CI and added a nightly `cargo packager --formats deb` schedule.
2. (Completed 2025-10-08) Wired the existing smoke harness into GitHub Actions and gate merges on the GUI automation job.
3. (Completed 2025-10-08) Added `cargo dist manifest` automation with documentation updates in `docs/OPERATIONS.md`.
4. (Completed 2025-10-08) Replaced deprecated GitHub Actions usage (e.g. `actions-rs/toolchain@v1`) with `dtolnay/rust-toolchain@stable` to silence `set-output` warnings.
5. (Completed 2025-10-09) Drafted the signing/notarisation runbook in `docs/SIGNING.md` (macOS Developer ID, Windows SignPath) including credential prerequisites.
6. (Completed 2025-10-09) Logged the dependency migration roadmap in `docs/DEPENDENCY_MIGRATION.md`; future PRs will reference the schedule when dropping GTK3 bindings/`instant`.
7. (Completed 2025-10-08) Introduced automated cleanup targets (`make cleanup-packaging` / `scripts/cleanup-packaging.sh`) to purge staging artefacts with `KEEP_PACKAGING=1` opt-out, and documented the rule in the operations guide.

## Phase 4 – Security Hardening
- [x] Perform security review of hash verification pipeline (path handling, canonicalisation) and record findings in `docs/security/THREAT_MODEL.md`.
- [x] Integrate supply-chain scanning (`cargo audit`, `cargo deny`).
- [x] Publish signature verification instructions for end users (checksum + signature workflow in `docs/security/VERIFICATION_GUIDE.md`).
- [ ] Automate Windows signing via SignPath Foundation (see `docs/SIGNING.md`); the `windows_sign` / `publish` pipeline landed on 2025-10-13 but requires real SignPath secrets before we can mark it complete. macOS remains unsigned with documented Gatekeeper guidance.
- [ ] Record Vagrant-based release validation steps and traceability for signed artefacts.
- [ ] Establish a monthly dependency refresh cadence (resolve yanked crates immediately, update `Cargo.lock` regularly, document major upgrades).
- [ ] Maintain builder toolchain schedule (Docker base images, `cargo-packager`, `rustup` components) and update alongside the dependency refresh.
- [ ] Document the fallback automation script (`make deps-refresh`) and link it to the monthly checklist.
- [ ] Create and maintain a Vagrant validation playbook (box versions, log locations, quarterly run cadence).

## Phase 5 – Stretch Improvements
- Core feature growth:
  - [ ] Directory hashing & manifest export/import.
  - [ ] Batch comparison reports (JSON/CSV).
  - [ ] Plugin interface for custom algorithms / SDK bindings.
- Observability & performance:
  - [ ] Add optional structured logging/telemetry for large file jobs.
  - [ ] Introduce regression benchmarks (criterion) for common workloads.
- Distribution & automation:
  - [ ] Automate reproducible builds (`cargo dist build --dry-run`) in CI.
  - [ ] Add winget/homebrew manifest generation scripts and validation in CI.

## Lessons Learned & Guardrails
- Allocate large IO buffers on the heap (e.g. `Vec<u8>`) to avoid Windows stack overflows such as error `0xC00000FD` observed in October 2025.
- Manage temp resources via `TempDir`/`TempPath` in CLI tests to remain portable across Windows path semantics and file-handle behaviour.
- After a CI failure, record the root cause in PLAN/TASKS and pivot remediation instead of repeating the same fix attempt.
- Keep README and public docs polished (badges, ToC, installer instructions) so contributors immediately understand project health and release expectations.

Progress is tracked via `docs/TASKS.md`, with backlog items in `docs/BACKLOG.md`.

## Phase 6 – Governance & Automation (new)
- [ ] Finalise SignPath production rollout, rotation procedures, and public verification instructions.
- [ ] Publish credential/runbook documentation (issuance, rotation, revocation, emergency response).
- [ ] Establish automated maintenance cadence:
  - Monthly `make deps-refresh` workflow (auto-issue on failure).
  - Quarterly Vagrant smoke + signing evidence archived under `logs/release-history/`.
- [ ] Enforce branch protection for release workflows (`.github/workflows/release.yml`) and security docs (CODEOWNERS review).
- [ ] Define contributor onboarding (SECURITY.md update, issue templates, triage labels).
## Release Readiness Checklist
- [x] Implement GUI regression automation and include it in CI.
- [x] Add installer smoke jobs to validate `.dmg`, `.deb`, and Windows portable packages.
- [x] Provide end-user checksum verification instructions alongside releases.
- [x] Run supply-chain scanning (`cargo audit`, `cargo deny`) as part of the gating process.
- [x] Capture GUI screenshots/fixtures for documentation and QA reference.
- [ ] Produce ICO/ICNS bundles from the PNG assets when introducing signed distributions.
