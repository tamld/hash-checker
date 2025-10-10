#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
LOG_DIR="${DEPS_REFRESH_LOG_DIR:-$ROOT_DIR/logs}"
TIMESTAMP="$(date +%Y%m%d-%H%M%S)"
LOG_FILE="$LOG_DIR/deps-refresh-$TIMESTAMP.log"

mkdir -p "$LOG_DIR"

{
  echo "[deps-refresh] timestamp=$TIMESTAMP"
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
