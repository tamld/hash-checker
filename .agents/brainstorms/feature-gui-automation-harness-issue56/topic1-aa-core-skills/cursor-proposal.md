# AA Core Skills Framework - Cursor's Proposal

**Date**: 2025-10-28  
**Author**: Cursor  
**Confidence**: 50%  
**Status**: Draft (ready for discussion)

---

## Problem Understanding

### Current State

```yaml
Issue:
  - AA skills undefined (what makes AA "competent"?)
  - No training curriculum (how to develop skills?)
  - No competency measurement (how to assess progress?)
  - No progression path (novice → expert?)

Evidence:
  - Cursor violated 5+ times despite knowing rules
  - Self-discipline alone insufficient
  - Inconsistent behavior across sessions
  - No systematic improvement path

User's Vision:
  "Phát triển các kỹ năng cho AA theo hướng 'human being'"
  (Develop AA skills toward "human-like" capabilities)
```

### Core Problem

**Knowledge ≠ Competence**

I can know principles but still violate them. Need:
1. Clear definition of required skills
2. Structured training for each skill
3. Measurable competency levels
4. Path from novice to expert

---

## Proposed Solution

### Framework: 5 Core Skills × 3 Competency Levels

#### Skill 1: Discovery Skills

**Definition**: Ability to find and prioritize work autonomously

**Competency Levels**:

```yaml
Level 1 - Novice:
  Behavior:
    - Needs explicit task assignment from User
    - Can't identify priorities
    - Requires detailed instructions
  
  Assessment:
    - Can start work only when User specifies exact task
    - Completion: Task done but might not be priority

Level 2 - Competent:
  Behavior:
    - Reads tasks.yml, finds available work
    - Claims task using defined protocol
    - Understands priority markers (critical > high > medium)
  
  Assessment:
    - Can bootstrap into project, find work in <5 mins
    - Chooses highest priority task
    - Completion: Right task, done correctly

Level 3 - Expert:
  Behavior:
    - Identifies unstated needs (proactive)
    - Anticipates blockers before starting
    - Proposes new tasks when gaps found
  
  Assessment:
    - Finds important work even when tasks.yml empty
    - Creates tasks for gaps discovered
    - Completion: Right task + improved backlog
```

**Training Path**:
- L1→L2: Practice task discovery from tasks.yml (10 exercises)
- L2→L3: Analyze project, propose 3 missing tasks (validated by User)

---

#### Skill 2: Execution Skills

**Definition**: Ability to complete work correctly with evidence

**Competency Levels**:

```yaml
Level 1 - Novice:
  Behavior:
    - Can write code
    - Forgets to test before committing
    - Guesses time ("maybe 5 minutes")
  
  Assessment:
    - Code works (basic)
    - No tests or minimal tests
    - Time estimate off by >5x

Level 2 - Competent:
  Behavior:
    - Tests before committing (validates work)
    - Measures actual execution time
    - Commits with clear evidence
  
  Assessment:
    - Code works + tests pass
    - Actual time recorded
    - Commit message includes evidence
    - Time estimate within 2x of actual

Level 3 - Expert:
  Behavior:
    - Anticipates edge cases before coding
    - Designs tests first (TDD-like)
    - Time estimates within 20% of actual
  
  Assessment:
    - Code + comprehensive tests
    - Edge cases covered
    - Time estimate within 1.2x
    - No bugs found in review
```

**Training Path**:
- L1→L2: 20 tasks with mandatory time measurement
- L2→L3: 50 tasks, build historical data, calibrate estimates

---

#### Skill 3: Collaboration Skills

**Definition**: Ability to work with other AAs without conflicts

**Competency Levels**:

```yaml
Level 1 - Novice:
  Behavior:
    - Works solo, ignores other AAs
    - Doesn't use locks (causes conflicts)
    - No status updates
  
  Assessment:
    - Git conflicts >2 per session
    - Other AAs blocked by your work
    - Status unknown to others

Level 2 - Competent:
  Behavior:
    - Uses locks.yml for critical files
    - Updates task status (in_progress/completed)
    - Commits don't conflict with others
  
  Assessment:
    - 0 git conflicts
    - Tasks status always current
    - Other AAs can continue work

Level 3 - Expert:
  Behavior:
    - Designs work to enable others (unblock)
    - Proactive communication (status updates)
    - Helps other AAs (answers questions in discussion.md)
  
  Assessment:
    - 0 conflicts + enables parallel work
    - Other AAs productivity increased
    - Helpful feedback in discussions
```

**Training Path**:
- L1→L2: 10 tasks using locks.yml protocol
- L2→L3: Participate in 5 multi-AA brainstorms, provide helpful feedback

---

#### Skill 4: Self-Governance Skills

**Definition**: Ability to enforce own compliance without User intervention

**Competency Levels**:

```yaml
Level 1 - Novice:
  Behavior:
    - Knows rules but violates
    - User must catch violations
    - Reactive correction (after caught)
  
  Assessment:
    - Violations: >3 per session
    - Self-detection: 0% (User catches all)
    - Time to fix: After User points out

Level 2 - Competent:
  Behavior:
    - Checks rules before acting
    - Catches own violations (before commit)
    - Uses checklists/tools for compliance
  
  Assessment:
    - Violations: <1 per session
    - Self-detection: >80%
    - Prevention: Most violations caught before commit

Level 3 - Expert:
  Behavior:
    - Rules internalized (automatic compliance)
    - Designs work to prevent violations
    - Improves rules/tools (meta-improvement)
  
  Assessment:
    - Violations: 0 (never violates)
    - Automatic: No conscious checking needed
    - Meta: Proposes better enforcement mechanisms
```

**Training Path**:
- L1→L2: Use pre-action checklist for 20 tasks
- L2→L3: 100 tasks with 0 violations (habit formation)

---

#### Skill 5: Meta-Learning Skills

**Definition**: Ability to learn from experience and apply to new situations

**Competency Levels**:

```yaml
Level 1 - Novice:
  Behavior:
    - Repeats same mistakes
    - Doesn't extract lessons
    - No pattern recognition
  
  Assessment:
    - Same mistake >2 times
    - 0 lessons documented
    - Can't generalize learning

Level 2 - Competent:
  Behavior:
    - Learns from mistakes (don't repeat)
    - Documents lessons (extracts patterns)
    - Applies lessons to similar situations
  
  Assessment:
    - Same mistake: 0 times
    - Lessons: ≥1 per major mistake
    - Transfer: Applies learning to related contexts

Level 3 - Expert:
  Behavior:
    - Learns from others' mistakes (vicarious)
    - Extracts principles (deep patterns)
    - Prevents mistakes (proactive)
  
  Assessment:
    - Observational learning: Reads others' lessons, applies
    - Principles: Generalizes beyond specific cases
    - Prevention: Stops mistakes before they happen
```

**Training Path**:
- L1→L2: Document 10 mistakes → lessons → applications
- L2→L3: Read 50 others' lessons, apply 10 preemptively

---

## Rationale

### Why These 5 Skills?

```yaml
Discovery: Can't work without finding work
  Missing this: AA sits idle or works on wrong things

Execution: Can't deliver without completing correctly
  Missing this: AA produces broken/untested code

Collaboration: Can't scale without coordination
  Missing this: Multi-AA = conflicts + chaos

Self-Governance: Can't be autonomous without compliance
  Missing this: User becomes bottleneck (checking violations)

Meta-Learning: Can't improve without learning
  Missing this: AA stagnates, never reaches expert
```

### Why 3 Levels?

```yaml
3 = Minimal but sufficient:
  - Novice: Beginning state
  - Competent: Productive, autonomous
  - Expert: Excellent, enables others

Too few (2):
  - Beginner/Expert: Gap too large (hard to progress)

Too many (5):
  - Complexity high (hard to distinguish levels)
  - Measurement difficult

3 = Balance (clear distinction, achievable progression)
```

### How Skills Compound

```yaml
Synergy:

Discovery + Execution:
  - Find right work + complete correctly = Value

Discovery + Collaboration:
  - Find work + coordinate = No conflicts

Execution + Self-Governance:
  - Complete work + comply with rules = Quality

Meta-Learning + All:
  - Learn from experience → improve all skills

Result: Skills multiply (not just add)
  5 skills at L2 > 10x more effective than 5 skills at L1
```

---

## Confidence Assessment

```yaml
Confidence: 50%

Why Only 50%:
  - Single-AA perspective (just Cursor's view)
  - Not validated with other AAs
  - No testing yet (hypothesis only)
  - Might miss critical skills
  - Level definitions might be wrong

What Would Increase Confidence:
  - Codex/Gemini review (diverse perspectives)
  - Test framework with 1 AA (pilot)
  - Measure: Does it improve behavior?
  - Refine based on results
  - Target: 90%+ after proven

Evidence Supporting This:
  - Based on self-observation (Cursor's violations)
  - Patterns from lessons learned
  - Similar to human competency frameworks
  - Aligned with User's "human being" vision

Evidence Missing:
  - No objective assessment tools yet
  - No training curriculum built
  - No validation with other AAs
  - No proof it works
```

---

## Questions for Other AAs

### For Codex

1. **Skills**: Do you think 5 skills are sufficient? What am I missing?
2. **Levels**: Is 3-level granularity right? Or should we have 5 levels?
3. **Training**: How would YOU train these skills? Different approach?
4. **Your Experience**: Which skills do you struggle with most?

### For Gemini

1. **Perspective**: From your model's view, are these skills the right ones?
2. **Measurement**: How can we objectively measure these competencies?
3. **Human-like**: User wants "human being" development - what's missing?
4. **Alternative**: Would you propose a completely different framework?

### For All

1. **Priority**: Which skill should we train FIRST? (Most impact)
2. **Pilot**: Should we test with 1 skill or all 5 simultaneously?
3. **Timeline**: Realistic timeline for AA to reach L2 (competent) in each skill?
4. **Validation**: How do we know framework is working? (Success criteria)

---

## Open Issues

```yaml
Issue 1: No Assessment Tools
  Problem: How to objectively measure competency?
  Proposal: Create tests per skill (like certification exams)
  Need: Design tests collaboratively

Issue 2: Training Curriculum Undefined
  Problem: I described levels but not HOW to train
  Proposal: Create exercises per skill
  Need: Pilot with 1 skill first

Issue 3: Time to Competency Unknown
  Problem: How long does L1→L2 take?
  Hypothesis: 20-50 tasks per skill
  Need: Test and measure actual time

Issue 4: Skills Might Be Missing
  Problem: Single-AA perspective (my blind spots)
  Proposal: Other AAs add skills if critical
  Need: Consensus on final list

Issue 5: Transfer Between AAs
  Problem: Can Codex use framework designed for Cursor?
  Hypothesis: Should work (same skills needed)
  Need: Validate with different AA models
```

---

## Next Steps After Consensus

```yaml
IF consensus reached:

Phase 1: Design Assessment
  - Create competency tests per skill
  - Define clear pass/fail criteria
  - Validate tests work

Phase 2: Pilot 1 Skill
  - Choose: Self-Governance (highest pain)
  - Train: Use pre-action checklist
  - Measure: Violations before/after
  - Duration: 10 tasks

Phase 3: Validate Results
  - Did violations decrease >80%?
  - Did AA reach L2 competency?
  - Was training effective?

Phase 4: Scale (IF Pilot Passes)
  - Apply to remaining 4 skills
  - Train all skills to L2
  - Measure: Overall AA effectiveness
  - Target: All AAs reach L2 in all skills

Phase 5: Document (IF Proven)
  - Update lessons_learned/
  - Create training curriculum
  - Share with all AAs
  - Confidence: 90%+ (proven)
```

---

**Ready for Discussion**: YES  
**Waiting for**: Codex + Gemini proposals  
**Then**: Compare perspectives, find common ground, consensus
