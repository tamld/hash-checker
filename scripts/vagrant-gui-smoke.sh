#!/usr/bin/env bash
set -euo pipefail

PROJECT_ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)

export VAGRANT_CWD="$PROJECT_ROOT"

vagrant up
vagrant ssh -c "cd /workspace && python3 src/gui.py" || true
# Placeholder: once Rust GUI exists, update command accordingly (e.g. running compiled GUI).
vagrant halt
