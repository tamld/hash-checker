#!/bin/bash
# AA Session Registration Helper
# Usage: aa_register_session.sh <aa_type> <issue_number> [focus]
# Example: aa_register_session.sh codex 56

set -e

aa_type=$1
issue_num=$2
focus=$3

if [ -z "$aa_type" ] || [ -z "$issue_num" ]; then
  echo "Usage: aa_register_session.sh <aa_type> <issue_number> [focus]"
  echo "Example: aa_register_session.sh codex 56"
  echo "         aa_register_session.sh codex 56 implementation"
  exit 1
fi

# Generate tag
if [ -n "$focus" ]; then
  tag="${aa_type}-issue${issue_num}-${focus}"
else
  tag="${aa_type}-issue${issue_num}"
fi

echo "🔍 Checking for existing session: $tag"

# Check if tag already exists
if grep -q "tag: $tag" .agents/active/sessions.yml 2>/dev/null; then
  echo "❌ ERROR: Session '$tag' already active"
  echo ""
  echo "Active sessions:"
  yq '.sessions[] | select(.status=="active") | "  - \(.tag) (\(.work_context.issue_title))"' .agents/active/sessions.yml
  exit 1
fi

# Fetch issue info from GitHub
echo "📡 Fetching issue info from GitHub..."
if command -v gh &> /dev/null; then
  issue_title=$(gh issue view $issue_num --json title -q .title 2>/dev/null || echo "Issue #${issue_num}")
  issue_url=$(gh issue view $issue_num --json url -q .url 2>/dev/null || echo "")
else
  echo "⚠️  Warning: gh CLI not found, using default issue title"
  issue_title="Issue #${issue_num}"
  issue_url=""
fi

# Register session
echo "📝 Registering session: $tag"

# Create sessions.yml if it doesn't exist
if [ ! -f .agents/active/sessions.yml ]; then
  cat > .agents/active/sessions.yml << EOF
# Active AA Sessions Registry
sessions: []
metadata:
  total_sessions: 0
  active_sessions: 0
  ended_sessions: 0
EOF
fi

# Add session entry
timestamp=$(date -u +%Y-%m-%dT%H:%M:%SZ)

yq -i ".sessions += [{
  \"tag\": \"$tag\",
  \"aa_type\": \"$aa_type\",
  \"work_context\": {
    \"type\": \"issue\",
    \"issue_number\": $issue_num,
    \"issue_title\": \"$issue_title\",
    \"issue_url\": \"$issue_url\"
  },
  \"started_at\": \"$timestamp\",
  \"started_by\": \"$(whoami)\",
  \"status\": \"active\",
  \"claims\": {
    \"tasks\": [],
    \"files\": []
  },
  \"metrics\": {
    \"commits\": 0,
    \"files_modified\": 0
  },
  \"last_activity\": \"$timestamp\"
}]" .agents/active/sessions.yml

# Update metadata
total=$(yq '.sessions | length' .agents/active/sessions.yml)
active=$(yq '.sessions[] | select(.status=="active") | .tag' .agents/active/sessions.yml | wc -l | tr -d ' ')

yq -i ".metadata.total_sessions = $total" .agents/active/sessions.yml
yq -i ".metadata.active_sessions = $active" .agents/active/sessions.yml
yq -i ".metadata.last_updated = \"$timestamp\"" .agents/active/sessions.yml

echo "✅ Session registered successfully!"
echo ""
echo "Tag: $tag"
echo "Issue: #$issue_num - $issue_title"
echo "Started: $timestamp"
echo ""
echo "Active sessions: $active"
