# AI Agent Handoff Workflow

> **Purpose**: Standardized process for receiving and continuing work from another AI agent.
> **Scope**: Applies when taking over EXISTING work (not starting fresh).

---

## Table of Contents
- [Overview](#overview)
- [Phase 1: Handoff Reception](#phase-1-handoff-reception)
- [Phase 2: Continuity Planning](#phase-2-continuity-planning)
- [Phase 3: Implementation](#phase-3-implementation)
- [Phase 4: Completion](#phase-4-completion)
- [Handoff Quality Checklist](#handoff-quality-checklist)

---

## Overview

### When to Use This Workflow
- ✅ Another agent tagged you in handoff comment
- ✅ Handoff document exists in `.agents/backlog/`
- ✅ Work is partially complete (code/branch exists)
- ✅ Clear continuation criteria provided

### When NOT to Use
- ❌ Fresh issue (no code yet) → Use `issue_claim_workflow.md`
- ❌ No handoff document → Request one before starting
- ❌ Unclear continuation scope → Ask clarifying questions first

### Key Principles
1. **Respect Previous Work**: Don't rewrite without strong justification
2. **Maintain Continuity**: Match existing patterns and style
3. **Validate Thoroughly**: Ensure previous work actually functions
4. **Complete the Vision**: Finish what was started

---

## Phase 1: Handoff Reception

**Duration**: 45-60 minutes  
**Goal**: Understand and validate existing work

### Step 1.1: Locate Handoff Document (2 mins)

```bash
# Find handoff document
ls .agents/backlog/handoff-*.yml

# Read it
cat .agents/backlog/handoff-YYYYMMDD-issue<N>.yml
```

**Expected Content:**
- Who handed off (from)
- Who receives (to)
- What's completed
- What remains
- Technical notes
- Known issues
- Files modified
- Test commands

### Step 1.2: Read Handoff Document Thoroughly (10-15 mins)

**Critical Questions:**
- [ ] What was completed?
- [ ] What remains to be done?
- [ ] What are the completion criteria?
- [ ] Any known issues or concerns?
- [ ] What decisions were made and why?
- [ ] Are there specific constraints?
- [ ] Who to contact for questions?

**Document Your Understanding:**
```markdown
## Handoff Reception Notes

### Completed Work
- [Item 1]
- [Item 2]

### Remaining Work
- [Item 1]
- [Item 2]

### Key Decisions by Previous Agent
- [Decision 1]: [Rationale]
- [Decision 2]: [Rationale]

### Questions/Concerns
- [Question 1]
- [Question 2]
```

### Step 1.3: Code Review (20-30 mins)

```bash
# Checkout branch
git checkout <branch-name>

# View commit history
git log --oneline -20

# See full diff from main
git diff main...HEAD

# Read modified files
cat <file1> <file2> <file3>

# Search for TODOs/FIXMEs
rg "TODO|FIXME|XXX|HACK" <directory>
```

**Review Checklist:**
- [ ] Read ALL modified files carefully
- [ ] Understand design decisions
- [ ] Identify incomplete parts
- [ ] Check for TODOs/FIXMEs
- [ ] Review test coverage
- [ ] Understand error handling
- [ ] Check documentation

**Code Quality Assessment:**
```markdown
## Code Review Notes

### Architecture
- [Description of approach]
- [Opinion: solid/needs improvement]

### Code Quality
- Style: [consistent/inconsistent]
- Tests: [good coverage/gaps]
- Documentation: [adequate/needs work]

### Technical Debt
- [Item 1]
- [Item 2]

### Praise (what was done well)
- [Highlight good work]
```

### Step 1.4: Validation Testing (10-15 mins)

```bash
# Run existing tests
cargo test --package <package-name>
# OR
npm test

# Run linters
cargo clippy
# OR
npm run lint

# Test manually (follow handoff test commands)
cargo run -- <flags-from-handoff>
```

**Validation Checklist:**
- [ ] All existing tests pass
- [ ] Linters clean (or known issues documented)
- [ ] Manual test works as described
- [ ] No merge conflicts with main
- [ ] Dependencies properly declared
- [ ] Branch up-to-date with main (or close)

### Step 1.5: Gap Analysis (5-10 mins)

**Compare:**
- Handoff says "completed" vs actual code state
- Handoff says "remaining" vs what you see

**Questions:**
- Are there undocumented changes?
- Is scope creep evident?
- Are estimates still realistic?
- Should scope be adjusted?

**Document Gaps:**
```markdown
## Gap Analysis

### Discrepancies Found
- [ ] Handoff claims X complete, but [finding]
- [ ] Code has Y not mentioned in handoff

### Additional Work Identified
- [Item 1]: [Why needed]
- [Item 2]: [Why needed]

### Scope Adjustments Needed?
- [Propose changes if needed]
```

---

## Phase 2: Continuity Planning

**Duration**: 15-20 minutes  
**Goal**: Plan continuation while respecting previous work

### Step 2.1: Acknowledge Handoff (2 mins)

**Comment on GitHub Issue:**
```bash
gh issue comment <N> --body-file .agents/workflows/templates/handoff_reception_comment.md
```

**Use Template**: `.agents/workflows/templates/handoff_reception_comment.md`

### Step 2.2: Decide: Continue or Adjust (5-10 mins)

**Decision Matrix:**

| Scenario | Action |
|----------|--------|
| Previous work solid, clear path | Continue as planned |
| Previous work solid, scope unclear | Clarify with human before proceeding |
| Previous work has issues | Document issues, propose fixes |
| Architecture fundamentally flawed | Discuss with human + original agent |
| Missing critical context | Request more info from original agent |

**If Issues Found:**
```bash
# Comment on issue
gh issue comment <N> --body "Handoff received. Found some concerns:

1. [Concern 1]
2. [Concern 2]

Proposed resolution:
- [Resolution 1]
- [Resolution 2]

@<original-agent> can you clarify [X]?
@<human> please advise on approach."
```

**Wait for clarification before proceeding if issues are significant.**

### Step 2.3: Honor Previous Decisions (Critical!)

**Unless fundamentally flawed, you must:**
- ✅ Keep same architecture
- ✅ Follow same naming conventions
- ✅ Use same dependencies (don't swap without reason)
- ✅ Match code style
- ✅ Continue established patterns

**Only deviate if:**
- 🚨 Security issue found
- 🚨 Correctness bug found
- 🚨 Architecture blocks completion
- 🚨 Human approves deviation

**Document All Deviations:**
```markdown
## Deviations from Original Plan

### Change 1
- **Original**: [What previous agent did]
- **Changed to**: [What you're doing]
- **Reason**: [Strong justification]
- **Approved by**: [Human/issue comment link]
```

### Step 2.4: Create Continuation Plan (5-10 mins)

**Use TodoWrite for remaining work:**

```yaml
todos:
  - id: task-<N>-cont-1
    content: "[Next logical step from handoff]"
    status: pending
    
  - id: task-<N>-cont-2
    content: "[Following step]"
    status: pending
```

**Update active_tasks.yml:**
```yaml
# Update existing entry
- id: task-<issue-N>
  title: "[Issue title]"
  status: in_progress
  assignee: <your-agent-name>  # Changed from previous agent
  notes: "Handoff from <previous-agent>. Continuing: [scope]"
  handoff_date: YYYY-MM-DD
  original_assignee: <previous-agent>
```

---

## Phase 3: Implementation

**Duration**: Varies  
**Goal**: Complete remaining work with consistency

### Step 3.1: Continuity-First Development

**Principles:**
- ✅ Match existing code style
- ✅ Follow established patterns
- ✅ Keep tests passing
- ✅ Build on (don't replace) existing work

**Commit Message Format:**
```bash
git commit -m "<type>(<scope>): <short description>

Continue work from @<previous-agent> (handoff-YYYYMMDD-issue<N>)

- Detail 1
- Detail 2

Relates to #<N>
Co-authored-by: <PreviousAgent> <email>"
```

### Step 3.2: Incremental Progress

**At Each Step:**
1. Implement change
2. Run tests (ensure previous work still passes!)
3. Update handoff checklist
4. Commit with clear message

```bash
# Critical: Always verify no regression
cargo test --all-features

# Before:
# test module::previous_test ... ok

# After your change:
# test module::previous_test ... ok  ← MUST STILL PASS
# test module::new_test ... ok      ← Your addition
```

### Step 3.3: Update Handoff Checklist

**As you complete items from handoff:**
```yaml
# In .agents/backlog/handoff-YYYYMMDD-issue<N>.yml
remaining:
  - ✅ Comparison logic (completed YYYY-MM-DD by <your-name>)
  - ⏳ CI integration (in progress)
  - ❌ Documentation (not started)
```

### Step 3.4: Progress Updates

**Keep issue updated:**
```bash
gh issue comment <N> --body "Handoff progress update:

✅ Completed:
- [Item from handoff]
- [Item from handoff]

⏳ In progress:
- [Current work]

📝 Next:
- [Next item]

On track for completion by [date]."
```

### Step 3.5: Handle Discoveries

**If you find issues in previous work:**

1. **Minor issues** (style, small bugs):
   - Fix inline
   - Document in commit message
   - Mention in PR

2. **Major issues** (logic errors, security):
   - Stop immediately
   - Document issue clearly
   - Tag original agent + human
   - Get approval before fixing

```bash
gh issue comment <N> --body "⚠️ Found issue in previous implementation:

**Issue**: [Description]
**Impact**: [Severity]
**Location**: [File/line]

**Proposed fix**: [Solution]

@<original-agent> @<human> please advise before I proceed."
```

---

## Phase 4: Completion

**Duration**: 30-45 minutes  
**Goal**: Finish, validate, credit properly

### Step 4.1: Complete Handoff Criteria

**Verify all items from handoff checklist:**
```yaml
# All items should be ✅
remaining:
  - ✅ Item 1
  - ✅ Item 2
  - ✅ Item 3
```

### Step 4.2: Final Validation

**Same as fresh issue, plus:**
- [ ] Previous tests still pass (no regression!)
- [ ] Handoff criteria met
- [ ] Code style consistent throughout
- [ ] Original architecture preserved (or deviations documented)

```bash
# Full CI
make ci-linux-local 2>&1 | tee logs/ci-handoff-$(date +%Y%m%d-%H%M%S).log
```

### Step 4.3: Open/Update Pull Request

**If PR exists:**
```bash
# Continue on same PR
git push origin <branch-name>

# Comment on PR
gh pr comment <PR-N> --body "Handoff from @<previous-agent> complete.

Completed:
- [Item 1]
- [Item 2]

All handoff criteria met. Ready for review."
```

**If PR doesn't exist:**
```bash
gh pr create --title "[Feature] <Description>" --body "$(cat <<'EOF'
Fixes #<N>

## Summary
[What this PR does]

**Note**: Continuation of work started by @<previous-agent>

## Handoff Context
- Handoff date: YYYY-MM-DD
- Handoff document: .agents/backlog/handoff-YYYYMMDD-issue<N>.yml
- Original work by: @<previous-agent>

## What Was Completed Before Handoff
- [Item 1]
- [Item 2]

## What I Completed
- [Item 3]
- [Item 4]

## Testing
- [x] All tests pass (including previous work)
- [x] No regressions introduced
- [x] Handoff criteria met

## Logs
See logs/ci-handoff-<timestamp>.log

Co-authored-by: <PreviousAgent> <email>
EOF
)"
```

### Step 4.4: Update Tracking

```yaml
# Move to completed_tasks.yml
- id: task-<issue-N>
  title: "[Issue title]"
  status: completed
  completed_date: YYYY-MM-DD
  github_issue: <N>
  pull_request: <PR-N>
  original_assignee: <previous-agent>
  completed_by: <your-agent-name>
  handoff_date: YYYY-MM-DD
  notes: "Handoff completion. Original work by <prev>, continuation by <you>"
```

### Step 4.5: Thank Original Agent

**In PR or issue:**
```markdown
@<original-agent> - Excellent foundation! Your [specific praise] made 
the continuation smooth. Thanks for the thorough handoff document. 🙏
```

### Step 4.6: Lessons Learned

**Document handoff experience:**
```markdown
# Lesson: Handoff from <Agent> on Issue #<N>

## What Worked Well
- [Good practice from previous agent]
- [What made handoff smooth]

## What Could Improve
- [Suggestion for better handoffs]

## Tips for Future Handoffs
- [Tip 1]
- [Tip 2]
```

---

## Handoff Quality Checklist

**Before marking complete, verify:**

### Handoff Criteria
- [ ] All items from handoff checklist completed
- [ ] Handoff document updated with progress
- [ ] Original agent's work respected and credited

### Code Continuity
- [ ] Previous tests still pass (no regression!)
- [ ] Code style consistent with previous work
- [ ] Architecture unchanged (or deviations approved)
- [ ] Naming conventions followed

### Quality (same as fresh issue)
- [ ] All tests pass
- [ ] Linters clean
- [ ] Documentation updated
- [ ] No new warnings

### Attribution
- [ ] Co-authored-by in commits
- [ ] Credit given in PR description
- [ ] Thank you comment to original agent

---

## Common Scenarios

### Scenario 1: Handoff is Clean and Clear

**Example**: Codex hands off Issue #56 with excellent documentation

**Actions**:
1. ✅ Validate previous work (quick)
2. ✅ Continue seamlessly
3. ✅ Complete remaining items
4. ✅ Credit appropriately

**Estimated overhead**: ~1 hour for handoff reception

---

### Scenario 2: Handoff Has Minor Issues

**Example**: Previous work has small bugs or style inconsistencies

**Actions**:
1. ⚠️ Document issues found
2. ⚠️ Fix inline (with clear commit messages)
3. ⚠️ Mention in PR
4. ✅ Continue with completion

**Estimated overhead**: +1-2 hours for fixes

---

### Scenario 3: Handoff Has Major Issues

**Example**: Architecture blocking completion or security issues

**Actions**:
1. 🚨 Stop immediately
2. 🚨 Document issues thoroughly
3. 🚨 Tag original agent + human
4. 🚨 Get explicit approval for fixes
5. 🚨 May need to refactor significantly

**Estimated overhead**: +4-8 hours, requires discussion

---

### Scenario 4: Handoff Document Missing/Incomplete

**Example**: Agent tagged you but no proper handoff doc

**Actions**:
1. ❌ Do NOT start work
2. ❌ Request proper handoff document
3. ❌ Use template: `.agents/workflows/templates/handoff_document.yml`
4. ✅ Wait for complete handoff before proceeding

```bash
gh issue comment <N> --body "@<previous-agent> - To ensure smooth handoff, 
please create handoff document using:
.agents/workflows/templates/handoff_document.yml

Need to understand:
- What's completed
- What remains
- Technical decisions made
- Known issues

Will start after handoff doc is ready. Thanks!"
```

---

## Troubleshooting

### If Previous Work Doesn't Work

1. **Verify your environment matches theirs**
   - Same Rust version?
   - Same dependencies?
   - Same OS/platform?

2. **Check handoff notes for setup steps**
   - Any special flags needed?
   - Environment variables?
   - Test data required?

3. **Contact original agent**
   ```bash
   gh issue comment <N> --body "@<agent> - Handoff received but encountering issue:
   
   [Description of problem]
   
   Steps I tried:
   1. [Step]
   2. [Step]
   
   Can you clarify [X]?"
   ```

### If Scope Has Drifted

1. **Document scope drift**
2. **Assess if drift is acceptable**
3. **Discuss with human**
4. **Get explicit approval for adjusted scope**

### If Architecture Feels Wrong

1. **Document concerns specifically**
2. **Propose alternative (with rationale)**
3. **Discuss with original agent + human**
4. **Get approval before major refactor**
5. **Remember**: "Different" ≠ "Wrong"

---

## Examples

### Example 1: Clean Handoff (Issue #56)

**Handoff from Codex:**
- Excellent documentation
- Clear completion criteria
- All previous tests pass
- No major issues found

**Reception time**: 45 mins  
**Continuation**: Smooth, 4 hours to complete  
**Result**: On-time delivery with co-authorship

---

### Example 2: Handoff with Minor Fixes (Issue #42)

**Handoff from automation bot:**
- Dependency updates completed
- Tests passing but style inconsistent
- Some clippy warnings

**Reception time**: 60 mins  
**Fix time**: +2 hours for cleanup  
**Result**: Completed with improvements documented

---

## Related Documents
- Issue claim workflow: `.agents/workflows/issue_claim_workflow.md`
- Agent registry: `.agents/agents_registry.md`
- Repository guidance: `.agents/AGENTS.md`
- Templates: `.agents/workflows/templates/`

---

**Version**: 1.0  
**Last Updated**: 2025-10-26  
**Maintained By**: Cursor (Claude 4.5 Sonnet)
