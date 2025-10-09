#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
LOG_DIR="${CI_LINUX_LOG_DIR:-$ROOT_DIR/logs}"
TIMESTAMP="$(date +%Y%m%d-%H%M%S)"
LOG_FILE="$LOG_DIR/ci-linux-$TIMESTAMP.log"
IMAGE="${CI_LINUX_IMAGE:-rust:1.83}"

mkdir -p "$LOG_DIR"

run_in_container() {
  docker run --rm \
    -v "$ROOT_DIR:/workspace" \
    -w /workspace \
    -e CARGO_TERM_COLOR=always \
    "$IMAGE" \
    bash -lc "set -euo pipefail; \
      rustup component add rustfmt clippy >/dev/null; \
      apt-get update >/dev/null && apt-get install -y --no-install-recommends pkg-config libgtk-3-dev >/dev/null; \
      cargo fmt --manifest-path rust/hash-checker/Cargo.toml --check; \
      cargo fmt --manifest-path rust/hash-checker-gui/Cargo.toml --check; \
      cargo clippy --manifest-path rust/hash-checker/Cargo.toml --all-targets -- -D warnings; \
      cargo clippy --manifest-path rust/hash-checker-gui/Cargo.toml --all-targets -- -D warnings; \
      cargo test --manifest-path rust/hash-checker/Cargo.toml --all; \
      cargo test --manifest-path rust/hash-checker-gui/Cargo.toml --all;"
}

{
  echo "[ci-linux-local] image=$IMAGE timestamp=$TIMESTAMP";
  run_in_container
} | tee "$LOG_FILE"

echo "Local Linux CI completed. Log saved to $LOG_FILE"
