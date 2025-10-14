#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
LOG_DIR="${DEPS_REFRESH_LOG_DIR:-$ROOT_DIR/logs}"
TIMESTAMP="$(date +%Y%m%d-%H%M%S)"
LOG_FILE="$LOG_DIR/deps-refresh-$TIMESTAMP.log"

mkdir -p "$LOG_DIR"

{
  echo "[deps-refresh] timestamp=$TIMESTAMP"
  echo "# rustup update" && echo
  rustup update
  echo
  echo "# docker pull rust:1.83" && echo
  docker pull rust:1.83 || echo "(warning) docker not available"
  echo
  echo "# cargo install cargo-packager@0.11.7 --locked" && echo
  cargo install cargo-packager@0.11.7 --locked
  echo
  echo "# cargo update" && echo
  (cd "$ROOT_DIR" && cargo update)
  echo
  echo "# cargo audit" && echo
  (cd "$ROOT_DIR" && cargo audit || true)
  echo
  echo "# cargo deny check advisories" && echo
  (cd "$ROOT_DIR" && cargo deny check advisories || true)
} | tee "$LOG_FILE"

echo "Dependency refresh complete. Log: $LOG_FILE"
