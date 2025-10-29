#!/bin/bash
# End an AA session
# Usage: aa_end_session.sh <tag>
# Example: aa_end_session.sh cursor-issue56-brainstorm

set -e

tag=$1

if [ -z "$tag" ]; then
  echo "Usage: aa_end_session.sh <tag>"
  echo ""
  echo "Active sessions:"
  yq '.sessions[] | select(.status=="active") | "  - \(.tag)"' .agents/active/sessions.yml
  exit 1
fi

# Check if session exists
if ! grep -q "tag: $tag" .agents/active/sessions.yml; then
  echo "❌ ERROR: Session '$tag' not found"
  exit 1
fi

# Check if already ended
status=$(yq ".sessions[] | select(.tag==\"$tag\") | .status" .agents/active/sessions.yml)
if [ "$status" = "ended" ]; then
  echo "⚠️  Session '$tag' is already ended"
  exit 0
fi

# Mark as ended
timestamp=$(date -u +%Y-%m-%dT%H:%M:%SZ)

echo "🔚 Ending session: $tag"

yq -i "(.sessions[] | select(.tag==\"$tag\")).status = \"ended\"" .agents/active/sessions.yml
yq -i "(.sessions[] | select(.tag==\"$tag\")).ended_at = \"$timestamp\"" .agents/active/sessions.yml

# Update metadata
active=$(yq '.sessions[] | select(.status=="active") | .tag' .agents/active/sessions.yml | wc -l | tr -d ' ')
ended=$(yq '.sessions[] | select(.status=="ended") | .tag' .agents/active/sessions.yml | wc -l | tr -d ' ')

yq -i ".metadata.active_sessions = $active" .agents/active/sessions.yml
yq -i ".metadata.ended_sessions = $ended" .agents/active/sessions.yml
yq -i ".metadata.last_updated = \"$timestamp\"" .agents/active/sessions.yml

# Show summary
echo "✅ Session ended successfully"
echo ""
echo "Tag: $tag"
echo "Ended: $timestamp"
echo ""
yq ".sessions[] | select(.tag==\"$tag\") | {
  started: .started_at,
  ended: .ended_at,
  commits: .metrics.commits,
  files_modified: .metrics.files_modified
}" .agents/active/sessions.yml
echo ""
echo "Active sessions remaining: $active"
