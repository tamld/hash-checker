#!/usr/bin/env bash
set -euo pipefail

PROJECT_ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
export VAGRANT_CWD="$PROJECT_ROOT"

vagrant up
set +e
vagrant ssh -c "cd /workspace/rust/hash-checker-gui && cargo run --release -- --smoke-test"
status=$?
set -e
vagrant halt
exit $status
