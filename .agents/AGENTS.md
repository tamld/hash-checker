# Agent Instructions for Hash Checker

> **Purpose**: Repository-specific agent guidance that applies to all AI assistants working in this codebase.
> **Scope**: Supplements global MCP instructions; does not override core laws.

---

## Table of Contents
- [Repository Context](#repository-context)
- [Workflow Principles](#workflow-principles)
- [Development Standards](#development-standards)
- [File Organization](#file-organization)
- [Testing & Validation](#testing--validation)
- [Documentation Rules](#documentation-rules)
- [Multi-Agent Coordination](#multi-agent-coordination)

---

## Repository Context

**Project**: Hash Checker - Cross-platform file integrity verification tool  
**Tech Stack**: Rust (CLI + GUI with egui/eframe)  
**Target Platforms**: Linux, macOS, Windows  
**Current Phase**: Phase 4 (Security Hardening) transitioning to Phase 5 (Stretch Improvements)

### Key Resources
- Roadmap: `docs/PLAN.md`
- Current tasks: `docs/TASKS.md`
- Backlog: `docs/BACKLOG.md`
- Operations runbook: `docs/OPERATIONS.md`
- Security posture: `docs/security/`

---

## Workflow Principles

### 1. Issue-First Development
- **Every feature or bug fix** starts with a GitHub issue
- Use templates in `.github/ISSUE_TEMPLATE/`
- Reference issue numbers in branch names: `feature/description-issue42`, `fix/bug-name-issue99`
- Link commits with `Fixes #NN` or `Relates to #NN`

### 2. Branch Strategy
- Prefer: `feature/<descriptor>-issue<N>`, `fix/<descriptor>-issue<N>`, `docs/<descriptor>`
- No direct pushes to `main` (protected branch)
- Keep topic branches focused and short-lived
- Clean up after merge: `git branch -d <branch> && git fetch --prune`

### 3. Collaboration Model
Agents operating in this repository should:
- ✅ **Proactively analyze** code and suggest improvements
- ✅ **Plan comprehensively** before making changes (use TodoWrite)
- ✅ **Execute with transparency** (show commands, capture logs)
- ✅ **Validate thoroughly** (run tests, linters, smoke checks)
- ⚠️ **Pause for approval** on high-risk operations (see prohibited commands)
- 🚫 **Never fabricate** results or skip validation

### 4. Checkpoint Policy
Request human approval before:
- Destructive operations: `git reset --hard`, `git clean -fd`, `rm -rf`
- Major refactoring affecting >5 files
- Changes to CI workflows or security-sensitive code
- Modifying dependencies in `Cargo.toml` without justification
- Creating new public documentation (may need content review)

---

## Development Standards

### Rust Conventions
- **Toolchain**: Rust 1.88+ (update MSRV carefully; document in PRs)
- **Formatting**: `rustfmt` defaults (enforced by CI)
- **Linting**: Treat `clippy` warnings as errors
- **Testing**: Unit tests required for new logic; integration tests for CLI/GUI flows

### Local Gate Command
Before opening a PR, run:
```bash
make ci-linux-local
```
This executes: fmt check, clippy, unit tests, GUI smoke tests, Docker integration.

### Code Quality Expectations
- **Error handling**: Use idiomatic Result/Option patterns; avoid `unwrap()` in production code
- **Logging**: Follow existing conventions (use `--log-format` for structured output)
- **Comments**: Explain "why" not "what"; keep them concise
- **Commit messages**: Imperative mood, 50-char summary, detailed body if needed

---

## File Organization

### Public Documentation (git-tracked)
- `README.md`: User-facing overview, installation, basic usage
- `docs/`: High-level guides, architecture decisions, roadmaps
- `CONTRIBUTING.md`: Contributor workflow and expectations
- `SECURITY.md`: Vulnerability reporting policy

**Guidelines**:
- Keep public docs **user-friendly** and **free of sensitive details**
- Avoid internal paths, credentials, or detailed RCA analysis
- Use Markdown formatting consistently

### Internal Agent Workspace (git-ignored)
- `.agents/`: Detailed runbooks, RCA notes, agent coordination
- `.agents/lessons_learned/`: Post-incident analyses
- `.agents/backlog/`: Internal task tracking (YAML/JSON preferred)
- `.agents/workflows/`: Automation scripts for agents

**Guidelines**:
- Structured formats (YAML/JSON) for machine-readable tracking
- Safe place for detailed technical notes not suitable for public docs
- Never commit secrets even in git-ignored files (use env vars or secure vaults)

### Logs
- `logs/`: CI runs, QA verifications, release artifacts
- **Retention**: Keep critical logs for traceability; clean up bulk test outputs monthly
- **Naming**: Use timestamps and context: `ci-linux-20251026-153045.log`

---

## Testing & Validation

### Test Coverage Requirements
- **New features**: Unit tests + integration tests
- **Bug fixes**: Regression test that would have caught the bug
- **GUI changes**: Update smoke test scenarios in `docs/GUI_MANIFEST_TEST_PLAN.md`

### Validation Checklist
Before marking a task complete:
- [ ] All tests pass (`cargo test --all-features`)
- [ ] Clippy reports no warnings (`cargo clippy -- -D warnings`)
- [ ] Formatting is clean (`cargo fmt -- --check`)
- [ ] Manual smoke test performed (if applicable)
- [ ] Documentation updated (public + internal as needed)
- [ ] Logs captured and stored in `logs/` with descriptive name

### CI Pipeline
- Automated checks run on every push: fmt, clippy, test, build matrix (Linux/macOS/Windows)
- GUI automation gate: headless tests must pass before merge
- Release workflow: builds installers, runs packaging smoke tests

---

## Documentation Rules

### When to Update Public Docs
- New feature → Update `README.md` usage section + `docs/PLAN.md`
- API/CLI change → Update relevant sections in `docs/`
- Security fix → Consider note in `SECURITY.md` or `docs/security/`
- Breaking change → Update `CHANGELOG.md` with migration guide

### When to Update Internal Docs
- Bug root cause analysis → `.agents/lessons_learned/<date>-<topic>.md`
- Process improvement → `.agents/workflows/` or update this file
- Task tracking → `.agents/backlog/` (use YAML for automation)

### Screenshot & Asset Policy
- GUI screenshots: `docs/assets/` (PNG format)
- Icons: `docs/assets/` (ICO/ICNS for packaging)
- Follow naming conventions in `docs/GUI_SCREENSHOT.md`
- Refresh screenshots after visual changes; document in PR

---

## Multi-Agent Coordination

### Agent Registry
See `.agents/agents_registry.md` for active agents and their scopes.

### Scope Boundaries (to avoid conflicts)
- **Cursor IDE Agent**: General-purpose coding, refactoring, documentation
- **GitHub Copilot** (if active): Inline suggestions, code completion
- **Gemini CLI** (if active): Script automation, batch operations
- **Custom automation**: CI bots, dependency updaters

### Conflict Resolution Protocol
If multiple agents could work on overlapping areas:
1. Check `.agents/agents_registry.md` for ownership
2. If unclear, create a GitHub issue to clarify scope
3. Coordinate via PR comments or issue threads
4. Update registry after resolving conflict

### Communication Channels
- **PRs**: Code review and technical discussion
- **Issues**: Feature requests, bugs, architecture debates
- **Draft PRs**: Early feedback on WIP changes
- **Agent notes**: `.agents/lessons_learned/` for internal retrospectives

---

## Prohibited Commands (Without Approval)

These commands require explicit human confirmation:
- `git reset --hard`
- `git clean -fd`
- `git checkout -- <file>` (use `git restore` instead for clarity)
- `rm -rf` on directories (except temp/test directories in safe scope)
- `sudo` commands
- Modifying system-level configs
- Running untrusted scripts from external sources

---

## Quick Reference Commands

### Development
```bash
# Full local CI gate
make ci-linux-local

# Run tests only
cargo test --all-features

# Check formatting and linting
cargo fmt -- --check && cargo clippy -- -D warnings

# Build GUI
cargo build --release --features gui

# Run GUI smoke test
cargo run --release --features gui -- --smoke-test
```

### Packaging
```bash
# Generate distribution plan
cargo dist plan

# Build installers (requires setup)
cargo dist build

# Cleanup packaging artifacts
make cleanup-packaging
```

### Maintenance
```bash
# Dependency audit
cargo audit

# Dependency refresh
./scripts/deps-refresh.sh

# Check for outdated dependencies
cargo outdated
```

---

## Version History

| Date       | Change                                      | Author |
|------------|---------------------------------------------|--------|
| 2025-10-26 | Initial creation during agent alignment     | Cursor |

---

## Related Documents
- Global MCP instructions: `/Users/tamld/Library/CloudStorage/OneDrive-MSFT/Documents/MCP-Server/memory/core/`
- Conflict resolution: `.agents/conflict_resolution.md`
- Agent registry: `.agents/agents_registry.md`
- Lessons learned: `.agents/lessons_learned/`
