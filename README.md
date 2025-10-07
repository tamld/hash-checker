# Hash Checker

Cross-platform utility for verifying file integrity via cryptographic hashes.
The project is migrating from a legacy Python prototype to a Rust-based
implementation for easier cross-platform distribution.

## Status
- **Rust core** lives in `rust/hash-checker` and is the active codebase.
- **Legacy Python** remains available in `legacy/python` for reference and will only
  receive critical fixes.

## Features (Rust MVP)
- SHA-2 family, MD5, SHA-1, and BLAKE2 hashing with streaming IO.
- Automatic algorithm detection based on digest length.
- CLI parity with the original Python tool (`--algorithm`, `--list-algorithms`, exit codes).
- Containerized and VM-based workflows to avoid modifying the host machine.

## Rust Usage
```bash
cd rust/hash-checker
cargo build --release        # or run `make rust-build`
./target/release/hash-checker <FILE> <EXPECTED_HASH>
```
List supported algorithms:
```bash
./target/release/hash-checker --list-algorithms
```
Run the Rust tests (or `make rust-test`):
```bash
cargo test
```

## Legacy Python Prototype
The Python version now resides in `legacy/python`.
```bash
cd legacy/python
python -m hash_checker <FILE> <EXPECTED_HASH>
```
Run the old unit tests inside Docker to keep the host clean:
```bash
make python-test
```
To build a PyInstaller artifact for regression purposes:
```bash
make python-build
```

## Containerized Workflow
- `make python-test`: run legacy Python tests in Docker (read-only).
- `make rust-test`: run Rust tests in Docker.
- `make rust-build`: compile Rust release binary in Docker.
- `make rust-gui-smoke`: spin up the headless Vagrant VM (placeholder until the Rust GUI exists).

## Project Documents
- `docs/PLAN.md` – development roadmap (Rust migration phases).
- `docs/TASKS.md` – actionable task list for each phase.
- `docs/BACKLOG.md` – backlog for post-MVP features.
- `docs/GOALS.md` – project objectives in the Rust era.
- `docs/SECURITY_ROADMAP.md` – staged security improvements.
- `.agent/AGENTS.md` – operational guidelines for assistants.
