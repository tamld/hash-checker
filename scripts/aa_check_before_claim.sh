#!/bin/bash
# aa_check_before_claim.sh - Pre-claim validation to prevent "ảo giác"
# Usage: aa_check_before_claim.sh <task-id> <session-tag>
# Example: aa_check_before_claim.sh task-123 codex-issue56-sessionB

set -e

TASK_ID=$1
SESSION_TAG=$2

if [ -z "$TASK_ID" ] || [ -z "$SESSION_TAG" ]; then
  echo "Usage: aa_check_before_claim.sh <task-id> <session-tag>"
  echo "Example: aa_check_before_claim.sh task-123 codex-issue56-sessionB"
  exit 1
fi

echo "🔍 Checking if task '$TASK_ID' is available for session '$SESSION_TAG'..."
echo ""

# Pull latest to get fresh data
echo "📥 Pulling latest changes..."
git pull --rebase --quiet

# Extract my AA type (e.g., "codex" from "codex-issue56-sessionB")
MY_AA=$(echo "$SESSION_TAG" | cut -d'-' -f1)

echo "✓ Your AA type: $MY_AA"
echo ""

# Check 1: Is another session of MY AA type already claiming this task?
echo "🔍 Check 1: Searching sessions.yml for conflicts..."

OTHER_SESSION=$(yq ".sessions[] | select(.claims.tasks[]? == \"$TASK_ID\") | select(.tag != \"$SESSION_TAG\") | .tag" .agents/active/sessions.yml 2>/dev/null | head -1)

if [ -n "$OTHER_SESSION" ]; then
  echo ""
  echo "❌ ===== CONFLICT DETECTED: 'ẢO GIÁC' PREVENTION ====="
  echo ""
  echo "   Task '$TASK_ID' is ALREADY CLAIMED by another session!"
  echo ""
  echo "   Your session:  $SESSION_TAG"
  echo "   Other session: $OTHER_SESSION"
  echo ""
  
  # Check if it's same AA type
  OTHER_AA=$(echo "$OTHER_SESSION" | cut -d'-' -f1)
  if [ "$OTHER_AA" == "$MY_AA" ]; then
    echo "   ⚠️  This is another $MY_AA session (same AA, different session)"
    echo "   ⚠️  This is the 'ảo giác' scenario User warned about!"
  else
    echo "   ⚠️  This is a different AA: $OTHER_AA"
  fi
  
  echo ""
  echo "   Options:"
  echo "     1. Choose a different task"
  echo "     2. Wait for $OTHER_SESSION to complete"
  echo "     3. If you control both sessions, coordinate manually"
  echo ""
  echo "======================================================"
  echo ""
  exit 1
fi

echo "✓ No other sessions have claimed this task"
echo ""

# Check 2: Is task available in tasks.yml?
echo "🔍 Check 2: Validating task status in tasks.yml..."

TASK_STATUS=$(yq ".tasks[] | select(.id==\"$TASK_ID\") | .status" .agents/active/tasks.yml 2>/dev/null)
TASK_ASSIGNEE=$(yq ".tasks[] | select(.id==\"$TASK_ID\") | .assignee" .agents/active/tasks.yml 2>/dev/null)

if [ -z "$TASK_STATUS" ]; then
  echo ""
  echo "❌ ERROR: Task '$TASK_ID' not found in tasks.yml"
  echo ""
  exit 1
fi

if [ "$TASK_STATUS" != "pending" ]; then
  echo ""
  echo "❌ ERROR: Task '$TASK_ID' is not available"
  echo "   Status: $TASK_STATUS"
  echo "   Assignee: $TASK_ASSIGNEE"
  echo ""
  echo "   Task must be 'pending' to claim"
  echo ""
  exit 1
fi

if [ "$TASK_ASSIGNEE" != "null" ] && [ "$TASK_ASSIGNEE" != "$MY_AA" ]; then
  echo ""
  echo "❌ ERROR: Task '$TASK_ID' already assigned"
  echo "   Assigned to: $TASK_ASSIGNEE"
  echo "   Your AA: $MY_AA"
  echo ""
  echo "   Task is assigned to a different AA"
  echo ""
  exit 1
fi

echo "✓ Task status: $TASK_STATUS (available)"
echo "✓ Task assignee: $TASK_ASSIGNEE (available for $MY_AA)"
echo ""

# Check 3: Does MY session already have active claims?
echo "🔍 Check 3: Checking if your session already has active claims..."

MY_CLAIMS=$(yq ".sessions[] | select(.tag==\"$SESSION_TAG\") | .claims.tasks | length" .agents/active/sessions.yml 2>/dev/null)

if [ -z "$MY_CLAIMS" ]; then
  MY_CLAIMS=0
fi

if [ "$MY_CLAIMS" -gt 0 ]; then
  ACTIVE_TASKS=$(yq ".sessions[] | select(.tag==\"$SESSION_TAG\") | .claims.tasks[]" .agents/active/sessions.yml 2>/dev/null)
  echo ""
  echo "⚠️  WARNING: Your session already has $MY_CLAIMS task(s) claimed:"
  echo "$ACTIVE_TASKS" | sed 's/^/     - /'
  echo ""
  echo "   Recommendation: Complete current task(s) before claiming new ones"
  echo "   (1 session = 1 task is recommended for clarity)"
  echo ""
  echo "   Continue claiming anyway? (You'll have multiple tasks in parallel)"
  read -p "   Type 'yes' to continue: " CONFIRM
  
  if [ "$CONFIRM" != "yes" ]; then
    echo ""
    echo "❌ Claim aborted by user"
    echo ""
    exit 1
  fi
  echo ""
fi

if [ "$MY_CLAIMS" -eq 0 ]; then
  echo "✓ Your session has no active claims (clean slate)"
  echo ""
fi

# All checks passed
echo "╔════════════════════════════════════════════════════════╗"
echo "║  ✅ ALL CHECKS PASSED - TASK IS AVAILABLE             ║"
echo "╚════════════════════════════════════════════════════════╝"
echo ""
echo "Task '$TASK_ID' is safe to claim for session '$SESSION_TAG'"
echo ""
echo "Next step:"
echo "  Use standard claiming process:"
echo "  yq -i '(.tasks[] | select(.id==\"$TASK_ID\")) |= (.status = \"in_progress\" | .assignee = \"$MY_AA\" | .started = \"'$(date -u +%Y-%m-%dT%H:%M:%SZ)'\")' .agents/active/tasks.yml"
echo "  yq -i '(.sessions[] | select(.tag==\"$SESSION_TAG\") | .claims.tasks) += [\"$TASK_ID\"]' .agents/active/sessions.yml"
echo "  git add .agents/active/tasks.yml .agents/active/sessions.yml"
echo "  git commit -m \"claim: $TASK_ID by $SESSION_TAG\""
echo "  git push"
echo ""
