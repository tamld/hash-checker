#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPORT_DIR="${ROOT_DIR}/logs/reports"
mkdir -p "${REPORT_DIR}"

DATE_TAG="$(date +%Y%m%d)"
REPORT_FILE="${REPORT_DIR}/deps-report-${DATE_TAG}.md"

TARGETS=("rust/hash-checker" "rust/hash-checker-gui")

{
  echo "# Dependency Refresh Report (${DATE_TAG})"
  echo
  echo "Generated: $(date -u '+%Y-%m-%d %H:%M:%S UTC')"
  echo

  for target in "${TARGETS[@]}"; do
    target_path="${ROOT_DIR}/${target}"
    echo "## Target: \`${target}\`"
    echo

    echo "### \`cargo outdated\`"
    echo
    if command -v cargo-outdated >/dev/null 2>&1; then
      echo '```text'
      (cd "${target_path}" && cargo outdated 2>&1 || true)
      echo '```'
    else
      echo "*(cargo-outdated not installed)*"
    fi
    echo

    echo "### \`cargo audit\`"
    echo
    if command -v cargo-audit >/dev/null 2>&1; then
      echo '```text'
      (cd "${target_path}" && cargo audit 2>&1 || true)
      echo '```'
    else
      echo "*(cargo-audit not installed)*"
    fi
    echo
  done
} > "${REPORT_FILE}"

echo "Report generated at: ${REPORT_FILE}"
exit 0
