# Windows Signing Guide (SignPath Foundation)

This document captures the end-to-end plan for automating Windows code signing with SignPath Foundation while keeping macOS unsigned (Gatekeeper bypass instructions remain in the README).

> **Status (2025-10-13):** SignPath onboarding is in progress. Releases prior to the integration ship **unsigned** Windows artefacts; users must verify them via checksums/GPG as documented in `docs/security/VERIFICATION_GUIDE.md`.

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

## 3. Configure GitHub Actions Secrets & Variables
Set the following **repository secrets**:

| Secret | Purpose |
| --- | --- |
| `SIGNPATH_API_TOKEN` | API token with permission to submit signing requests and download the signed artefacts. |

Set the following **repository variables** (`Settings → Secrets and variables → Actions → Variables`):

| Variable | Example | Description |
| --- | --- | --- |
| `SIGNPATH_ORGANIZATION_ID` | `2b652b8d-2f29-4f5a-8fa0-...` | Organisation GUID from the SignPath dashboard. |
| `SIGNPATH_PROJECT_SLUG` | `hash-checker` | Project slug configured in SignPath. |
| `SIGNPATH_SIGNING_POLICY_SLUG` | `release-signing` | Signing policy that approves public releases. |
| `SIGNPATH_ARTIFACT_CONFIGURATION_SLUG` | `windows-exe` | Artifact configuration covering `.exe` outputs (portable + installer). |

Optional:
- Additional configuration slugs if you maintain multiple release channels.
- A dedicated environment (e.g. `staging`) with narrowed permissions if you gate manual approvals.

## 4. Build Workflow Integration
1. The Windows job in `.github/workflows/release.yml` already uploads unsigned binaries and the NSIS installer as artefacts.
2. The `windows_sign` job consumes these artefacts and calls:
   ```yaml
   - uses: signpath/github-actions/actions/submit-signing-request@v0.1
     with:
       api-token: ${{ secrets.SIGNPATH_API_TOKEN }}
       organization-id: ${{ vars.SIGNPATH_ORGANIZATION_ID }}
       project-slug: ${{ vars.SIGNPATH_PROJECT_SLUG }}
       signing-policy-slug: ${{ vars.SIGNPATH_SIGNING_POLICY_SLUG }}
       artifact-configuration-slug: ${{ vars.SIGNPATH_ARTIFACT_CONFIGURATION_SLUG }}
       github-artifact-id: ${{ needs.windows.outputs.cli_artifact_id }}
       wait-for-completion: true
       output-artifact-directory: signed/cli
   ```
3. The same step is repeated for the GUI binary and NSIS installer.
4. The job repackages the portable ZIP with the signed executables and regenerates SHA256SUMS so downstream steps only see signed artefacts.
5. Finally, the signed artefacts are uploaded (`windows-*-signed`) and consumed by the `publish` job.

## 5. Release Workflow
- The `publish` job automatically prefers signed artefacts; if signing is skipped, it falls back to unsigned outputs and logs the decision.
- Ensure `SHA256SUMS` is generated **after** the signing step so checksums match the signed files.
- Release notes must include the SignPath signing status and the GPG fingerprint used to sign `SHA256SUMS`.

## 6. Monitoring & Maintenance
- Review SignPath audit logs periodically.
- Rotate `SIGNPATH_API_TOKEN` at least twice a year.
- Update this document whenever SignPath changes their configuration flow or GitHub Action version.
- Keep `CODE_OF_CONDUCT.md` up to date to satisfy SignPath OSS requirements.

> macOS DMG builds remain unsigned. README documents Gatekeeper bypass instructions so users can install without notarisation fees.
