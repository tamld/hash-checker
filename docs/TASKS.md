# Tasks (Rust Migration)

## Phase 0
- [x] Freeze Python codebase (read-only except hotfixes) and tag last Python release (pending actual tag).
- [x] Create migration checklist and share with stakeholders.

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
- [x] Scaffold `hash-checker-gui` crate with egui/eframe.
- [x] Wire GUI to hash core library for calculations.
- [x] Implement file picker + algorithm dropdown + result panel.
- [x] Add accessibility toggles (theme contrast, keyboard hints).
- [x] Integrate GUI smoke test into Vagrant pipeline.

- [x] Decide UI stack (egui/eframe) and document rationale (docs/GUI_DECISION.md).
- [x] Build minimal GUI shell calling Rust core.
- [x] Add drag-and-drop support + clipboard copy.
- [ ] Write Playwright (or equivalent) smoke tests.

## Phase 3 – Tooling & Security
- [ ] Set up GitHub Actions matrix running `cargo fmt`, `cargo clippy`, `cargo test`.
- [ ] Configure `cargo audit` in CI.
- [ ] Draft security hardening roadmap document.
- [ ] Plan artifact signing workflow.
