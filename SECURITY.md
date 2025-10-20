# Security Policy

We take the integrity of Hash Checker seriously. This document explains how to
report vulnerabilities and what to expect once a report is submitted.

## Supported Versions

- The `main` branch and the most recent tagged releases receive security fixes.
- Older releases may receive fixes case by case if the workaround is simple; in
  most situations we recommend upgrading to the latest release.

## Reporting a Vulnerability

1. Submit a private report through the **GitHub Security Advisory** workflow
   (`Security` tab → `Advisories` → `Report a vulnerability`). This keeps the
   details private until a fix is available.
2. If the advisory workflow is unavailable, send a private message to the
   maintainers through GitHub (mention @tamld) with the affected versions and a
   reproduction outline. Avoid filing public issues for security flaws.
3. Provide as much detail as possible:
   - Impact and severity (confidentiality/integrity/availability).
   - Steps to reproduce or proof-of-concept code.
   - Suggested mitigations, if known.

We aim to acknowledge new reports within **3 business days**. If you have not
heard back, please follow up on the advisory thread.

## Coordinated Disclosure Process

1. Maintainers validate the report and reproduce the issue.
2. A fix is developed and verified (`make ci-linux-local`, packaging smoke tests
   when relevant).
3. A new release is published with mitigation details.
4. The advisory is updated with credits for the reporter (unless you request
   anonymity) and full remediation guidance.

## Verification Guidance

Once a release is available, follow `docs/security/VERIFICATION_GUIDE.md` to
validate downloaded artefacts (checksums, GPG signatures when enabled, and
platform-specific notes).

Thank you for helping keep Hash Checker secure!
