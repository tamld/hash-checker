#!/usr/bin/env bash
set -euo pipefail

PROJECT_ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)

export VAGRANT_CWD="$PROJECT_ROOT"

vagrant up
vagrant ssh -c "cd /workspace/legacy/python && python3 src/gui.py" || true
# Placeholder: swap to Rust GUI once available.
vagrant halt
