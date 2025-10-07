# Security Roadmap

This document tracks security investments layered on top of the Rust MVP.

## Stage 1 – Baseline (with MVP)
- Defensive coding guidelines for file IO, error handling, and digest comparison.
- Ensure all logging avoids leaking sensitive file contents or hashes unless explicitly requested.
- Add unit tests for edge cases (symlinks, missing files, permission errors).

## Stage 2 – Tooling & Automation
- Integrate `cargo audit` and `cargo deny` into CI; fail builds on known vulnerabilities.
- Generate SBOM artifacts (`cargo sbom`) for each release.
- Add static analysis (Clippy `-D warnings`) and formatting checks to enforce consistency.

## Stage 3 – Release Integrity
- Implement binary signing (macOS notarization, Windows Authenticode, Linux GPG signatures).
- Publish checksums and signatures alongside release artifacts.
- Document verification steps for end users.

## Stage 4 – Threat Modeling & Hardening
- Conduct threat modeling workshop covering tampered files, path traversal, untrusted archives.
- Harden file handling: canonicalize paths, handle network/mounted volumes securely.
- Review dependency licenses and update cadence; establish patch SLAs.

## Stage 5 – Advanced Protections
- Explore sandboxing for hash operations in high-assurance environments.
- Implement telemetry opt-in with privacy redaction for anomaly detection.
- Schedule periodic penetration tests and document findings/action items.

Each stage should result in checklist updates and retrospective notes kept in `docs/security/` (to be created when work begins).
