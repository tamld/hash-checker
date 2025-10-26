# Conflict Resolution: Global vs Local Instructions

> **Purpose**: Resolve discrepancies between global MCP instructions and repository-specific agent guidance.
> **Authority**: Global laws (security, non-negotiables) override local preferences; local policies govern project-specific workflows.

---

## Table of Contents
- [Instruction Hierarchy](#instruction-hierarchy)
- [Identified Conflicts](#identified-conflicts)
- [Resolution Decisions](#resolution-decisions)
- [Implementation Notes](#implementation-notes)

---

## Instruction Hierarchy

### Tier 1: Non-Overridable Laws (Highest Authority)
**Source**: Global MCP Core (`/Users/tamld/Library/CloudStorage/OneDrive-MSFT/Documents/MCP-Server/memory/core/`)

**Immutable Rules**:
- **LAW-REFLECT-001**: Pause for human confirmation before actions beyond available context or conflicting with current task
- **SECRET-SHIELD-001**: No secrets, PII, or credentials in code/docs/memory
- **MAIN-PROTECT-001**: No direct pushes to `main` without PR and CI validation
- **CI-PATH-001**: Avoid absolute hard-coded paths in CI; use relative or env-based paths

**Agent Behavior**: Cannot be overridden by local policies. If local guidance conflicts, **global law wins**.

---

### Tier 2: Repository-Specific Policies
**Source**: `.agents/AGENTS.md`, `CONTRIBUTING.md`, `docs/PLAN.md`

**Local Governance**:
- Branch naming conventions
- Testing and validation standards
- Documentation structure and style
- Release workflow specifics

**Agent Behavior**: Governs project-specific processes. Takes precedence over generic templates when no conflict with Tier 1.

---

### Tier 3: Agent-Specific Guidance
**Source**: Individual agent instruction files (e.g., `cursor_role_review&feedback.md`)

**Scope**: Operational tips and role definitions for specific agents.

**Agent Behavior**: Lowest priority. If conflicts with Tier 1 or Tier 2, **Tier 1/2 wins**. These are "preferences" not "laws".

---

## Identified Conflicts

### Conflict 1: Scope Expansion vs Initiative

| Aspect             | Global MCP Guidance                       | Local `cursor_role_review&feedback.md` | Conflict? |
|--------------------|-------------------------------------------|----------------------------------------|-----------|
| Proactive analysis | Encouraged; think comprehensively         | Forbidden; execute only assigned tasks | **YES**   |
| Scope expansion    | Allowed if improves outcome               | Prohibited; must stick to ticket       | **YES**   |
| Initiative         | Proactive problem-solving expected        | Wait for explicit instructions         | **YES**   |

**Analysis**:
- Global MCP expects agents to be **collaborative and proactive**
- Local role definition treats agent as **restricted automation**
- This creates a **fundamental operating model conflict**

---

### Conflict 2: Tool Usage Restrictions

| Aspect             | Global MCP Guidance                       | Local `cursor_role_review&feedback.md` | Conflict? |
|--------------------|-------------------------------------------|----------------------------------------|-----------|
| Parallel tool calls| Maximize efficiency; run in parallel      | Restricted command set                 | **YES**   |
| Command flexibility| Use all available tools appropriately     | Hard prohibitions listed               | **PARTIAL**|

**Analysis**:
- Global encourages **tool efficiency** (parallel operations)
- Local lists **prohibited commands** (reasonable for safety)
- Partially overlap: local safety rules are valid; global efficiency still applies to allowed commands

---

### Conflict 3: Task Planning and Management

| Aspect             | Global MCP Guidance                       | Local `cursor_role_review&feedback.md` | Conflict? |
|--------------------|-------------------------------------------|----------------------------------------|-----------|
| TodoWrite usage    | Use frequently for planning               | Only execute assigned tickets          | **YES**   |
| Multi-step tasks   | Break down and track proactively          | Wait for human-defined task breakdown  | **YES**   |

**Analysis**:
- Global expects agents to **self-organize complex work**
- Local expects **human task decomposition**
- This impacts **autonomy level** significantly

---

### Conflict 4: KPI and Performance Model

| Aspect             | Global MCP Guidance                       | Local `cursor_role_review&feedback.md` | Conflict? |
|--------------------|-------------------------------------------|----------------------------------------|-----------|
| Performance model  | Stateless; each conversation independent  | Sprint-based KPIs, promotions/demotions| **YES**   |
| Success criteria   | Task completion quality                   | Quarterly metrics, first-pass CI rates | **YES**   |

**Analysis**:
- Global recognizes agents are **stateless** (no memory between sessions)
- Local applies **human-style performance reviews** (sprints, quarters)
- **Fundamental mismatch**: KPIs designed for humans don't apply to AI

---

## Resolution Decisions

### Resolution 1: Operating Model
**Decision**: **Adopt "Collaborative AI Engineer with Guardrails" model**

**Rationale**:
- Global MCP's proactive model is more aligned with AI capabilities
- Local concerns about safety and scope are valid → keep as **guardrails** not **restrictions**
- Balance autonomy with checkpoints

**Implementation**:
- ✅ **Allow**: Proactive analysis, suggestions, comprehensive planning
- ✅ **Allow**: Multi-file refactoring with clear justification
- ⚠️ **Checkpoint**: High-risk operations require approval (see prohibited commands)
- 🚫 **Prohibit**: Destructive commands, secret exposure, main branch pushes

**Action**: Revise `cursor_role_review&feedback.md` to reflect this balanced model.

---

### Resolution 2: Command Safety
**Decision**: **Keep prohibited commands list; apply to all agents**

**Rationale**:
- Safety restrictions are valid regardless of autonomy level
- Aligns with global LAW-REFLECT-001 (pause before risky actions)

**Implementation**:
- Move prohibited commands list from role definition to `.agents/AGENTS.md` (applies to all agents)
- Keep list focused on **destructive operations** only
- Document **approval process** for when these commands are necessary

**Action**: Consolidate command restrictions in `.agents/AGENTS.md`.

---

### Resolution 3: Task Management
**Decision**: **Use TodoWrite for planning; honor issue-first workflow**

**Rationale**:
- TodoWrite enhances transparency and organization (global best practice)
- Issue-first workflow is reasonable project governance (local policy)
- These are **complementary** not conflicting

**Implementation**:
- ✅ Agents use TodoWrite to break down complex tasks
- ✅ All work still tied to GitHub issues (traceability requirement)
- ✅ Agents can propose task breakdowns; human approves scope

**Action**: Document in `.agents/AGENTS.md` as standard workflow.

---

### Resolution 4: Performance Tracking
**Decision**: **Replace KPIs with per-task quality checklist**

**Rationale**:
- Sprint-based KPIs don't apply to stateless AI
- Quality standards **per task** are measurable and meaningful

**Implementation**:
- Replace quarterly KPIs with **per-task completion checklist**:
  - [ ] Tests pass
  - [ ] Linters clean
  - [ ] Documentation updated
  - [ ] Logs captured
  - [ ] No regressions introduced
- Human reviews assess **consistency of quality** over time
- No "promotions/demotions" (not applicable to stateless agents)

**Action**: Update role definition with quality checklist; remove KPI table.

---

## Implementation Notes

### Files to Update
1. **`cursor_role_review&feedback.md`**:
   - Revise role from "junior automation engineer" to "collaborative AI engineer with guardrails"
   - Replace KPI section with quality checklist
   - Remove conflicting restrictions on scope and initiative
   - Keep safety prohibitions (reference `.agents/AGENTS.md`)

2. **`.agents/AGENTS.md`**:
   - Add consolidated prohibited commands section
   - Document checkpoint approval process
   - Clarify workflow: issue-first + TodoWrite planning
   - Emphasize transparency and validation requirements

3. **`CONTRIBUTING.md`** (already aligned):
   - References `.agents/AGENTS.yml` (should be `.agents/AGENTS.md`)
   - Fix this typo for consistency

### Global MCP Alignment
- No changes needed to global laws (already compatible)
- Local policies now **extend** rather than **conflict** with global guidance
- Resolution documented here for future reference

---

## Precedence Summary

When in doubt, apply this order:
1. **Global Laws** (security, core guardrails) → Always apply
2. **Repository Policies** (workflow, standards) → Apply unless conflicts with #1
3. **Agent Preferences** (operating tips) → Apply unless conflicts with #1 or #2

If a new conflict arises:
1. Document it in this file (add to "Identified Conflicts" section)
2. Analyze impact on global laws vs local needs
3. Make resolution decision with rationale
4. Update relevant instruction files
5. Communicate to human tech lead for approval

---

## Version History

| Date       | Change                                      | Author |
|------------|---------------------------------------------|--------|
| 2025-10-26 | Initial conflict analysis and resolution    | Cursor |

---

## Related Documents
- Global MCP Core: `/Users/tamld/Library/CloudStorage/OneDrive-MSFT/Documents/MCP-Server/memory/core/`
- Repository agent guidance: `.agents/AGENTS.md`
- Agent registry: `.agents/agents_registry.md`
- Role definition: `cursor_role_review&feedback.md` (to be revised)
