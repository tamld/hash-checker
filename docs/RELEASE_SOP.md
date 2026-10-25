# Release Engineering, Binary Matrix & Standalone Installer SOP (`hash-checker`)

> **Document Class:** Normative Operational Standard (Modeled on `g8s` Release SOP)  
> **Applies To:** All `vX.Y.Z` tags, GitHub Releases, Multi-OS Binaries, Setup Installers, and Standalone Scripts  
> **Automated Pipeline:** [`.github/workflows/release.yml`](../.github/workflows/release.yml)  
> **Standalone Installers:** [`scripts/install.sh`](../scripts/install.sh) (macOS/Linux) · [`scripts/install.ps1`](../scripts/install.ps1) (Windows)

---

## 1. Release Principles

1. **Zero-Drift Version SSoT**: [`VERSION`](../VERSION), [`manifest.json`](../manifest.json), [`rust/hash-checker/Cargo.toml`](../rust/hash-checker/Cargo.toml), [`rust/hash-checker-gui/Cargo.toml`](../rust/hash-checker-gui/Cargo.toml), and [`CHANGELOG.md`](../CHANGELOG.md) MUST match the Git tag (`vX.Y.Z`) 100%, enforced mechanically by [`tools/ci_version_sync.sh`](../tools/ci_version_sync.sh) and the `verify` job in [`.github/workflows/release.yml`](../.github/workflows/release.yml).
2. **Dual-Track Distribution (Portable CLI+GUI Archives + Native OS Setup Packages)**: Every release publishes both:
   - **Portable Archives (`tar.gz` / `.zip`)** containing both `hash-checker` (CLI) and `hash-checker-gui` (GUI) for 1-line script installation (`scripts/install.sh` / `scripts/install.ps1`) and CI/server automation.
   - **Native Desktop Setup Packages** (`.dmg` universal macOS bundle, `.deb` + `.AppImage` Linux desktop packages, and `*-setup.exe` Windows NSIS installer).
3. **Fail-Closed Checksum Verification**: Every release asset is hashed into both `checksums.txt` and `SHA256SUMS` at the release root. Standalone installers verify the SHA-256 digest before extracting or executing any binary.

---

## 2. Multi-Platform Release Asset Matrix

| Platform | Architecture | Portable Archive (CLI + GUI) | Native Desktop Setup / Package |
| :--- | :--- | :--- | :--- |
| **macOS** | Universal (`arm64` + `x86_64`) | `hash-checker_<version>_darwin_all.tar.gz` | `hash-checker.dmg` (Signed-ready `.app` bundle) |
| **Linux** | `amd64` (`x86_64`) | `hash-checker_<version>_linux_amd64.tar.gz` | `hash-checker-gui_<version>_amd64.deb`, `hash-checker-gui_<version>_x86_64.AppImage` |
| **Windows** | `amd64` (`x86_64`) | `hash-checker_<version>_windows_amd64.zip` (`+ hash-checker-windows-portable.zip`) | `hash-checker-gui_<version>_x64-setup.exe` (NSIS Setup) |
| **Manifests** | All | `checksums.txt` & `SHA256SUMS` | Unified SHA-256 digests for all archives & installers |

---

## 3. Standalone One-Line Installation & Verification

### 3.1 macOS & Linux (`scripts/install.sh`)
```bash
# Install latest release (both hash-checker CLI and hash-checker-gui) to ~/.local/bin
curl -fsSL https://raw.githubusercontent.com/tamld/hash-checker/main/scripts/install.sh | bash

# Pin a specific version and custom install directory
HASH_CHECKER_VERSION=v0.2.0 INSTALL_DIR=/usr/local/bin ./scripts/install.sh
```

### 3.2 Windows PowerShell (`scripts/install.ps1`)
```powershell
irm https://raw.githubusercontent.com/tamld/hash-checker/main/scripts/install.ps1 | iex
```

### 3.3 Manual SHA-256 Verification
```bash
# Linux
sha256sum -c checksums.txt --ignore-missing

# macOS
shasum -a 256 -c checksums.txt --ignore-missing
```

---

## 4. Pre-Release Verification & Tagging Ritual

```bash
# 1. Verify all 4 SDD & Version CI gates + Feature Map
make verify-release

# 2. Build local binaries & run full Rust CLI + GUI test suites
make local
make test-host

# 3. Tag and push to trigger .github/workflows/release.yml
git tag -a vX.Y.Z -m "Release vX.Y.Z"
git push origin main --tags
```
