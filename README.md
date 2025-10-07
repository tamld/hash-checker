# Hash Checker

Cross-platform utility for verifying file integrity via cryptographic hashes. The
project is migrating from a legacy Python prototype to a Rust-based implementation
for easier cross-platform distribution.

## Feature Highlights
- Multiple algorithms: SHA-2 family, SHA-1, MD5, BLAKE2.
- Automatic algorithm detection from digest length.
- Command-line and egui desktop interfaces built from the same Rust core.
- Container-first workflows (Docker/Vagrant) to keep the host clean.

## Quick Start (Containerised)
Prerequisites: Docker (build/test) and Vagrant + VMware Fusion (optional for GUI smoke test).

| Command | Purpose |
| --- | --- |
| `make rust-test` | Run Rust CLI unit/integration tests inside Docker |
| `make python-test` | Run legacy Python tests inside Docker |
| `make rust-build` | Build Rust CLI release binary in Docker |
| `make rust-gui-build` | Build Rust GUI release binary in Docker (installs GTK) |
| `make rust-gui-smoke` | Launch Vagrant VM and run GUI smoke test |
| `make rust-build-temp` | Build CLI+GUI in Docker and copy artefacts to `/tmp/hash-checker-build` |
| `make clean` | Remove build artefacts and prune Docker volumes |

## Manual Host Build (Optional)
If you prefer to work directly on the host:
```bash
make rust-build-host         # cargo build --release (CLI)
make rust-gui-build-host     # cargo build --release (GUI)
make rust-gui-smoke-host     # cargo run -- --smoke-test
```
Or run the equivalent `cargo` commands manually after installing Rust and the required GTK packages.

## Temporary Artefacts
```bash
make rust-build-temp
ls /tmp/hash-checker-build
```
Produces `hash-checker`, `hash-checker-gui`, and `SHA256SUMS` for quick manual QA without touching the repo tree.

## Legacy Python Prototype
Legacy sources live in `legacy/python` (frozen except hotfixes).
```bash
cd legacy/python
make python-test
make python-build
```

## CI Overview
- `.github/workflows/ci.yml` runs sequentially: Linux → macOS → Windows.
- Each job performs fmt/clippy/test, builds CLI & GUI, runs the GUI smoke test (`cargo run --release -- --smoke-test`), and uploads packaged artefacts with checksums.
- Docker helper scripts ensure build outputs in `target/` remain usable by host steps.

## Project Documents
- `docs/PLAN.md` – roadmap.
- `docs/TASKS.md` – actionable tasks by phase.
- `docs/BACKLOG.md` – backlog and long-term improvements.
- `docs/SECURITY_ROADMAP.md` – staged security work.
- `docs/GUI_DECISION.md`, `docs/GUI_MVP_DESIGN.md` – GUI architecture and rationale.
- `docs/MIGRATION_CHECKLIST.md` – Python → Rust migration tracker.
- `.agent/AGENTS.md` – operational guidelines for assistants.
