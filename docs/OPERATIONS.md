# Operations Guide

> Multi-agent workflow: this project adopts the global `policy/multi_agent_delivery`. Use CARE specs under `specs/`, keep `.agents/branch_progress.yml` current, run `.agents/scripts/validate_handoff.sh` before handoffs, and record entries in `.agents/metrics_log.yml` after each PR.

## Developer Quick Reference
> Minimum supported Rust version (MSRV): **1.88.0**


### Clone & Workspace Layout
```bash
# Clone the public repository
git clone https://github.com/tamld/hash-checker.git
cd hash-checker

# (Optional) work inside the assistant workspace layout
cd local-scripts/hash-checker
```

> Most scripts/Make targets expect you to run them from the repository root (`hash-checker/` or `local-scripts/hash-checker/`). Update the remote when working from a fork.

### Container Quick Start
Prerequisites: Docker for build/test; Vagrant + VMware Fusion (optional) for headless GUI smoke tests.

| Command | Purpose |
| --- | --- |
| `make rust-test` | Run Rust CLI unit/integration tests inside Docker |
| `make rust-build` | Build Rust CLI release binary in Docker |
| `make rust-gui-build` | Build Rust GUI release binary in Docker (installs portal + GTK deps và chạy pipeline snapshot). _Đã xác minh chạy xanh với Docker `rust:1.88` ngày 2025-10-28._ |
| `make rust-gui-smoke` | Launch Vagrant VM and run `cargo run -- --smoke-test` |
| `make rust-build-temp` | Build CLI + GUI in Docker and copy artefacts to `/tmp/hash-checker-build` |
| `make clean` | Remove build artefacts and prune Docker volumes |
| `make cleanup-packaging` | Remove packaging staging directories (`dist/linux`, `target/packager`, `/tmp/hash-checker-*`) |

> Recommendation: export `CARGO_TARGET_DIR` to an OS-specific temp path (for example `/tmp/hash-checker-target` or `%TEMP%\hash-checker-target`) so sync tools do not lock intermediate files. Clean up the directory or run `make clean` after finishing.

### Host Builds (Rust Installed)
```bash
# CLI
cargo build --release --manifest-path rust/hash-checker/Cargo.toml

# GUI (ensure XDG desktop portal on Linux; install pkg-config on macOS)
cargo build --release --manifest-path rust/hash-checker-gui/Cargo.toml
cargo run --release --manifest-path rust/hash-checker-gui/Cargo.toml -- --smoke-test

# Equivalent Make targets
make rust-build-host
make rust-gui-build-host
make rust-gui-smoke-host

```

### Golden Master Validation
- **Scenarios**: `minimal-scan`, `deep-tree`, `verify-mismatches`. Golden JSON baselines live under
  `test-fixtures/golden/<platform>/` and only reference sanitized demo paths
  (`/tmp/hash-checker-gui` on Unix, `C:\Temp\hash-checker-gui` on Windows) to
  avoid leaking local workspace locations.
- **Capture** a refreshed baseline (writes into the platform subdirectory or an
  override directory):
  ```bash
  HASH_CHECKER_GOLDEN_DIR=test-fixtures/golden \
    cargo run --manifest-path rust/hash-checker-gui/Cargo.toml -- \
      --headless --capture-golden minimal-scan
  ```
- **Compare** against the committed baseline (exit code `0` = match, `1` = diff,
  `2+` = error):
  ```bash
  HASH_CHECKER_GOLDEN_DIR=test-fixtures/golden \
    cargo run --manifest-path rust/hash-checker-gui/Cargo.toml -- \
      --headless --compare-golden minimal-scan
  ```
- Keep the environment variable pointed at a writable temp directory when
  experimenting locally so accidental captures do not pollute the committed
  fixtures. Use `git diff test-fixtures/golden` to inspect intentional updates
  before proposing changes.
- **CI guardrail**: `.github/workflows/golden-master-validation.yml` runs on PRs
  that touch the GUI or golden assets. The workflow provisions Rust 1.88 on
  Linux/macOS/Windows, executes `--compare-golden` for each scenario, and
  uploads per-platform logs to the `golden-comparison-<platform>` artifact even
  when mismatches occur. Download the artifact via the Actions run page to read
  the captured diffs.
- **Windows limitation**: không có runner Windows nội bộ, vì vậy mọi so khớp
  golden cho Windows phải thông qua GitHub Actions (`windows-latest`). macOS và
  Linux vẫn kiểm chứng được tại chỗ (mac host / Docker) trước khi đẩy code.

#### GTK4-native (optional, Linux)
- This feature is experimental and enabled via the `gtk4-native` feature. Since the local environment (macOS) does not have GTK4, enable/disable and test it on a Linux runner (GitHub Actions) or a dedicated VM.
- Install necessary packages before building:
  ```bash
  sudo apt-get update
  sudo apt-get install libgtk-4-dev libadwaita-1-dev libglib2.0-dev
  cargo check --manifest-path rust/hash-checker-gui/Cargo.toml --features gtk4-native
  ```
- When running in CI, add a step to install packages on the Ubuntu runner and use `cargo run --release --manifest-path rust/hash-checker-gui/Cargo.toml --features gtk4-native -- --smoke-test`.
- Flatpak and sandboxed environments must still use the portal (default feature). GTK4 snapshots/logs should be saved under `logs/qa/gtk4-<date>.md`.

### CLI Logging & Manifests
- By default the CLI prints only the verification outcome. Use `--log-format text|json` when you need structured progress information; logs write to `stderr` so `stdout` stays script-friendly.
- Export directory manifests with `hash-checker manifest export <path> -o <file> -r` (JSON default). Verify with `hash-checker manifest verify <file>`.
- Helpful flags: `--format csv|txt`, `--algorithm <algo>`, `--root <path>` (when verifying from a different base directory), `--report-limit <n>` to cap mismatch summaries.

### Batch Comparison Reports
- Define expected hashes in JSON or CSV and feed them to the batch command:

  ```json
  [
    { "path": "dist/hash-checker", "expected": "sha256:<digest>" },
    { "path": "README.md", "expected": "1097…", "algorithm": "md5" }
  ]
  ```

- Run the CLI and capture a structured report:

  ```bash
  hash-checker batch --input hashes.json --output report.json --output-format json
  hash-checker batch --input hashes.csv --input-format csv --output report.csv --output-format csv
  ```

- Exit codes: `0` (all matched), `3` (mismatched/missing entries), `2` (errors such as unsupported algorithms or I/O failures).
- Reports include a summary block plus `entries[]` with `status` (`match`, `mismatch`, `missing`, `error`) and the computed hash when applicable, making CI assertions straightforward.

### Distribution Automation
- Workflow **Distribution Dry Run** (`.github/workflows/dist-validation.yml`) runs weekly (Mon 06:00 UTC) and on demand:
  - Job 1 installs `cargo-dist@0.30.0`, runs `dist plan --output-format json`, captures `dist-manifest.json`, and save both artifacts for review.
  - Job 2 installs Debian deps, invokes `scripts/debian-smoke.sh`, and uploads the generated `.deb` plus CLI smoke logs.
- Reproduce locally:

  ```bash
  sudo apt-get install libasound2-dev xdg-desktop-portal xvfb
  ./scripts/debian-smoke.sh
  ```

  The script builds the `.deb`, installs it (using `sudo` when available), and runs `hash-checker --version` plus `hash-checker-gui -- --smoke-test` (through `xvfb` when available). Logs are written under `logs/cli-snapshots/`.

- Build the full matrix of artifacts on macOS (requires Zig, `cargo-xwin`, `cargo-zigbuild`, `rust-src`, and preferring `~/.cargo/bin` in the `PATH`):

  ```bash
  brew install zig
  cargo install cargo-zigbuild@0.18.2 --locked
  cargo install cargo-xwin@0.17.0 --locked
  rustup component add rust-src
  PATH="$HOME/.cargo/bin:$PATH" dist build --artifacts=local
  ```

  The command above creates the complete set of `tar.xz`/`zip` files for macOS (x86_64 + arm64), Linux, and Windows under `target/distrib/`. If using the `brew` version of cargo (not rustup), preferring `~/.cargo/bin` as shown is mandatory to avoid missing `rust-std` for cross-targets.

### Benchmarks
- The CLI ships with a Criterion suite under `rust/hash-checker/benches/hash_bench.rs`
  that measures hashing throughput on 1/10/50 MiB fixtures across SHA-2 and
  BLAKE2 algorithms.
- Execute the suite locally:
  ```bash
  cd rust/hash-checker
  cargo bench --bench hash_bench
  ```
  Criterion adapts the sampling strategy automatically. When running inside CI,
  set `CI=1` to shorten warm-up and measurement windows.
- If `gnuplot` is unavailable, Criterion falls back to the Plotters backend;
  install `gnuplot` when you need PNG charts for reports.
- Verification (2025-10-28): the command above completed successfully on the
  current toolchain, producing throughput in the ~500–1,300 MiB/s range. Store
  historical outputs under `logs/benchmarks/` when tracking regressions across
  releases.
