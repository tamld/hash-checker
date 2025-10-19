# GUI Manifest Test Plan

> Scope: validate the Folder Scan / Directory Manifest experience across functional,
> layout, and telemetry dimensions. This plan formalises the assumptions discussed
> during the October 2025 optimisation sessions.

## Guiding Assumptions

| ID | Assumption | Status | Verification |
|----|------------|--------|--------------|
| A1 | Scan should mark entries as **recorded** (not matched) until an explicit verify runs. | ✅ (2025-10-19) | Unit tests `manifest_from_scan_records_entries`, CLI harness JSON. |
| A2 | Summary banner reflects counts + duration using real manifest data. | ✅ | Screenshot `logs/gui-manifest/snapshots/manifest-20251019-114418.png`. |
| A3 | Folder Scan controls follow three-row layout; secondary buttons size to content. | ✅ (2025-10-19) | Snapshot `docs/assets/gui-folder-scan.png` shows clamped dropdowns, accent Scan, and drag-and-drop status banner. |
| A4 | Table must handle deep directory structures and long hashes without truncation. | ✅ (2025-10-19) | Details dialog `docs/assets/gui-folder-details.png` provides scrollable view for long paths/hash values. |
| A5 | Telemetry logs each scan/verify with recorded + mismatch counts. | ⏳ | Script `validate_telemetry.py` (queued). |
| A6 | Theme palette options provide comfortable contrast (Soft Light inclusive). | ⚠️ (2025-10-19) | Soft Light theme causes excessive brightness; see feedback screenshot. |

## Scenario Matrix

| Scenario | Purpose | Inputs | Expected Outcome | Artefacts |
|----------|---------|--------|------------------|-----------|
| S1 – Minimal scan | Baseline behaviour | `test-fixtures/sample.txt` | JSON shows `recorded=1`, table pending state. | Snapshot + `manifest-<ts>.json`. |
| S2 – Deep tree | Stress table layout | Temp dir with nested folders and 8+ files (see §3.1). | Scroll area expands; columns readable; `recorded=files`. | Snapshot `*-deep.png`, telemetry line. |
| S3 – Verify mismatches | Functional regression | Reuse S2 then mutate/delete/add files. | Summary shows mismatch/missing/extra counts; banner warning. | JSON `verify-<ts>.json`, telemetry. |
| S4 – Format export | File IO/regression | Run `Save list…` to JSON/CSV/TXT. | Files created with correct extensions; toast success. | Files under temp dir; CLI log. |
| S5 – Accessibility sweep | Keyboard + shortcuts | Trigger scan/save/check via Enter/Cmd+S/Cmd+O. | Actions fire without mouse; focus ring visible. | Manual checklist, video cap optional. |

## Automation Harness

1. **Temp Directory Builder**  
   - Use `tempfile::tempdir()` to create:
     ```
     root/
       ├─ alpha/
       │   ├─ file-a.bin   (random 32 KB)
       │   └─ file-b.txt   (lorem)
       ├─ bravo.log        (long hash display)
       ├─ charlie/
       │   └─ delta/
       │       └─ nested.json
       └─ emoji-😀.dat     (unicode path)
     ```
   - Record manifest via GUI CLI flag:
     ```bash
     cargo run --release --manifest-path rust/hash-checker-gui/Cargo.toml \
       -- --manifest-dir /tmp/<run>/root \
          --manifest-report logs/gui-manifest/reports/deep-scan.json
     ```
   - Keep JSON + telemetry entry as evidence.

2. **Mismatch Injection**
   - Modify `file-a.bin`, delete `nested.json`, add `rogue.tmp`.  
   - Run verify (`--manifest-dir` optional, rely on recorded root).  
   - Expect mismatch/missing/extra counts > 0; store `deep-verify.json`.

3. **Snapshot Capture**
   - Re-run with `--snapshot logs/.../deep-<ts>` to capture both tabs.  
   - Compare PNG with golden via visual diff script (todo) or manual review.

4. **Cleanup**
   - `tempdir` auto-deletes; confirm no residue under repo.  
   - Remove temporary reports once validated or move into `logs/gui-manifest/golden/`.

## Evidence Checklist

- [ ] JSON report committed/archived for each scenario.  
- [ ] Telemetry entries logged with timestamps.  
- [ ] Snapshot PNG compared to golden (document differences).  
- [ ] Lessons updated if assumptions fail.  
- [ ] `.agents/gui_manifest_todo.yml` reflects remaining actions.

## Next Actions

1. Implement deep-tree harness (S2+S3) and store golden artefacts.
2. Adjust table layout to expand columns for long hashes/paths (addresses feedback “số 2”).
3. Tune dropdown/button widths per blueprint (feedback “số 1”).
4. Wire telemetry validator, referencing harness outputs.
5. Integrate scenarios into CI workflow (`gui-validation.yml` planned).

Once all items show ✅ in the matrix, mark the manifest tab as MVP-ready for release.

## Execution Best Practice (per October 2025 workflow)

1. **Local changes** – Implement UI/logic updates, run `cargo fmt` and `cargo test --manifest-path rust/hash-checker-gui/Cargo.toml` (includes deep harness + smoke).  
2. **Scenario capture** – Run snapshot harness twice:  
   - Minimal fixture (`test-fixtures`) → baseline PNG/JSON.  
   - Deep tree fixture (`test-fixtures/gui-deep`) → stress PNG/JSON.  
   Review both File/Folder tab images.  
3. **Telemetry & logs** – Check `logs/gui-manifest/telemetry.log` for new entries (Scan + Verify when applicable).  
4. **Documentation** – Update `.agents/lessons.yml` with structured trigger/impact/evidence when issues found; adjust `.agents/gui_manifest_todo.yml` statuses.  
5. **Packaging** – When snapshots/tests pass, build macOS DMG via  
   ```bash
   CARGO_TARGET_DIR=/tmp/hash-checker-target make macos-dmg-universal
   ```  
   Record DMG path, mount instructions, and share with stakeholders.  
6. **Cleanup** – Remove temporary build dirs (`/tmp/hash-checker-target`, snapshot scratch) if not needed, keeping only golden artefacts in `logs/gui-manifest/snapshots/`.

## Current Test Outcomes (2025-10-19)

- Snapshots captured for baseline (`manifest-20251019-132647-*.png`) and deep-tree (`deep-20251019-132705-*.png`) fixtures.  
- Result summary:
  - Details dialog now available (A3/A4 fix attempted); requires UX review of the popup.  
  - Dropdown widths trimmed to ~170 px on both tabs.  
  - Soft Light theme removed from selector pending redesign (A6 mitigation).  
- Follow-up tasks tracked in `.agents/gui_manifest_todo.yml` (`control_width_tuning`, `table_responsive_layout`, `theme_palette_review`, `deep_tree_harness` continuation).  
- After review, rerun S1/S2 scenarios and rebuild DMG to validate and promote snapshots to golden.
