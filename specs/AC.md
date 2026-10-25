# Acceptance Criteria & 12D Scenario Matrix (AC) — Hash Checker

**Status**: APPLIED  
**Version**: 1.0.0  
**Last Updated**: 2026-10-25

---

## 1. Traceability Matrix (`US-*` $\to$ `REQ-FUNC-*` $\to$ `AC-*` $\to$ Mechanical Tests)

| User Story | Functional Req | AC ID | Mechanical Test Binding (`cargo test`) |
| :--- | :--- | :--- | :--- |
| `US-01` | `REQ-FUNC-01` | `AC-01` | `compute_hash_sha256_matches_expected` |
| `US-01` | `REQ-FUNC-02` | `AC-02` | `verify_hash_infers_blake2s` |
| `US-01` | `REQ-FUNC-03` | `AC-03` | `verify_hash_rejects_conflicting_algorithm_and_prefix` |
| `US-02` | `REQ-FUNC-04` | `AC-04` | `deep_tree_manifest_detects_mismatch_missing_and_extra` |
| `US-02` | `REQ-FUNC-05` | `AC-05` | `manifest_normalizes_backslashes_and_rejects_traversal` |
| `US-02`, `US-05` | `REQ-FUNC-06` | `AC-06` | `compute_hash_with_options_reports_progress_and_cancels` |
| `US-03` | `REQ-FUNC-07` | `AC-07` | `run_batch_reports_match_and_mismatch` |
| `US-04` | `REQ-FUNC-08` | `AC-08` | `parses_all_eight_known_prefixes` |
| `US-04`, `US-05` | `REQ-FUNC-09` | `AC-09` | `theme_choice_includes_soft_light_slate_and_high_contrast` |
| `US-05`, `US-06` | `REQ-FUNC-10` | `AC-10` | `gui_smoke_test_runs` |

---

## 2. Acceptance Criteria (`Given / When / Then`)

#### Scenario: AC-01 Streaming Single-File Digest Calculation (`US-01`, `REQ-FUNC-01`)
<!-- tests: process-enforced -->
- **Given** a valid regular file on disk and any of the 8 supported algorithms (`md5`, `sha1`, `sha224`, `sha256`, `sha384`, `sha512`, `blake2s`, `blake2b`),
- **When** `compute_hash(path, algorithm)` is invoked,
- **Then** the file is streamed in 1 MiB heap chunks and the lowercase hexadecimal digest is returned without loading the entire file into memory.

#### Scenario: AC-02 Prefix and Length Algorithm Inference (`US-01`, `REQ-FUNC-02`)
<!-- tests: process-enforced -->
- **Given** an expected hash string with either an explicit `<algo>:` prefix or a standard hex length (`32`, `40`, `56`, `64`, `96`, `128`),
- **When** `verify_hash(path, expected_hash, None)` is executed,
- **Then** the engine automatically infers the candidate algorithm(s) (including secondary fallback from `sha256` to `blake2s` and `sha512` to `blake2b`) and returns `Ok((true, computed))` upon match.

#### Scenario: AC-03 Fail-Closed Conflict and Malformed Input Rejection (`US-01`, `REQ-FUNC-03`)
<!-- tests: process-enforced -->
- **Given** a verification request with conflicting `--algorithm` and `<prefix>:` values, an empty expected hash, non-hex characters, or a directory path instead of a regular file,
- **When** `verify_hash` is called,
- **Then** it transitions `ValidatingInput --> Failed` and returns the typed `HashError` (`ConflictingAlgorithms`, `EmptyExpectedHash`, `InvalidExpectedHash`, or `NotAFile`).

#### Scenario: AC-04 Recursive Directory Manifest Export and Verification (`US-02`, `REQ-FUNC-04`)
<!-- tests: process-enforced -->
- **Given** a nested directory tree exported to a `Manifest` (`Json`, `Csv`, or `Plain`),
- **When** files are mutated, deleted, or added and `verify_manifest(&manifest, root)` is executed,
- **Then** the `VerificationReport` accurately partitions entries into `matched`, `mismatched`, `missing`, and `extra`.

#### Scenario: AC-05 Cross-Platform Backslash Normalization and Path Traversal Guard (`US-02`, `REQ-FUNC-05`)
<!-- tests: process-enforced -->
- **Given** a manifest containing Windows-style backslash paths (`alpha\file-a.bin`) or an untrusted path traversal entry (`../secret.txt`),
- **When** `read_manifest` or `verify_manifest` processes the manifest entries,
- **Then** valid backslash paths are normalized to forward slashes (`alpha/file-a.bin`) and any entry containing `..` parent traversal or absolute path prefixes is rejected with `HashError::ManifestParse`.

#### Scenario: AC-06 Cooperative Cancellation and Streaming Progress Reporting (`US-02`, `US-05`, `REQ-FUNC-06`)
<!-- tests: process-enforced -->
- **Given** an active `compute_hash_with_options` or `generate_manifest_with_options` call with an `AtomicBool` cancellation token and a progress callback,
- **When** chunks or files are processed or the cancellation flag is set to `true`,
- **Then** the progress callback receives monotonically increasing `(bytes_processed, total_bytes)` / `(files_processed, total_files)` updates and setting the cancel token immediately transitions `StreamingDigest --> Cancelled` (`Err(HashError::Cancelled)`).

#### Scenario: AC-07 Batch Verification Summary and CI Exit Code Mapping (`US-03`, `REQ-FUNC-07`)
<!-- tests: process-enforced -->
- **Given** a batch list of `BatchInput` items containing matched, mismatched, missing, or errored entries,
- **When** `run_batch` evaluates the list,
- **Then** `BatchReport::exit_code()` returns `0` when all entries match, `3` when mismatches or missing files occur without errors, and `2` when any entry errors.

#### Scenario: AC-08 GUI 8-Algorithm Prefix Auto-Detection and Resolved Algorithm Attribution (`US-04`, `REQ-FUNC-08`)
<!-- tests: process-enforced -->
- **Given** a user pasting a prefixed hash (`md5:`, `sha1:`, `sha224:`, `sha256:`, `sha384:`, `sha512:`, `blake2s:`, `blake2b:`) or unprefixed digest in `auto` mode in `hash-checker-gui`,
- **When** `parse_prefixed_hash` or `calculate()` executes,
- **Then** all 8 prefixes are recognized without false warnings and `last_algorithm_used` records the actual inferred algorithm name rather than defaulting to `SHA256`.

#### Scenario: AC-09 GUI Theme Selector Completeness and Scan Cancellation State (`US-04`, `US-05`, `REQ-FUNC-09`)
<!-- tests: process-enforced -->
- **Given** the `hash-checker-gui` application state (`FSM-02`),
- **When** the user inspects available themes or cancels an in-flight folder scan (`LoadingScan --> Empty`),
- **Then** all 3 themes (`Slate`, `Soft Light`, `High Contrast`) are selectable and cancelling an active scan transitions `ManifestState` back to `Empty` with an informational banner.

#### Scenario: AC-10 Headless GUI Smoke Verification and Canonical macOS DMG Naming (`US-05`, `US-06`, `REQ-FUNC-10`)
<!-- tests: process-enforced -->
- **Given** the `hash-checker-gui` binary invoked with `--smoke-test` and the macOS universal packaging script,
- **When** the smoke test executes,
- **Then** `hash-checker-gui --smoke-test` exits `0` after round-trip SHA-256 verification and `scripts/macos-universal-dmg.sh` defaults `DMG_NAME` to `hash-checker.dmg`.

---

## 3. FSM Transition Summary (`FSM-01` & `FSM-02`)

```text
[STATE: Idle] --> [STATE: ValidatingInput] --> [STATE: StreamingDigest] --> [STATE: ComparingDigest] --> [STATE: Matched | Mismatched | Cancelled | Failed]
[STATE: Empty] --> [STATE: Loading] --> [STATE: Ready | Error | Empty(Cancelled)]
```

---

## 4. 12-Dimension Adversarial QA Scenario Matrix

| # | Dimension | Threat / Edge Case | Mitigation & Verification |
| :--- | :--- | :--- | :--- |
| 1 | `boundary` | Zero-byte empty file (`0` bytes) and exact `CHUNK_SIZE` (`1048576` bytes) boundary reads. | Verified in `compute_hash_with_options_reports_progress_and_cancels` (`min`/`max`/`zero` byte files and chunk boundaries). |
| 2 | `null_empty` | Empty expected hash (`""`), whitespace-only expected hash (`"   "`), or missing/blank algorithm flag. | Returns `HashError::EmptyExpectedHash`; blank algorithm falls back to prefix/length inference. |
| 3 | `concurrency_race` | Concurrent background folder scan thread vs UI thread polling `TryRecvError` and `AtomicBool` cancel / `AtomicUsize` progress. | Lock-free `Arc<AtomicBool>` and `Arc<AtomicUsize>` with `mpsc::channel` single-consumer ownership. |
| 4 | `network_timeout` | Offline/disconnected execution and bounded timeout during headless CI/GUI smoke runs. | Zero network calls in core/GUI; offline deterministic execution within deadline. |
| 5 | `permission_authz` | Unauthorized read on unreadable file or directory path; S3-M `WriteReceipt` scope isolation. | Fails closed with `HashError::Io` (`PermissionDenied`) mapped to `BatchStatus::Error`. |
| 6 | `idempotency_retry` | Re-running `generate_manifest` or `verify_manifest` on identical directory trees or retrying after cancellation. | Deterministic BTreeMap/lexicographical path sorting ensures idempotent manifest entries across re-runs. |
| 7 | `resource_exhaustion` | Hashing multi-gigabyte files under tight memory quota or disk limits. | Fixed 1 MiB heap buffer (`CHUNK_SIZE`) prevents memory exhaustion and stack overflow. |
| 8 | `encoding_i18n` | Unicode / UTF-8 filenames (`emoji-😀.dat`) and Windows `\` vs POSIX `/` path separators in manifests. | `to_manifest_path` and `normalize_manifest_rel_path` preserve UTF-8 while normalizing `\` to `/`. |
| 9 | `state_transition` | Invalid FSM transitions (e.g., triggering export when `ManifestState::Empty` or `Loading`, or cancelling a running scan). | Guarded in `trigger_export`, `trigger_scan`, and `cancel_active_job` (`queued`/`running`/`completed`/`failed` state hygiene). |
| 10 | `clock_skew` | Files with pre-epoch timestamps or missing `modified` metadata (`UNIX_EPOCH` clock skew). | `duration_since(UNIX_EPOCH).ok()` safely degrades to `modified: None` without panicking. |
| 11 | `rollback_recovery` | Interrupted manifest scan or temp file cleanup on smoke test failure. | `cancel_active_job` restores clean `ManifestState::Empty`; temp files use RAII `TempDir` / `remove_file`. |
| 12 | `security_injection` | Malicious manifest containing `../` path traversal injection or symlink escape. | `follow_links(false)` + `normalize_manifest_rel_path` rejects `..` and leading `/` or drive prefixes. |
