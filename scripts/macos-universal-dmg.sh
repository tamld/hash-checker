#!/usr/bin/env bash
set -euo pipefail

# Build Hash Checker GUI for both macOS architectures, combine them into a universal
# binary, and package a DMG locally using cargo-packager. Outputs the DMG path at the end.

if [[ "$(uname -s)" != "Darwin" ]]; then
  echo "This script must be run on macOS." >&2
  exit 1
fi

PROJECT_ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
MANIFEST="$PROJECT_ROOT/rust/hash-checker-gui/Cargo.toml"
BUILD_ROOT="${TMPDIR:-/tmp}/hash-checker-universal.$(date +%s)"
DIST_DIR="$PROJECT_ROOT/dist/macos-universal"
LOG_DIR="$PROJECT_ROOT/logs/local-test"

if [[ -d /opt/homebrew/opt/rustup/bin ]]; then
  export PATH="/opt/homebrew/opt/rustup/bin:$PATH"
fi

mkdir -p "$BUILD_ROOT" "$DIST_DIR" "$LOG_DIR"

cleanup() {
  rm -rf "$BUILD_ROOT"
}
trap cleanup EXIT

echo "[macos-universal] using temp build dir: $BUILD_ROOT"

rustup target add aarch64-apple-darwin >/dev/null
rustup target add x86_64-apple-darwin >/dev/null

echo "[macos-universal] building aarch64 slice"
CARGO_TARGET_DIR="$BUILD_ROOT/target" cargo build --release --target aarch64-apple-darwin --manifest-path "$MANIFEST" | tee "$LOG_DIR/build-aarch64.log"

echo "[macos-universal] building x86_64 slice"
CARGO_TARGET_DIR="$BUILD_ROOT/target" cargo build --release --target x86_64-apple-darwin --manifest-path "$MANIFEST" | tee "$LOG_DIR/build-x86_64.log"

UNIVERSAL_DIR="$BUILD_ROOT/target/universal/release"
mkdir -p "$UNIVERSAL_DIR"

echo "[macos-universal] creating universal binary"
lipo -create \
  -output "$UNIVERSAL_DIR/hash-checker-gui" \
  "$BUILD_ROOT/target/aarch64-apple-darwin/release/hash-checker-gui" \
  "$BUILD_ROOT/target/x86_64-apple-darwin/release/hash-checker-gui"

lipo -info "$UNIVERSAL_DIR/hash-checker-gui" | tee "$LOG_DIR/lipo-info.log"

# Ensure cargo-packager sees the universal binary when skip-build is used
install -m 755 "$UNIVERSAL_DIR/hash-checker-gui" "$BUILD_ROOT/target/aarch64-apple-darwin/release/hash-checker-gui"
install -m 755 "$UNIVERSAL_DIR/hash-checker-gui" "$BUILD_ROOT/target/x86_64-apple-darwin/release/hash-checker-gui"
mkdir -p "$BUILD_ROOT/target/release"
install -m 755 "$UNIVERSAL_DIR/hash-checker-gui" "$BUILD_ROOT/target/release/hash-checker-gui"
REPO_RELEASE="$PROJECT_ROOT/rust/hash-checker-gui/target/release"
mkdir -p "$REPO_RELEASE"
install -m 755 "$UNIVERSAL_DIR/hash-checker-gui" "$REPO_RELEASE/hash-checker-gui"

rm -rf "$PROJECT_ROOT/rust/hash-checker-gui/target/packager"

if ! command -v cargo-packager >/dev/null 2>&1; then
  echo "[macos-universal] installing cargo-packager@0.11.7"
  cargo install cargo-packager@0.11.7 --locked
fi

PACKAGER_OUT="$BUILD_ROOT/packager-out"
rm -rf "$PACKAGER_OUT"

echo "[macos-universal] packaging DMG"
CARGO_TARGET_DIR="$BUILD_ROOT/target" \
CARGO_PACKAGER_SKIP_BUILD=true \
cargo packager --release --formats app --manifest-path "$MANIFEST" \
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
DMG_NAME="$(basename "$APP_PATH" ".app").dmg"
DEST_PATH="$DIST_DIR/$DMG_NAME"
rm -f "$DEST_PATH"

hdiutil create -volname "$VOLNAME" -srcfolder "$DMG_STAGING" -ov -format UDZO "$DEST_PATH" >/tmp/hdiutil-create.log 2>&1
cat /tmp/hdiutil-create.log >> "$LOG_DIR/dmg-pack.log"
rm -f /tmp/hdiutil-create.log

shasum -a 256 "$DEST_PATH" | tee "$LOG_DIR/dmg-sha256.log"

find "$LOG_DIR" -type f -empty -delete

echo "[macos-universal] DMG ready: $DEST_PATH"
