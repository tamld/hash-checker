#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 2 ]]; then
  echo "Usage: $0 <version> <download-base-url>" >&2
  echo "Example: $0 0.1.1 https://github.com/tamld/hash-checker/releases/download/v0.1.1" >&2
  exit 1
fi

VERSION="$1"
BASE_URL="$2"
OUT_DIR="dist/manifests"
mkdir -p "$OUT_DIR/winget" "$OUT_DIR/homebrew"

cat >"$OUT_DIR/winget/hash-checker.yaml" <<TEMPLATE
# Winget manifest template generated on $(date -u)
PackageIdentifier: tamld.HashChecker
PackageVersion: ${VERSION}
PackageLocale: en-US
Publisher: tamld
PublisherUrl: https://github.com/tamld/hash-checker
PackageName: Hash Checker
ShortDescription: Cross-platform integrity checker written in Rust.
Installers:
  - Architecture: x64
    InstallerType: zip
    InstallerUrl: ${BASE_URL}/hash-checker-windows-portable.zip
    InstallerSha256: <fill-with-sha256>
    Commands:
      - hash-checker
      - hash-checker-gui
  - Architecture: x64
    InstallerType: exe
    InstallerUrl: ${BASE_URL}/hash-checker-gui-setup.exe
    InstallerSha256: <fill-with-sha256>
ManifestType: singleton
ManifestVersion: 1.6.0
TEMPLATE

cat >"$OUT_DIR/homebrew/hash-checker.rb" <<TEMPLATE
# Homebrew formula template generated on $(date -u)
class HashChecker < Formula
  desc "Cross-platform integrity checker"
  homepage "https://github.com/tamld/hash-checker"
  version "${VERSION}"
  url "${BASE_URL}/hash-checker-macos-universal.tar.gz"
  sha256 "<fill-with-sha256>"

  def install
    bin.install "hash-checker"
    bin.install "hash-checker-gui"
  end

  test do
    system "#{bin}/hash-checker", "--help"
  end
end
TEMPLATE

echo "Generated manifests in ${OUT_DIR}" >&2
