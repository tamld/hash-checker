# SignPath Onboarding Checklist (Placeholder)

_Status: awaiting SignPath OSS approval._

Use this template once SignPath provisions the OSS subscription and test certificate. Fill each section during onboarding.

## 1. Subscription Details
- Organization ID: `<pending>`
- Project slug: `<pending>`
- Signing policy slug: `<pending>`
- Artifact configuration slug: `<pending>`

## 2. GitHub Actions Secrets / Variables
| Name | Type | Value / Notes | Set on (date) |
| --- | --- | --- | --- |
| `SIGNPATH_API_TOKEN` | Secret | `<pending>` | |
| `SIGNPATH_ORGANIZATION_ID` | Variable | `<pending>` | |
| `SIGNPATH_PROJECT_SLUG` | Variable | `<pending>` | |
| `SIGNPATH_SIGNING_POLICY_SLUG` | Variable | `<pending>` | |
| `SIGNPATH_ARTIFACT_CONFIGURATION_SLUG` | Variable | `<pending>` | |

## 3. Test Certificate Validation
- Upload artefact used: `<pending>`
- SignPath signing request ID: `<pending>`
- Result: `<pending>`
- Log URL: `<pending>`

## 4. Production Certificate Rollout
- Date requested: `<pending>`
- Date imported: `<pending>`
- Verification run (`release.yml` dispatch ID): `<pending>`
- Release tag containing first signed artefacts: `<pending>`

## 5. Documentation Updates
- [ ] README (signing status & verification instructions)
- [ ] `docs/security/VERIFICATION_GUIDE.md` (Authenticode section)
- [ ] `docs/security/CI_SIGNING.md` (update tables with actual values)
- [ ] `docs/OPERATIONS.md` (release checklist)
- [ ] Release notes template / `.github` resources

## 6. Incident & Rotation Notes
- Test token rotation schedule: `<pending>`
- Production certificate rotation schedule: `<pending>`
- Emergency contacts / escalation path: `<pending>`

Keep this file in sync as onboarding progresses.
