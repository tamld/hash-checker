# Tasks (Rust Migration)

## Phase 0
- [ ] Freeze Python codebase (read-only except hotfixes) and tag last Python release.
- [ ] Create migration checklist and share with stakeholders.

## Phase 1 – Rust Core MVP
- [x] Scaffold Rust crate (`cargo new hash-checker`).
- [x] Implement `compute_hash` and `detect_algorithm` in Rust with streaming IO.
- [x] Implement CLI using `clap` with parity to Python flags.
- [x] Add unit tests for hashing functions (SHA256, MD5, Blake2).
- [x] Add integration tests for CLI using `assert_cmd` & temp files.
- [x] Document build/run instructions in README.


- [x] Create Docker scripts for Python/Rust tests and builds.
- [x] Add Vagrantfile + smoke script for headless GUI testing.
- [ ] Implement Rust GUI and update Vagrant smoke test to launch it.

## Phase 2 – GUI MVP
- [ ] Decide UI stack (Tauri vs egui) and spike prototypes.
- [ ] Build minimal GUI shell calling Rust core.
- [ ] Add drag-and-drop support + clipboard copy.
- [ ] Write Playwright (or equivalent) smoke tests.

## Phase 3 – Tooling & Security
- [ ] Set up GitHub Actions matrix running `cargo fmt`, `cargo clippy`, `cargo test`.
- [ ] Configure `cargo audit` in CI.
- [ ] Draft security hardening roadmap document.
- [ ] Plan artifact signing workflow.
