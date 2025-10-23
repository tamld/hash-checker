#!/usr/bin/env bash
# Handoff validation script (LL-014 enforcement)
# Usage: .agents/scripts/validate_handoff.sh <branch_name>

set -euo pipefail

BRANCH="${1:-}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

if [[ -z "$BRANCH" ]]; then
  echo "Usage: $0 <branch_name>"
  echo "Example: $0 feature/gui-gtk4-123-agemini-rcodex"
  exit 1
fi

echo "=== Handoff Validation for $BRANCH (LL-014) ==="
echo ""

if ! git rev-parse --verify "$BRANCH" >/dev/null 2>&1; then
  echo "❌ Branch '$BRANCH' not found"
  exit 1
fi

git fetch origin "$BRANCH" >/dev/null 2>&1 || true
PROGRESS_FILE=".agents/branch_progress.yml"

if ! git show "$BRANCH:$PROGRESS_FILE" >/dev/null 2>&1; then
  echo "❌ Missing: $PROGRESS_FILE not found in branch"
  echo "   Action: Copy .agents/templates/branch_progress_template.yml and fill sections"
  exit 1
fi

echo "✅ Found: $PROGRESS_FILE"
echo ""

CONTENT=$(git show "$BRANCH:$PROGRESS_FILE")

SECTIONS=(
  "context:"
  "roles:"
  "handoff_ready:"
  "handoff_checklist:"
  "milestones:"
  "verification:"
  "rollback:"
  "communication:"
  "metrics:"
)

MISSING=()
for section in "${SECTIONS[@]}"; do
  if ! echo "$CONTENT" | grep -q "^${section}"; then
    MISSING+=("$section")
  fi
done

if [[ ${#MISSING[@]} -gt 0 ]]; then
  echo "❌ Missing required sections:" 
  for missing in "${MISSING[@]}"; do
    echo "   - $missing"
  done
  exit 1
fi

echo "✅ Required sections present"
echo ""

HANDOFF_READY=$(echo "$CONTENT" | grep "^handoff_ready:" | awk '{print $2}')
if [[ "$HANDOFF_READY" != "true" ]]; then
  echo "⚠️  handoff_ready: $HANDOFF_READY (expected: true)"
  echo "   Action: Set handoff_ready: true when ready for next agent"
  exit 1
fi

echo "✅ handoff_ready: true"
echo ""

CONTEXT_WHY=$(echo "$CONTENT" | grep -A2 "why_this_approach:" | tail -1)
if echo "$CONTEXT_WHY" | grep -qi "Brief explanation"; then
  echo "⚠️  context.why_this_approach still template text"
  echo "   Action: Describe actual reasoning behind approach"
  exit 1
fi

echo "✅ context.why_this_approach filled"
echo ""

if ! echo "$CONTENT" | grep -A5 "verification:" | grep -q "command:"; then
  echo "⚠️  verification section missing test commands"
  echo "   Action: Add cargo/make commands and expected output"
  exit 1
fi

echo "✅ verification commands documented"
echo ""

if ! echo "$CONTENT" | grep -A5 "discussions:" | grep -q "location:"; then
  echo "⚠️  communication.discussions missing references"
  echo "   Action: Record PR, brainstorm, or issue links"
  exit 1
fi

echo "✅ communication references present"
echo ""

echo "=========================================="
echo "✅ Handoff validation PASSED"
echo "=========================================="
echo ""
echo "Next steps:"
echo "1. Update docs/TASKS.md or backlog with 'Ready for handoff' status."
echo "2. Commit: git commit -m 'docs: prepare handoff for task #X'"
echo "3. Notify next agent or tag @codex in PR"
