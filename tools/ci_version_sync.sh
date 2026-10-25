#!/usr/bin/env bash
# ==============================================================================
# CI Gate: SSoT Version Synchronization Check (Modeled on g8s RELEASE_SOP)
# Verifies VERSION, manifest.json, Cargo.toml (CLI + GUI), and CHANGELOG.md match.
# ==============================================================================
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "${ROOT_DIR}"

if [ ! -f VERSION ]; then
  echo "❌ [version-sync] Missing VERSION file at repository root" >&2
  exit 1
fi

SSOT_VERSION="$(tr -d '[:space:]' < VERSION)"
if [ -z "${SSOT_VERSION}" ]; then
  echo "❌ [version-sync] VERSION file is empty" >&2
  exit 1
fi

MANIFEST_VERSION="$(python3 -c 'import json; print(json.load(open("manifest.json"))["version"])')"
CLI_VERSION="$(awk -F'"' '/^version = / {print $2; exit}' rust/hash-checker/Cargo.toml)"
GUI_VERSION="$(awk -F'"' '/^version = / {print $2; exit}' rust/hash-checker-gui/Cargo.toml)"

ERRORS=0

if [ "${MANIFEST_VERSION}" != "${SSOT_VERSION}" ]; then
  echo "❌ [version-sync] manifest.json version (${MANIFEST_VERSION}) != VERSION (${SSOT_VERSION})" >&2
  ERRORS=$((ERRORS + 1))
fi

if [ "${CLI_VERSION}" != "${SSOT_VERSION}" ]; then
  echo "❌ [version-sync] rust/hash-checker/Cargo.toml (${CLI_VERSION}) != VERSION (${SSOT_VERSION})" >&2
  ERRORS=$((ERRORS + 1))
fi

if [ "${GUI_VERSION}" != "${SSOT_VERSION}" ]; then
  echo "❌ [version-sync] rust/hash-checker-gui/Cargo.toml (${GUI_VERSION}) != VERSION (${SSOT_VERSION})" >&2
  ERRORS=$((ERRORS + 1))
fi

if ! grep -q "## \[${SSOT_VERSION}\]" CHANGELOG.md; then
  echo "❌ [version-sync] CHANGELOG.md missing section '## [${SSOT_VERSION}]'" >&2
  ERRORS=$((ERRORS + 1))
fi

if [ "${ERRORS}" -gt 0 ]; then
  echo "❌ [version-sync] Failed with ${ERRORS} version mismatch(es)." >&2
  exit 1
fi

echo "✅ [version-sync] All version anchors synchronized at v${SSOT_VERSION} (VERSION, manifest.json, CLI Cargo.toml, GUI Cargo.toml, CHANGELOG.md)"
