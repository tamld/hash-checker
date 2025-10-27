# Internal Backlog Tracking

> **Purpose**: Agent-managed task tracking using structured formats (YAML/JSON) for automation.
> **Scope**: Internal planning workspace; complements public `docs/TASKS.md` and `docs/BACKLOG.md`.

---

## Directory Structure

```
backlog/
├── README.md (this file)
├── active_tasks.yml (current sprint/cycle work)
├── blocked_tasks.yml (waiting on dependencies)
├── completed_tasks.yml (archive of finished work)
└── <agent-name>_tasks.yml (agent-specific queues)
```

---

## File Formats

### Active Tasks (`active_tasks.yml`)

```yaml
tasks:
  - id: task-001
    title: "Implement GTK4 dialog backend"
    status: in_progress
    priority: high
    assignee: cursor
    created: 2025-10-26
    due: 2025-11-15
    github_issue: 39
    dependencies: []
    notes: "Spike completed; implementation in progress"
    
  - id: task-002
    title: "Automate GUI snapshot harness"
    status: pending
    priority: medium
    assignee: cursor
    created: 2025-10-20
    due: 2025-11-30
    github_issue: 33
    dependencies: [task-001]
    notes: "Blocked on GTK4 migration"
```

### Blocked Tasks (`blocked_tasks.yml`)

```yaml
tasks:
  - id: task-003
    title: "SignPath integration"
    status: blocked
    blocker: "Awaiting OSS credentials"
    priority: high
    created: 2025-09-15
    github_issue: null
    notes: "Need to contact SignPath support"
```

### Completed Tasks (`completed_tasks.yml`)

Archive format (append-only):

```yaml
tasks:
  - id: task-999
    title: "Example completed task"
    status: completed
    completed_date: 2025-10-15
    github_issue: 20
    pull_request: 25
    notes: "Delivered directory hashing CLI support"
```

---

## Workflow

### 1. Task Creation
When starting new work:
1. Check if GitHub issue exists; create one if not
2. Add entry to `active_tasks.yml` with unique ID
3. Set status to `pending`
4. Link to GitHub issue number

### 2. Task Execution
When working on a task:
1. Update status to `in_progress`
2. Add progress notes as you work
3. If blocked, move to `blocked_tasks.yml` with blocker reason

### 3. Task Completion
When finishing a task:
1. Update status to `completed`
2. Move entry from `active_tasks.yml` to `completed_tasks.yml`
3. Link to merged PR number
4. Archive any related notes in `.agents/lessons_learned/`

### 4. Task Cancellation
If a task is no longer needed:
1. Update status to `cancelled`
2. Add cancellation reason in notes
3. Move to `completed_tasks.yml` (for historical record)

---

## Task Status Values

| Status       | Meaning                                |
|--------------|----------------------------------------|
| `pending`    | Not yet started; queued for work       |
| `in_progress`| Actively being worked on               |
| `blocked`    | Waiting on external dependency         |
| `review`     | Implementation done; awaiting PR review|
| `completed`  | Merged and deployed                    |
| `cancelled`  | No longer needed                       |

---

## Priority Levels

| Priority | Criteria                                      |
|----------|-----------------------------------------------|
| `critical`| Blocking release or causing production issues|
| `high`   | Important feature or significant bug          |
| `medium` | Nice-to-have improvement                      |
| `low`    | Backlog item; no immediate urgency            |

---

## Agent-Specific Queues

Create separate files for individual agents if needed:

**`cursor_tasks.yml`**: Tasks specifically managed by Cursor  
**`automation_tasks.yml`**: Bot-driven tasks (dependency updates, etc.)

Use same YAML schema as `active_tasks.yml`.

---

## Automation Integration

### Querying Tasks
Agents can parse YAML to:
- List all in-progress tasks
- Check blockers
- Find next pending task by priority

Example script:
```python
import yaml

with open('.agents/backlog/active_tasks.yml') as f:
    data = yaml.safe_load(f)
    in_progress = [t for t in data['tasks'] if t['status'] == 'in_progress']
    print(f"Currently working on: {len(in_progress)} tasks")
```

### Reporting
Generate status reports from YAML:
- Tasks completed this week
- Blocked tasks requiring attention
- Average task completion time

---

## Sync with Public Docs

### When to Update Public `docs/TASKS.md`
- After completing a major milestone
- When changing release priorities
- To communicate status to external contributors

### When to Keep Internal Only
- Detailed work-in-progress notes
- Internal blockers not ready to publicize
- Experimental tasks that may be cancelled

---

## Maintenance

### Weekly
- Review `active_tasks.yml` for stale entries
- Move completed tasks to archive
- Update blockers if dependencies resolved

### Monthly
- Analyze `completed_tasks.yml` for velocity trends
- Clean up old entries (keep last 3 months)
- Sync priorities with `docs/PLAN.md` roadmap

---

## Related Documents
- Public task tracker: `docs/TASKS.md`
- Public backlog: `docs/BACKLOG.md`
- Agent coordination: `.agents/agents_registry.md`
