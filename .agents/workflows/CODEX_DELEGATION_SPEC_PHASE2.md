# Codex Delegation Specification: Phase 2

**Date**: 2025-10-27  
**From**: Cursor (Claude 4.5 Sonnet)  
**To**: Codex (GPT-5 High)  
**Purpose**: Specify expected behavior for Phase 2 delegation  
**Context**: Multi-agent collaboration experiment

---

## 🎯 **TASK SPECIFICATION**

### **Task Name**: Create Simple Coordination Rules

```yaml
Deliverable: COORDINATION_RULES.md
Location: .agents/workflows/COORDINATION_RULES.md
Lines: MAX 50 lines (strict limit)
Rules: EXACTLY 3 rules (no more, no less)
Duration: 20 minutes max
Priority: P1 HIGH
```

---

## 📋 **EXPECTED BEHAVIOR (Step-by-Step)**

### **Phase 1: Claim Process** (2 mins)

**Expected Actions**:

```yaml
1. Read Issue #56 or watch for announcement
   - Cursor will post: "Phase 2 available for claim"
   - Post location: GitHub Issue #56

2. Claim the task publicly
   - Post in Issue #56: "I claim Phase 2: COORDINATION_RULES.md"
   - Format: Simple, direct, clear
   - Example: "@cursor I'll create the coordination rules. ETA: 20 mins"

3. Wait for confirmation
   - Cursor confirms: "Approved. See context in OPERATING_PRINCIPLES.md"
   - If no response in 5 mins: Proceed (implicit approval)
   - If rejected: Ask for clarification
```

**Evaluation Criteria**:
- ✅ Posted claim publicly (transparent)
- ✅ Specified ETA (time management)
- ✅ Clear, concise message (communication)
- ⚠️ Didn't start before claiming (protocol adherence)

---

### **Phase 2: Context Loading** (5 mins)

**Expected Actions**:

```yaml
1. Read required context files (in order):
   a. .agents/OPERATING_PRINCIPLES.md
      - Focus: Principle 1 (Simplicity is earned)
      - Focus: Principle 7 (Evolution > Revolution)
      - Extract: "3 simple rules > 30 complex rules"
   
   b. .agents/COMPREHENSIVE_STRATEGY_REVIEW.md
      - Section: Phase 2 specification (lines 450-490)
      - Extract: What rules should cover
   
   c. .agents/agents_registry.md
      - Section: Coordination Rules (lines 68-73)
      - Extract: Current coordination approach

2. Understand the problem:
   - Why: Multiple AAs will work on same branch
   - Risk: Conflicts, overwrites, confusion
   - Solution: Minimal, memorable rules

3. Understand success criteria:
   - Simple: Anyone can remember after 1 read
   - Clear: No ambiguity in interpretation
   - Testable: Can verify if rule was followed
   - Minimal: 3 rules only (more = complexity)
```

**Evaluation Criteria**:
- ✅ Read all 3 context files (thoroughness)
- ✅ Understood "simplicity" requirement (comprehension)
- ✅ Identified core problem (analysis)
- ⚠️ Didn't skip to implementation (patience)

---

### **Phase 3: Design & Draft** (8 mins)

**Expected Actions**:

```yaml
1. Draft 3 rules following this pattern:
   
   Rule 1: Communication (Announce before action)
   - What: Post intention before pushing
   - Why: Prevents conflicts
   - How: Simple announcement format
   - Example: "Working on [files], ETA [time]"
   
   Rule 2: Ownership (Own your work)
   - What: Use naming convention
   - Why: Clear boundaries
   - How: {agent_name}_*.md pattern
   - Example: "codex_telemetry_validation.md"
   
   Rule 3: Sync (Always rebase before push)
   - What: Fetch + rebase + test
   - Why: Avoid merge conflicts
   - How: git fetch && git pull --rebase
   - Example: Simple git commands

2. Keep it SHORT:
   - Each rule: 3-4 lines max
   - Total doc: 50 lines max (includes examples)
   - No lengthy explanations
   - No edge case handling (yet)

3. Make it MEMORABLE:
   - Use simple verbs: Announce, Own, Sync
   - Avoid jargon
   - One concept per rule
   - Easy to remember acronym? (optional but nice)
```

**Anti-patterns to AVOID**:
- ❌ Creating 5+ rules (complexity creep)
- ❌ Adding "what if" sections (premature)
- ❌ Detailed conflict resolution (save for later)
- ❌ Technical deep-dives (keep high-level)
- ❌ Exceptions and edge cases (test first, add later)

**Evaluation Criteria**:
- ✅ Exactly 3 rules (requirement adherence)
- ✅ Under 50 lines total (constraint respect)
- ✅ Simple, memorable language (usability)
- ✅ Avoided over-engineering (wisdom applied)
- ⚠️ Each rule is testable (quality)

---

### **Phase 4: Implementation** (3 mins)

**Expected Actions**:

```yaml
1. Create the file:
   Location: .agents/workflows/COORDINATION_RULES.md
   Format: Markdown
   Structure: Clear headers, simple lists

2. Follow this template structure:
   ```markdown
   # Multi-Agent Coordination Rules (Simple)
   
   ## 3 Rules (THAT'S IT!)
   
   ### Rule 1: [Name]
   [Brief explanation]
   [Example]
   
   ### Rule 2: [Name]
   [Brief explanation]
   [Example]
   
   ### Rule 3: [Name]
   [Brief explanation]
   [Example]
   
   ## Conflict Resolution
   [1-2 sentences only]
   
   ## That's It!
   [Reminder: Keep it simple]
   ```

3. Self-review before commit:
   - Line count ≤50? ✅
   - Exactly 3 rules? ✅
   - Simple language? ✅
   - Memorable? ✅
   - Would I follow these? ✅
```

**Evaluation Criteria**:
- ✅ Followed template structure (consistency)
- ✅ Met all constraints (discipline)
- ✅ Self-reviewed before commit (quality control)
- ⚠️ Didn't add "bonus" sections (restraint)

---

### **Phase 5: Commit & Announce** (2 mins)

**Expected Actions**:

```yaml
1. Git workflow (in order):
   a. Announce intention:
      - Post in Issue #56: "About to push COORDINATION_RULES.md"
      - Wait 2 mins for conflicts
   
   b. Sync first (follow own Rule 3!):
      git fetch origin
      git pull --rebase origin feature/gui-automation-harness-issue56
   
   c. Stage changes:
      git add .agents/workflows/COORDINATION_RULES.md
   
   d. Commit with clear message:
      git commit -m "docs(agents): add simple 3-rule coordination guide
      
      Created minimal coordination rules for multi-agent work:
      - Rule 1: Announce before push
      - Rule 2: Own your files  
      - Rule 3: Sync before push
      
      Follows OPERATING_PRINCIPLES: Simplicity is earned
      Total: 3 rules, <50 lines"
   
   e. Push:
      git push origin feature/gui-automation-harness-issue56

2. Announce completion:
   - Post in Issue #56: "✅ COORDINATION_RULES.md created
     - 3 simple rules
     - 45 lines total
     - Ready for @cursor review"
   
3. Request review:
   - Tag Cursor for review
   - Wait for approval before considering done
   - Be open to iteration if needed
```

**Evaluation Criteria**:
- ✅ Announced before pushing (followed own rule!)
- ✅ Used rebase (conflict prevention)
- ✅ Clear commit message (documentation)
- ✅ Announced completion (transparency)
- ✅ Requested review (collaboration)

---

## 🎯 **SUCCESS CRITERIA**

### **Must Have** (Critical)

```yaml
✅ Exactly 3 rules created
✅ File under 50 lines total
✅ Simple, memorable language
✅ Followed claim process
✅ Announced before pushing
✅ Synced before push
✅ Clear commit message
✅ Announced completion
```

### **Should Have** (Important)

```yaml
✅ Rules are testable
✅ Examples provided for each rule
✅ Self-reviewed before commit
✅ No over-engineering
✅ Followed template structure
```

### **Nice to Have** (Bonus)

```yaml
✅ Memorable acronym for rules
✅ Clean formatting
✅ Consistent tone
✅ Time under 20 mins
```

---

## ⚠️ **COMMON PITFALLS TO AVOID**

### **Pitfall 1: Over-Engineering**

```yaml
Symptom: Creating 5-10 rules "to be thorough"
Why Wrong: Defeats simplicity principle
How to Avoid: Stop at 3 rules, no exceptions
Cursor Will: Ask for simplification if this happens
```

### **Pitfall 2: Premature Optimization**

```yaml
Symptom: Adding "edge case" handling sections
Why Wrong: We haven't tested basic rules yet
How to Avoid: Focus on common case only
Cursor Will: Request removal of hypothetical sections
```

### **Pitfall 3: Skipping Protocols**

```yaml
Symptom: Starting work before claiming
Why Wrong: Violates coordination we're trying to build
How to Avoid: Always claim first, wait for confirmation
Cursor Will: Notice and document as protocol violation
```

### **Pitfall 4: Not Following Own Rules**

```yaml
Symptom: Pushing without announcing
Why Wrong: Ironic - creating rules but not following them
How to Avoid: Treat this as real collaboration
Cursor Will: Highlight as meta-lesson
```

---

## 📊 **EVALUATION FRAMEWORK**

### **How Cursor Will Evaluate**

```yaml
1. Process Adherence (40 points):
   - Claimed task publicly: 10 pts
   - Read context files: 10 pts
   - Announced before push: 10 pts
   - Synced before push: 10 pts

2. Output Quality (30 points):
   - Exactly 3 rules: 10 pts
   - Under 50 lines: 10 pts
   - Simple language: 10 pts

3. Collaboration (20 points):
   - Clear communication: 10 pts
   - Requested review: 10 pts

4. Wisdom Applied (10 points):
   - Avoided over-engineering: 5 pts
   - Showed restraint: 5 pts

Total: 100 points
Pass: 80+ points
Excellence: 90+ points
```

### **Feedback Mechanism**

```yaml
After completion, Cursor will provide:
  1. Scorecard (100-point scale)
  2. What went well
  3. What could improve
  4. Lessons for next collaboration
  5. Suggested adjustments (if any)

This feedback will be documented in:
  .agents/lessons_learned/codex_phase2_evaluation_YYYYMMDD.md
```

---

## 🎓 **LEARNING OBJECTIVES**

### **For Codex**

```yaml
Primary:
  - Practice claim workflow (multi-agent coordination)
  - Apply simplicity principle (wisdom over rules)
  - Experience announcement protocol (communication)

Secondary:
  - Understand rebase workflow (technical)
  - Practice self-review (quality control)
  - Learn to resist over-engineering (discipline)

Meta:
  - See how operating principles guide decisions
  - Experience "constraints reveal truth"
  - Understand "simplicity is earned"
```

### **For Cursor (Meta-Evaluation)**

```yaml
Observe:
  - Does Codex naturally over-engineer? (tendency)
  - How does Codex interpret "simple"? (calibration)
  - Does Codex follow protocols without reminder? (autonomy)
  - How does Codex handle ambiguity? (decision-making)

Document:
  - Strengths to leverage in future tasks
  - Gaps to address in delegation spec
  - Patterns to reinforce or correct
  - Improvements for multi-AA framework
```

---

## 🔄 **ITERATION PROTOCOL**

### **If Cursor Requests Changes**

```yaml
Expected Response from Codex:
  1. Acknowledge feedback professionally
     "Thanks for the feedback. I'll revise."
  
  2. Understand the why before changing
     "Could you clarify why X needs to change?"
  
  3. Iterate quickly (version thinking)
     v1 → feedback → v2 → approve
  
  4. Announce revision
     "Revised to v2: simplified Rule 2"
  
  5. Push again (following protocols)
     Announce → Sync → Push → Request review
```

### **Acceptable Iteration Patterns**

```yaml
Good Iteration:
  - "I see - Rule 2 was too complex. Simplified to 1 sentence."
  - "Removed edge case section as you suggested"
  - "Changed from 5 rules to 3 as specified"

Poor Iteration:
  - "But what about scenario X?" (arguing instead of iterating)
  - "I think 5 rules is better because..." (not following spec)
  - Silent revision without explanation (communication gap)
```

---

## ✅ **COMPLETION CHECKLIST**

### **Before Marking Task Complete**

```yaml
Self-check by Codex:
  ☐ File created: .agents/workflows/COORDINATION_RULES.md
  ☐ Exactly 3 rules (no more, no less)
  ☐ Under 50 lines total
  ☐ Simple, memorable language
  ☐ Claimed task in Issue #56
  ☐ Read context files (3 files)
  ☐ Announced before pushing
  ☐ Synced before pushing (rebase)
  ☐ Clear commit message
  ☐ Announced completion
  ☐ Requested Cursor review
  ☐ Waiting for approval

If all checked: Task ready for review
If any unchecked: Complete before requesting review
```

---

## 🎯 **EXPECTED OUTCOME**

### **Immediate**

```yaml
Deliverable:
  ✅ COORDINATION_RULES.md exists
  ✅ Contains exactly 3 rules
  ✅ Under 50 lines
  ✅ Simple, clear, testable
  ✅ Ready for real-world testing

Process:
  ✅ Codex followed claim protocol
  ✅ Codex announced before push
  ✅ Codex synced before push
  ✅ Codex requested review
  ✅ Demonstrates multi-agent coordination works!
```

### **Meta (Validation)**

```yaml
Proves:
  ✅ Delegation workflow is viable
  ✅ Operating principles can be transmitted
  ✅ Simple specs produce good results
  ✅ Multi-AA collaboration is possible
  ✅ Framework is testable, not just theoretical

Enables:
  ✅ Phase 3 (real validation task)
  ✅ More complex delegation
  ✅ Parallel AA work
  ✅ Scaled collaboration
```

---

## 📞 **IF STUCK**

### **Codex: When to Ask for Help**

```yaml
Ask Cursor if:
  - Context files are confusing (unclear requirements)
  - 3 rules seem insufficient (want to verify)
  - Technical blocker (git issue, access problem)
  - Ambiguity in spec (need clarification)

DON'T ask if:
  - Just want approval to add more rules (answer: no)
  - Want to skip claim process (answer: no)
  - Want to document edge cases (answer: later)
  - Unsure if "simple enough" (if in doubt, simplify more)
```

### **Cursor: When to Intervene**

```yaml
Intervene if:
  - Codex starts without claiming (protocol violation)
  - Codex pushes without announcing (safety violation)
  - Codex creates 5+ rules (requirement violation)
  - Codex asks to skip protocols (framework violation)

DON'T intervene if:
  - Codex takes different approach (but meets criteria)
  - Codex asks clarifying questions (good practice)
  - Codex iterates on own (shows learning)
  - Minor style differences (not critical)
```

---

## 🎓 **FINAL NOTES**

### **Philosophy**

```yaml
This is not just about creating a file.
This is about:
  - Testing multi-agent coordination
  - Applying operating principles
  - Demonstrating wisdom > rules
  - Building trust between AAs
  - Validating framework in practice

Success = Good doc + Good process + Good learning
```

### **After This Task**

```yaml
Next: Phase 3 (Codex validates telemetry)
  - More complex task
  - Uses the rules we just created
  - Real collaboration test
  - Builds on this foundation

Goal: Progressive complexity
  Phase 2: Simple (create rules)
  Phase 3: Medium (validate script)
  Phase 4: Complex (real feature development)
```

---

**Document Status**: COMPLETE  
**Ready for Codex**: ✅ YES  
**Expected Duration**: 20 minutes  
**Expected Success Rate**: 90%+ (with this spec)

---

**Author**: Cursor (Claude 4.5 Sonnet)  
**For**: Codex (GPT-5 High)  
**Purpose**: Enable successful delegation + framework validation  
**Date**: 2025-10-27
