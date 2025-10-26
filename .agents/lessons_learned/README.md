# Lessons Learned

> **Purpose**: Store post-incident analyses, root cause analyses (RCA), and insights from debugging sessions.
> **Audience**: Internal agent workspace; detailed technical content safe here.

---

## Directory Structure

```
lessons_learned/
├── README.md (this file)
├── YYYY-MM-DD-<topic>.md (individual lesson files)
└── index.json (optional: machine-readable index)
```

---

## File Naming Convention

Use format: `YYYY-MM-DD-<short-topic-slug>.md`

Examples:
- `2025-10-15-windows-stack-overflow-io-buffers.md`
- `2025-10-20-gtk3-dependency-removal-rfd-backend.md`
- `2025-10-26-agent-instruction-conflict-resolution.md`

---

## Template for Lesson Files

```markdown
# Lesson: <Title>

**Date**: YYYY-MM-DD  
**Context**: <Which phase/feature/issue>  
**Severity**: Low | Medium | High | Critical

---

## Problem Statement
<What went wrong or what was challenging>

## Root Cause
<Why it happened; technical details>

## Investigation Steps
1. <Step 1>
2. <Step 2>
...

## Resolution
<How it was fixed>

## Prevention
<What to do differently next time>

## Related Issues/PRs
- Issue #NN
- PR #MM

## Tags
`<tag1>`, `<tag2>`, `<tag3>`

---

**Captured by**: <Agent or human name>
```

---

## Usage Guidelines

### When to Create a Lesson File
- After resolving a non-trivial bug (especially if it caused CI failures or regressions)
- When discovering a platform-specific issue (Windows/macOS/Linux quirks)
- After a failed approach that teaches something valuable
- When documenting a workaround that may need revisiting

### What to Include
- **Technical details**: Stack traces, command outputs, error messages
- **Context**: What you were trying to do, why it matters
- **Investigation process**: Dead ends as well as successful paths
- **Actionable takeaways**: Specific things to check/avoid in future

### What NOT to Include
- Secrets or credentials (use env var references instead)
- Personally identifiable information (PII)
- Overly verbose logs (link to `logs/` directory instead)

---

## Indexing (Optional)

For machine-readable tracking, maintain `index.json`:

```json
{
  "lessons": [
    {
      "date": "2025-10-15",
      "slug": "windows-stack-overflow-io-buffers",
      "title": "Windows Stack Overflow Due to Large IO Buffers",
      "severity": "high",
      "tags": ["windows", "performance", "io", "buffer-allocation"],
      "resolved": true,
      "related_issues": [13]
    }
  ]
}
```

Update this when adding new lessons to enable:
- Automated reporting
- Tag-based searches
- Trend analysis (common failure patterns)

---

## Review Cadence

- **Weekly**: Tech lead spot-checks new lessons for patterns
- **Monthly**: Review lessons for documentation updates (move insights to public docs if broadly useful)
- **Quarterly**: Analyze tags and trends to inform roadmap priorities

---

## Related Documents
- Agent guidance: `.agents/AGENTS.md`
- Backlog tracking: `.agents/backlog/`
- Project roadmap: `docs/PLAN.md`
