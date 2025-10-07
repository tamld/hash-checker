# Development Plan (Rust Migration)

## Phase 0: Preparation (In Progress)
- [x] Consolidate documentation under `docs/` and sanitize internal paths.
- [x] Finalize technology decision (Rust) and communicate scope to stakeholders.
- [ ] Freeze Python implementation except for critical fixes (code freeze for migration).

## Phase 1: Rust Core MVP
- [x] Integrate Docker-based testing pipeline (Python & Rust).
- [x] Configure Vagrant headless VM for GUI smoke tests on macOS host.
- [x] Scaffold Rust workspace (`cargo new hash-checker`).
- [x] Port hashing primitives (SHA-2 family, MD5, BLAKE2) with auto-detect support.
- [x] Implement CLI with argument parity (`clap`), exit codes, and basic logging.
- [x] Write unit tests for hashing functions and integration tests for CLI (`assert_cmd`).
- [x] Provide build instructions for Windows/macOS/Linux using `cargo build --release`.

## Phase 2: GUI MVP (Post-Core)
- [ ] Evaluate UI frameworks (Tauri vs egui) and select for MVP.
- [ ] Implement minimal GUI flow: select file → select algorithm/auto → display digest + compare state.
- [ ] Add accessibility essentials (keyboard navigation, high-contrast theme toggle).
- [ ] Create automated GUI smoke test (Playwright or Specta).

## Phase 3: Tooling & Distribution
- [ ] Set up GitHub Actions matrix (Linux/macOS/Windows) running `cargo fmt`, `cargo clippy`, `cargo test`.
- [ ] Enable cross-compilation artifacts via `cargo-dist` or `tauri build`.
- [ ] Publish binary release instructions and package manifests.

## Phase 4: Security Hardening Roadmap
- [ ] Perform security review on hash verification pipeline (inputs, canonicalization, error handling).
- [ ] Integrate supply-chain scanning (`cargo audit`, `cargo deny`).
- [ ] Add checksum signing for release artifacts.
- [ ] Draft threat model and response playbook.

## Phase 5: Stretch Improvements
- [ ] Directory hashing with manifest export/import.
- [ ] Multi-file comparison report (JSON/CSV).
- [ ] Plugin interface for custom algorithms.
