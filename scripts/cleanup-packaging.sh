#!/usr/bin/env bash
set -euo pipefail

# Skip if caller explicitly wants to retain artefacts
if [[ "${KEEP_PACKAGING:-0}" == "1" ]]; then
  echo "[cleanup-packaging] KEEP_PACKAGING=1 -> skipping cleanup."
  exit 0
fi

PROJECT_ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)

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

echo "[cleanup-packaging] Removed packaging artefacts. Set KEEP_PACKAGING=1 to retain them next time."
