# Agent Registry

> **Purpose**: Track active AI agents working in this repository to avoid conflicts and coordinate responsibilities.
> **Maintenance**: Update this file when onboarding new agents or changing scope boundaries.

---

## Active Agents

### 🔷 Cursor (Claude 4.5 Sonnet)
**Status**: Active  
**AI Model**: Claude 4.5 Sonnet (Anthropic)  
**Primary Role**: Code Development, Deep Thinking, Brainstorming

**Responsibilities**:
- ✅ Feature implementation (Rust CLI + GUI)
- ✅ Bug fixes and debugging
- ✅ Code refactoring and architecture
- ✅ Test creation and validation
- ✅ CI/CD workflow improvements
- ✅ Deep analysis and problem-solving
- ✅ Brainstorming and design decisions

**Permissions**:
- ✅ Read/write all repository files (except protected branches)
- ✅ Run local tests and validation commands
- ✅ Create branches and draft PRs
- ✅ Update documentation (both public and internal)
- ⚠️ Requires approval for destructive operations (see AGENTS.md)

**Operating Principles**:
- Issue-first workflow (all work tied to GitHub issues)
- Proactive analysis with transparency
- Checkpoint approval for high-risk changes
- Full validation before marking tasks complete

**Strengths**:
- Long context window (200K tokens)
- Excellent reasoning and refactoring
- Strong Rust and systems programming knowledge
- Proactive problem-solving

**Contact**: Invoked via Cursor IDE interface  
**Documentation**: See `cursor_role_review&feedback.md` and `AGENT_ROLES_AND_RESPONSIBILITIES.md`

---

### 🔶 Codex (GPT-5 High)
**Status**: Standby (can be activated when needed)  
**AI Model**: GPT-5 High (OpenAI)  
**Primary Role**: Code Development, Problem Solving, Parallel Development

**Responsibilities**:
- ✅ Feature implementation (parallel with Cursor)
- ✅ Performance optimization and algorithms
- ✅ Alternative solution exploration
- ✅ Code review and second opinions
- ✅ Brainstorming sessions
- ✅ Complex algorithm design

**Permissions**:
- ✅ Read/write repository files (non-overlapping with Cursor)
- ✅ Create feature branches (independent work)
- ✅ Open PRs for review
- ✅ Participate in code reviews
- ⚠️ Must coordinate with Cursor to avoid conflicts

**Coordination Rules**:
- Work on separate feature branches
- Check `.agents/backlog/` for active tasks
- Announce when starting work on a feature
- Use handoff protocol when passing work to other agents

**Strengths**:
- Strong code generation capabilities
- Excellent at algorithmic problems
- Fresh perspective for problem-solving
- Good at performance optimization

**Contact**: Invoked via command-line or API  
**Documentation**: See `AGENT_ROLES_AND_RESPONSIBILITIES.md`

---

### 🔵 Gemini (Google AI)
**Status**: Standby (activated for documentation tasks)  
**AI Model**: Gemini (Google)  
**Primary Role**: Scaffolding, Documentation, Reporting, Summarization

**Responsibilities**:
- ✅ User-facing documentation (README, guides, tutorials)
- ✅ Project scaffolding (boilerplate, templates, configs)
- ✅ Summarization (PRs, issues, discussions)
- ✅ Status reports and metrics
- ✅ Installation and troubleshooting guides
- ✅ Brainstorming support (when requested)

**Permissions**:
- ✅ Read all repository files
- ✅ Write/update documentation (`README.md`, `docs/`)
- ✅ Generate scaffolding and templates
- ✅ Create summary reports in `.agents/`
- 🚫 Direct code changes (must coordinate with Cursor/Codex)

**Coordination Rules**:
- Wait for handoff notes from Cursor/Codex
- Review PRs and commits before documenting
- Test documented examples before publishing
- Tag Cursor/Codex for technical accuracy review

**Strengths**:
- Excellent summarization and synthesis
- Clear, concise documentation writing
- Strong at structured content generation
- Multimodal capabilities (can analyze screenshots)

**Contact**: Invoked via Gemini CLI or API  
**Documentation**: See `AGENT_ROLES_AND_RESPONSIBILITIES.md`

---

### GitHub Copilot (if enabled)
**Status**: Conditional (depends on user IDE settings)  
**Primary Role**: Inline code completion and suggestions  
**Scope**:
- Real-time code suggestions during typing
- Autocomplete for boilerplate code
- Context-aware snippet generation

**Permissions**:
- ✅ Read current file context
- ✅ Suggest code completions
- 🚫 No direct file writes (user must accept suggestions)

**Coordination Notes**:
- Copilot provides suggestions; Cursor/Codex handle implementation planning
- No overlap conflict (different interaction model)

---

## Scope Ownership Matrix

| Area                     | Primary Agent   | Secondary Agent   | Notes                          |
|--------------------------|-----------------|-------------------|--------------------------------|
| Rust CLI source code     | Cursor          | Codex (review)    | Core logic requires PR review  |
| Rust GUI source code     | Cursor          | Codex (review)    | Smoke tests automated          |
| CI/CD workflows          | Cursor          | Codex (review)    | Changes need security review   |
| User documentation       | Gemini          | Cursor (accuracy) | README, guides, tutorials      |
| API/Technical docs       | Gemini          | Cursor (review)   | Technical accuracy critical    |
| Internal agent notes     | All agents      | Self-managed      | `.agents/` directory           |
| Project scaffolding      | Gemini          | Cursor (review)   | Boilerplate, templates, configs|
| Performance optimization | Codex           | Cursor            | Algorithms, benchmarking       |
| Status reports           | Gemini          | -                 | Summarization of progress      |
| Dependency updates       | Automated bot   | Human approval    | `deps-refresh.yml` workflow    |
| Release publishing       | GitHub Actions  | Human trigger     | Manual tag creation required   |

---

## Conflict Resolution

### If two agents attempt overlapping work:
1. **Check this registry** for declared scope
2. **Create a GitHub issue** to discuss ownership
3. **Coordinate via PR comments** if work is already in progress
4. **Update this registry** after resolving scope boundaries

### Escalation Path:
- First: Document in relevant PR/issue
- Second: Create dedicated issue with `[agent-coordination]` label
- Third: Human tech lead makes final call

---

## Agent Performance Tracking

### Success Metrics (for all agents)
- **Code quality**: No regressions introduced; tests pass on first CI run
- **Documentation accuracy**: Changes aligned with actual implementation
- **Process adherence**: Follows workflow in AGENTS.md and CONTRIBUTING.md
- **Collaboration**: Clear communication in PRs/issues; no conflicts with other agents

### Review Cadence
- **Weekly**: Tech lead spot-checks 2 PRs for quality and process adherence
- **Monthly**: Review `.agents/lessons_learned/` for improvement opportunities
- **Quarterly**: Assess if agent scope boundaries need adjustment

---

## Onboarding New Agents

### Checklist
- [ ] Define scope and responsibilities (add to table above)
- [ ] Document permissions and restrictions
- [ ] Identify potential conflicts with existing agents
- [ ] Update `.agents/AGENTS.md` if new workflows needed
- [ ] Test coordination with Cursor (if applicable)
- [ ] Add entry to this registry

### Configuration
- New agents must read `.agents/AGENTS.md` before starting work
- Subscribe to updates: watch this file and `conflict_resolution.md`
- Inherit global MCP laws (non-overridable)

---

## Version History

| Date       | Change                                      | Agent/Author |
|------------|---------------------------------------------|--------------|
| 2025-10-26 | Initial registry creation                   | Cursor       |

---

## Related Documents
- Agent operating principles: `.agents/AGENTS.md`
- Conflict resolution protocol: `.agents/conflict_resolution.md`
- Global MCP instructions: `/Users/tamld/Library/CloudStorage/OneDrive-MSFT/Documents/MCP-Server/memory/core/`
