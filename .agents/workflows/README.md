# Agent Workflows

> **Purpose**: Store automation scripts, runbooks, and workflow helpers for agents.
> **Scope**: Internal tools to streamline repetitive tasks and enforce standards.

---

## Directory Structure

```
workflows/
├── README.md (this file)
├── scripts/ (executable automation scripts)
├── templates/ (file templates for agents)
└── runbooks/ (step-by-step guides for complex operations)
```

---

## Automation Scripts

### Purpose
Reusable scripts that agents can invoke to:
- Validate code changes before committing
- Generate boilerplate for new features
- Parse CI logs and extract failures
- Update documentation automatically

### Example Scripts (to be created)

**`scripts/pre-commit-check.sh`**:
```bash
#!/bin/bash
# Run before committing changes
set -e

echo "Running pre-commit checks..."
cargo fmt -- --check
cargo clippy -- -D warnings
cargo test --all-features
echo "✅ All checks passed"
```

**`scripts/update-task-status.py`**:
```python
#!/usr/bin/env python3
# Update task status in .agents/backlog/active_tasks.yml

import yaml
import sys

def update_status(task_id, new_status):
    # Implementation here
    pass

if __name__ == '__main__':
    update_status(sys.argv[1], sys.argv[2])
```

**`scripts/generate-lesson-file.sh`**:
```bash
#!/bin/bash
# Generate a lesson learned template

DATE=$(date +%Y-%m-%d)
SLUG="$1"
FILE=".agents/lessons_learned/${DATE}-${SLUG}.md"

cat > "$FILE" <<EOF
# Lesson: ${SLUG}

**Date**: ${DATE}
**Context**: 
**Severity**: 

---

## Problem Statement


## Root Cause


## Resolution


## Prevention


## Tags

---

**Captured by**: Cursor
EOF

echo "Created: $FILE"
```

---

## Templates

### Purpose
Standardized file templates for consistency.

### Available Templates (to be created)

**`templates/lesson_learned.md`**: Template for RCA files  
**`templates/pr_description.md`**: Standard PR description format  
**`templates/issue_rfc.md`**: RFC-style issue template  
**`templates/task.yml`**: YAML task entry template

Example usage by agent:
```bash
cp .agents/workflows/templates/lesson_learned.md \
   .agents/lessons_learned/2025-10-26-my-lesson.md
```

---

## Runbooks

### Purpose
Step-by-step guides for complex operations that agents may need to perform.

### Example Runbooks (to be created)

**`runbooks/release-process.md`**:
- How to prepare a release
- Checklist before tagging
- Post-release verification steps

**`runbooks/ci-failure-triage.md`**:
- How to parse CI logs
- Common failure patterns
- Escalation criteria

**`runbooks/dependency-update.md`**:
- How to safely update Cargo.toml
- Testing strategy for dep changes
- Rollback procedure if issues arise

**`runbooks/multi-agent-handoff.md`**:
- How to hand off work between agents
- Documentation requirements
- State synchronization checklist

---

## Usage Guidelines

### For Agents
1. **Before starting repetitive work**: Check if a script exists here
2. **When creating a pattern**: Extract to a script and document it
3. **When uncertain about a process**: Check runbooks first

### For Humans
- Review scripts before allowing agents to execute them
- Keep runbooks updated as processes evolve
- Add new workflows as the project scales

---

## Best Practices

### Script Guidelines
- **Idempotent**: Safe to run multiple times
- **Defensive**: Check for required files/env vars before proceeding
- **Verbose**: Echo what the script is doing (helps with debugging)
- **Exit codes**: Return 0 on success, non-zero on failure

### Template Guidelines
- **Placeholders**: Use `<PLACEHOLDER>` or `${VAR}` syntax
- **Comments**: Explain what each section is for
- **Examples**: Include sample content when helpful

### Runbook Guidelines
- **Step-by-step**: Numbered actions, one per step
- **Prerequisites**: List requirements upfront
- **Verification**: Include check steps after each action
- **Troubleshooting**: Add common issues and solutions

---

## Maintenance

### Weekly
- Test scripts after major codebase changes (ensure they still work)

### Monthly
- Review runbooks for accuracy (update if processes changed)
- Archive obsolete scripts/templates

### Quarterly
- Evaluate if new workflows are needed based on repetitive manual tasks

---

## Related Documents
- Agent guidance: `.agents/AGENTS.md`
- Task tracking: `.agents/backlog/`
- Lessons learned: `.agents/lessons_learned/`
