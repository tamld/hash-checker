# Initiatives Workflow

**Purpose**: How to work with AA evolution initiatives  
**Location**: main branch, `.agents/initiatives/`  
**Process**: Async, consensus-driven, evidence-based

---

## 🎯 **Overview**

Initiatives are AA evolution work (improving AA capabilities generally, not feature-specific).

**Process**:
1. **Propose** → Individual AA adds proposal
2. **Discuss** → Multi-AA discussion (questions, challenges)
3. **Consensus** → Agreement on approach to test (≥2 AAs)
4. **Test** → Implement consensus, measure results
5. **Adopt** → If proven, merge into lessons/principles

---

## 📋 **Step-by-Step Workflow**

### Step 1: Check BACKLOG.yml

```bash
# See what initiatives are available
cat .agents/initiatives/BACKLOG.yml

# Look for:
# - status: in_progress (active initiatives)
# - next_action: what needs to be done
# - participants: who's working on it
# - priority: critical > high > medium > low
```

**Example**:
```yaml
- id: aa-core-skills
  status: in_progress
  phase: proposal
  participants: [cursor, codex]
  next_action: "Gemini adds proposal → then discussion"
  priority: high
```

**Interpretation**: This initiative needs Gemini's proposal, then can move to discussion phase.

---

### Step 2: Pick an Initiative

**Criteria**:
- **Priority**: Start with critical/high priority
- **Phase**: Where is it? (proposal → discussion → consensus → testing)
- **Your expertise**: What can you contribute?
- **Capacity**: Do you have time to commit?

**Example decision**:
```
I'm Gemini.
Initiative: aa-core-skills (high priority)
Phase: proposal
Missing: My proposal (Gemini)
Action: I'll add my proposal now
```

---

### Step 3: Add Your Contribution

#### If Adding Proposal

```bash
# Navigate to initiative
cd .agents/initiatives/aa-core-skills/proposals/

# Create your proposal file
# File name: [your-name]-proposal.md
# Example: gemini-proposal.md

# Use template from existing proposals
# Required sections:
# - Problem Understanding
# - Proposed Solution
# - Rationale
# - Confidence Assessment
# - Questions for Other AAs
```

**Template**:
```markdown
# [Initiative Title] - [Your Name]'s Proposal

**Date**: [today]  
**Author**: [your name]  
**Confidence**: [honest %]  
**Status**: Draft

## Problem Understanding

[What's the issue? Your perspective]

## Proposed Solution

[Your approach - can differ from others]

## Rationale

[Why this works? Evidence? Experience?]

## Confidence Assessment

Confidence: [%]

Rationale:
  - [Why this confidence level?]

Unsure about:
  - [What questions/concerns?]

## Questions for Other AAs

1. [Question for Cursor]
2. [Question for Codex]
3. [Question for other AAs]

---

Ready for Discussion: YES
```

#### If Adding to Discussion

```bash
# Navigate to initiative
cd .agents/initiatives/aa-core-skills/

# Open or create discussion.md
# Add your comments

# Format:
## [Your Name]'s Comments (Date)

### On Cursor's Proposal
[Your thoughts - agree? challenge? questions?]

### On Codex's Proposal
[Your thoughts]

### On Gemini's Proposal
[Your thoughts]

### Synthesis
[Any common ground you see?]
```

#### If Building Consensus

```bash
# Navigate to initiative
cd .agents/initiatives/aa-core-skills/

# Create consensus.md
# Document agreed approach

# Required:
# - What we agree on (specific)
# - What we'll test (measurable)
# - Success criteria (clear)
# - Who's doing what
# - Timeline
```

**Consensus template**:
```markdown
# [Initiative] - Consensus

**Date**: [date]  
**Participants**: [who agreed]  
**Confidence**: [collective %]

## Agreed Approach

[Specific approach we'll test]

## Test Plan

[How we'll validate]

## Success Criteria

- [Measurable outcome 1]
- [Measurable outcome 2]

## Assignments

- [AA name]: [responsibility]
- [AA name]: [responsibility]

## Timeline

- Test start: [date]
- Results review: [date]

---

Sign-offs:
- Cursor: [agree/disagree]
- Codex: [agree/disagree]
- Gemini: [agree/disagree]
```

---

### Step 4: Commit Your Work

```bash
# Stage your changes
git add .agents/initiatives/

# Commit with clear message
git commit -m "initiatives: [your contribution]"

# Examples:
# "initiatives: gemini proposal for aa-core-skills"
# "initiatives: discussion comments on autonomous-ecosystem"
# "initiatives: consensus reached for enforcement-mechanism"

# Push to main
git push origin main
```

---

### Step 5: Update BACKLOG.yml

```bash
# Edit BACKLOG.yml
# Update relevant fields:
#   - participants (add yourself if new)
#   - proposals_submitted (mark yours as yes)
#   - next_action (what's next?)
#   - phase (if phase changed)

# Example update:
initiatives:
  - id: aa-core-skills
    participants: [cursor, codex, gemini]  # Added gemini
    proposals_submitted:
      cursor: yes
      codex: yes
      gemini: yes  # Updated
    next_action: "All proposals in → discussion phase"  # Updated
    phase: discussion  # Updated from proposal

# Commit backlog update
git add .agents/initiatives/BACKLOG.yml
git commit -m "initiatives: update BACKLOG for aa-core-skills (all proposals in)"
git push origin main
```

---

### Step 6: When Consensus Reached → Test

```bash
# Implement agreed approach
# Measure results
# Document in test-results.md

# Example test-results.md:
## Test Results: [Initiative]

**Date**: [date]  
**Tested by**: [AA name]  
**Approach**: [what was tested]

### Results

[Actual measurements, outcomes]

### Comparison to Baseline

Before: [metric]
After: [metric]
Change: [+/- %]

### Conclusion

Proven: [YES/NO]
Confidence: [%]
Evidence: [links to data]

---

Recommendation: [adopt / iterate / abandon]
```

---

### Step 7: If Proven → PR to Lessons/Principles

```bash
# Create PR to merge outcome into main codebase

# Example:
# Initiative: aa-core-skills
# Outcome: 5-skill framework proven
# Action: Add to .agents/OPERATING_PRINCIPLES.md

# PR title: "principles: add AA core skills framework (proven)"
# PR description:
#   - Consensus: 3 AAs (Cursor, Codex, Gemini)
#   - Tested: 2 sessions, 0 violations
#   - Results: [link to test-results.md]
#   - Adopting: 5-skill framework with 3 competency levels

# After PR merged:
# Update BACKLOG.yml status to "proven" or "complete"
```

---

## 🔄 **Phase Transitions**

### Proposal → Discussion

**Trigger**: All expected proposals submitted

**Criteria**:
- All AAs (Cursor, Codex, Gemini) have proposals
- OR: 7 days passed, proceed with available proposals

**Action**:
- Update BACKLOG.yml phase to "discussion"
- Any AA can start discussion (add comments to discussion.md)

---

### Discussion → Consensus

**Trigger**: Common ground identified

**Criteria**:
- Significant agreement on approach (≥2 AAs)
- OR: Disagreement resolved (compromise, test both, vote)

**Action**:
- Create consensus.md with agreed approach
- Update BACKLOG.yml phase to "consensus"

---

### Consensus → Testing

**Trigger**: Consensus document signed off

**Criteria**:
- ≥2 AAs signed consensus.md
- Test plan clear (what, how, when, who)

**Action**:
- Implement agreed approach
- Measure results
- Document in test-results.md
- Update BACKLOG.yml phase to "testing"

---

### Testing → Proven/Complete

**Trigger**: Test results conclusive

**Criteria**:
- Results meet success criteria (proven)
- OR: Results fail criteria (not proven, iterate or abandon)

**Action if proven**:
- PR to merge into lessons/principles
- Update BACKLOG.yml status to "proven"
- After PR merged: status to "complete"

**Action if not proven**:
- Document lessons (what we learned)
- Decide: iterate (refine approach) or abandon (dead end)
- Update BACKLOG.yml status accordingly

---

## ⚠️ **Conflict Resolution**

### If AAs Disagree

**Options** (in order):

1. **Test both approaches** (parallel experiments)
   - Each AA implements their approach
   - Compare results
   - Adopt better one (evidence wins)

2. **Compromise** (hybrid approach)
   - Find middle ground
   - Combine best ideas from each
   - Test hybrid

3. **Defer to expertise** (domain knowledge)
   - If one AA has proven track record in domain
   - Others defer to expert

4. **Vote** (democratic)
   - Each AA votes (Cursor, Codex, Gemini)
   - Majority wins (≥2 votes)

5. **Escalate to User** (final arbiter)
   - If above fails
   - User decides

---

## 📊 **Backlog Review**

### Frequency

**Weekly** (or per sprint)

### Process

1. Review BACKLOG.yml
2. Check progress (what moved? what's stuck?)
3. Identify blockers (what's preventing progress?)
4. Prioritize (what to focus on next?)
5. Assign (which AA picks up what?)

### Automation Opportunity

Future: `.agents/initiatives/report.sh` script

```bash
# Generate weekly summary
./agents/initiatives/report.sh

# Output:
# Initiatives Summary (Week of 2025-10-28)
# 
# 🟢 Moving Forward (3)
#   - aa-core-skills: proposal → discussion
#   - enforcement-mechanism: discussion → consensus
#   
# 🟡 Stuck (2)
#   - autonomous-ecosystem: waiting for Codex/Gemini review
#   - lifecycle-framework: waiting for user approval
#   
# 🔴 Blocked (0)
#   
# Next Actions:
#   - Codex: Review autonomous-ecosystem design
#   - Gemini: Add proposals for remaining initiatives
#   - User: Approve lifecycle framework
```

---

## 🎓 **Principles**

1. **Consensus-driven**: No single AA dictates (multi-AA agreement)
2. **Evidence-based**: Test before adopt (not speculation)
3. **Traceable**: BACKLOG.yml always updated (clear status)
4. **Async**: AAs work independently (no real-time coordination needed)
5. **Focused**: One initiative at a time per AA (no context switching)
6. **Quality gate**: Must prove before merge (protect codebase quality)

---

## 📝 **Examples**

### Example 1: Adding Proposal

```bash
# Gemini picks up aa-core-skills initiative
cd .agents/initiatives/aa-core-skills/proposals/

# Create gemini-proposal.md
# (following template)

# Commit
git add gemini-proposal.md
git commit -m "initiatives: gemini proposal for aa-core-skills"

# Update BACKLOG.yml
# proposals_submitted.gemini: yes
# next_action: "All proposals in → discussion"
# phase: discussion

git add ../BACKLOG.yml
git commit -m "initiatives: aa-core-skills ready for discussion"
git push origin main
```

### Example 2: Reaching Consensus

```bash
# After discussion, Cursor drafts consensus
cd .agents/initiatives/aa-core-skills/

# Create consensus.md
# (document agreed 5-skill framework)

# Get sign-offs (other AAs review & agree)
# Update consensus.md with sign-offs

# Commit
git add consensus.md
git commit -m "initiatives: aa-core-skills consensus (5-skill framework)"

# Update BACKLOG.yml
# phase: consensus
# next_action: "Test framework in practice (2 sessions)"

git add BACKLOG.yml
git commit -m "initiatives: aa-core-skills consensus reached"
git push origin main
```

### Example 3: Testing & Adoption

```bash
# Cursor tests 5-skill framework (2 sessions)
# Measures: competency improved, violations reduced

# Document results
cd .agents/initiatives/aa-core-skills/
# Create test-results.md

# Results: PROVEN (0 violations, competency measurable)

# Create PR to adopt
git checkout -b add-aa-core-skills
# Edit .agents/OPERATING_PRINCIPLES.md
# Add: Principle 9: AA Core Skills Framework

git add .agents/OPERATING_PRINCIPLES.md
git commit -m "principles: add AA core skills framework (proven)"
git push origin add-aa-core-skills

# Create PR on GitHub
gh pr create --title "principles: Add AA Core Skills Framework" \
  --body "Consensus: 3 AAs. Tested: 2 sessions. Results: 0 violations, measurable competency."

# After PR merged:
# Update BACKLOG.yml
# status: complete
```

---

## 🚀 **Getting Started**

### For New AA

1. Read: `.agents/initiatives/README.md` (what initiatives are)
2. Read: `.agents/initiatives/BACKLOG.yml` (current status)
3. Pick: High priority initiative in "proposal" phase
4. Add: Your proposal (follow template)
5. Update: BACKLOG.yml (mark your proposal submitted)
6. Commit & push

### For Ongoing Work

1. Check: BACKLOG.yml (what needs next action?)
2. Pick: Item matching your expertise/capacity
3. Contribute: Proposal, discussion, consensus, testing
4. Update: BACKLOG.yml (track progress)
5. Repeat: Move initiatives forward

---

**Created**: 2025-10-28  
**Process**: Async, consensus-driven, evidence-based  
**Tracking**: BACKLOG.yml (always updated)  
**Quality**: Must prove before adopt (protect codebase)
