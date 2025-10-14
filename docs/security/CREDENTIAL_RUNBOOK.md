# Credential Runbook

_Updated: 2025-10-14_

This playbook describes how to manage signing credentials (SignPath API tokens, certificates, and GPG keys) for the Hash Checker release pipeline.

## 1. Scope
- SignPath-issued certificates (test + release).
- GitHub Actions secrets/variables: `SIGNPATH_API_TOKEN`, `SIGNPATH_ORGANIZATION_ID`, `SIGNPATH_PROJECT_SLUG`, `SIGNPATH_SIGNING_POLICY_SLUG`, `SIGNPATH_ARTIFACT_CONFIGURATION_SLUG`.
- GPG private key/passphrase used to sign `SHA256SUMS`.

## 2. Storage & Access Control
- Store long-lived secrets in GitHub Actions secrets (repo or org level) with access limited to maintainers.
- Keep the GPG private key + passphrase in the internal secret vault (never committed). Require MFA for retrieval.
- Keep SignPath IDs (`vars`) non-sensitive; only the API token belongs in secrets.

## 3. Onboarding / Provisioning
1. Complete the SignPath OSS onboarding to obtain the test certificate.
2. Record the event at `logs/credentials/<yyyy-mm-dd>-signpath-onboarding.md` (template in section 6).
3. Populate GitHub secrets/variables listed above.
4. Update `.github/workflows/release.yml` if artefact paths or configuration change.

## 4. Regular Rotation
- Rotate SignPath API tokens and the GPG passphrase at least every 6 months.
- After rotation: update secrets, trigger `gh workflow run release.yml --ref main --field run_windows=true --field run_linux=false --field run_macos=false` as a dry run, and capture the run URL.
- Log the rotation outcome in `logs/credentials/<yyyy-mm-dd>-rotation.md`.

## 5. Incident Response
- Suspected leak: revoke the SignPath token, temporarily limit workflow permissions (e.g., `permissions: contents: read`).
- Revoke certificates via SignPath ticket, issue a patched release with the new certificate.
- File an incident report at `logs/credentials/<yyyy-mm-dd>-incident.md`.

## 6. Log Templates
```text
# SignPath onboarding log
Date:
Operator:
Certificate type: test | release
Secrets updated:
Verification (workflow URL):
Notes:
```

```text
# Credential rotation log
Date:
Operator:
Secrets rotated:
Verification (workflow URL):
Notes:
```

## 7. Follow-up Tasks
- Keep this runbook synced with internal documentation.
- Update `docs/security/SIGNPATH_CHECKLIST.md` and `docs/security/CI_SIGNING.md` when SignPath moves to production.
