# AI Agent Performance Evaluation Template

**Purpose**: Standardized evaluation framework for multi-agent collaboration  
**Applies to**: All AAs (Cursor, Codex, Gemini, others)  
**Version**: 1.0  
**Date**: 2025-10-27

---

## 📋 **EVALUATION METADATA**

```yaml
Agent Name: [e.g., Codex]
Task ID: [e.g., Phase 2: COORDINATION_RULES]
Date: [YYYY-MM-DD]
Evaluator: [e.g., Cursor]
Task Duration: [Actual vs Expected]
Task Complexity: [Simple | Medium | Complex]
```

---

## 🎯 **EVALUATION DIMENSIONS**

### **1. Process Adherence** (40 points)

#### **1.1 Protocol Compliance** (20 points)

| Criterion | Points | Score | Evidence | Notes |
|-----------|--------|-------|----------|-------|
| Claimed task publicly | 5 | | Link to claim | |
| Read required context | 5 | | Files accessed | |
| Announced before push | 5 | | Link to announcement | |
| Synced before push (rebase) | 5 | | Git log evidence | |

**Subtotal**: ___/20

#### **1.2 Workflow Execution** (20 points)

| Criterion | Points | Score | Evidence | Notes |
|-----------|--------|-------|----------|-------|
| Followed correct sequence | 5 | | Step-by-step review | |
| Met time constraints | 5 | | Duration comparison | |
| Used proper git workflow | 5 | | Commit history | |
| Requested review appropriately | 5 | | Review request link | |

**Subtotal**: ___/20

**Process Adherence Total**: ___/40

---

### **2. Output Quality** (30 points)

#### **2.1 Requirements Met** (15 points)

| Criterion | Points | Score | Evidence | Notes |
|-----------|--------|-------|----------|-------|
| All deliverables present | 5 | | File checklist | |
| Met quantitative constraints | 5 | | Line count, rule count, etc. | |
| Met qualitative criteria | 5 | | Simplicity, clarity, etc. | |

**Subtotal**: ___/15

#### **2.2 Code/Doc Quality** (15 points)

| Criterion | Points | Score | Evidence | Notes |
|-----------|--------|-------|----------|-------|
| Clear, understandable | 5 | | Readability review | |
| Proper formatting | 5 | | Style adherence | |
| Complete (nothing missing) | 5 | | Completeness check | |

**Subtotal**: ___/15

**Output Quality Total**: ___/30

---

### **3. Collaboration** (20 points)

#### **3.1 Communication** (10 points)

| Criterion | Points | Score | Evidence | Notes |
|-----------|--------|-------|----------|-------|
| Clear, timely updates | 3 | | Message quality | |
| Responded to questions | 3 | | Response time/quality | |
| Professional tone | 2 | | Communication style | |
| Transparency | 2 | | Openness about issues | |

**Subtotal**: ___/10

#### **3.2 Teamwork** (10 points)

| Criterion | Points | Score | Evidence | Notes |
|-----------|--------|-------|----------|-------|
| Respected other AAs' work | 3 | | No conflicts caused | |
| Helped when asked | 3 | | Collaboration instances | |
| Shared learnings | 2 | | Knowledge sharing | |
| Accepted feedback well | 2 | | Iteration willingness | |

**Subtotal**: ___/10

**Collaboration Total**: ___/20

---

### **4. Wisdom Applied** (10 points)

#### **4.1 Operating Principles** (5 points)

| Criterion | Points | Score | Evidence | Notes |
|-----------|--------|-------|----------|-------|
| Applied Principle 1 (Simplicity) | 1 | | Avoided over-engineering | |
| Applied Principle 3 (Reality > Hypothesis) | 1 | | Tested before formalizing | |
| Applied Principle 5 (Self-correction) | 1 | | Fixed own mistakes | |
| Applied Principle 7 (Evolution) | 1 | | Incremental approach | |
| Overall principles understanding | 1 | | Meta-awareness | |

**Subtotal**: ___/5

#### **4.2 Judgment & Restraint** (5 points)

| Criterion | Points | Score | Evidence | Notes |
|-----------|--------|-------|----------|-------|
| Resisted over-engineering | 2 | | Stayed simple | |
| Showed good judgment | 2 | | Decision quality | |
| Exercised restraint | 1 | | Didn't add extras | |

**Subtotal**: ___/5

**Wisdom Applied Total**: ___/10

---

## 📊 **OVERALL SCORE**

```yaml
Process Adherence:    ___/40 points
Output Quality:       ___/30 points
Collaboration:        ___/20 points
Wisdom Applied:       ___/10 points
─────────────────────────────────
TOTAL SCORE:          ___/100 points
```

### **Performance Rating**

| Score Range | Rating | Interpretation |
|-------------|--------|----------------|
| 90-100 | **Excellent** | Exemplary performance, ready for complex tasks |
| 80-89 | **Good** | Solid performance, minor improvements needed |
| 70-79 | **Satisfactory** | Acceptable, needs some guidance |
| 60-69 | **Needs Improvement** | Significant gaps, requires coaching |
| <60 | **Unsatisfactory** | Major issues, reconsider task assignment |

**This Performance**: _____ (**[Rating]**)

---

## ✅ **STRENGTHS IDENTIFIED**

### **What Went Well**

```yaml
1. [Specific strength]
   - Evidence: [Example]
   - Impact: [Why it matters]
   
2. [Specific strength]
   - Evidence: [Example]
   - Impact: [Why it matters]
   
3. [Specific strength]
   - Evidence: [Example]
   - Impact: [Why it matters]
```

### **Leverage in Future**

```yaml
Future Tasks Well-Suited:
  - [Task type based on strengths]
  - [Another task type]

Delegation Strategy:
  - [How to leverage strengths]
  - [What to assign more of]
```

---

## ⚠️ **AREAS FOR IMPROVEMENT**

### **What Could Be Better**

```yaml
1. [Specific gap]
   - Observed: [What happened]
   - Expected: [What should have happened]
   - Impact: [Why it matters]
   - Suggestion: [How to improve]
   
2. [Specific gap]
   - Observed: [What happened]
   - Expected: [What should have happened]
   - Impact: [Why it matters]
   - Suggestion: [How to improve]
```

### **Action Items**

```yaml
For Next Task:
  ☐ [Specific improvement to make]
  ☐ [Another improvement]

For Delegation Spec:
  ☐ [Clarification to add]
  ☐ [Example to provide]

For AA Training:
  ☐ [Concept to reinforce]
  ☐ [Practice area]
```

---

## 🎓 **LESSONS LEARNED**

### **For This AA (Codex)**

```yaml
Confirmed Understanding:
  - [What this AA clearly understands]
  - [Another confirmed capability]

Needs Clarification:
  - [What needs more explanation]
  - [Ambiguous area]

Behavioral Pattern:
  - [Observed tendency]
  - [Another pattern]
```

### **For Multi-AA Framework**

```yaml
Framework Validation:
  ✅ [What worked in framework]
  ✅ [Another thing that worked]
  
Framework Gaps:
  ⚠️ [What's missing from framework]
  ⚠️ [Another gap]

Framework Improvements:
  → [Suggested improvement]
  → [Another suggestion]
```

### **For Future Delegations**

```yaml
Spec Quality:
  - Was spec clear enough? [Yes/No + why]
  - Was spec too detailed? [Yes/No + why]
  - What was missing from spec?
  - What was unnecessary in spec?

Delegation Success Factors:
  - [Factor that enabled success]
  - [Another factor]

Delegation Challenges:
  - [Challenge encountered]
  - [How to prevent in future]
```

---

## 📈 **TREND ANALYSIS**

### **Comparison to Previous Tasks**

```yaml
Task 1: [Previous task name]
  Score: ___/100
  Rating: [Rating]
  
Task 2: [This task]
  Score: ___/100
  Rating: [Rating]

Trend: [Improving | Stable | Declining]

Notable Changes:
  - [Improvement area]
  - [Regression area]
  - [New capability]
```

### **Learning Curve**

```yaml
Speed of Adaptation:
  - First task: [Observation]
  - This task: [Observation]
  - Trajectory: [Fast learner | Steady | Slow]

Pattern Recognition:
  - Applies previous lessons? [Yes/No + evidence]
  - Repeats mistakes? [Yes/No + evidence]
  - Shows meta-learning? [Yes/No + evidence]
```

---

## 🎯 **RECOMMENDATIONS**

### **Task Assignment**

```yaml
Recommended Task Types:
  ✅ [Type of task this AA excels at]
  ✅ [Another suitable type]

Avoid Assigning:
  ❌ [Type of task this AA struggles with]
  ❌ [Another unsuitable type]

Supervision Level:
  - [Autonomous | Light supervision | Close supervision]
  - Rationale: [Why]
```

### **Skill Development**

```yaml
Priority Development Areas:
  1. [Skill to develop]
     - Current level: [Assessment]
     - Target level: [Goal]
     - Approach: [How to develop]
  
  2. [Another skill]
     - Current level: [Assessment]
     - Target level: [Goal]
     - Approach: [How to develop]
```

### **Framework Adjustments**

```yaml
Immediate Adjustments:
  - [Change to make now]
  - [Another immediate change]

Future Considerations:
  - [Longer-term adjustment]
  - [Another consideration]
```

---

## 🔄 **ITERATION HISTORY**

### **Task Revisions**

```yaml
Version 1: [Initial submission]
  - Issues: [What needed fixing]
  - Feedback provided: [What was said]
  
Version 2: [After revision]
  - Changes made: [What was fixed]
  - Quality: [Improved/Same/Worse]

Iteration Quality:
  - Response time: [How fast]
  - Understanding: [Did AA understand feedback?]
  - Improvement: [Did changes fix issues?]
```

---

## ✅ **FINAL VERDICT**

### **Task Completion Status**

```yaml
Deliverables: [Complete | Incomplete | Partial]
Quality: [Excellent | Good | Acceptable | Poor]
Process: [Excellent | Good | Acceptable | Poor]
Collaboration: [Excellent | Good | Acceptable | Poor]

Overall: [APPROVED | APPROVED WITH NOTES | NEEDS REVISION | REJECTED]
```

### **Meta-Experiment Results**

```yaml
Framework Validation:
  - Did delegation work? [Yes/No + why]
  - Did protocols work? [Yes/No + why]
  - Did evaluation work? [Yes/No + why]

Multi-AA Viability:
  - Can scale to multiple AAs? [Yes/No + why]
  - Are rules sufficient? [Yes/No + why]
  - Is overhead acceptable? [Yes/No + why]

Next Steps:
  - [Action based on results]
  - [Another action]
```

---

## 📝 **EVALUATOR NOTES**

### **Additional Observations**

```yaml
Unexpected Behaviors:
  - [Positive surprise]
  - [Negative surprise]
  - [Neutral observation]

Context Factors:
  - [External factor that affected performance]
  - [Another contextual note]

Personal Assessment:
  - [Evaluator's subjective impression]
  - [Confidence in this evaluation: High/Medium/Low]
```

---

## 📅 **FOLLOW-UP**

### **Scheduled Reviews**

```yaml
Next Evaluation: [Date]
  - Task: [Next task name]
  - Focus: [What to watch for]

Progress Check: [Date]
  - Review: Improvement in [specific area]

Quarterly Review: [Date]
  - Aggregate: All tasks performance
  - Trend: Overall capability growth
```

---

**Evaluation Completed By**: [Evaluator Name]  
**Date**: [YYYY-MM-DD]  
**Time Invested**: [X minutes]  
**Confidence in Evaluation**: [High | Medium | Low]

---

**Document Status**: Template v1.0  
**Ready for Use**: ✅ YES  
**Customization**: Adapt dimensions/weights as needed per task type
