# Threat Model – Hash Verification Pipeline

Updated: 2025-10-13

## Scope
- Rust CLI (`hash-checker`) and GUI (`hash-checker-gui`) binaries.
- Hash computation and verification paths invoked by CI (`make ci-linux-local`) and release workflows.
- Packaging artefacts distributed through GitHub Releases.

## Assets
- Integrity of release binaries and installer artefacts.
- Expected digests recorded in `SHA256SUMS` files or release notes.
- User-provided file paths and expected hashes supplied through CLI arguments or GUI input.

## Adversaries & Attack Surface
- Local attackers attempting path traversal or symlink tricks to hash an unintended target.
- Tampered release artefacts or compromised distribution channel.
- Dependency or toolchain compromise introducing backdoors during builds.
- Resource exhaustion (very large files, slow/blocking IO) leading to denial of service.

## Mitigations Implemented
- Canonicalise user-supplied paths and reject non-regular files before hashing.
- Stream file reads in 1 MiB chunks to avoid loading full files into memory.
- Require `cargo fmt`, `cargo clippy`, and tests (`make ci-linux-local`) before commits to surface regressions early.
- Integrate `cargo audit`/`cargo deny` in CI to detect vulnerable dependencies.
- Document checksum verification steps for end users (see `docs/security/VERIFICATION_GUIDE.md`).

## Residual Risks & Follow-Ups
- Time-of-check-to-time-of-use (TOCTOU) between canonicalisation and file read remains a theoretical risk; mitigated by local execution context but should be documented in release notes when high assurance is required.
- Windows/macOS signing automation (SignPath, notarisation) remains pending; follow `docs/SIGNING.md`.
- Monthly dependency refresh cadence must be enforced to avoid stale toolchains.
- Consider sandboxing hash operations for high-assurance environments (tracked in Phase 5).

## Action Items
- Keep this document updated whenever threat assumptions or mitigations change.
- Capture signed artefact validation steps in `docs/OPERATIONS.md` once signing is automated.
- Reference this document in release planning and postmortems involving security findings.
