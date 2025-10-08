# Operations Guide

## Release Checklist
1. Ensure CI (`ci.yml`) is green on the target commit.
2. Confirm the Release Readiness checklist in `docs/PLAN.md` is satisfied.
3. Prepare release notes including:
   - Semantic version (e.g. `v0.4.0`).
   - Summary of changes / primary purpose for the release.
   - Known issues or manual steps (Gatekeeper bypass, etc.).
4. Tag the commit (`git tag vX.Y.Z && git push origin vX.Y.Z`).
5. After the automated workflow publishes artefacts:
   - Edit the GitHub Release description with the prepared notes.
   - Verify `.dmg`, `.deb`, and Windows `.zip` artefacts download and launch successfully.
6. Update `docs/PLAN.md` / `docs/TASKS.md` if new follow-up work is identified.

Keep this document with the repo to standardise release expectations.

## Build Diagnostics (2025-10-08)

### Successful checks
- `cargo fmt --check` for CLI and GUI crates.
- `cargo clippy --all-targets -- -D warnings` on both crates.
- `cargo test` (CLI crate) and `make rust-test` inside Docker.
- `cargo run --release -- --smoke-test` for the GUI crate.
- `cargo packager --release --formats deb` inside `rust:1.83` Docker after icon fix.

### Common failure modes
- `cargo packager` rejects legacy keys such as `[package.metadata.packager.macos].icons` and `[package.metadata.packager.windows].icon-path`; consolidate under the root `icons` array.
- Debian packaging fails with `Invalid PNG signature` if `docs/assets/icon-hash-checker-*.png` contains JPEG data—re-export icons as real PNGs (`sips -s format png …`).

### Cleanup rule
- Packaging targets run `scripts/cleanup-packaging.sh` by default, removing staging artefacts (`dist/linux`, `rust/hash-checker-gui/target/packager`, `/tmp/hash-checker-*`) once validation completes.
- To retain artefacts for debugging, set `KEEP_PACKAGING=1` before invoking `make dist-linux` or `make cleanup-packaging`.
- Manual cleanup is available via `make cleanup-packaging`.
- `make clean` now calls `scripts/clean.sh`, which handles cross-platform deletion (Rust `target/`, `dist/`, `/tmp/hash-checker-*` plus `${TMPDIR}`/`%TEMP%` mirrors) and prunes Docker volumes unless `CLEAN_DOCKER=0`.

### CI warning mitigation (2025-10-08)
- Thay `actions-rs/toolchain@v1` bang `dtolnay/rust-toolchain@stable` trong toan bo workflow de tranh canh bao `set-output` bi deprecate.
- Bo sung kiem tra `brew list` truoc khi cai `gtk+3`/`pkg-config` trong job macOS de Homebrew khong spam thong bao "pkgconf ... already installed".
