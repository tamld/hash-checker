# AA Workspace - Start Here

**Welcome!** If you're an AI Assistant (AA) joining this project, this is your entry point.

---

## 🎯 **Quick Start (30 seconds)**

```yaml
1. Read this file (you are here)
2. Check available tasks → .agents/active/tasks.yml
3. Claim a task (follow protocol)
4. Use locks to avoid conflicts → .agents/active/locks.yml
5. Do the work
6. Mark task complete
```

---

## 📋 **How to Find Work**

### Step 1: Read Task List

```bash
# Location
.agents/active/tasks.yml

# Find available tasks
yq '.tasks[] | select(.status=="pending" and .blocked_by==null) | .id + ": " + .title' .agents/active/tasks.yml
```

### Step 2: Claim a Task

```bash
# Update task in tasks.yml
yq -i '(.tasks[] | select(.id=="TASK_ID")) |= (.status = "in_progress" | .assignee = "YOUR_NAME")' .agents/active/tasks.yml

# Commit
git add .agents/active/tasks.yml
git commit -m "task: claim TASK_ID"
git push
```

### Step 3: Do the Work

Follow task's validation criteria (success requirements)

### Step 4: Mark Complete

```bash
# Update status
yq -i '(.tasks[] | select(.id=="TASK_ID")) |= (.status = "completed")' .agents/active/tasks.yml

# Commit
git add .agents/active/tasks.yml
git commit -m "task: complete TASK_ID"
git push
```

---

## 🔒 **How to Avoid Conflicts**

### Critical Files Need Locks

Before editing these files, acquire lock:
- `.github/workflows/*.yml`
- `.agents/handoffs/*.md`
- `.agents/active/tasks.yml`
- `Cargo.toml`, `README.md`, etc.

### Lock Protocol

```bash
# 1. Check if locked
yq '.locks[] | select(.file=="PATH")' .agents/active/locks.yml

# 2. If not locked, acquire
yq -i '.locks += [{"file": "PATH", "holder": "YOUR_NAME", "acquired": "'$(date -u +%Y-%m-%dT%H:%M:%SZ)'"}]' .agents/active/locks.yml
git add .agents/active/locks.yml
git commit -m "lock: acquire PATH"
git push

# 3. Edit the file (you have exclusive access)

# 4. Commit your changes

# 5. Release lock
yq -i 'del(.locks[] | select(.file=="PATH"))' .agents/active/locks.yml
git commit -m "lock: release PATH"
git push
```

**Full details**: `.agents/active/locks.yml`

---

## 🗺️ **Workspace Structure**

```
.agents/
├── README.md (this file - START HERE)
│
├── active/
│   ├── tasks.yml (task registry - discover work here)
│   └── locks.yml (file locks - coordinate access)
│
├── handoffs/
│   └── *.md (session handoffs - context preservation)
│
├── lessons_learned/
│   └── *.md (proven lessons - learn from experience)
│
├── workflows/
│   └── *.md (process specs - how we work)
│
├── brainstorms/
│   └── *.md (proposals - ideas under discussion)
│
└── backlog/
    └── *.yml (planned work - future tasks)
```

---

## 🎓 **Key Documents**

### Operating Principles
- Location: `.agents/OPERATING_PRINCIPLES.md`
- Purpose: 7 core principles (Simplicity, Root Cause, Reality, etc.)
- When: Read before starting work

### Lesson Creation Workflow
- Location: `.agents/lessons_learned/CORRECT_LESSON_CREATION_WORKFLOW.md`
- Purpose: How to create validated lessons (not speculation)
- Process: Brainstorm → Experiment → Proven → THEN Lesson

### Latest Handoff
- Location: `.agents/handoffs/` (most recent date)
- Purpose: Current state, what happened, what's next
- When: Read to understand current context

---

## ⚠️ **Important Rules**

### 1. File Limit
- Max 3 new files per session
- Update existing rather than create new
- Consolidate when possible

### 2. Language Policy
- All files: 100% English
- User communication: Vietnamese OK
- Zero tolerance: No Vietnamese in files

### 3. Evidence-Based
- Test before document
- Measure before claim
- No >500 words without evidence

### 4. Priority Order
- Check handoff for priority list
- Do P1 (BLOCKING) first
- Don't skip to "interesting" tasks

### 5. Read Primary Sources
- Read handoff files (not summaries)
- Read tasks.yml (not memory)
- Read locks.yml (check before edit)

---

## 🤝 **Multi-AA Coordination**

### When Multiple AAs Work:

1. **Use tasks.yml**
   - Claim different tasks (parallel work)
   - Update status immediately (visibility)

2. **Use locks.yml**
   - Acquire lock before editing critical files
   - Release lock after commit
   - Prevents git conflicts

3. **Communicate**
   - Comment on tasks (GitHub issue/PR)
   - Update handoffs (document decisions)
   - Report completion (next AA knows state)

---

## 📊 **Current Status**

Check current state:
```bash
# Tasks overview
yq '.tasks | group_by(.status) | map({status: .[0].status, count: length})' .agents/active/tasks.yml

# Active locks
yq '.locks | length' .agents/active/locks.yml

# Latest handoff
ls -t .agents/handoffs/*.md | head -1
```

---

## 🚀 **Ready to Start?**

1. ✅ Read this README (done!)
2. → Check `.agents/active/tasks.yml` (find work)
3. → Read latest handoff (get context)
4. → Claim a task (follow protocol)
5. → Use locks (avoid conflicts)
6. → Do great work! 🎉

---

## ❓ **Need Help?**

- **Can't find task?** Check `.agents/active/tasks.yml`
- **File locked?** Check `.agents/active/locks.yml` - wait or contact holder
- **Confused?** Read latest handoff in `.agents/handoffs/`
- **Need context?** Check Operating Principles: `.agents/OPERATING_PRINCIPLES.md`

---

**Last Updated**: 2025-10-28T13:10:00Z by cursor  
**Status**: Operational (infrastructure ready for multi-AA)  
**Version**: 1.0 (initial release)
