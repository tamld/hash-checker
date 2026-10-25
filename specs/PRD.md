# Product Requirements Document (PRD) — Hash Checker

**Status**: APPLIED  
**Version**: 1.0.0  
**Last Updated**: 2026-10-25

---

## 1. Product Vision & Scope

Hash Checker is a lightweight, cross-platform cryptographic file and directory integrity verification tool written in Rust. It provides a shared core library (`hash-checker`) powering both an automation-ready command-line interface (`hash-checker`) and an accessible desktop GUI (`hash-checker-gui` built on `egui`/`eframe`).

### In-Scope Capabilities
- Single-file hash computation and verification across 8 cryptographic algorithms (`md5`, `sha1`, `sha224`, `sha256`, `sha384`, `sha512`, `blake2s`, `blake2b`).
- Automatic algorithm detection from `<algo>:<digest>` prefixes and digest hex length.
- Streaming, bounded-memory I/O with cooperative cancellation and progress callbacks.
- Directory manifest generation and verification (`json`, `csv`, `txt`/`mf`) with cross-platform path normalization and path-traversal protection.
- Batch verification reports (`json`, `csv`) with deterministic CI exit codes (`0`, `1`, `2`, `3`, `4`, `5`).
- Desktop GUI (`egui`) with drag-and-drop, clipboard export, live folder scan progress/cancellation, and accessible themes (`Slate`, `Soft Light`, `High Contrast`).

---

## 2. User Stories (`US-01` .. `US-06`)

| ID | Persona | User Story | Linked Functional Requirements | Linked Acceptance Criteria |
| :--- | :--- | :--- | :--- | :--- |
| **US-01** | Release Engineer / Developer | As a developer verifying downloaded artifacts, I want to verify a single file against an expected hex digest or `<algorithm>:<digest>` string so that I can confirm artifact integrity with zero manual algorithm lookup. | `REQ-FUNC-01`, `REQ-FUNC-02`, `REQ-FUNC-03` | `AC-01`, `AC-02`, `AC-03` |
| **US-02** | Security / Operations Engineer | As an operations engineer auditing a directory tree, I want to export and verify recursive manifests in JSON, CSV, or plain-text formats with normalized relative paths so that tampered, missing, or rogue extra files are deterministically detected across Windows, macOS, and Linux. | `REQ-FUNC-04`, `REQ-FUNC-05`, `REQ-FUNC-06` | `AC-04`, `AC-05`, `AC-06` |
| **US-03** | CI/CD Pipeline Author | As a CI pipeline author, I want to feed batch verification inputs via JSON or CSV (file or stdin) and receive structured reports and distinct exit codes (`0` match, `2` input/error, `3` mismatch/missing) so automated gates fail closed. | `REQ-FUNC-07` | `AC-07` |
| **US-04** | Desktop Operator | As a desktop user, I want to hash files and scan folders via drag-and-drop, auto-detect any of the 8 supported algorithm prefixes when pasting digests, switch between `Slate`, `Soft Light`, and `High Contrast` themes, and copy `<ALGO>:<DIGEST>` to the clipboard. | `REQ-FUNC-08`, `REQ-FUNC-09` | `AC-08`, `AC-09` |
| **US-05** | Performance-Conscious User | As a user hashing multi-gigabyte ISOs or deep directory trees, I want streaming chunked I/O with progress reporting and cooperative cancellation so that memory stays bounded ($\le 1\text{ MiB}$ buffer) and long scans can be aborted immediately. | `REQ-FUNC-09`, `REQ-FUNC-10` | `AC-09`, `AC-10` |
| **US-06** | Package Maintainer | As a release maintainer, I want reproducible cross-platform packaging scripts (including canonical `hash-checker.dmg` naming on macOS) and headless GUI smoke/snapshot modes so release artifacts pass automated verification. | `REQ-FUNC-10` | `AC-10` |
