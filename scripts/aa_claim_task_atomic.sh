#!/bin/bash
# aa_claim_task_atomic.sh - Atomic task claiming with lock protection
# Usage: aa_claim_task_atomic.sh <task-id> <session-tag>
# Example: aa_claim_task_atomic.sh task-123 codex-issue56-sessionB
#
# This script provides ATOMIC task claiming to prevent race conditions:
#   1. Acquires lock on tasks.yml
#   2. Pulls latest changes (ensures fresh data)
#   3. Validates task availability
#   4. Claims task (updates tasks.yml + sessions.yml)
#   5. Pushes immediately
#   6. Releases lock

set -e

TASK_ID=$1
SESSION_TAG=$2

if [ -z "$TASK_ID" ] || [ -z "$SESSION_TAG" ]; then
  echo "Usage: aa_claim_task_atomic.sh <task-id> <session-tag>"
  echo "Example: aa_claim_task_atomic.sh task-123 codex-issue56-sessionB"
  exit 1
fi

# Extract my AA type
MY_AA=$(echo "$SESSION_TAG" | cut -d'-' -f1)

echo "╔════════════════════════════════════════════════════════╗"
echo "║       ATOMIC TASK CLAIMING (Lock-Protected)           ║"
echo "╚════════════════════════════════════════════════════════╝"
echo ""
echo "Task: $TASK_ID"
echo "Session: $SESSION_TAG"
echo "AA: $MY_AA"
echo ""

# Step 1: Check if session already has active claims (Work Limit: 1 task/session)
echo "═══ Step 1: Pre-flight checks ═══"
echo ""

MY_CLAIMS=$(yq ".sessions[] | select(.tag==\"$SESSION_TAG\") | .claims.tasks | length" .agents/active/sessions.yml 2>/dev/null || echo "0")

if [ "$MY_CLAIMS" -gt 0 ]; then
  ACTIVE_TASKS=$(yq ".sessions[] | select(.tag==\"$SESSION_TAG\") | .claims.tasks[]" .agents/active/sessions.yml 2>/dev/null)
  echo "❌ ERROR: Session $SESSION_TAG already has $MY_CLAIMS task(s) claimed:"
  echo "$ACTIVE_TASKS" | sed 's/^/   - /'
  echo ""
  echo "Work Limit: 1 session = 1 task maximum (prevents multi-tasking confusion)"
  echo ""
  echo "Complete current task before claiming another, or use a different session."
  echo ""
  exit 1
fi

echo "✓ Session has no active claims (ready to claim)"
echo ""

# Step 2: Acquire lock on tasks.yml
echo "═══ Step 2: Acquiring lock on tasks.yml ═══"
echo ""

LOCK_ID="tasks-yml-claim-$(date +%s)"

# Check if already locked
EXISTING_LOCK=$(yq ".locks[] | select(.resource==\"tasks.yml\")" .agents/active/locks.yml 2>/dev/null)

if [ -n "$EXISTING_LOCK" ]; then
  LOCK_HOLDER=$(echo "$EXISTING_LOCK" | yq ".holder")
  LOCK_TIME=$(echo "$EXISTING_LOCK" | yq ".acquired_at")
  echo "⏳ WARNING: tasks.yml is currently locked"
  echo "   Holder: $LOCK_HOLDER"
  echo "   Since: $LOCK_TIME"
  echo ""
  echo "Waiting 5 seconds for lock to release..."
  sleep 5
  
  # Check again
  EXISTING_LOCK=$(yq ".locks[] | select(.resource==\"tasks.yml\")" .agents/active/locks.yml 2>/dev/null)
  if [ -n "$EXISTING_LOCK" ]; then
    echo ""
    echo "❌ ERROR: tasks.yml still locked after waiting"
    echo "   Another session is claiming a task right now"
    echo ""
    echo "Options:"
    echo "  1. Wait and retry in 1-2 minutes"
    echo "  2. Choose a different task"
    echo ""
    exit 1
  fi
fi

# Acquire lock
yq -i ".locks += [{
  \"id\": \"$LOCK_ID\",
  \"resource\": \"tasks.yml\",
  \"holder\": \"$SESSION_TAG\",
  \"acquired_at\": \"$(date -u +%Y-%m-%dT%H:%M:%SZ)\",
  \"reason\": \"atomic claiming of $TASK_ID\"
}]" .agents/active/locks.yml

git add .agents/active/locks.yml
git commit -m "lock: acquire tasks.yml for claiming $TASK_ID by $SESSION_TAG" --quiet
git push --quiet

echo "✓ Lock acquired: $LOCK_ID"
echo ""

# Step 3: Pull latest and validate
echo "═══ Step 3: Pull latest and validate ═══"
echo ""

echo "📥 Pulling latest changes..."
git pull --rebase --quiet

# Validate task availability
TASK_STATUS=$(yq ".tasks[] | select(.id==\"$TASK_ID\") | .status" .agents/active/tasks.yml 2>/dev/null)
TASK_ASSIGNEE=$(yq ".tasks[] | select(.id==\"$TASK_ID\") | .assignee" .agents/active/tasks.yml 2>/dev/null)

if [ -z "$TASK_STATUS" ]; then
  echo "❌ ERROR: Task $TASK_ID not found in tasks.yml"
  echo ""
  echo "Releasing lock..."
  yq -i "del(.locks[] | select(.id==\"$LOCK_ID\"))" .agents/active/locks.yml
  git add .agents/active/locks.yml
  git commit -m "unlock: release after failed claim (task not found)" --quiet
  git push --quiet
  exit 1
fi

if [ "$TASK_STATUS" != "pending" ]; then
  echo "❌ ERROR: Task $TASK_ID is not available"
  echo "   Status: $TASK_STATUS"
  echo "   Assignee: $TASK_ASSIGNEE"
  echo ""
  echo "Another session claimed this task while we were acquiring the lock!"
  echo "This is the 'ảo giác' prevention in action ✅"
  echo ""
  echo "Releasing lock..."
  yq -i "del(.locks[] | select(.id==\"$LOCK_ID\"))" .agents/active/locks.yml
  git add .agents/active/locks.yml
  git commit -m "unlock: release after failed claim (task unavailable)" --quiet
  git push --quiet
  exit 1
fi

# Check if another session of same AA claimed it
if [ "$TASK_ASSIGNEE" == "$MY_AA" ]; then
  OTHER_SESSION=$(yq ".sessions[] | select(.claims.tasks[]? == \"$TASK_ID\") | .tag" .agents/active/sessions.yml 2>/dev/null | head -1)
  if [ -n "$OTHER_SESSION" ] && [ "$OTHER_SESSION" != "$SESSION_TAG" ]; then
    echo "❌ ERROR: Task $TASK_ID already claimed by another $MY_AA session"
    echo "   Session: $OTHER_SESSION"
    echo ""
    echo "This is the 'ảo giác' scenario - another session of SAME AA claimed it!"
    echo ""
    echo "Releasing lock..."
    yq -i "del(.locks[] | select(.id==\"$LOCK_ID\"))" .agents/active/locks.yml
    git add .agents/active/locks.yml
    git commit -m "unlock: release after failed claim (same AA conflict)" --quiet
    git push --quiet
    exit 1
  fi
fi

echo "✓ Task is available (status: $TASK_STATUS)"
echo ""

# Step 4: Claim task
echo "═══ Step 4: Claiming task ═══"
echo ""

# Update tasks.yml
yq -i "(.tasks[] | select(.id==\"$TASK_ID\")) |= (
  .status = \"in_progress\" |
  .assignee = \"$MY_AA\" |
  .started = \"$(date -u +%Y-%m-%dT%H:%M:%SZ)\"
)" .agents/active/tasks.yml

echo "✓ Updated tasks.yml (status: in_progress, assignee: $MY_AA)"

# Update sessions.yml
yq -i "(.sessions[] | select(.tag==\"$SESSION_TAG\") | .claims.tasks) += [\"$TASK_ID\"]" .agents/active/sessions.yml
yq -i "(.sessions[] | select(.tag==\"$SESSION_TAG\") | .last_activity) = \"$(date -u +%Y-%m-%dT%H:%M:%SZ)\"" .agents/active/sessions.yml

echo "✓ Updated sessions.yml (added to claims[])"
echo ""

# Commit both files
git add .agents/active/tasks.yml .agents/active/sessions.yml
git commit -m "claim: $TASK_ID by $SESSION_TAG" --quiet
git push --quiet

echo "✓ Committed and pushed claim"
echo ""

# Step 5: Release lock
echo "═══ Step 5: Releasing lock ═══"
echo ""

yq -i "del(.locks[] | select(.id==\"$LOCK_ID\"))" .agents/active/locks.yml
git add .agents/active/locks.yml
git commit -m "unlock: release tasks.yml after successful claim" --quiet
git push --quiet

echo "✓ Lock released"
echo ""

# Success!
echo "╔════════════════════════════════════════════════════════╗"
echo "║         ✅ TASK CLAIMED SUCCESSFULLY (ATOMIC)          ║"
echo "╚════════════════════════════════════════════════════════╝"
echo ""
echo "Task: $TASK_ID"
echo "Session: $SESSION_TAG"
echo "Status: in_progress"
echo "Assignee: $MY_AA"
echo ""
echo "Other sessions will see this claim when they pull ✅"
echo "'Ảo giác' prevented - only ONE session claimed this task!"
echo ""
