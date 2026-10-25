#!/usr/bin/env bash
set -euo pipefail

# Build Hash Checker CLI + GUI for both macOS architectures (arm64 + x86_64),
# combine them into universal fat binaries via lipo, package a DMG locally using
# cargo-packager, and emit a portable hash-checker_<version>_darwin_all.tar.gz archive.

if [[ "$(uname -s)" != "Darwin" ]]; then
  echo "This script must be run on macOS." >&2
  exit 1
fi

PROJECT_ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
CLI_MANIFEST="$PROJECT_ROOT/rust/hash-checker/Cargo.toml"
GUI_MANIFEST="$PROJECT_ROOT/rust/hash-checker-gui/Cargo.toml"
VERSION="$(tr -d '[:space:]' < "$PROJECT_ROOT/VERSION" 2>/dev/null || awk -F'"' '/^version = / {print $2; exit}' "$CLI_MANIFEST")"
TIMESTAMP="$(date +%Y%m%d-%H%M%S)"
BUILD_ROOT="${TMPDIR:-/tmp}/hash-checker-universal.${TIMESTAMP}"
OUT_ROOT="${TMPDIR:-/tmp}/hash-checker-builds"
OUT_DIR="$OUT_ROOT/${TIMESTAMP}"
LOG_DIR="${TMPDIR:-/tmp}/hash-checker-logs/${TIMESTAMP}"

if [[ -d /opt/homebrew/opt/rustup/bin ]]; then
  export PATH="/opt/homebrew/opt/rustup/bin:$PATH"
fi
if [[ -d "$HOME/.cargo/bin" ]]; then
  export PATH="$HOME/.cargo/bin:$PATH"
fi

mkdir -p "$BUILD_ROOT" "$OUT_DIR" "$LOG_DIR"

cleanup() {
  rm -rf "$BUILD_ROOT"
}
trap cleanup EXIT

echo "[macos-universal] using temp build dir: $BUILD_ROOT (v${VERSION})"

rustup target add aarch64-apple-darwin >/dev/null
rustup target add x86_64-apple-darwin >/dev/null

echo "[macos-universal] building CLI & GUI aarch64 slices"
CARGO_TARGET_DIR="$BUILD_ROOT/target" cargo build --release --target aarch64-apple-darwin --manifest-path "$CLI_MANIFEST" | tee "$LOG_DIR/build-cli-aarch64.log"
CARGO_TARGET_DIR="$BUILD_ROOT/target" cargo build --release --target aarch64-apple-darwin --manifest-path "$GUI_MANIFEST" | tee "$LOG_DIR/build-aarch64.log"

echo "[macos-universal] building CLI & GUI x86_64 slices"
CARGO_TARGET_DIR="$BUILD_ROOT/target" cargo build --release --target x86_64-apple-darwin --manifest-path "$CLI_MANIFEST" | tee "$LOG_DIR/build-cli-x86_64.log"
CARGO_TARGET_DIR="$BUILD_ROOT/target" cargo build --release --target x86_64-apple-darwin --manifest-path "$GUI_MANIFEST" | tee "$LOG_DIR/build-x86_64.log"

UNIVERSAL_DIR="$BUILD_ROOT/target/universal/release"
mkdir -p "$UNIVERSAL_DIR"

echo "[macos-universal] creating universal fat binaries (CLI + GUI)"
lipo -create \
  -output "$UNIVERSAL_DIR/hash-checker" \
  "$BUILD_ROOT/target/aarch64-apple-darwin/release/hash-checker" \
  "$BUILD_ROOT/target/x86_64-apple-darwin/release/hash-checker"

lipo -create \
  -output "$UNIVERSAL_DIR/hash-checker-gui" \
  "$BUILD_ROOT/target/aarch64-apple-darwin/release/hash-checker-gui" \
  "$BUILD_ROOT/target/x86_64-apple-darwin/release/hash-checker-gui"

lipo -info "$UNIVERSAL_DIR/hash-checker" "$UNIVERSAL_DIR/hash-checker-gui" | tee "$LOG_DIR/lipo-info.log"

# Ensure cargo-packager sees the universal binary when skip-build is used
install -m 755 "$UNIVERSAL_DIR/hash-checker-gui" "$BUILD_ROOT/target/aarch64-apple-darwin/release/hash-checker-gui"
install -m 755 "$UNIVERSAL_DIR/hash-checker-gui" "$BUILD_ROOT/target/x86_64-apple-darwin/release/hash-checker-gui"
mkdir -p "$BUILD_ROOT/target/release"
install -m 755 "$UNIVERSAL_DIR/hash-checker" "$BUILD_ROOT/target/release/hash-checker"
install -m 755 "$UNIVERSAL_DIR/hash-checker-gui" "$BUILD_ROOT/target/release/hash-checker-gui"

if ! command -v cargo-packager >/dev/null 2>&1; then
  echo "[macos-universal] installing cargo-packager@0.11.7"
  cargo install cargo-packager@0.11.7 --locked
fi

PACKAGER_OUT="$BUILD_ROOT/packager-out"
rm -rf "$PACKAGER_OUT"

echo "[macos-universal] packaging DMG"
CARGO_TARGET_DIR="$BUILD_ROOT/target" \
CARGO_PACKAGER_SKIP_BUILD=true \
cargo packager --release --formats app --manifest-path "$GUI_MANIFEST" \
  --out-dir "$PACKAGER_OUT" \
  --binaries-dir "$BUILD_ROOT/target/release" | tee "$LOG_DIR/packager.log"

APP_PATH="$(find "$PACKAGER_OUT" -maxdepth 1 -type d -name '*.app' | head -n1)"
if [[ -z "$APP_PATH" ]]; then
  echo "Application bundle not found in $PACKAGER_OUT" >&2
  exit 1
fi

install -m 755 "$UNIVERSAL_DIR/hash-checker-gui" "$APP_PATH/Contents/MacOS/hash-checker-gui"

DMG_STAGING="$BUILD_ROOT/dmg-root"
rm -rf "$DMG_STAGING"
mkdir -p "$DMG_STAGING"
cp -R "$APP_PATH" "$DMG_STAGING/"
ln -sf /Applications "$DMG_STAGING/Applications"

VOLNAME="Hash Checker"
DMG_NAME="${DMG_NAME_OVERRIDE:-hash-checker.dmg}"
DEST_PATH="$OUT_DIR/$DMG_NAME"
rm -f "$DEST_PATH"

hdiutil create -volname "$VOLNAME" -srcfolder "$DMG_STAGING" -ov -format UDZO "$DEST_PATH" >/tmp/hdiutil-create.log 2>&1
cat /tmp/hdiutil-create.log >> "$LOG_DIR/dmg-pack.log"
rm -f /tmp/hdiutil-create.log

TAR_NAME="hash-checker_${VERSION}_darwin_all.tar.gz"
TAR_STAGING="$BUILD_ROOT/tar-root"
mkdir -p "$TAR_STAGING"
install -m 755 "$UNIVERSAL_DIR/hash-checker" "$TAR_STAGING/hash-checker"
install -m 755 "$UNIVERSAL_DIR/hash-checker-gui" "$TAR_STAGING/hash-checker-gui"
cp "$PROJECT_ROOT/LICENSE" "$PROJECT_ROOT/README.md" "$TAR_STAGING/"
tar -czf "$OUT_DIR/$TAR_NAME" -C "$TAR_STAGING" hash-checker hash-checker-gui LICENSE README.md

(cd "$OUT_DIR" && shasum -a 256 "$DMG_NAME" "$TAR_NAME" > SHA256SUMS && cp SHA256SUMS checksums.txt) | tee "$LOG_DIR/dmg-sha256.log"

find "$LOG_DIR" -type f -empty -delete

echo "[macos-universal] DMG ready: $DEST_PATH"
echo "[macos-universal] Universal archive ready: $OUT_DIR/$TAR_NAME"
echo "[macos-universal] Logs stored under $LOG_DIR"

DIST_DIR="$PROJECT_ROOT/dist/macos-universal"
mkdir -p "$DIST_DIR"
cp "$DEST_PATH" "$OUT_DIR/$TAR_NAME" "$OUT_DIR/SHA256SUMS" "$OUT_DIR/checksums.txt" "$DIST_DIR/"
