# CI Signing Pipeline Reference

Updated: 2025-10-13

## Overview
- `release.yml` now supports GPG signing of the aggregated `SHA256SUMS` file and Authenticode signing via SignPath (when configured).
- Unsigned artefacts are still published as a fallback whenever signing secrets or environment variables are absent.

## Required Secrets and Variables (GitHub Actions)
| Type | Name | Description |
| --- | --- | --- |
| Secret | `GPG_PRIVATE_KEY` | ASCII-armoured OpenPGP private key. Export with `gpg --armor --export-secret-keys`. |
| Secret | `GPG_PASSPHRASE` | Passphrase protecting the GPG key. |
| Secret | `SIGNPATH_API_TOKEN` | SignPath API token with permission to submit signing requests. |
| Variable | `SIGNPATH_ORGANIZATION_ID` | SignPath organisation GUID. |
| Variable | `SIGNPATH_PROJECT_SLUG` | Project slug (e.g. `hash-checker`). |
| Variable | `SIGNPATH_SIGNING_POLICY_SLUG` | Signing policy slug (e.g. `release-signing`). |
| Variable | `SIGNPATH_ARTIFACT_CONFIGURATION_SLUG` | Artifact configuration slug shared by the CLI, GUI, and NSIS artefacts. |

> Leave the secrets unset if you are not ready to sign yet—the workflow will run and publish unsigned artefacts automatically.

## release.yml Flow
1. Jobs `linux`, `macos`, and `windows` build and upload unsigned artefacts. The Windows job also uploads individual executables and the NSIS installer.
2. `windows_sign` (runs when SignPath secrets are present):
   - Submits each `.exe` to SignPath using `signpath/github-actions/actions/submit-signing-request@v0.1`.
   - Downloads the signed files, rebuilds the portable ZIP, and regenerates the accompanying SHA256 checksums.
   - Uploads `windows-*-signed` artefacts for the publish job.
3. `publish`:
   - Downloads every artefact, preferring signed versions when available and falling back to the unsigned ones otherwise.
   - Creates `release-final/SHA256SUMS` and signs it with GPG (`crazy-max/ghaction-import-gpg`).
   - Attaches `SHA256SUMS` and `SHA256SUMS.sig` to the GitHub Release.

## Practical Notes
- The `windows_sign` job requires `actions: read` and `contents: read` permissions.
- If SignPath is not configured, the release will contain unsigned artefacts plus checksums; users can still verify integrity with GPG or manual hashing.
- Once SignPath signing is live, extend `docs/security/VERIFICATION_GUIDE.md` with `signtool` / `Get-AuthenticodeSignature` usage.
- Run a dry run (`workflow_dispatch`) on a staging branch to validate SignPath integration before tagging a public release.
- After a successful run, confirm that new artefacts appear under `windows-*-signed` and that `release-final/SHA256SUMS` is regenerated from the signed outputs.

## Maintenance
- Rotate the GPG key and passphrase periodically (at least every six months) and refresh the stored secrets.
- Monitor the `windows_sign` job for each release; if it fails, communicate that the artefacts are unsigned in the release notes.
- Document the GPG fingerprint in README and release notes so users can verify the signature.
