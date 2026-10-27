#!/usr/bin/env bash
# ==============================================================================
# hash-checker — Cross-Platform Standalone Installer (CLI + GUI)
# Modeled on g8s Zero-Trust Release & Checksum Verification Standard
#
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/tamld/hash-checker/main/scripts/install.sh | bash
#   HASH_CHECKER_VERSION=v0.2.1 INSTALL_DIR=/usr/local/bin ./scripts/install.sh
# ==============================================================================
set -euo pipefail

REPO="${HASH_CHECKER_REPO:-tamld/hash-checker}"
VERSION="${HASH_CHECKER_VERSION:-latest}"
INSTALL_DIR="${INSTALL_DIR:-}"

log()  { printf "\033[1;34m[hash-checker-install]\033[0m %s\n" "$*"; }
ok()   { printf "\033[1;32m[hash-checker-install]\033[0m %s\n" "$*"; }
err()  { printf "\033[1;31m[hash-checker-install] ERROR:\033[0m %s\n" "$*" >&2; exit 1; }

# 1. Detect OS
OS_RAW="$(uname -s)"
case "${OS_RAW}" in
  Linux*)  OS="linux" ;;
  Darwin*) OS="darwin" ;;
  *)       err "Unsupported operating system: ${OS_RAW} (supported: Linux, macOS; on Windows use scripts/install.ps1)" ;;
esac

# 2. Detect Architecture
ARCH_RAW="$(uname -m)"
case "${ARCH_RAW}" in
  x86_64|amd64)  ARCH="amd64" ;;
  aarch64|arm64) ARCH="arm64" ;;
  *)             err "Unsupported architecture: ${ARCH_RAW} (supported: amd64, arm64)" ;;
esac

# 3. Resolve Target Version
if [ "${VERSION}" = "latest" ]; then
  log "Resolving latest release from https://github.com/${REPO} ..."
  VERSION="$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/' || true)"
  if [ -z "${VERSION}" ]; then
    err "Could not resolve latest release tag from GitHub API. Set HASH_CHECKER_VERSION=vX.Y.Z explicitly."
  fi
fi

VERSION_NUM="${VERSION#v}"
BASE_URL="https://github.com/${REPO}/releases/download/${VERSION}"

# macOS uses universal fat binary archive (darwin_all) by default, with arch-specific fallback
if [ "${OS}" = "darwin" ]; then
  ARCHIVE="hash-checker_${VERSION_NUM}_darwin_all.tar.gz"
else
  ARCHIVE="hash-checker_${VERSION_NUM}_${OS}_${ARCH}.tar.gz"
fi

CHECKSUMS_URL="${BASE_URL}/checksums.txt"
SHA256SUMS_URL="${BASE_URL}/SHA256SUMS"

# 4. Determine Install Directory
if [ -z "${INSTALL_DIR}" ]; then
  if [ -d "${HOME}/.local/bin" ] || mkdir -p "${HOME}/.local/bin" 2>/dev/null; then
    INSTALL_DIR="${HOME}/.local/bin"
  elif [ -w "/usr/local/bin" ]; then
    INSTALL_DIR="/usr/local/bin"
  else
    err "Cannot write to ~/.local/bin or /usr/local/bin. Set INSTALL_DIR=/path/to/bin."
  fi
fi
mkdir -p "${INSTALL_DIR}"

# 5. Download & Verify SHA-256 Checksum (Fail-Closed)
TMP_DIR="$(mktemp -d)"
trap 'rm -rf "${TMP_DIR}"' EXIT

log "Downloading ${ARCHIVE} (${VERSION}) ..."
curl -fsSL "${BASE_URL}/${ARCHIVE}" -o "${TMP_DIR}/${ARCHIVE}"

log "Downloading checksums manifest ..."
if ! curl -fsSL "${CHECKSUMS_URL}" -o "${TMP_DIR}/checksums.txt" 2>/dev/null; then
  curl -fsSL "${SHA256SUMS_URL}" -o "${TMP_DIR}/checksums.txt"
fi

log "Verifying SHA-256 checksum (fail-closed) ..."
EXPECTED_SHA="$(awk -v f="${ARCHIVE}" '$2 == f || $2 == ("*" f) {print $1}' "${TMP_DIR}/checksums.txt" | head -n 1)"
if [ -z "${EXPECTED_SHA}" ]; then
  err "Archive ${ARCHIVE} not found in checksums manifest!"
fi

if command -v sha256sum >/dev/null 2>&1; then
  ACTUAL_SHA="$(sha256sum "${TMP_DIR}/${ARCHIVE}" | awk '{print $1}')"
elif command -v shasum >/dev/null 2>&1; then
  ACTUAL_SHA="$(shasum -a 256 "${TMP_DIR}/${ARCHIVE}" | awk '{print $1}')"
else
  err "Neither sha256sum nor shasum found on system; cannot verify binary integrity."
fi

if [ "${EXPECTED_SHA}" != "${ACTUAL_SHA}" ]; then
  err "SHA-256 mismatch for ${ARCHIVE}! Expected ${EXPECTED_SHA}, got ${ACTUAL_SHA}"
fi
ok "SHA-256 verified: ${ACTUAL_SHA}"

# 6. Extract & Install Binaries
tar -xzf "${TMP_DIR}/${ARCHIVE}" -C "${TMP_DIR}"
if [ ! -f "${TMP_DIR}/hash-checker" ]; then
  err "Binary 'hash-checker' not found in extracted archive!"
fi

install -m 0755 "${TMP_DIR}/hash-checker" "${INSTALL_DIR}/hash-checker"
ok "Installed CLI to ${INSTALL_DIR}/hash-checker"

if [ -f "${TMP_DIR}/hash-checker-gui" ]; then
  install -m 0755 "${TMP_DIR}/hash-checker-gui" "${INSTALL_DIR}/hash-checker-gui"
  ok "Installed GUI to ${INSTALL_DIR}/hash-checker-gui"
fi

# 7. Post-Install Smoke Check
"${INSTALL_DIR}/hash-checker" --version || true
if [ -x "${INSTALL_DIR}/hash-checker-gui" ]; then
  "${INSTALL_DIR}/hash-checker-gui" --version || true
fi

case ":${PATH}:" in
  *":${INSTALL_DIR}:"*) ;;
  *)
    printf "\n\033[1;33mNOTE:\033[0m Add %s to your PATH:\n  export PATH=\"%s:\$PATH\"\n" "${INSTALL_DIR}" "${INSTALL_DIR}"
    ;;
esac
