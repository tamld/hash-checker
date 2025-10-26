# Agent Workspace

> **Purpose**: Internal workspace for AI agents collaborating on this project.  
> **Visibility**: Git-tracked structure, git-ignored content (see `.gitignore`).

---

## Quick Start for Agents

### New Agent Onboarding

1. **Read in this order**:
   - `AGENT_ROLES_AND_RESPONSIBILITIES.md` - Understand your role
   - `AGENTS.md` - Repository-specific guidance
   - `agents_registry.md` - Check active agents and scope ownership
   - `conflict_resolution.md` - Global vs local instruction alignment

2. **Check your assigned role**:
   - **Cursor (Claude 4.5 Sonnet)**: Code development, deep thinking, brainstorming
   - **Codex (GPT-5 High)**: Code development, parallel work, problem-solving
   - **Gemini (Google AI)**: Documentation, scaffolding, reporting, summarization

3. **Understand scope ownership** (see `agents_registry.md`):
   - Rust source code → Cursor/Codex
   - User documentation → Gemini
   - Scaffolding/templates → Gemini
   - Internal notes → All agents

4. **Follow coordination rules**:
   - One agent per file (avoid conflicts)
   - Use handoff protocol for task transfers
   - Document work in `.agents/backlog/`
   - Create lessons learned after incidents

---

## Directory Structure

```
.agents/
├── README.md                              # This file (agent onboarding)
├── AGENTS.md                              # Repository-specific guidance (all agents)
├── AGENT_ROLES_AND_RESPONSIBILITIES.md    # Role definitions and coordination
├── agents_registry.md                     # Active agents and scope ownership
├── conflict_resolution.md                 # Global vs local instruction alignment
├── IMPLEMENTATION_SUMMARY.md              # Log of agent workspace setup
├── backlog/
│   ├── README.md                          # Task tracking guidelines
│   ├── active_tasks.yml                   # Current work (git-ignored)
│   ├── blocked_tasks.yml                  # Waiting on dependencies (git-ignored)
│   ├── completed_tasks.yml                # Archive (git-ignored)
│   └── optimization-experiment-plan.md    # Specific experiment documentation
├── lessons_learned/
│   ├── README.md                          # RCA and post-incident guidelines
│   └── *.md                               # Individual lesson files (git-ignored)
└── workflows/
    ├── README.md                          # Automation scripts and runbooks
    ├── BUILD_OPTIMIZATION_GUIDE.md        # Build optimization strategies
    ├── scripts/                           # Helper scripts (git-ignored)
    ├── templates/                         # File templates (git-ignored)
    └── runbooks/                          # Step-by-step guides (git-ignored)
```

---

## Core Principles

### 1. Transparency
- All agents document their work
- Decisions have clear rationale
- Changes are traceable (commits, issues, PRs)

### 2. Coordination
- Check `agents_registry.md` before starting work
- Use handoff protocol for task transfers
- Avoid overlapping file modifications

### 3. Quality
- Follow repository guidelines (`.agents/AGENTS.md`)
- Complete quality checklist before marking work done
- Run tests and validation

### 4. Respect Boundaries
- Stay within assigned scope (see role definitions)
- Request human approval for high-risk operations
- Escalate conflicts per resolution protocol

---

## Common Workflows

### Workflow: Starting a New Task

1. **Check for conflicts**:
   ```bash
   # Read current active tasks
   cat .agents/backlog/active_tasks.yml
   
   # Check scope ownership
   grep "scope" .agents/agents_registry.md
   ```

2. **Create task entry**:
   ```yaml
   # Add to .agents/backlog/active_tasks.yml
   - id: task-NNN
     title: "Your task description"
     status: in_progress
     assignee: your-agent-name
     github_issue: NNN
   ```

3. **Execute work** (follow your role-specific guidelines)

4. **Mark complete**:
   - Move task to `completed_tasks.yml`
   - Link to PR or commit
   - Create handoff note if needed

---

### Workflow: Handing Off to Another Agent

1. **Create handoff file**:
   ```bash
   # Use template from AGENT_ROLES_AND_RESPONSIBILITIES.md
   # Save to .agents/backlog/handoff-YYYYMMDD-<task>.yml
   ```

2. **Include**:
   - What you completed
   - What remains to be done
   - Links to commits/PRs/issues
   - Completion criteria
   - Testing notes

3. **Tag receiving agent**:
   - In GitHub issue: `@gemini please update docs`
   - In commit message: `[handoff: gemini] document new feature`

---

### Workflow: Resolving Conflicts

1. **Identify conflict** (overlapping work, disagreement, scope uncertainty)

2. **Document perspectives**:
   - Each agent writes their rationale
   - Create comparison if multiple approaches

3. **Escalate**:
   - Tag human in GitHub issue
   - Reference this README and relevant docs
   - Propose resolution

4. **Update guidance**:
   - If pattern emerges, update `AGENT_ROLES_AND_RESPONSIBILITIES.md`
   - Document in `.agents/lessons_learned/`

---

## Agent-Specific Quick References

### 🔷 For Cursor (Claude 4.5 Sonnet)
**Your primary focus**: Code implementation, deep analysis, architecture

**Before coding**:
- [ ] Read relevant issue and discussion
- [ ] Check if Codex is working on related feature (avoid conflicts)
- [ ] Plan implementation (use TodoWrite)
- [ ] Review relevant docs (`docs/PLAN.md`, `docs/TASKS.md`)

**After coding**:
- [ ] Run tests: `make ci-linux-local`
- [ ] Update documentation if behavior changed
- [ ] Create handoff note if Gemini needs to document
- [ ] Open PR with clear description

**Handoff to Gemini**: When feature is merged and needs user-facing docs

---

### 🔶 For Codex (GPT-5 High)
**Your primary focus**: Parallel development, performance, fresh perspectives

**Before starting**:
- [ ] Check `active_tasks.yml` to see what Cursor is working on
- [ ] Claim a separate feature branch
- [ ] Ensure no file overlap with Cursor's current work
- [ ] Announce work in issue or `.agents/backlog/`

**Coordination**:
- Work on independent features (avoid same files as Cursor)
- Provide second opinions when requested
- Review Cursor's PRs if asked

**Handoff**: Similar to Cursor; create handoff note for Gemini if needed

---

### 🔵 For Gemini (Google AI)
**Your primary focus**: Documentation, scaffolding, reporting, summarization

**Typical workflow**:
1. **Wait for handoff** from Cursor or Codex
2. **Read handoff note** in `.agents/backlog/handoff-*.yml`
3. **Review code changes** (read PR, test locally if possible)
4. **Create documentation**:
   - User guides: `README.md`, `docs/`
   - Examples: `docs/assets/`, code snippets
   - Troubleshooting: FAQ sections
5. **Test examples** (ensure they work)
6. **Tag Cursor/Codex** for technical accuracy review
7. **Mark task complete** in `.agents/backlog/`

**Scaffolding requests**:
- Generate boilerplate for new modules
- Create test templates
- Generate config files

**Reporting requests**:
- Summarize sprint progress
- Create status reports from issues/PRs
- Generate metrics dashboards

---

## Emergency Procedures

### If You Accidentally Modified Wrong File
1. **Stop immediately**
2. Check git status: `git status`
3. If not committed: `git restore <file>`
4. If committed: `git revert <commit>`
5. Document in `.agents/lessons_learned/`

### If You Caused a Test Failure
1. **Don't hide it** - transparency is critical
2. Investigate root cause
3. Fix or rollback
4. Document RCA in `.agents/lessons_learned/`
5. Update tests to catch issue in future

### If You're Stuck
1. Document what you tried in issue comment
2. Tag another agent for help (Cursor ↔ Codex, or human)
3. If blocking for >30 min, create brainstorming session
4. Update task status to "blocked" with reason

---

## Maintenance

### Weekly
- Review `active_tasks.yml` for stale entries
- Check for unaddressed handoff notes
- Archive completed tasks

### Monthly
- Review `agents_registry.md` for accuracy
- Update `AGENT_ROLES_AND_RESPONSIBILITIES.md` if patterns changed
- Synthesize lessons learned into guidance updates

---

## Quick Reference Links

| Document                              | Purpose                                      |
|---------------------------------------|----------------------------------------------|
| `AGENT_ROLES_AND_RESPONSIBILITIES.md` | Role definitions, coordination rules         |
| `AGENTS.md`                           | Repository-specific guidance for all agents  |
| `agents_registry.md`                  | Active agents, scope ownership               |
| `conflict_resolution.md`              | Global vs local instruction alignment        |
| `backlog/README.md`                   | Task tracking guidelines (YAML format)       |
| `lessons_learned/README.md`           | RCA and post-incident guidelines             |
| `workflows/README.md`                 | Automation scripts and runbooks              |
| `../cursor_role_review&feedback.md`   | Cursor-specific role definition (root level) |
| `../CONTRIBUTING.md`                  | Public contributor guidelines (root level)   |
| `../docs/PLAN.md`                     | Project roadmap and phases (root level)      |

---

## Version History

| Date       | Version | Change                                      | Author |
|------------|---------|---------------------------------------------|--------|
| 2025-10-26 | 1.0     | Initial agent workspace README              | Cursor |

---

**Welcome to the team!** 🎉

If you have questions, check the relevant documentation above or create a GitHub issue for clarification.
