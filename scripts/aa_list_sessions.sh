#!/bin/bash
# List all AA sessions (active and ended)
# Usage: aa_list_sessions.sh [--active|--ended|--all]

filter=${1:-"--active"}

echo "╔════════════════════════════════════════════════════════════════╗"
echo "║                    AA Sessions Registry                       ║"
echo "╚════════════════════════════════════════════════════════════════╝"
echo ""

if [ ! -f .agents/active/sessions.yml ]; then
  echo "⚠️  No sessions registry found (.agents/active/sessions.yml)"
  exit 0
fi

case $filter in
  --active)
    echo "🟢 ACTIVE SESSIONS:"
    echo "-------------------"
    yq '.sessions[] | select(.status=="active")' .agents/active/sessions.yml | \
      yq -o=json | \
      jq -r '
        "[\(.tag)]",
        "  Issue: #\(.work_context.issue_number) - \(.work_context.issue_title)",
        "  Started: \(.started_at)",
        "  Tasks: \(.claims.tasks | length)",
        "  Files: \(.claims.files | length)",
        "  Commits: \(.metrics.commits)",
        ""
      '
    ;;
  
  --ended)
    echo "⚫ ENDED SESSIONS:"
    echo "-----------------"
    yq '.sessions[] | select(.status=="ended")' .agents/active/sessions.yml | \
      yq -o=json | \
      jq -r '
        "[\(.tag)]",
        "  Issue: #\(.work_context.issue_number) - \(.work_context.issue_title)",
        "  Duration: \(.started_at) → \(.ended_at)",
        "  Commits: \(.metrics.commits)",
        ""
      '
    ;;
  
  --all)
    $0 --active
    echo ""
    $0 --ended
    ;;
  
  *)
    echo "Usage: aa_list_sessions.sh [--active|--ended|--all]"
    exit 1
    ;;
esac

# Summary
total=$(yq '.sessions | length' .agents/active/sessions.yml)
active=$(yq '.sessions[] | select(.status=="active") | .tag' .agents/active/sessions.yml | wc -l | tr -d ' ')
ended=$(yq '.sessions[] | select(.status=="ended") | .tag' .agents/active/sessions.yml | wc -l | tr -d ' ')

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Summary: $total total ($active active, $ended ended)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
