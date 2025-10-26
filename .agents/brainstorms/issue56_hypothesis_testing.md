# Issue #56 Hypothesis-Driven Investigation

**Date**: 2025-10-26  
**Investigator**: Cursor (Claude 4.5 Sonnet)  
**Method**: Hypothesis → Evidence → Conclusion

---

## 🎯 **Issue #56 Understanding**

**Title**: `[Automation] Implement GUI Golden Master Testing Framework`  
**Description**: "Implement comprehensive GUI automation framework to enable regression testing, performance tracking, and CI/CD integration without relying on manual screenshot comparison."

---

## 📋 **Context from Codex's Work**

### What Codex Completed (Stabilization Phase)
1. ✅ Fixed invalid GUI tests (removed assumptions about non-existent CLI flags)
2. ✅ Restored CI workflow reliability (fixed matrix issues)
3. ✅ Added safety guardrails (prevent data loss in scripts)
4. ✅ Hardened Python tooling (argparse, dependencies)
5. ✅ Restored Cargo.lock integrity

### What Codex Identified as Missing
1. ❌ True headless CLI contract undefined
2. ❌ No regression guards for telemetry scripts
3. ❌ CI matrix unproven on GitHub-hosted runners

### Codex's Brainstorm Goals
- Publish real headless CLI spec
- Add integration tests for telemetry scripts
- Run GUI workflow in CI for baseline timings

---

## 🔬 **HYPOTHESIS TESTING**

### **H1: "Golden Master" means storing baseline GUI state for regression detection**

**Hypothesis**: Issue #56 wants us to capture and store "golden master" snapshots/state of GUI that can be compared in CI to detect regressions.

**Evidence Collection**:
```bash
# Check for existing golden masters
find . -name "*golden*" -type f
# Result: 0 files found ❌

# Check GUI source for snapshot capability
rg "snapshot|golden" rust/hash-checker-gui/src/main.rs
# Result: Found SNAPSHOT_DEFAULT_WIDTH/HEIGHT constants ✅

# Check test-fixtures
ls test-fixtures/
# Result: gui-deep/ directory exists with nested files ✅
```

**Findings**:
- ✅ GUI has snapshot dimensions defined (1280x800, 1440x900)
- ✅ Test fixtures exist (test-fixtures/gui-deep/)
- ❌ NO golden master files stored anywhere
- ❌ NO comparison logic exists

**Conclusion**: ✅ **HYPOTHESIS CONFIRMED**  
Golden master testing is NOT implemented yet. Only infrastructure exists (snapshot dimensions, fixtures).

---

### **H2: GUI has headless/automated capture capability**

**Hypothesis**: GUI can already run headless and capture snapshots automatically.

**Evidence Collection**:
```rust
// From main.rs analysis:
const TELEMETRY_DIR: &str = "logs/gui-manifest";
const TELEMETRY_FILE: &str = "telemetry.log";
const SNAPSHOT_DEFAULT_WIDTH: u32 = 1280;
const SNAPSHOT_DEFAULT_HEIGHT: u32 = 800;
```

**Test**:
```bash
# Check CLI flags
cargo run --manifest-path rust/hash-checker-gui/Cargo.toml -- --help
# Lists: --manifest-dir, --manifest-report, --manifest-algorithm
# Missing: --snapshot, --headless, --capture
```

**Current CLI Flags** (from gui_cli_smoke.rs):
- `--manifest-dir`: Pre-load manifest directory
- `--manifest-report`: Set report output path
- `--manifest-algorithm`: Set hashing algorithm
- `--help`: Show help
- ❌ `--version`: Not implemented
- ❌ `--snapshot`: Not implemented
- ❌ `--headless`: Not implemented
- ❌ `--capture-golden`: Not implemented

**Conclusion**: ❌ **HYPOTHESIS REJECTED**  
GUI has constants for snapshots but NO CLI flags to trigger automated capture. Codex is correct: "Headless-mode CLI contract undefined."

---

### **H3: Codex stopped before implementing golden master logic**

**Hypothesis**: Codex only stabilized existing tests, did NOT implement golden master testing framework.

**Evidence**:
1. **Codex's own words** (from `gui_automation_harness_issue56.md`):
   - "GUI CLI Smoke Tests **Refreshed**" (fixed, not created new)
   - "Workflow Reliability **Restored**" (fixed existing)
   - Follow-up: "Re-introduce Headless Testing **Properly**" (future work)

2. **File Analysis**:
   - `gui_cli_smoke.rs`: Only tests --help, invalid flags, --version absence
   - No new snapshot/golden master tests added
   - No comparison logic added

3. **Brainstorm** (`gui_automation_validation.yml`):
   - Status: "open"
   - Problem: "lacks true headless coverage"
   - Deliverables: "Headless CLI RFC" (not done), "Telemetry script unit tests" (not done)

**Conclusion**: ✅ **HYPOTHESIS CONFIRMED**  
Codex did stabilization ONLY. Issue #56 golden master framework is NOT implemented.

---

### **H4: Issue #56 scope = Codex stabilization + Golden Master implementation**

**Hypothesis**: Issue #56 encompasses BOTH what Codex did (stabilization) AND what remains (golden master framework).

**Evidence**:
- **Issue title**: "Implement GUI Golden Master Testing Framework"
- **Issue description**: "comprehensive GUI automation framework"
- **Codex deliverables**: Stabilization ≠ Golden master implementation

**Scope Breakdown**:

| Component | Codex Status | Needed for Issue #56 |
|-----------|--------------|----------------------|
| **Fix broken tests** | ✅ Done | ✅ Prerequisite |
| **CI workflow reliability** | ✅ Done | ✅ Prerequisite |
| **Safety guardrails** | ✅ Done | ✅ Prerequisite |
| **Headless CLI spec** | ❌ Not done | ✅ **Core requirement** |
| **Golden master storage** | ❌ Not done | ✅ **Core requirement** |
| **Snapshot capture automation** | ❌ Not done | ✅ **Core requirement** |
| **Comparison logic** | ❌ Not done | ✅ **Core requirement** |
| **Regression detection** | ❌ Not done | ✅ **Core requirement** |
| **CI integration** | ❌ Not done | ✅ **Core requirement** |

**Conclusion**: ✅ **HYPOTHESIS CONFIRMED**  
Codex completed ~30% (prerequisites). Remaining ~70% is the CORE of Issue #56.

---

### **H5: GUI_MANIFEST_TEST_PLAN.md already defines golden master approach**

**Hypothesis**: The test plan document already outlines how golden masters should work.

**Evidence** (from `GUI_MANIFEST_TEST_PLAN.md`):
```markdown
## Automation Harness

3. **Snapshot Capture**
   - Re-run with `--snapshot logs/.../deep-<ts>` to capture both tabs.
   - Compare PNG with golden via visual diff script (todo) or manual review.

4. **Cleanup**
   - Remove temporary reports once validated or move into `logs/gui-manifest/golden/`.
```

**Key Findings**:
- ✅ Mentions `--snapshot` flag (doesn't exist yet!)
- ✅ Mentions "Compare PNG with golden" (comparison logic missing!)
- ✅ Mentions `logs/gui-manifest/golden/` directory (doesn't exist!)
- ⚠️ Says "todo" and "or manual review" (not automated!)

**Conclusion**: ⚠️ **HYPOTHESIS PARTIALLY CONFIRMED**  
Test plan DESCRIBES desired state but implementation is missing. It's a SPEC, not reality.

---

## 📊 **GAP ANALYSIS: What's Missing**

### **Category A: CLI Contract (Highest Priority)**
- [ ] `--headless` flag: Run GUI without display server
- [ ] `--snapshot <path>` flag: Capture GUI state to file
- [ ] `--capture-golden <scenario>` flag: Save as golden master
- [ ] `--compare-golden <scenario>` flag: Compare current vs golden
- [ ] `--exit-after-capture` flag: Exit after snapshot (for CI)

### **Category B: Golden Master Storage**
- [ ] `test-fixtures/golden/` directory structure
- [ ] Naming convention: `golden-<scenario>-<platform>.json`
- [ ] Platform-specific goldens (Linux, macOS, Windows)
- [ ] Metadata format (timestamp, version, platform)

### **Category C: Comparison Logic**
- [ ] Exact match comparison
- [ ] Fuzzy match (tolerate minor pixel differences)
- [ ] Structural comparison (ignore timestamps, dynamic values)
- [ ] Diff output format (human-readable)

### **Category D: CI Integration**
- [ ] GitHub Actions job: "golden-master-validation"
- [ ] Matrix: [Linux, macOS, Windows]
- [ ] Baseline collection phase
- [ ] Regression detection phase
- [ ] Artifact upload (diffs on failure)

### **Category E: Telemetry Validation**
- [ ] Unit tests for `analyze_telemetry.py`
- [ ] Unit tests for `check_performance_regression.py`
- [ ] Integration test: Full capture → analyze → report flow

---

## 💡 **RECOMMENDATION: Phased Approach**

### **Phase 1: Headless CLI Contract** (2-3 hours)
**Goal**: Define and implement headless flags

1. Design CLI spec (RFC style)
2. Implement `--headless` mode
3. Implement `--snapshot <path>` flag
4. Add exit-after-capture logic
5. Unit tests for new flags

**Acceptance**:
- `cargo run -- --headless --snapshot output.json` works
- Captures GUI state to JSON
- Exits cleanly after capture

---

### **Phase 2: Golden Master Storage** (1-2 hours)
**Goal**: Create storage structure

1. Create `test-fixtures/golden/` directory
2. Define golden master JSON schema
3. Implement save/load logic
4. Platform detection (Linux/macOS/Windows)
5. Capture initial golden masters for scenarios

**Acceptance**:
- `test-fixtures/golden/scenario1-linux.json` exists
- Contains: window dimensions, widget states, telemetry
- Can be loaded and parsed

---

### **Phase 3: Comparison Logic** (3-4 hours)
**Goal**: Implement diff detection

1. Exact match comparator
2. Fuzzy match (configurable tolerance)
3. Structural comparator (ignore dynamic fields)
4. Diff formatter (JSON, human-readable)
5. CLI flag: `--compare-golden <scenario>`
6. Unit tests for all comparators

**Acceptance**:
- `cargo run -- --headless --compare-golden scenario1` works
- Detects differences
- Outputs clear diff report
- Exit code: 0 = match, 1 = diff, 2 = error

---

### **Phase 4: CI Integration** (2-3 hours)
**Goal**: Automate in GitHub Actions

1. Create `golden-master-test.yml` workflow
2. Matrix: [Linux, macOS, Windows]
3. Run golden master capture
4. Run comparison
5. Upload artifacts on failure
6. Document in OPERATIONS.md

**Acceptance**:
- CI runs on every PR touching GUI code
- Detects regressions automatically
- Clear failure messages
- Artifacts downloadable

---

### **Phase 5: Telemetry Validation** (2-3 hours)
**Goal**: Test Python scripts

1. `pytest` setup for scripts/
2. Unit tests for `analyze_telemetry.py`
3. Unit tests for `check_performance_regression.py`
4. Mock telemetry data for testing
5. Integration test: end-to-end flow

**Acceptance**:
- `pytest scripts/` passes
- Coverage > 80%
- CI runs Python tests

---

## 🎯 **TOTAL EFFORT ESTIMATE**

| Phase | Effort | Priority |
|-------|--------|----------|
| Phase 1: Headless CLI | 2-3h | **Critical** |
| Phase 2: Golden Storage | 1-2h | **Critical** |
| Phase 3: Comparison Logic | 3-4h | **Critical** |
| Phase 4: CI Integration | 2-3h | High |
| Phase 5: Telemetry Tests | 2-3h | Medium |
| **TOTAL** | **10-15h** | **Over 2-3 sessions** |

---

## ✅ **VALIDATION COMPLETE**

**Summary of Hypotheses**:
- H1: Golden master = baseline state ✅ CONFIRMED
- H2: GUI has headless capability ❌ REJECTED (needs implementation)
- H3: Codex did stabilization only ✅ CONFIRMED
- H4: Issue #56 = stabilization + framework ✅ CONFIRMED
- H5: Test plan is spec, not implementation ⚠️ PARTIALLY CONFIRMED

**Key Insight**: 
Codex completed PREREQUISITE work (30%). Issue #56 CORE work (70%) remains:
- Headless CLI
- Golden master storage
- Comparison logic
- CI integration
- Telemetry validation

**Ready to proceed** with claim and implementation plan.

---

**Prepared by**: Cursor (Claude 4.5 Sonnet)  
**Methodology**: Hypothesis-driven investigation  
**Quality**: All claims backed by evidence ✅
