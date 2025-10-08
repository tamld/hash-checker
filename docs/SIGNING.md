# Windows Signing Guide (SignPath Foundation)

This document captures the end-to-end plan for automating Windows code signing with SignPath Foundation while keeping macOS unsigned (Gatekeeper bypass instructions remain in the README).

## 1. Prerequisites
- Hash Checker repository must be **public**.
- GitHub Actions workflow capable of producing unsigned artefacts (`.zip`, `.exe`, `.msi`/NSIS installer).
- Project maintainer GitHub account ready to authenticate with SignPath.
- Dedicated GitHub environment/secrets for the signing pipeline.

## 2. Register the Project
1. Visit <https://signpath.org/>.
2. Create an organisation (or join an existing one) and start a *SignPath for Open Source* subscription.
3. Import the public GitHub repository and create a **Project Configuration** named `hash-checker`.
4. Note the generated *Organisation ID* and *Project ID*. These will be used inside the CI pipeline.

## 3. Configure GitHub Actions Secrets
Add the following secrets to the repository (or organisation):

| Secret name           | Description                                      |
|-----------------------|--------------------------------------------------|
| `SIGNPATH_ORG`        | Organisation ID from the SignPath dashboard.     |
| `SIGNPATH_PROJECT`    | Project ID referencing the hash-checker project. |
| `SIGNPATH_API_TOKEN`  | API token with permission to upload/download artefacts. |

Optional (recommended):
- `SIGNPATH_ENVIRONMENT` if you use non-default release channels.
- `SIGNPATH_STRICT` = `true` to fail the pipeline when signing is rejected.

## 4. Build Workflow Integration
1. Modify the Windows job in `.github/workflows/ci.yml` / `release.yml` to:
   - Produce unsigned portable ZIP and NSIS installer artefacts.
   - Upload those artefacts as workflow outputs.
2. Insert a signing step **after build** and **before publish**:
   ```yaml
   - name: Submit artefact for signing
     uses: signpath/signpath-github-action@v1
     with:
       organization-id: ${{ secrets.SIGNPATH_ORG }}
       project-slug:    ${{ secrets.SIGNPATH_PROJECT }}
       api-token:       ${{ secrets.SIGNPATH_API_TOKEN }}
       artifact-path:   dist/hash-checker-windows-portable.zip
       configuration:   Release
       wait-for-completion: true
   ```
3. Repeat for the NSIS installer if produced (`dist/win/installer/*.exe`).
4. Download the signed artefacts to replace the unsigned ones prior to the release step.

## 5. Release Workflow
- Keep the publish job gated on successful signing.
- Attach only signed artefacts to GitHub Releases.
- Retain SHA256SUMS generated **after** signing to ensure checksums reflect the signed files.

## 6. Monitoring & Maintenance
- Review SignPath audit logs periodically.
- Rotate `SIGNPATH_API_TOKEN` at least twice a year.
- Update this document whenever SignPath changes their configuration flow or GitHub Action version.

> macOS DMG builds remain unsigned. README documents Gatekeeper bypass instructions so users can install without notarisation fees.
