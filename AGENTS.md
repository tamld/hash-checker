# AGENTS.md

## Rust Workspace Layout

- `rust/hash-checker` — CLI binary + shared core library (hashing, manifests, batch verification).
- `rust/hash-checker-gui` — desktop GUI (Slint).
- `rust/packaging` — packaging assets and configuration (cargo-dist / cargo-packager).

## Build & Test

```sh
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

Per-crate commands used in CI:

```sh
cargo fmt --manifest-path rust/hash-checker/Cargo.toml --check
cargo clippy --manifest-path rust/hash-checker/Cargo.toml --all-targets -- -D warnings
cargo test --manifest-path rust/hash-checker/Cargo.toml --all
cargo audit   # run in rust/hash-checker and rust/hash-checker-gui (3 attempts, 5s retry)
```

## Zero-Trust SDD Rules

- Specs (`specs/PRD.md`, `specs/SRS.md`, `specs/AC.md`, `specs/FSM.md`) are the source of truth. Every code change must trace to a `REQ-FUNC-*` / `AC-*` entry.
- Fail closed: invalid input, conflicts, and traversal attempts return errors; never silently succeed.
- No type-error suppression (`as any`, `@ts-ignore`, `@ts-expect-error`) and no empty catch blocks.
- Path safety: `follow_links(false)`, reject `..` traversal and absolute roots in manifests.
- Bounded memory: stream files in 1 MiB heap chunks (`CHUNK_SIZE = 1048576`); never load whole files.
- Cooperative cancellation: honour the `AtomicBool` flag within one chunk read.
- Deterministic CLI exit codes (0 match, 1 runtime, 2 usage/input, 3 mismatch, 4 extra, 5 unsupported).
- Verify before claiming done: `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, and `lsp_diagnostics` clean.