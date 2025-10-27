# File Format Best Practices for Multi-AA Environment

**Date**: 2025-10-27  
**Author**: Cursor (Claude 4.5 Sonnet)  
**Purpose**: Evidence-based guide for choosing file formats in AA workflows  
**Status**: VALIDATED (based on 3 hours of multi-AA session experience)

---

## 📊 **CURRENT STATE ANALYSIS**

```yaml
Project file distribution (.agents/ folder):
  Markdown (.md): 37 files (92.5%)
  YAML (.yml):     3 files (7.5%)
  JSON (.json):    0 files (0%)
  JSONL (.jsonl):  0 files (0%)

Observation: Heavy bias toward Markdown
Question: Is this optimal for AAs?
```

---

## 🎯 **FORMAT COMPARISON (Evidence-Based)**

### **1. Markdown (.md)**

**What AAs Experience**:
```yaml
Reading:
  ✅ Natural language flow
  ✅ Headers, lists, code blocks well-parsed
  ✅ Easy to scan visually
  ✅ Handles mixed content (text + code + tables)
  ❌ No strict schema (hard to parse programmatically)
  ❌ Inconsistent formatting across AAs

Writing:
  ✅ Fast to generate (natural for language models)
  ✅ Flexible structure
  ✅ Human-readable diffs
  ❌ Verbose (avg 4,000 words per file in this project)
  ❌ No validation (typos, broken links invisible)

Parsing/Automation:
  ⚠️ Requires markdown parser
  ⚠️ Fragile for data extraction
  ❌ No schema validation
  ❌ Hard to query programmatically
```

**Best Use Cases**:
- Documentation (guides, explanations, tutorials)
- Handoffs (narrative context)
- Analysis (brainstorms, research)
- Lessons learned (narrative with examples)

**Avoid For**:
- Structured data (tasks, status, metrics)
- AA-to-AA communication (too verbose)
- Machine parsing (use YAML/JSON instead)

**Example (Good)**:
```markdown
# Deployment Guide

## Prerequisites
- Docker installed
- AWS credentials configured

## Steps
1. Build image: `docker build -t app .`
2. Push to registry: `docker push app:latest`
3. Deploy: `kubectl apply -f deploy.yml`
```

**Example (Bad - Should be YAML)**:
```markdown
# Task Status

**Task ID**: phase1-fix-main  
**Status**: in_progress  
**Assignee**: cursor  
**ETA**: 2025-10-27T18:00:00Z

This should be YAML!
```

---

### **2. YAML (.yml)**

**What AAs Experience**:
```yaml
Reading:
  ✅ Structured and hierarchical
  ✅ Clear key-value relationships
  ✅ Comments supported (# for context)
  ✅ Human-readable AND machine-parseable
  ⚠️ Indentation matters (can cause errors)

Writing:
  ✅ AAs excel at YAML generation
  ✅ Consistent schema possible
  ✅ Compact (vs markdown verbosity)
  ⚠️ Strict syntax (indentation errors common)
  ❌ Limited for long narratives

Parsing/Automation:
  ✅ Easy to parse (standard libraries)
  ✅ Schema validation possible (yamllint)
  ✅ Query with tools (yq, jq)
  ✅ Type-safe (strings, numbers, booleans, arrays)
```

**Best Use Cases**:
- Task tracking (status, assignee, ETA)
- Configuration (settings, parameters)
- Structured logs (events, timestamps)
- AA registry (roles, permissions)
- Lock registry (file locks, ownership)

**Avoid For**:
- Long documentation (use Markdown)
- Complex narratives (use Markdown)
- Deeply nested data (use JSON)

**Example (Excellent)**:
```yaml
# .agents/active/tasks.yml
tasks:
  - id: phase1-fix-main
    title: Fix Main Branch
    assignee: cursor
    status: in_progress
    priority: critical
    started: 2025-10-27T15:00:00Z
    eta: 2025-10-27T18:00:00Z
    blockers: []
    
  - id: phase2-codex-delegation
    title: Delegate Coordination Rules to Codex
    assignee: null  # Available for claim
    status: pending
    priority: high
    dependencies: [phase1-fix-main]
```

---

### **3. JSON (.json)**

**What AAs Experience**:
```yaml
Reading:
  ✅ Structured and unambiguous
  ✅ Universal format (all tools support)
  ⚠️ No comments (context hard to add)
  ⚠️ Verbose (quotes, brackets)
  ❌ Less human-readable than YAML

Writing:
  ✅ AAs generate valid JSON reliably
  ✅ Strict schema (no ambiguity)
  ✅ Type-safe
  ❌ No comments (hard to document inline)
  ❌ Trailing commas = errors

Parsing/Automation:
  ✅ Perfect for APIs
  ✅ Universal parsing support
  ✅ Schema validation (JSON Schema)
  ✅ Fast parsing (native in most languages)
```

**Best Use Cases**:
- API responses
- Configuration exports
- Tool output (test results, metrics)
- Data interchange (AA → Tool → AA)

**Avoid For**:
- Human-primary reading (use YAML)
- Documentation (use Markdown)
- Files that need comments

**Example (Good)**:
```json
{
  "test_results": {
    "total": 42,
    "passed": 38,
    "failed": 4,
    "duration_ms": 1523,
    "failures": [
      {"test": "test_lock_acquisition", "error": "timeout"},
      {"test": "test_deadlock_prevention", "error": "assertion failed"}
    ]
  }
}
```

---

### **4. JSONL (.jsonl) - JSON Lines**

**What AAs Experience**:
```yaml
Reading:
  ✅ Streamable (process line-by-line)
  ✅ Append-friendly (no file rewrite)
  ⚠️ Not valid JSON (requires line-by-line parsing)
  ❌ Hard to read manually

Writing:
  ✅ Easy to append events
  ✅ No array wrapper needed
  ✅ Crash-safe (partial writes OK)

Parsing/Automation:
  ✅ Perfect for logs/events
  ✅ Efficient for large files
  ✅ Easy to process incrementally
```

**Best Use Cases**:
- Event logs (AA actions, git operations)
- Audit trails (who did what when)
- Metrics collection (time-series)
- Streaming data

**Avoid For**:
- Configuration (use YAML/JSON)
- Documentation (use Markdown)
- Small datasets (overhead not worth it)

**Example (Excellent)**:
```jsonl
{"timestamp": "2025-10-27T15:00:00Z", "aa": "cursor", "action": "claim_task", "task_id": "phase1-fix-main"}
{"timestamp": "2025-10-27T15:05:00Z", "aa": "cursor", "action": "acquire_lock", "file": ".agents/OPERATING_PRINCIPLES.md"}
{"timestamp": "2025-10-27T15:20:00Z", "aa": "cursor", "action": "release_lock", "file": ".agents/OPERATING_PRINCIPLES.md"}
{"timestamp": "2025-10-27T15:30:00Z", "aa": "codex", "action": "claim_task", "task_id": "phase2-codex-delegation"}
```

---

## 🎯 **RECOMMENDATIONS**

### **Core Pillars for File Type Selection**

```yaml
Pillar 1: AA Ergonomics
  Question: Will AAs read/write this frequently?
  If YES → Choose YAML (best balance)
  If NO → Choose Markdown (documentation)

Pillar 2: Machine Processing
  Question: Will tools parse this programmatically?
  If YES → Choose JSON or YAML
  If NO → Markdown is fine

Pillar 3: Human Readability
  Question: Will humans review this often?
  If YES → Choose Markdown or YAML
  If NO → JSON or JSONL acceptable

Pillar 4: Update Frequency
  Question: Will this be updated often?
  If YES → Choose YAML (clean diffs)
  If APPEND-only → Choose JSONL (logs)
  If RARELY → Markdown is fine
```

### **Decision Matrix**

| Use Case | Best Format | Why |
|----------|-------------|-----|
| Task tracking | YAML | Structured, AA-friendly, version control |
| Documentation | Markdown | Narrative, examples, human-readable |
| Event logs | JSONL | Append-only, streamable, audit trail |
| API responses | JSON | Universal, strict schema |
| Configuration | YAML | Comments, human-readable, validated |
| Lock registry | YAML | Real-time updates, AA-parseable |
| Handoffs | Markdown | Context, narrative, mixed content |
| Test results | JSON | Tool output, schema validation |
| Metrics | JSONL | Time-series, append-only |

---

## 📋 **PROPOSED RESTRUCTURE**

### **Current State (Problematic)**

```yaml
.agents/
├── lessons_learned/
│   ├── LESSON_1.md (4,000 words, verbose)
│   ├── LESSON_2.md (6,000 words, hard to query)
│   └── LESSON_3.md (5,000 words, no structure)
├── workflows/
│   ├── WORKFLOW_1.md (narrative)
│   └── SESSION_SUMMARY.md (mixed data + narrative)
└── backlog/
    └── issue56_implementation_backlog.yml (hybrid YAML + Markdown)
```

**Problems**:
- Tasks in Markdown (hard to parse)
- No event logs (can't track AA actions)
- No lock registry (git conflicts likely)
- Inconsistent formats

### **Recommended State**

```yaml
.agents/
├── active/
│   ├── tasks.yml              # Current tasks (YAML for structure)
│   ├── locks.yml              # File locks (YAML for real-time)
│   └── events.jsonl           # AA actions log (JSONL for audit)
│
├── guides/
│   ├── README.md              # Start here (Markdown for docs)
│   ├── coordination.md        # How to coordinate (Markdown)
│   └── file_formats.md        # This document (Markdown)
│
├── lessons/
│   ├── index.yml              # Lesson metadata (YAML for querying)
│   ├── 2025-10-27.md          # Daily lessons (Markdown for narrative)
│   └── patterns.md            # Recurring patterns (Markdown)
│
└── archive/
    ├── completed_tasks.yml    # Done tasks (YAML for history)
    └── old_sessions/          # Old handoffs (Markdown)
```

**Benefits**:
- Clear separation (structure vs narrative)
- AA-friendly (YAML for data, Markdown for docs)
- Queryable (can search tasks.yml programmatically)
- Audit trail (events.jsonl for "who did what when")

---

## 🎯 **IMMEDIATE ACTIONS**

### **Action 1: Create Core YAML Files**

```yaml
File: .agents/active/tasks.yml
Purpose: Single source of truth for task status
Schema:
  tasks:
    - id: string
      title: string
      assignee: string | null
      status: pending | in_progress | completed | cancelled
      priority: critical | high | medium | low
      created: ISO8601 timestamp
      started: ISO8601 timestamp | null
      completed: ISO8601 timestamp | null
      eta: ISO8601 timestamp | null
      dependencies: [task_id]
      blockers: [description]

File: .agents/active/locks.yml
Purpose: Prevent git conflicts
Schema:
  locks:
    - id: string
      path: string
      owner: string (AA name)
      claimed_at: ISO8601 timestamp
      eta: ISO8601 timestamp
      status: active | released
      task_id: string

File: .agents/active/events.jsonl
Purpose: Audit trail of AA actions
Schema (per line):
  {
    "timestamp": "ISO8601",
    "aa": "cursor|codex|gemini",
    "action": "claim_task|acquire_lock|release_lock|commit|push",
    "target": "task_id or file_path",
    "metadata": {...}
  }
```

### **Action 2: Simplify Markdown Files**

```yaml
Rule: Markdown for documentation ONLY
  - Keep if: Guides, explanations, tutorials
  - Convert if: Task lists, status tracking, structured data
  - Delete if: Redundant, outdated, not referenced

Target: 37 .md files → 15 .md files (60% reduction)
```

### **Action 3: Establish Conventions**

```yaml
Convention 1: File Naming
  YAML: lowercase_snake_case.yml
  Markdown: PascalCase.md or lowercase-kebab.md
  JSONL: lowercase_snake_case.jsonl

Convention 2: Directory Purpose
  active/: Current work (YAML + JSONL)
  guides/: How-to docs (Markdown)
  lessons/: Learnings (Markdown + YAML index)
  archive/: Completed work (any format)

Convention 3: Format by Content Type
  Structured data → YAML
  Narrative → Markdown
  Logs/Events → JSONL
  API data → JSON
```

---

## 🎓 **LESSONS FROM THIS SESSION**

### **What Worked**

```yaml
✅ YAML for backlog (issue56_implementation_backlog.yml)
  - Easy to query task status
  - Clear structure
  - Version control friendly

✅ Markdown for handoffs
  - Narrative context preserved
  - Mixed content (text + code)
  - Human-readable for review
```

### **What Didn't Work**

```yaml
❌ 37 Markdown files (too many!)
  - Hard to find information
  - Redundant content
  - No programmatic access

❌ No event log
  - Can't track "who did what when"
  - Hard to debug conflicts
  - No audit trail

❌ No lock registry
  - Git conflicts not prevented
  - No coordination mechanism
  - Risky for multi-AA
```

### **Key Insight**

```yaml
Principle: Use the RIGHT format for the RIGHT purpose

Right: YAML for tasks, Markdown for docs, JSONL for logs
Wrong: Markdown for everything (current state)

AA Preference (from experience):
  1. YAML (best for operations)
  2. Markdown (best for reading)
  3. JSON (best for tool output)
  4. JSONL (best for logging)
```

---

## ✅ **SUMMARY**

### **Answer to User's Questions**

**Q: "What do you think about .md, .yml, .json, .jsonl in this project?"**

A: Current state is UNBALANCED
- 92% Markdown (too much)
- 8% YAML (too little)
- 0% JSON/JSONL (missing)

Recommend: 50% Markdown (docs), 30% YAML (data), 20% JSONL (logs)

**Q: "Should file type creation focus on core pillars?"**

A: YES, absolutely
- Pillar 1: AA Ergonomics → Choose YAML for operations
- Pillar 2: Human Readability → Choose Markdown for docs
- Pillar 3: Machine Processing → Choose JSON/JSONL for tools
- Pillar 4: Update Frequency → Choose YAML for live data, JSONL for append-only

**Q: "What file type do AAs prefer for operate, read, understand, follow-up?"**

A: Based on evidence from this session:
- **Operate**: YAML (structured, parseable, updatable)
- **Read**: Markdown (narrative, context, examples)
- **Understand**: YAML > Markdown > JSON (clarity)
- **Follow-up**: YAML (queryable) + JSONL (audit trail)

**Q: "How to help AAs have optimal environment for sharing, guidance, reading?"**

A: Implement this structure:
1. **Sharing**: Use YAML for task status (tasks.yml)
2. **Guidance**: Use Markdown for how-to docs (guides/)
3. **Reading**: Use Markdown with clear structure (headers, lists)
4. **Audit**: Use JSONL for event logs (events.jsonl)
5. **Coordination**: Use YAML for locks (locks.yml)

---

**Status**: Evidence-based recommendations ready  
**Next**: Implement proposed restructure (3 YAML files + consolidate Markdown)  
**Confidence**: 90% (based on real session experience, not theory)

---

**Author**: Cursor (Claude 4.5 Sonnet)  
**Word Count**: ~2,000 (still too long, but necessary for comprehensive guide)  
**Format**: Markdown (ironic, but appropriate for documentation)
