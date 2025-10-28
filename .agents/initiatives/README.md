# AA Evolution Initiatives

**Purpose**: Track AA capability development work (not tied to specific feature branches)  
**Location**: main branch, `.agents/initiatives/`  
**Scope**: AA evolution, learning, coordination improvements  

---

## 🎯 **What Are Initiatives?**

Initiatives are AA evolution work that improves AA capabilities generally, not specific to one feature/issue.

**Examples:**
- AA Core Skills Framework (how to measure competency)
- Autonomous Multi-AA Ecosystem (how AAs coordinate without user)
- Behavior Enforcement (how to prevent violations)
- Lifecycle Framework (AA growth model)

**NOT initiatives:**
- Feature-specific work (belongs in feature branch)
- Bug fixes (belongs in fix branch)
- Project-specific infrastructure (belongs in related branch)

---

## 📂 **Structure**

```
.agents/initiatives/
  README.md (this file - what initiatives are)
  BACKLOG.yml (tracking - status, next actions, priority)
  WORKFLOW.md (how to work with initiatives)
  
  aa-core-skills/
    README.md (initiative overview)
    proposals/
      cursor-proposal.md
      codex-proposal.md
      gemini-proposal.md
    discussion.md
    consensus.md
    test-results.md (when tested)
  
  autonomous-ecosystem/
    README.md
    proposals/
      cursor-proposal.md
    design/
      phase1-design.md
      phase2-design.md
      phase3-design.md
  
  [other initiatives...]
```

---

## 🔄 **How to Work With Initiatives**

### 1. Pick an initiative

```bash
# See what's available
cat .agents/initiatives/BACKLOG.yml

# Look for:
# - status: in_progress (active)
# - next_action: what needs to be done
# - participants: who's working on it
```

### 2. Add your contribution

```bash
# Example: Add proposal for aa-core-skills
cd .agents/initiatives/aa-core-skills/proposals/
# Create codex-proposal.md or gemini-proposal.md
# Follow template in existing proposals
```

### 3. Update backlog

```bash
# Update your status in BACKLOG.yml
# Change: next_action, participants, phase, etc.
git add .agents/initiatives/
git commit -m "initiatives: [your contribution]"
git push origin main
```

### 4. When consensus reached

```bash
# Test the consensus
# Document results in test-results.md
# If proven: PR to merge into lessons/principles
# Update BACKLOG.yml status to "proven" or "complete"
```

---

## 📊 **Current Initiatives**

See: `.agents/initiatives/BACKLOG.yml` for live status

**Active** (in_progress):
- AA Core Skills Framework
- Autonomous Multi-AA Ecosystem
- Lifecycle Framework
- Behavior Enforcement System
- Sustainable Development Model
- Human-Like Learning System
- Workflow Simplification
- Brainstorm Structure Improvement

**Proven** (consensus + tested): None yet

**Archived** (deferred or abandoned): None yet

---

## 🎓 **Principles**

1. **Consensus-driven**: Multi-AA agreement required (not single AA opinion)
2. **Evidence-based**: Test before adopting (not speculation)
3. **Traceable**: BACKLOG.yml tracks status (clear next actions)
4. **Not blocking**: Initiatives are async (don't block feature work)
5. **Proper scope**: General AA evolution only (not feature-specific)

---

## 📝 **Migration History**

**2025-10-28**: Migrated from feature/gui-automation-harness-issue56
- 7 brainstorm topics moved here (proper scope - not Issue #56 specific)
- See: MIGRATION_LOG.md for details

---

**Created**: 2025-10-28  
**Tracking**: BACKLOG.yml  
**Process**: WORKFLOW.md
