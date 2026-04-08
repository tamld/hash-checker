#!/usr/bin/env bash
set -euo pipefail

# gui-snapshot-harness.sh
# Automates the GUI snapshot and telemetry validation scenarios.
# Requires: cargo, python3, and a display (or xvfb-run).

PROJECT_ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
TELEMETRY_DIR="$PROJECT_ROOT/logs/gui-manifest"
TELEMETRY_LOG="$TELEMETRY_DIR/telemetry.log"
SNAPSHOT_DIR="$TELEMETRY_DIR/snapshots"

mkdir -p "$SNAPSHOT_DIR"

# Clean telemetry log to ensure we validate only the current run
rm -f "$TELEMETRY_LOG"

echo "Building Hash Checker GUI..."
cargo build --release --manifest-path "$PROJECT_ROOT/rust/hash-checker-gui/Cargo.toml"
BIN="$PROJECT_ROOT/rust/hash-checker-gui/target/release/hash-checker-gui"

echo "Running snapshot harness (Readme preset)..."
CMD="$BIN --snapshot $SNAPSHOT_DIR/gui-main.png --snapshot-preset readme"

if command -v xvfb-run >/dev/null; then
    xvfb-run --auto-servernum --server-args="-screen 0 1280x1024x24" $CMD
elif [ -n "${DISPLAY:-}" ]; then
    $CMD
else
    echo "Error: No DISPLAY and xvfb-run not found. Cannot run GUI automation."
    exit 1
fi

echo "Validating Telemetry..."

# Count expected files in test-fixtures/gui-deep
EXPECTED_RECORDED=$(find "$PROJECT_ROOT/test-fixtures/gui-deep" -type f | wc -l)

# 1. Scan (ManifestSummary scenario)
echo "Validating Scan telemetry (recorded=$EXPECTED_RECORDED)..."
python3 "$PROJECT_ROOT/scripts/validate_telemetry.py" "$TELEMETRY_LOG" Scan \
  --expect recorded="$EXPECTED_RECORDED"

# 2. Verify (ManifestDetails scenario)
# Mocks: mismatched=1, extra=1. Missing=1 (since >1 file).
echo "Validating Verify telemetry..."
python3 "$PROJECT_ROOT/scripts/validate_telemetry.py" "$TELEMETRY_LOG" Verify \
  --expect mismatched=1 \
  --expect extra=1 \
  --expect missing=1

echo "Harness completed successfully."
