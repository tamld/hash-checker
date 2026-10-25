# Software Requirements Specification (SRS) — Hash Checker

**Status**: APPLIED  
**Version**: 1.0.0  
**Last Updated**: 2026-10-25

---

## 1. Functional Requirements (`REQ-FUNC-01` .. `REQ-FUNC-10`)

| ID | Title | Specification | Traced User Story | Traced AC |
| :--- | :--- | :--- | :--- | :--- |
| **REQ-FUNC-01** | Multi-Algorithm Streaming Digest | The core engine SHALL compute digests using a 1 MiB heap-allocated streaming buffer (`CHUNK_SIZE = 1048576`) across `md5`, `sha1`, `sha224`, `sha256`, `sha384`, `sha512`, `blake2s`, and `blake2b`. | `US-01` | `AC-01` |
| **REQ-FUNC-02** | Prefix & Length Algorithm Inference | `detect_algorithm` and `verify_hash` SHALL infer the algorithm from an explicit `<algo>:<hex>` prefix (for all 8 algorithms, case-insensitive) or fall back to hex length (`32`->`md5`, `40`->`sha1`, `56`->`sha224`, `64`->`sha256`/`blake2s`, `96`->`sha384`, `128`->`sha512`/`blake2b`). | `US-01` | `AC-02` |
| **REQ-FUNC-03** | Conflict & Input Validation | `verify_hash` SHALL fail closed with `HashError::ConflictingAlgorithms` when both an explicit `--algorithm` flag and a `<prefix>:` are supplied and disagree, `HashError::EmptyExpectedHash` on empty digests, and `HashError::InvalidExpectedHash` on non-hex characters. | `US-01` | `AC-03` |
| **REQ-FUNC-04** | Directory Manifest Export & Verification | `generate_manifest` and `verify_manifest` SHALL support recursive and non-recursive directory traversal (`follow_links(false)`), reporting `matched`, `mismatched`, `missing`, and `extra` files across JSON, CSV, and Plain (`txt`/`mf`) formats. | `US-02` | `AC-04` |
| **REQ-FUNC-05** | Cross-Platform Manifest Path Normalization & Traversal Guard | Manifest relative paths SHALL normalize Windows backslashes (`\`) to POSIX forward slashes (`/`) on load and verification, and SHALL reject relative paths containing parent-directory traversal (`..`) or absolute roots with `HashError::ManifestParse`. | `US-02` | `AC-05` |
| **REQ-FUNC-06** | Cooperative Cancellation & Progress Callbacks | The core engine SHALL expose `compute_hash_with_options` and `generate_manifest_with_options` accepting an optional cancellation token (`&AtomicBool`) and progress callback (`FnMut(u64, u64)`), returning `HashError::Cancelled` immediately when signalled. | `US-02`, `US-05` | `AC-06` |
| **REQ-FUNC-07** | Batch Verification & Deterministic Exit Codes | `run_batch` and the CLI `batch` subcommand SHALL process JSON and CSV batch inputs from file or stdin (`-`), classifying each item as `match`, `mismatch`, `missing`, or `error`, and returning exit code `0` (all match), `3` (mismatch/missing without error), or `2` (errored entries). | `US-03` | `AC-07` |
| **REQ-FUNC-08** | GUI Prefix Auto-Detection & Algorithm Parity | The desktop GUI (`hash-checker-gui`) SHALL recognize all 8 supported algorithm prefixes (`md5`, `sha1`, `sha224`, `sha256`, `sha384`, `sha512`, `blake2s`, `blake2b`) when pasted into the expected hash input, and when `auto` is selected, SHALL record the actual inferred/verified algorithm for clipboard copy (`<ALGO>:<DIGEST>`). | `US-04` | `AC-08` |
| **REQ-FUNC-09** | GUI Theme Accessibility & Live Scan Cancellation | The desktop GUI SHALL provide 3 accessible themes (`Slate`, `Soft Light`, `High Contrast`) in the theme selector and display live processed-file counters and a `Cancel` action while a background folder scan or manifest verification is in `Loading` state. | `US-04`, `US-05` | `AC-09` |
| **REQ-FUNC-10** | Canonical Packaging & Headless Smoke Contracts | `hash-checker-gui --smoke-test` SHALL verify end-to-end hashing headlessly, and `scripts/macos-universal-dmg.sh` SHALL emit a deterministic `hash-checker.dmg` artifact name (with optional `DMG_NAME_OVERRIDE`). | `US-05`, `US-06` | `AC-10` |

---

## 2. Non-Functional Requirements (`NFR-01` .. `NFR-04`)

- **NFR-01 (Bounded Heap Memory)**: Streaming file hashing MUST allocate its 1 MiB chunk buffer on the heap (`vec![0u8; CHUNK_SIZE]`) to prevent Windows thread stack overflows and maintain $O(1)$ memory overhead regardless of file size.
- **NFR-02 (Cancellation Responsiveness)**: When an `AtomicBool` cancellation flag is set, streaming file hashing MUST abort within at most one `CHUNK_SIZE` (1 MiB) read iteration and return `HashError::Cancelled`.
- **NFR-03 (Path Safety & No Symlink Loops)**: Directory walks MUST set `follow_links(false)` and reject `..` traversal segments inside untrusted manifest files.
- **NFR-04 (Deterministic CLI Exit Codes)**:
  - `0`: Verification succeeded (all hashes matched).
  - `1`: Runtime, I/O, or unsupported algorithm/manifest error.
  - `2`: CLI usage error or batch input error.
  - `3`: Hash mismatch or missing file detected.
  - `4`: Extra unrecorded files detected on disk during manifest verification.
  - `5`: Unsupported mode flag (`--gui` / `--no-cli` on CLI binary).
