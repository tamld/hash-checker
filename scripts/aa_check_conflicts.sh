#!/bin/bash
# Check for potential conflicts between active sessions
# Usage: aa_check_conflicts.sh

echo "🔍 Checking for conflicts between active sessions..."
echo ""

if [ ! -f .agents/active/sessions.yml ]; then
  echo "⚠️  No sessions registry found"
  exit 0
fi

# Get all active sessions
sessions=$(yq '.sessions[] | select(.status=="active") | .tag' .agents/active/sessions.yml)
session_count=$(echo "$sessions" | wc -l | tr -d ' ')

if [ "$session_count" -eq 0 ]; then
  echo "No active sessions"
  exit 0
fi

if [ "$session_count" -eq 1 ]; then
  echo "Only 1 active session - no conflicts possible"
  echo ""
  yq '.sessions[] | select(.status=="active") | "  [\(.tag)] \(.work_context.issue_title)"' .agents/active/sessions.yml
  exit 0
fi

echo "Active sessions: $session_count"
echo ""

# Check for file conflicts
conflicts_found=0

while IFS= read -r session1; do
  files1=$(yq ".sessions[] | select(.tag==\"$session1\") | .claims.files[]?" .agents/active/sessions.yml 2>/dev/null | sort)
  
  while IFS= read -r session2; do
    if [ "$session1" != "$session2" ] && [ "$session1" \< "$session2" ]; then
      files2=$(yq ".sessions[] | select(.tag==\"$session2\") | .claims.files[]?" .agents/active/sessions.yml 2>/dev/null | sort)
      
      # Find common files
      if [ -n "$files1" ] && [ -n "$files2" ]; then
        common=$(comm -12 <(echo "$files1") <(echo "$files2"))
        
        if [ -n "$common" ]; then
          conflicts_found=$((conflicts_found + 1))
          echo "⚠️  CONFLICT #$conflicts_found:"
          echo "   Sessions: $session1 ⇄ $session2"
          echo "   Shared files:"
          echo "$common" | sed 's/^/     - /'
          echo ""
        fi
      fi
    fi
  done <<< "$sessions"
done <<< "$sessions"

# Check for task conflicts
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

while IFS= read -r session1; do
  tasks1=$(yq ".sessions[] | select(.tag==\"$session1\") | .claims.tasks[]?" .agents/active/sessions.yml 2>/dev/null | sort)
  
  while IFS= read -r session2; do
    if [ "$session1" != "$session2" ] && [ "$session1" \< "$session2" ]; then
      tasks2=$(yq ".sessions[] | select(.tag==\"$session2\") | .claims.tasks[]?" .agents/active/sessions.yml 2>/dev/null | sort)
      
      # Find common tasks
      if [ -n "$tasks1" ] && [ -n "$tasks2" ]; then
        common=$(comm -12 <(echo "$tasks1") <(echo "$tasks2"))
        
        if [ -n "$common" ]; then
          echo "🚨 TASK CONFLICT:"
          echo "   Sessions: $session1 ⇄ $session2"
          echo "   Shared tasks:"
          echo "$common" | sed 's/^/     - /'
          echo ""
          conflicts_found=$((conflicts_found + 1))
        fi
      fi
    fi
  done <<< "$sessions"
done <<< "$sessions"

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

if [ $conflicts_found -eq 0 ]; then
  echo "✅ No conflicts detected - safe to work in parallel"
else
  echo "⚠️  Found $conflicts_found potential conflict(s)"
  echo "   Recommendation: Coordinate with other sessions before proceeding"
fi
