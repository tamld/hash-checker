#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WORK_DIR="${WORK_DIR:-${TMPDIR:-/tmp}/hash-checker-debian-smoke}"
LOG_DIR="${LOG_DIR:-${ROOT_DIR}/logs/cli-snapshots}"
mkdir -p "${WORK_DIR}"
mkdir -p "${LOG_DIR}"

echo "[debian-smoke] workspace: ${WORK_DIR}"

export CARGO_TARGET_DIR="${WORK_DIR}/target"

echo "[debian-smoke] installing cargo-packager@0.11.7"
cargo install cargo-packager@0.11.7 --locked

echo "[debian-smoke] building Debian package via cargo packager"
pushd "${ROOT_DIR}/rust/hash-checker-gui" >/dev/null
cargo packager --release --formats deb
popd >/dev/null

DEB_PATH="$(find "${ROOT_DIR}/rust/hash-checker-gui/target/packager" -name '*.deb' -print -quit)"
if [[ -z "${DEB_PATH}" ]]; then
  echo "[debian-smoke] error: unable to locate .deb artefact" >&2
  exit 1
fi
echo "[debian-smoke] located artefact: ${DEB_PATH}"

if command -v sudo >/dev/null 2>&1; then
  sudo_cmd="sudo"
elif [ "$(id -u)" -eq 0 ]; then
  sudo_cmd=""
else
  echo "[debian-smoke] sudo not found and not running as root; aborting"
  exit 1
fi

echo "[debian-smoke] installing package"
${sudo_cmd} dpkg -i "${DEB_PATH}" || ${sudo_cmd} apt-get install -f -y

TIMESTAMP="$(date +%Y%m%d-%H%M%S)"
CLI_LOG="${LOG_DIR}/debian-smoke-${TIMESTAMP}.log"

run_cli() {
  local cmd="$1"
  echo "\$ ${cmd}" >> "${CLI_LOG}"
  eval "${cmd}" >> "${CLI_LOG}" 2>&1
  echo "" >> "${CLI_LOG}"
}

echo "[debian-smoke] running CLI smoke commands (logs -> ${CLI_LOG})"
run_cli "hash-checker --version"
run_cli "hash-checker --help | head -n 5"

echo "[debian-smoke] running GUI smoke test"
if command -v xvfb-run >/dev/null 2>&1; then
  run_cli "xvfb-run --auto-servernum hash-checker-gui -- --smoke-test"
else
  run_cli "hash-checker-gui -- --smoke-test"
fi

echo "[debian-smoke] debian smoke completed"
