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
- [ ] Add automated GUI regression tests (Playwright or similar).
- [ ] Capture screenshots / fixture hashes for documentation.

## Phase 3 – Tooling & Distribution
- [x] Set up GitHub Actions matrix (Linux/macOS/Windows) running fmt, clippy, tests, Docker builds and packaging.
- [ ] Integrate GUI automation into CI.
- [ ] Add `cargo-dist` (or equivalent) release pipeline and generate release notes.
- [ ] Design binary signing/notarisation workflow (macOS, Windows).

## Phase 4 – Security Hardening
- [ ] Perform security review of hash verification pipeline (path handling, canonicalisation).
- [ ] Integrate supply-chain scanning (`cargo audit`, `cargo deny`).
- [ ] Add checksum/signature verification instructions for end users.

## Phase 5 – Stretch Improvements
- [ ] Directory hashing & manifest export/import.
- [ ] Batch comparison reports (JSON/CSV).
- [ ] Plugin interface for custom algorithms / SDK bindings.

## Lessons Learned & Guardrails
- Allocate large IO buffers on the heap (e.g. `Vec<u8>`) to avoid Windows stack overflows such as error `0xC00000FD` observed in October 2025.
- Manage temp resources via `TempDir`/`TempPath` in CLI tests to remain portable across Windows path semantics and file-handle behaviour.
- After a CI failure, record the root cause in PLAN/TASKS and pivot remediation instead of repeating the same fix attempt.
- Keep README and public docs polished (badges, ToC, installer instructions) so contributors immediately understand project health and release expectations.

Progress is tracked via `docs/TASKS.md`, with backlog items in `docs/BACKLOG.md`.
## Release Readiness Checklist
- Implement GUI regression automation and include it in CI.
- Add installer smoke jobs to validate `.dmg`, `.deb`, and Windows portable packages.
- Provide end-user checksum verification instructions alongside releases.
- Run supply-chain scanning (`cargo audit`, `cargo deny`) as part of the gating process.
- Capture GUI screenshots/fixtures for documentation and QA reference.

