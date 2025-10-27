# Operational Readiness Checklist - Issue #56
# GUI Golden Master Testing Framework

**Date**: 2025-10-26  
**Issue**: #56  
**Beyond**: Security + Testing

---

## 🎯 **Core Pillars for Production Readiness**

Beyond security and testing, we need **8 additional pillars**:

1. **Performance & Scalability**
2. **Observability & Debugging**
3. **Documentation & Onboarding**
4. **Backward Compatibility & Migration**
5. **Deployment & Rollback**
6. **User Experience & Accessibility**
7. **Maintenance Burden & Technical Debt**
8. **Cross-Platform Consistency**

---

## 1️⃣ **PERFORMANCE & SCALABILITY**

### **Why This Matters for Issue #56**:
```
Golden master testing will run in CI on EVERY PR touching GUI code.
If slow → CI becomes bottleneck → Developer frustration → People disable it
```

### **Concerns**:

#### **A. Snapshot Capture Speed**
```rust
// Potential bottleneck: Serializing large GUI state
let snapshot = capture_gui_state(); // How long does this take?
```

**Questions**:
- How large is a typical snapshot JSON? (1KB? 100KB? 1MB?)
- Does serialization block the GUI thread?
- Can we capture in background thread?

**Acceptance Criteria**:
```yaml
Performance Targets:
  snapshot_capture: <500ms (per scenario)
  json_serialization: <100ms
  file_write: <50ms
  total_per_scenario: <1 second

CI Impact:
  current_ci_time: ~16 minutes (from baseline)
  acceptable_increase: +2 minutes (for all golden master validations)
  unacceptable: +5 minutes or more

Measurement:
  tool: criterion benchmarks
  baseline: capture before Phase 1
  comparison: after each phase
```

**Action Items**:
- [ ] Add criterion benchmarks for snapshot capture
- [ ] Profile hot paths with `cargo flamegraph`
- [ ] Document performance in `.agents/performance/golden_master_benchmarks.md`
- [ ] Set CI timeout budget (fail if exceeds 2 min overhead)

---

#### **B. Comparison Speed**
```rust
// Comparing two large JSON trees
let diff = compare_snapshots(golden, current); // O(n) or O(n²)?
```

**Questions**:
- What's the time complexity of comparison?
- Can we early-exit on first difference? (fail fast)
- Should we parallelize multiple scenario comparisons?

**Acceptance Criteria**:
```yaml
Comparison Targets:
  exact_match: <50ms
  diff_generation: <200ms
  human_readable_output: <100ms

Scalability:
  scenarios_to_test: 5-10 initially
  worst_case_total: <2 seconds (all scenarios)
```

**Action Items**:
- [ ] Benchmark comparison algorithms
- [ ] Consider serde_json::from_str vs streaming parser
- [ ] Document algorithmic complexity in code comments
- [ ] Add timeout per scenario (fail if >5s = likely infinite loop)

---

#### **C. CI Concurrency**
```yaml
# .github/workflows/golden-master-validation.yml
# If we run 3 platforms * 5 scenarios = 15 jobs
# GitHub free tier: 20 concurrent jobs max
```

**Questions**:
- Do we run all platforms in parallel?
- Do we run all scenarios in parallel per platform?
- What's the GitHub Actions cost? (minutes used)

**Acceptance Criteria**:
```yaml
Concurrency Strategy:
  platforms: sequential OR parallel (decide based on cost)
  scenarios_per_platform: sequential (simpler) OR parallel (faster)
  
Cost Budget:
  current_monthly_ci_minutes: [check with team]
  acceptable_increase: 20%
  monitoring: Track in .agents/metrics/ci_cost_tracking.yml
```

**Action Items**:
- [ ] Measure CI minute cost BEFORE and AFTER
- [ ] Document in PR: "This adds X minutes per PR to CI"
- [ ] Get team approval if cost increase >20%
- [ ] Consider matrix strategy vs sequential

---

### **Red Flags** (Performance):
🚩 Snapshot capture takes >2 seconds  
🚩 Comparison takes >5 seconds  
🚩 CI overhead >5 minutes  
🚩 No benchmarks to track regression  
🚩 No timeout guards (infinite loops possible)

---

## 2️⃣ **OBSERVABILITY & DEBUGGING**

### **Why This Matters**:
```
When golden master test FAILS in CI:
Developer needs to understand WHY within 2 minutes
Otherwise: "This test is flaky, disable it" ← Failure!
```

### **Concerns**:

#### **A. Clear Failure Messages**
```rust
// BAD:
assert_eq!(golden, current); // Panic: "assertion failed"

// GOOD:
if golden != current {
    eprintln!("Golden master mismatch detected!");
    eprintln!("Scenario: {}", scenario_name);
    eprintln!("Differences:");
    for diff in compute_diff(&golden, &current) {
        eprintln!("  - {}: {} → {}", diff.field, diff.expected, diff.actual);
    }
    eprintln!("To inspect: download artifact 'golden-master-diff.json'");
    std::process::exit(1);
}
```

**Acceptance Criteria**:
```yaml
Failure Message Requirements:
  - Scenario name (which test failed)
  - Specific fields that differ (not just "mismatch")
  - Expected vs Actual values
  - Link to downloadable artifact
  - Instructions to reproduce locally
  
Example Good Message:
  "Golden master mismatch: scenario 'deep-tree-scan'
   Field 'window.width': expected 1280, got 1024
   Field 'theme': expected 'dark', got 'light'
   
   To reproduce:
   cargo run -- --compare-golden deep-tree-scan
   
   To update golden master (if intentional):
   cargo run -- --capture-golden deep-tree-scan
   git add test-fixtures/golden/deep-tree-scan-linux.json
   
   Diff artifact: [CI download link]"
```

**Action Items**:
- [ ] Implement rich diff formatter
- [ ] Test failure message readability (ask teammate to debug)
- [ ] Add screenshot to docs showing example failure
- [ ] Include "how to fix" instructions in every error

---

#### **B. Logging & Telemetry**
```rust
// What information do we log?
info!("Capturing golden master for scenario: {}", name);
debug!("GUI state: {:?}", state);
info!("Snapshot written to: {}", path);
info!("Capture took: {:?}", duration);
```

**Questions**:
- What level of logging? (debug, info, warn, error)
- Should we log to file or just stderr?
- Do we need structured logging (JSON) for parsing?
- What metrics do we track?

**Acceptance Criteria**:
```yaml
Logging Requirements:
  levels:
    - INFO: High-level operations (capture started/completed)
    - DEBUG: Detailed state (for troubleshooting)
    - WARN: Non-fatal issues (missing optional fields)
    - ERROR: Fatal failures (comparison failed)
  
  destination:
    - stderr: Human-readable (for local dev)
    - file: logs/golden-master-{timestamp}.log (for CI)
  
  structured: JSON format for CI parsing (optional)
  
Metrics to Track:
  - capture_duration_ms
  - comparison_duration_ms
  - snapshot_size_bytes
  - diff_count (number of differences)
  - scenario_name
  - platform
  - timestamp
```

**Action Items**:
- [ ] Add env_logger or tracing for structured logs
- [ ] Log to both stderr and file
- [ ] Document log format in OPERATIONS.md
- [ ] Add log parsing example (grep for failures)

---

#### **C. CI Artifacts**
```yaml
# What do we upload on failure?
artifacts:
  - golden-master-diff.json (machine-readable)
  - golden-master-diff.txt (human-readable)
  - current-snapshot.json (what we captured)
  - golden-master.json (what we expected)
  - logs/golden-master-run.log (full logs)
```

**Acceptance Criteria**:
```yaml
Artifact Requirements:
  retention: 7 days (balance cost vs usability)
  naming: Include scenario name + platform + timestamp
  size_limit: <10MB per artifact (warn if exceeded)
  
Download Experience:
  - One-click download from CI UI
  - Clear naming (no generic "artifact.zip")
  - README.txt in artifact explaining contents
```

**Action Items**:
- [ ] Configure artifact upload in CI workflow
- [ ] Test artifact download experience
- [ ] Add README.txt to artifact explaining how to use
- [ ] Document in OPERATIONS.md where to find artifacts

---

### **Red Flags** (Observability):
🚩 Failure message just says "test failed" (no context)  
🚩 No way to download diff locally  
🚩 No logs to debug what happened  
🚩 Can't reproduce failure locally  
🚩 No metrics to track performance over time

---

## 3️⃣ **DOCUMENTATION & ONBOARDING**

### **Why This Matters**:
```
6 months from now, new developer joins team:
"How do I update a golden master?"
If answer not in docs → Asks you → You forgot → Wasted time
```

### **Concerns**:

#### **A. User-Facing Documentation**
```markdown
# Where to document?
docs/OPERATIONS.md - For operators/developers
docs/TESTING.md (new?) - Testing guidelines
README.md - High-level mention
```

**Required Sections**:
```markdown
## Golden Master Testing

### What is it?
[Explain concept in 2-3 sentences]

### When does it run?
- Automatically on every PR touching rust/hash-checker-gui/**
- Manually: `cargo run -- --compare-golden <scenario>`

### How to update a golden master?
1. Make your GUI changes
2. Capture new golden master:
   cargo run -- --capture-golden <scenario>
3. Review diff carefully (git diff test-fixtures/golden/)
4. Commit if intentional:
   git add test-fixtures/golden/
   git commit -m "test: update golden master for <scenario> (reason: [X])"

### How to debug failures?
[Step-by-step with screenshots]

### FAQ
- Q: "Golden master failed but my change is intentional, what do I do?"
  A: [Answer]
- Q: "How do I add a new scenario?"
  A: [Answer]
- Q: "Test fails on CI but passes locally, why?"
  A: [Platform differences, see troubleshooting]
```

**Action Items**:
- [ ] Write docs/TESTING.md (new file)
- [ ] Update docs/OPERATIONS.md (add Golden Master section)
- [ ] Add brief mention to README.md
- [ ] Include screenshots of common workflows
- [ ] Review docs with someone unfamiliar with feature

---

#### **B. Code Documentation**
```rust
/// Captures the current GUI state as a golden master snapshot.
///
/// # Arguments
/// * `scenario` - Name of the scenario (e.g., "minimal-scan", "deep-tree")
/// * `output_path` - Where to save the snapshot JSON
///
/// # Returns
/// * `Ok(Snapshot)` - Successfully captured snapshot
/// * `Err(CaptureError)` - If capture failed (reason in error message)
///
/// # Performance
/// Typical capture takes ~300ms. If >1s, investigate performance regression.
///
/// # Example
/// ```
/// let snapshot = capture_golden_master("deep-tree", Path::new("test.json"))?;
/// ```
pub fn capture_golden_master(scenario: &str, output_path: &Path) 
    -> Result<Snapshot, CaptureError> {
    // ...
}
```

**Acceptance Criteria**:
```yaml
Code Documentation Requirements:
  - All public functions have docstrings
  - Examples for non-trivial usage
  - Performance notes (if critical)
  - Error cases documented
  - Links to related functions
  
Coverage Target: 100% of public API documented
```

**Action Items**:
- [ ] Add docstrings to all public functions
- [ ] Run `cargo doc --open` and review generated docs
- [ ] Add doctests (executable examples)
- [ ] Document JSON schema in comments

---

#### **C. Runbook for Common Issues**
```markdown
# Golden Master Troubleshooting Runbook

## Issue: "Golden master mismatch on CI but passes locally"

Possible Causes:
1. Platform differences (fonts, DPI, window decorations)
2. Timezone differences (timestamps in snapshot)
3. Random values (session IDs, UUIDs)

Solutions:
1. Check if field should be in "ignore list"
2. Use fuzzy matching for platform-dependent fields
3. Seed RNG for deterministic tests

Debug Steps:
1. Download artifact from CI
2. Compare local vs CI snapshot:
   diff golden-local.json golden-ci.json
3. Identify which fields differ
4. Decide: Update golden OR add to ignore list

## Issue: "Golden master capture is slow (>2s)"

[Troubleshooting steps]
```

**Action Items**:
- [ ] Create docs/troubleshooting/golden_master.md
- [ ] Document top 5 common issues
- [ ] Include actual error messages (searchable)
- [ ] Add links to relevant code sections

---

### **Red Flags** (Documentation):
🚩 No docs on how to update golden masters  
🚩 No troubleshooting guide  
🚩 No code documentation (cargo doc empty)  
🚩 README doesn't mention golden master testing  
🚩 No onboarding guide for new contributors

---

## 4️⃣ **BACKWARD COMPATIBILITY & MIGRATION**

### **Why This Matters**:
```
Current project: No golden masters
After Issue #56: Golden masters required in CI

Migration path: How do we introduce without breaking existing PRs?
```

### **Concerns**:

#### **A. Gradual Rollout**
```yaml
# Should we:
Option A: All-or-nothing (add golden masters, CI fails if mismatch)
Option B: Warning-only mode (log mismatches but don't fail CI)
Option C: Opt-in (only run if PR title contains "[golden-master]")
```

**Recommendation**: **Option B (Warning-only) → Option A (Enforced)**

```yaml
Phase 1 (Week 1-2): Warning-only mode
  - Capture golden masters for key scenarios
  - Run comparisons in CI but don't fail
  - Log mismatches to identify flaky tests
  - Tune ignore list and fuzzy matching
  
Phase 2 (Week 3-4): Enforced mode
  - Switch CI to fail on mismatch
  - Document update process
  - Train team on workflow
  
Rollback Plan:
  - If >30% of PRs blocked by flaky tests
  - Revert to warning-only
  - Fix flakiness
  - Re-enable enforcement
```

**Action Items**:
- [ ] Implement `--golden-master-mode=warn|enforce` flag
- [ ] Start with warning-only in CI
- [ ] Monitor false positive rate (2 weeks)
- [ ] Switch to enforce only if FP rate <5%

---

#### **B. Golden Master Versioning**
```json
{
  "version": "1.0.0",
  "schema_version": "1",
  "created_at": "2025-10-26T12:00:00Z",
  "platform": "linux",
  "scenario": "deep-tree",
  "gui_state": { ... }
}
```

**Why Version?**
- GUI evolves → snapshot format changes
- Old golden masters may not parse with new code
- Need migration path

**Strategy**:
```yaml
Version Policy:
  schema_version: Bump when snapshot format changes
  version: Bump when comparison logic changes
  
Migration:
  - Auto-migrate v1 → v2 on load (if possible)
  - Provide migration tool: cargo run -- --migrate-goldens
  - Warn if golden master >6 months old (may be stale)
  
Compatibility:
  - New code reads old goldens (backward compatible)
  - Fail gracefully if unsupported version
  - Document breaking changes in CHANGELOG.md
```

**Action Items**:
- [ ] Add version field to JSON schema
- [ ] Implement version check on load
- [ ] Create migration tool (if needed later)
- [ ] Document versioning policy in docs/

---

#### **C. Rollback Safety**
```
Scenario: Golden master testing has bugs, need to disable quickly
```

**Rollback Mechanisms**:
```yaml
Level 1 (Quick): Feature flag
  - Add: GOLDEN_MASTER_ENABLED=true/false env var
  - CI: Set to false to disable all golden master checks
  - Deployment: <5 minutes
  
Level 2 (Medium): Revert PR
  - git revert <PR-commit>
  - Re-run CI
  - Deployment: ~15 minutes
  
Level 3 (Nuclear): Disable CI job
  - Comment out golden-master-validation.yml workflow
  - Emergency only (breaks process)
  - Deployment: <1 minute
```

**Action Items**:
- [ ] Implement GOLDEN_MASTER_ENABLED feature flag
- [ ] Document rollback procedure in OPERATIONS.md
- [ ] Test rollback in staging (before production)

---

### **Red Flags** (Compatibility):
🚩 No migration path from current state  
🚩 All-or-nothing rollout (risky)  
🚩 No versioning (future changes break old goldens)  
🚩 No rollback plan (stuck if bugs found)  
🚩 No warning period (team not prepared)

---

## 5️⃣ **DEPLOYMENT & ROLLBACK**

### **Why This Matters**:
```
Golden master testing is CI INFRASTRUCTURE
If it breaks → All PRs blocked → Production at risk
Need deployment safety and rollback speed
```

### **Concerns**:

#### **A. CI Workflow Changes**
```yaml
# This is not application code, it's CI INFRASTRUCTURE
# Changes here affect ALL developers
```

**Deployment Strategy**:
```yaml
Testing:
  - Test workflow changes on fork first
  - Run on draft PR before merging
  - Monitor first 5 real PRs closely
  
Gradual Rollout:
  - Enable for 1 developer first (testing)
  - Enable for team (week 1)
  - Enable for all PRs (week 2)
  
Monitoring:
  - False positive rate (how often blocks valid PRs?)
  - False negative rate (how often misses real issues?)
  - Performance impact (CI time increase)
```

**Action Items**:
- [ ] Test workflow on fork repository
- [ ] Create test PR to validate workflow
- [ ] Monitor false positive rate first week
- [ ] Document deployment checklist

---

#### **B. Breaking Changes**
```
What if golden master format changes incompatibly?
All existing PRs fail CI → Nightmare
```

**Protection Strategy**:
```yaml
Before Breaking Change:
  1. Announce to team (1 week notice)
  2. Provide migration script
  3. Update all golden masters in main
  4. Deploy during low-traffic window
  5. Monitor for 24 hours
  
Communication:
  - Slack/Teams announcement
  - Email to all developers
  - Add to sprint planning notes
  - Update CHANGELOG.md prominently
```

**Action Items**:
- [ ] Define "breaking change" criteria
- [ ] Create announcement template
- [ ] Document migration process
- [ ] Schedule during sprint boundaries

---

### **Red Flags** (Deployment):
🚩 No testing on fork before main deployment  
🚩 No gradual rollout plan  
🚩 No monitoring of false positive rate  
🚩 Breaking changes without team notice  
🚩 No deployment window defined

---

## 6️⃣ **USER EXPERIENCE & ACCESSIBILITY**

### **Why This Matters for Issue #56**:
```
Golden master testing is DEVELOPER-FACING feature
But it affects DEVELOPER PRODUCTIVITY
Bad UX → Developers bypass/disable tests → Defeats purpose
```

### **Concerns**:

#### **A. Developer Workflow Friction**
```bash
# BAD: Too many steps
cargo run -- --capture-golden scenario1
cargo run -- --capture-golden scenario2
cargo run -- --capture-golden scenario3
git add test-fixtures/golden/*.json
git commit -m "update goldens"

# GOOD: One command
make update-goldens
# → Captures all scenarios, git adds, prompts for commit message
```

**Action Items**:
- [ ] Add `make update-goldens` convenience command
- [ ] Add `cargo run -- --capture-all-goldens` flag
- [ ] Interactive mode: "Which scenarios to update? [1,2,3]"
- [ ] Dry-run mode: Show what would change without saving

---

#### **B. Error Message Quality**
```rust
// BAD:
panic!("Golden master mismatch");

// GOOD:
eprintln!("❌ Golden master mismatch detected");
eprintln!();
eprintln!("Scenario: deep-tree-scan");
eprintln!("Platform: linux");
eprintln!();
eprintln!("Differences found:");
eprintln!("  • window.width: 1280 → 1024");
eprintln!("  • theme: 'dark' → 'light'");
eprintln!();
eprintln!("📝 To fix:");
eprintln!("  1. Review changes: git diff test-fixtures/golden/");
eprintln!("  2. If intentional: cargo run -- --capture-golden deep-tree-scan");
eprintln!("  3. Commit update: git add test-fixtures/golden/...");
eprintln!();
eprintln!("🔍 For debugging:");
eprintln!("  • See full diff: cat logs/golden-master-diff.json");
eprintln!("  • Reproduce: cargo run -- --compare-golden deep-tree-scan");
```

**Acceptance Criteria**:
```yaml
Error Message Quality:
  - Uses emojis for visual clarity (❌✅🔍📝)
  - Structured sections (problem, fix, debug)
  - Actionable commands (copy-paste ready)
  - Links to documentation
  - Estimated fix time: "This should take ~5 minutes"
```

**Action Items**:
- [ ] Review all error messages with fresh eyes
- [ ] User test with someone unfamiliar with feature
- [ ] Add color output (if terminal supports)
- [ ] Include "Estimated impact" (critical vs minor)

---

#### **C. Accessibility for Different Skill Levels**
```markdown
Junior Developer: Needs step-by-step guidance
Senior Developer: Wants quick commands
CI Bot: Needs machine-readable output
```

**Multi-Level Support**:
```bash
# For juniors: Interactive mode
cargo run -- --interactive
# → Shows menu: "What do you want to do?"
# → 1. Capture golden master
# → 2. Compare with golden master
# → 3. Update golden master
# → 4. View diff

# For seniors: Direct commands
cargo run -- --capture-golden <scenario>

# For CI: Machine-readable
cargo run -- --compare-golden <scenario> --output=json
```

**Action Items**:
- [ ] Add `--interactive` mode for beginners
- [ ] Add `--output=json` for machine parsing
- [ ] Keep direct commands for power users
- [ ] Document all modes in README

---

### **Red Flags** (UX):
🚩 Developer needs 10+ steps to update golden master  
🚩 Error messages unclear ("something failed")  
🚩 No convenience commands (must type long flags)  
🚩 No interactive mode for beginners  
🚩 No machine-readable output for automation

---

## 7️⃣ **MAINTENANCE BURDEN & TECHNICAL DEBT**

### **Why This Matters**:
```
Every new feature = Long-term maintenance cost
Golden master testing = New code to maintain
Need to minimize future burden
```

### **Concerns**:

#### **A. Code Complexity**
```rust
// How complex is the implementation?
// Can someone else maintain it?
```

**Maintainability Checklist**:
```yaml
Code Quality:
  - No clever tricks (prefer explicit over clever)
  - Standard Rust idioms (not exotic patterns)
  - Small functions (<50 lines each)
  - Clear naming (no abbreviations)
  - Comments explain WHY not WHAT
  
Dependencies:
  - Minimize new dependencies (each is maintenance burden)
  - Use well-maintained crates (serde, serde_json)
  - Avoid unmaintained or alpha-stage crates
  - Document dependency rationale
  
Architecture:
  - Modular design (easy to extend)
  - Clear interfaces (trait-based)
  - Testable (dependency injection where needed)
```

**Action Items**:
- [ ] Code review with focus on maintainability
- [ ] Run `cargo tree` to audit dependencies
- [ ] Document architecture in docs/architecture/golden_master.md
- [ ] Add "maintenance burden" section to PR description

---

#### **B. Golden Master Maintenance**
```
Over time, golden masters accumulate:
- 5 scenarios today
- 20 scenarios in 6 months
- 50 scenarios in 1 year

Who maintains them? How?
```

**Strategy**:
```yaml
Golden Master Lifecycle:
  creation:
    - Requires PR review (not automatic)
    - Must justify: "Why this scenario vs existing ones?"
  
  maintenance:
    - Monthly review: Are old scenarios still relevant?
    - Delete stale scenarios (UI changed significantly)
    - Archive deprecated scenarios (for historical reference)
  
  ownership:
    - CODEOWNERS: Assign team/person responsible
    - Rotate responsibility quarterly
    
Storage:
  location: test-fixtures/golden/
  size_limit: <5MB per scenario (warn if exceeded)
  retention: Active scenarios only (delete after 1 year unused)
```

**Action Items**:
- [ ] Add CODEOWNERS for test-fixtures/golden/
- [ ] Schedule quarterly golden master review
- [ ] Document in CONTRIBUTING.md who approves new scenarios
- [ ] Add size check in CI (warn if golden >5MB)

---

#### **C. Technical Debt Tracking**
```yaml
# Every shortcut taken = Technical debt
# Must track and pay down later
```

**Known Debt from Phase 1-3 (MVP)**:
```yaml
debt_items:
  - id: debt-1
    description: "Only exact matching, no fuzzy/structural"
    impact: "Platform differences cause false positives"
    paydown_effort: "3-4 hours"
    paydown_by: "Phase 4 or Sprint 2"
    
  - id: debt-2
    description: "JSON only, no PNG screenshots"
    impact: "Can't catch visual regressions"
    paydown_effort: "6-8 hours"
    paydown_by: "Phase 5 or later (low priority)"
    
  - id: debt-3
    description: "No golden master versioning yet"
    impact: "Breaking changes harder to manage"
    paydown_effort: "2-3 hours"
    paydown_by: "Sprint 2"
```

**Action Items**:
- [ ] Document technical debt in .agents/technical_debt.yml
- [ ] Link debt items to GitHub issues
- [ ] Review debt quarterly (address high-impact items)
- [ ] Include "debt paydown" in sprint planning

---

### **Red Flags** (Maintenance):
🚩 Complex, clever code (hard to understand)  
🚩 Many new dependencies (maintenance burden)  
🚩 No architecture documentation  
🚩 Golden masters pile up without review  
🚩 Technical debt not tracked  
🚩 No CODEOWNERS for new feature

---

## 8️⃣ **CROSS-PLATFORM CONSISTENCY**

### **Why This Matters for Issue #56**:
```
GUI renders differently on:
- Linux (fonts, window decorations)
- macOS (Retina display, DPI)
- Windows (fonts, DPI, themes)

Golden masters WILL differ across platforms
How do we handle this?
```

### **Concerns**:

#### **A. Platform-Specific Golden Masters**
```
test-fixtures/golden/
  ├── linux/
  │   ├── minimal-scan.json
  │   └── deep-tree.json
  ├── macos/
  │   ├── minimal-scan.json
  │   └── deep-tree.json
  └── windows/
      ├── minimal-scan.json
      └── deep-tree.json
```

**Questions**:
- Do we NEED 3x golden masters (one per platform)?
- Or can we use fuzzy matching to unify?
- What about Linux distros? (Ubuntu vs Fedora)

**Recommendation**: **Start with Linux-only, add others later**

```yaml
Phase 1 (MVP): Linux only
  - Capture golden masters on ubuntu-latest
  - Only run validation on Linux in CI
  - Document: "macOS/Windows support coming in Phase 4"
  
Phase 4: Multi-platform
  - Add macOS golden masters
  - Add Windows golden masters
  - Implement platform detection
  - Update CI matrix
```

**Action Items**:
- [ ] Start with Linux-only (simpler)
- [ ] Document platform strategy in backlog
- [ ] Plan multi-platform support for Phase 4
- [ ] Add `--platform` flag for future use

---

#### **B. Fuzzy Matching for Platform Differences**
```rust
// Fields that may differ across platforms
let ignore_fields = vec![
    "window.decoration_height", // macOS vs Linux
    "font.family", // System fonts differ
    "dpi", // Retina vs non-Retina
];

// Tolerance for dimensions
let dimension_tolerance = 5; // ±5 pixels acceptable
```

**Strategy**:
```yaml
Comparison Modes:
  exact:
    - Use for: Structural fields (tab count, button labels)
    - Fail if: Any difference
    
  fuzzy:
    - Use for: Dimensions, colors
    - Fail if: Difference > tolerance
    
  ignore:
    - Use for: Platform-specific fields
    - Don't compare these at all
    
Configuration:
  file: test-fixtures/golden/comparison_config.yml
  fields:
    - path: "window.width"
      mode: fuzzy
      tolerance: 5 # pixels
    - path: "font.family"
      mode: ignore # platform-specific
    - path: "theme"
      mode: exact # must match
```

**Action Items**:
- [ ] Create comparison_config.yml
- [ ] Implement fuzzy matching (Phase 3)
- [ ] Test on different platforms locally
- [ ] Document platform differences in troubleshooting

---

#### **C. CI Matrix Configuration**
```yaml
# Which platforms to test?
matrix:
  os: [ubuntu-latest] # Phase 1 (MVP)
  # os: [ubuntu-latest, macos-latest, windows-latest] # Phase 4
```

**Cost Considerations**:
```yaml
Current CI (no golden master):
  time: 16 minutes
  platforms: 3
  total: 48 CI minutes per PR
  
With golden master (3 platforms):
  time: 18 minutes (estimate)
  platforms: 3
  total: 54 CI minutes per PR
  increase: +6 minutes (+12.5%)
  
Cost:
  free_tier: 2000 minutes/month
  current_usage: ~1200 minutes/month (estimate)
  remaining: 800 minutes
  additional_cost: ~300 minutes/month
  verdict: Acceptable (within free tier)
```

**Action Items**:
- [ ] Measure current CI minute usage
- [ ] Calculate projected increase
- [ ] Get approval if exceeds budget
- [ ] Consider platform-specific triggers (only macOS for macOS changes)

---

### **Red Flags** (Cross-Platform):
🚩 Assume one golden master works for all platforms  
🚩 No fuzzy matching (brittle to platform differences)  
🚩 No testing on target platforms before release  
🚩 CI costs not calculated  
🚩 No platform detection logic

---

## 🎯 **SUMMARY: Complete Readiness Checklist**

### **Before claiming "Production Ready"**:

```yaml
Security: ✅ (assumed handled in core issue)
  - [ ] No secrets in golden masters
  - [ ] No sensitive paths leaked
  - [ ] Input validation (JSON parsing)

Testing: ✅ (core of Issue #56)
  - [ ] Unit tests for all modules
  - [ ] Integration tests for workflows
  - [ ] Edge case coverage
  - [ ] CI integration

Performance: ⏳ (THIS DOCUMENT)
  - [ ] Benchmarks created (criterion)
  - [ ] Capture <1s per scenario
  - [ ] Comparison <2s all scenarios
  - [ ] CI overhead <2 minutes
  - [ ] Timeout guards added

Observability: ⏳ (THIS DOCUMENT)
  - [ ] Rich failure messages
  - [ ] Structured logging
  - [ ] CI artifacts uploaded
  - [ ] Metrics tracked
  - [ ] Debugging docs

Documentation: ⏳ (THIS DOCUMENT)
  - [ ] User guide (OPERATIONS.md)
  - [ ] API documentation (cargo doc)
  - [ ] Troubleshooting runbook
  - [ ] FAQ section
  - [ ] Onboarding guide

Compatibility: ⏳ (THIS DOCUMENT)
  - [ ] Gradual rollout plan
  - [ ] Versioning strategy
  - [ ] Migration path
  - [ ] Rollback procedure
  - [ ] Feature flag

Deployment: ⏳ (THIS DOCUMENT)
  - [ ] Testing on fork
  - [ ] Gradual enablement
  - [ ] Monitoring plan
  - [ ] Communication strategy
  - [ ] Deployment checklist

UX: ⏳ (THIS DOCUMENT)
  - [ ] Convenience commands
  - [ ] Error message quality
  - [ ] Interactive mode
  - [ ] Multi-level support
  - [ ] User testing

Maintenance: ⏳ (THIS DOCUMENT)
  - [ ] Code complexity review
  - [ ] Dependency audit
  - [ ] Golden master lifecycle
  - [ ] Technical debt tracking
  - [ ] CODEOWNERS assigned

Cross-Platform: ⏳ (THIS DOCUMENT)
  - [ ] Platform strategy decided
  - [ ] Fuzzy matching implemented
  - [ ] CI matrix configured
  - [ ] Cost calculated
  - [ ] Platform detection
```

**Total Checklist Items**: **50+** (beyond security & testing)

---

## 📊 **Priority Matrix**

| Pillar | Priority | Phase | Effort | Risk if Skipped |
|--------|----------|-------|--------|-----------------|
| **Performance** | 🔴 CRITICAL | Phase 1-3 | 4h | CI becomes bottleneck |
| **Observability** | 🔴 CRITICAL | Phase 1-3 | 3h | Can't debug failures |
| **Documentation** | 🟡 HIGH | Phase 3-4 | 4h | Team confusion |
| **Compatibility** | 🟡 HIGH | Phase 1 | 2h | Breaking rollout |
| **Deployment** | 🟡 HIGH | Phase 4 | 2h | Risky releases |
| **UX** | 🟢 MEDIUM | Phase 3-4 | 3h | Developer friction |
| **Maintenance** | 🟢 MEDIUM | Ongoing | 2h | Future burden |
| **Cross-Platform** | 🟢 MEDIUM | Phase 4+ | 5h | Platform bugs |

**Total Additional Effort**: **25 hours** (beyond core implementation)

---

## 🚀 **Recommended Approach**

### **MVP (Phase 1-3): Focus on Critical**
```yaml
Include (MUST have):
  - Performance benchmarks (prevent CI bottleneck)
  - Rich error messages (enable debugging)
  - Basic documentation (OPERATIONS.md)
  - Feature flag (enable rollback)
  - Linux-only (simplify scope)
  
Defer (LATER):
  - Multi-platform support
  - Interactive mode
  - Advanced UX features
  - Comprehensive troubleshooting docs
```

### **Polish (Phase 4-5): Add High-Priority Items**
```yaml
Add:
  - Multi-platform golden masters
  - Fuzzy matching
  - Comprehensive documentation
  - UX improvements
  - Advanced observability
```

---

**Document Created**: 2025-10-26  
**Scope**: Operational readiness beyond security & testing  
**Total Concerns**: 8 pillars, 50+ checklist items  
**Estimated Additional Effort**: 25 hours

**Next Action**: Review with team, prioritize based on risk tolerance
