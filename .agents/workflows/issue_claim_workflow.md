# AI Agent Issue Claim Workflow

> **Purpose**: Standardized process for AI agents to claim and execute fresh GitHub issues.
> **Scope**: Applies when starting NEW work (not continuing from handoff).

---

## Table of Contents
- [Overview](#overview)
- [Phase 1: Discovery & Analysis](#phase-1-discovery--analysis)
- [Phase 2: Claim & Plan](#phase-2-claim--plan)
- [Phase 3: Implementation](#phase-3-implementation)
- [Phase 4: Completion & Handoff](#phase-4-completion--handoff)
- [Quality Checklist](#quality-checklist)

---

## Overview

### When to Use This Workflow
- ✅ Fresh GitHub issue (unassigned, no code yet)
- ✅ Issue within your agent scope (see `.agents/agents_registry.md`)
- ✅ No active work by other agents
- ✅ Clear requirements and acceptance criteria

### When NOT to Use
- ❌ Issue already assigned to another agent
- ❌ Work already in progress (use handoff workflow instead)
- ❌ Unclear requirements (discuss in issue first)
- ❌ Outside your agent scope (coordinate first)

---

## Phase 1: Discovery & Analysis

**Duration**: 60-90 minutes  
**Goal**: Build complete understanding before committing

### Step 1.1: Initial Research (25-30 mins)

```bash
# Read issue thoroughly
gh issue view <N> --comments

# Search for related patterns in codebase
rg -i "<key-term>" --type <lang>

# Find similar implementations
rg "<pattern>" --files-with-matches

# Review project documentation
cat docs/PLAN.md docs/TASKS.md
```

**Checklist:**
- [ ] Read issue description and ALL comments
- [ ] Understand problem statement clearly
- [ ] Review acceptance criteria
- [ ] Identify stakeholders and their concerns
- [ ] Search codebase for related code
- [ ] Find existing patterns to follow

### Step 1.2: Requirement Analysis (15-20 mins)

**Deep Questions:**
- What is the REAL problem being solved?
- What are the explicit requirements?
- What are the IMPLICIT requirements?
- What edge cases must be handled?
- What's IN scope? What's OUT of scope?
- What are the constraints (performance, compatibility, etc.)?

**Document:**
```markdown
## Requirements Analysis (internal notes)

### Explicit Requirements
- [List from issue]

### Implicit Requirements
- [Inferred from context]

### Edge Cases
- [What could go wrong?]

### Out of Scope
- [What we're NOT doing]

### Constraints
- Platform: [Linux/macOS/Windows/All]
- Performance: [Expectations]
- Dependencies: [Restrictions]
```

### Step 1.3: Technical Investigation (20-30 mins)

```bash
# Explore codebase structure
tree -L 3 <relevant-directory>

# Identify integration points
rg "struct|trait|fn" --type rust <relevant-file>

# Review existing test patterns
find . -name "*test*.rs" -exec grep -l "<pattern>" {} \;

# Check CI/CD implications
cat .github/workflows/*.yml
```

**Checklist:**
- [ ] Understand codebase structure
- [ ] Identify where new code will live
- [ ] Find integration points
- [ ] Review existing test patterns
- [ ] Check CI/CD constraints
- [ ] Investigate dependencies (Cargo.toml, package.json, etc.)

### Step 1.4: Risk Assessment (5-10 mins)

**Questions:**
- Breaking changes? (API, CLI flags, behavior)
- Security implications? (auth, secrets, validation)
- Performance impact? (profiling needed?)
- Migration path? (existing data/configs)
- Platform compatibility? (OS-specific code)

**Risk Matrix:**
```markdown
| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| [Risk description] | H/M/L | H/M/L | [Plan] |
```

---

## Phase 2: Claim & Plan

**Duration**: 30-45 minutes  
**Goal**: Get approval and create clear implementation plan

### Step 2.1: Request Approval (5 mins)

**Use Template**: `.agents/workflows/templates/issue_claim_comment.md`

```bash
# Comment on issue with claim request
gh issue comment <N> --body-file .agents/workflows/templates/issue_claim_comment.md
```

**Wait for human approval before proceeding.**

### Step 2.2: Self-Assign Issue (1 min)

```bash
# After approval
gh issue edit <N> --add-assignee @me
```

### Step 2.3: Architecture Design (15-20 mins)

**Design Decisions:**
- Component structure
- File organization
- Interfaces/traits
- Dependencies to add
- Testing strategy

**Document (if complex):**
```markdown
## Architecture Design

### Components
- [Component 1]: [Purpose]
- [Component 2]: [Purpose]

### File Structure
```
src/
  new_feature/
    mod.rs
    component1.rs
    component2.rs
tests/
  new_feature_tests.rs
```

### Key Interfaces
```rust
trait MyTrait {
    fn do_something(&self) -> Result<T, E>;
}
```

### Dependencies
- `crate_name = "version"` (reason: [why needed])
```

### Step 2.4: Create Implementation Plan (10-15 mins)

**Use TodoWrite to break down work:**

```yaml
todos:
  - id: task-<N>-1
    content: "Create module structure"
    status: pending
    
  - id: task-<N>-2
    content: "Implement core logic"
    status: pending
    
  - id: task-<N>-3
    content: "Add unit tests"
    status: pending
    
  - id: task-<N>-4
    content: "Add integration tests"
    status: pending
    
  - id: task-<N>-5
    content: "Update documentation"
    status: pending
    
  - id: task-<N>-6
    content: "Manual smoke test"
    status: pending
```

### Step 2.5: Update Internal Tracking (2-3 mins)

```yaml
# Add to .agents/backlog/active_tasks.yml
- id: task-<issue-N>
  title: "[Issue title]"
  status: in_progress
  priority: high|medium|low
  assignee: <your-agent-name>
  created: YYYY-MM-DD
  due: YYYY-MM-DD  # Estimate
  github_issue: <N>
  dependencies: []
  notes: "Fresh claim, full implementation"
```

### Step 2.6: Create Feature Branch (1 min)

```bash
git checkout -b feature/<descriptor>-issue<N>
# OR
git checkout -b fix/<descriptor>-issue<N>
```

---

## Phase 3: Implementation

**Duration**: Varies by complexity  
**Goal**: Deliver high-quality, tested solution

### Step 3.1: Incremental Development

**Principles:**
- ✅ Small, focused commits
- ✅ Test as you go
- ✅ Run linters frequently
- ✅ Update TodoWrite status

**Commit Message Format:**
```bash
git commit -m "<type>(<scope>): <short description>

- Detail 1
- Detail 2

Relates to #<N>"
```

### Step 3.2: Testing Strategy

**At Each Stage:**
```bash
# Run tests
cargo test --all-features
# OR
npm test

# Run linters
cargo clippy -- -D warnings
# OR
npm run lint

# Format code
cargo fmt
# OR
npm run format
```

### Step 3.3: Progress Updates

**Update issue every major milestone:**
```bash
gh issue comment <N> --body "✅ Completed: [milestone]
⏳ In progress: [current work]
📝 Next: [next step]"
```

**Update active_tasks.yml:**
```yaml
notes: "Completed core logic, working on tests"
```

### Step 3.4: Documentation

**As You Build:**
- [ ] Update code comments (explain WHY, not WHAT)
- [ ] Add docstrings for public APIs
- [ ] Update README if behavior changes
- [ ] Add examples if needed
- [ ] Update CHANGELOG.md

---

## Phase 4: Completion & Handoff

**Duration**: 30-60 minutes  
**Goal**: Validate, document, and optionally hand off

### Step 4.1: Final Validation

**Quality Gate:**
```bash
# Run full CI locally
make ci-linux-local
# OR equivalent

# Capture logs
make ci-linux-local 2>&1 | tee logs/ci-local-$(date +%Y%m%d-%H%M%S).log
```

**Checklist (see [Quality Checklist](#quality-checklist)):**
- [ ] All tests pass
- [ ] All linters clean
- [ ] Manual smoke test done
- [ ] Documentation updated
- [ ] No regressions introduced
- [ ] Performance acceptable

### Step 4.2: Open Pull Request

```bash
# Push branch
git push -u origin feature/<descriptor>-issue<N>

# Create PR
gh pr create --title "[Feature] <Description>" --body "$(cat <<'EOF'
Fixes #<N>

## Summary
[What this PR does]

## Changes
- [Change 1]
- [Change 2]

## Testing
- [x] Unit tests pass
- [x] Integration tests pass
- [x] Manual smoke test performed

## Documentation
- [x] Code comments added
- [x] README updated (if needed)
- [x] CHANGELOG updated

## Logs
See logs/ci-local-<timestamp>.log

## Screenshots (if applicable)
[Attach if GUI/visual changes]
EOF
)"
```

### Step 4.3: Update Tracking

```yaml
# Move to completed_tasks.yml
- id: task-<issue-N>
  title: "[Issue title]"
  status: completed
  completed_date: YYYY-MM-DD
  github_issue: <N>
  pull_request: <PR-N>
  notes: "Full implementation complete"
```

### Step 4.4: Handoff (if needed)

**If another agent needs to continue (e.g., Gemini for docs):**

1. Create handoff document:
   ```bash
   cp .agents/workflows/templates/handoff_document.yml \
      .agents/backlog/handoff-$(date +%Y%m%d)-issue<N>.yml
   ```

2. Fill in handoff details (see template)

3. Tag receiving agent:
   ```bash
   gh issue comment <N> --body "[handoff: <agent-name>] Please [task description]"
   ```

### Step 4.5: Lessons Learned (optional but recommended)

**If you learned something valuable:**
```bash
cp .agents/workflows/templates/lesson_learned.md \
   .agents/lessons_learned/$(date +%Y-%m-%d)-<topic>.md
```

Document:
- What worked well
- What was challenging
- What would you do differently
- Tips for future similar work

---

## Quality Checklist

**Before marking task complete, verify:**

### Code Quality
- [ ] All tests pass (`cargo test --all-features`)
- [ ] Linters clean (`cargo clippy -- -D warnings`)
- [ ] Formatted (`cargo fmt -- --check`)
- [ ] No compiler warnings
- [ ] No TODOs/FIXMEs without issue tracking

### Testing
- [ ] Unit tests for new logic
- [ ] Integration tests for new flows
- [ ] Edge cases covered
- [ ] Error handling tested
- [ ] Manual smoke test performed

### Documentation
- [ ] Code comments explain WHY
- [ ] Public APIs documented
- [ ] README updated (if user-facing changes)
- [ ] CHANGELOG.md updated
- [ ] Examples added (if complex feature)

### Integration
- [ ] No breaking changes (or documented migration path)
- [ ] CI passes
- [ ] No regressions (existing tests still pass)
- [ ] Backward compatible (where applicable)
- [ ] Cross-platform tested (if applicable)

### Security
- [ ] No secrets/credentials in code
- [ ] Input validation added
- [ ] Error messages don't leak sensitive info
- [ ] Dependencies audited (`cargo audit`)

### Performance
- [ ] No obvious performance degradation
- [ ] Large inputs tested (if applicable)
- [ ] Memory leaks checked (if applicable)

---

## Troubleshooting

### If Stuck During Discovery
1. Comment on issue with specific questions
2. Tag human or other agent for input
3. Create brainstorming session
4. Update task status to "blocked" with reason

### If Scope Seems Too Large
1. Discuss scope reduction in issue
2. Propose phased approach (MVP → enhancements)
3. Create follow-up issues for future work
4. Get approval before proceeding

### If Architecture Uncertain
1. Create proof-of-concept (POC) branch
2. Test approach with small implementation
3. Share POC for feedback
4. Iterate based on input

### If Tests Failing
1. Don't hide failures (transparency!)
2. Debug systematically
3. Add more logging if needed
4. Ask for help if stuck >30 mins
5. Document RCA in lessons learned

---

## Examples

### Example 1: Fresh Issue #56 - GUI Golden Master Testing

**Discovery:**
- Analyzed existing test infrastructure
- Found smoke tests but no golden masters
- Identified need for snapshot comparison

**Design:**
- Extend `gui_cli_smoke.rs` with capture
- Use `serde_json` for serialization
- Store in `test-fixtures/golden/`

**Implementation:**
- 6 subtasks via TodoWrite
- Incremental commits
- Tests green at each step

**Result:**
- PR opened with full documentation
- CI passing
- Handoff to Gemini for user docs

---

## Related Documents
- Handoff workflow: `.agents/workflows/handoff_workflow.md`
- Agent registry: `.agents/agents_registry.md`
- Repository guidance: `.agents/AGENTS.md`
- Templates: `.agents/workflows/templates/`

---

**Version**: 1.0  
**Last Updated**: 2025-10-26  
**Maintained By**: Cursor (Claude 4.5 Sonnet)
