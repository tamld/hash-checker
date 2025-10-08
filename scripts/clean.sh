#!/usr/bin/env bash
set -euo pipefail

PROJECT_ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)

# Re-use packaging cleanup (respects KEEP_PACKAGING flag)
KEEP_PACKAGING="${KEEP_PACKAGING:-0}" "${PROJECT_ROOT}/scripts/cleanup-packaging.sh"

# Remove generic build artefacts
echo "[clean] Removing build and dist directories"
rm -rf "${PROJECT_ROOT}/dist" \
       "${PROJECT_ROOT}/build" \
       "${PROJECT_ROOT}/rust/hash-checker/target" \
       "${PROJECT_ROOT}/rust/hash-checker-gui/target" \
       "${PROJECT_ROOT}/target" \
       "${PROJECT_ROOT}/.vagrant"

# Runner temp folders (macOS/Linux) + Windows Git Bash (/tmp same path)
for temp_path in /tmp/hash-checker-build \
                 /tmp/hash-checker-gui \
                 /tmp/hash-checker-deb \
                 /tmp/hash-checker-win; do
  if [[ -e "${temp_path}" ]]; then
    rm -rf "${temp_path}"
  fi
done

# Remove Windows build artefacts that might linger under project root
find "${PROJECT_ROOT}" -maxdepth 1 -type f \( -name '*.exe' -o -name '*.zip' -o -name '*.msi' \) -print0 | xargs -0 -r rm -f

# Conditional Docker cleanup (supports runners without Docker)
if [[ "${CLEAN_DOCKER:-1}" == "1" ]]; then
  if command -v docker >/dev/null 2>&1; then
    echo "[clean] Pruning Docker volumes (requires permission)"
    docker volume prune -f
  else
    echo "[clean] Docker not available; skipping volume prune."
  fi
fi

echo "[clean] Completed cross-platform cleanup. Set CLEAN_DOCKER=0 to skip Docker prune."
