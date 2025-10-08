#!/usr/bin/env bash
set -euo pipefail

# Skip if caller explicitly wants to retain artefacts
if [[ "${KEEP_PACKAGING:-0}" == "1" ]]; then
  echo "[cleanup-packaging] KEEP_PACKAGING=1 -> skipping cleanup."
  exit 0
fi

PROJECT_ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
TMP_ROOT="${TMPDIR:-${TEMP:-${TMP:-}}}"

canonical_path() {
  local raw="$1"
  if [[ "$raw" =~ ^[A-Za-z]:\\ ]] && command -v cygpath >/dev/null 2>&1; then
    cygpath "$raw"
  else
    printf '%s\n' "$raw"
  fi
}

# Remove packager staging directory
rm -rf "${PROJECT_ROOT}/rust/hash-checker-gui/target/packager"

# Remove dist work directories but keep final archives (tar/zip sit at dist root)
if [[ -d "${PROJECT_ROOT}/dist/linux" ]]; then
  rm -rf "${PROJECT_ROOT}/dist/linux"
fi

# Prune well-known temporary export directories
for temp_path in /tmp/hash-checker-build \
                 /tmp/hash-checker-gui \
                 /tmp/hash-checker-deb \
                 /tmp/hash-checker-win; do
  if [[ -e "${temp_path}" ]]; then
    rm -rf "${temp_path}"
  fi
done

if [[ -n "${TMP_ROOT}" ]]; then
  for suffix in hash-checker-build hash-checker-gui hash-checker-deb hash-checker-win; do
    target_path="$(canonical_path "${TMP_ROOT%/}/${suffix}")"
    if [[ -e "${target_path}" ]]; then
      rm -rf "${target_path}"
    fi
  done
fi

echo "[cleanup-packaging] Removed packaging artefacts. Set KEEP_PACKAGING=1 to retain them next time."
