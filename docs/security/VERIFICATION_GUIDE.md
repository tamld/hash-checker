# End-User Checksum Verification Guide

Updated: 2025-10-13

Use these steps to confirm that downloaded Hash Checker artefacts match the expected digests published alongside each release.

## 1. Download Artefacts and Checksums
1. Fetch the desired installer/binary from the GitHub Release page.
2. Download the accompanying `SHA256SUMS` file.
3. Place both files in the same directory.

## 2. Verify on macOS or Linux
```bash
# Replace artefact name as needed
shasum -a 256 hash-checker-<platform>.tar.gz

# Compare the output against SHA256SUMS
grep hash-checker-<platform>.tar.gz SHA256SUMS
```

Alternatively, use the Rust CLI:
```bash
hash-checker hash-checker-<platform>.tar.gz "$(grep hash-checker-<platform>.tar.gz SHA256SUMS | cut -d' ' -f1)"
```

## 3. Verify on Windows (PowerShell)
```powershell
Get-FileHash .\hash-checker-windows-portable.zip -Algorithm SHA256

# Compare with the digest recorded in SHA256SUMS
Select-String -Path .\SHA256SUMS -Pattern "hash-checker-windows-portable.zip"
```

To cross-check using the Rust CLI (after installing via `cargo install hash-checker` or running the packaged binary):
```powershell
.\hash-checker.exe .\hash-checker-windows-portable.zip <expected-digest>
```

## 4. Expected Outcomes
- Matching hashes confirm the download is intact (`Hashes match ✅` from the CLI).
- A mismatch indicates corruption or tampering; re-download the artefact and re-run the check. If the mismatch persists, halt the installation and contact maintainers.

## 5. Verify Release GPG Signature
Releases ship both `SHA256SUMS` and `SHA256SUMS.sig`. Import the maintainers’ public key (check the fingerprint in the release notes), then:

```bash
gpg --verify SHA256SUMS.sig SHA256SUMS
```

On Windows, install Gpg4win and run:
```powershell
gpg --verify .\SHA256SUMS.sig .\SHA256SUMS
```

If the signature is valid, GPG prints `Good signature`; otherwise do not trust the artefacts.

## 6. Verify Windows Authenticode Signature (when available)
When SignPath signing is enabled, use either command:

```powershell
Get-AuthenticodeSignature .\hash-checker-gui-setup.exe | Format-List
```

Look for `Status : Valid` and the expected publisher (SignPath Foundation certificate). Alternatively, with the Windows SDK:

```powershell
signtool verify /pa /all .\hash-checker-gui-setup.exe
```

If verification fails, stop the installation and contact the maintainers.

## 7. Verify macOS Artefacts
When macOS builds are notarised or signed, run:

```bash
spctl --assess --type exec --verbose "Hash Checker.app"
```

For DMG files:
```bash
codesign --verify --deep --strict "Hash Checker.app"
```

Unsigned builds will trigger Gatekeeper prompts; follow the workaround in `README.md` until notarisation is available.

## 8. Troubleshooting
- Ensure the checksum file corresponds to the same release/tag as the artefact.
- On macOS, Gatekeeper may flag unsigned binaries; follow the manual bypass instructions in `README.md`.
- If CLI verification reports `path ... is not a regular file`, confirm you are pointing to an actual file and not a directory or device.
