#!/usr/bin/env bash
set -euo pipefail

IMAGE="rust:1.83"
PROJECT_ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
LOCAL_UID=$(id -u)
LOCAL_GID=$(id -g)

exec docker run --rm \
    -e LOCAL_UID="$LOCAL_UID" \
    -e LOCAL_GID="$LOCAL_GID" \
    -v "${PROJECT_ROOT}:/workspace" \
    -w /workspace/rust/hash-checker \
    ${IMAGE} \
    bash -lc 'export PATH="/usr/local/cargo/bin:$PATH"; cargo test; if [ -d /workspace/rust/hash-checker/target ]; then chown -R ${LOCAL_UID}:${LOCAL_GID} /workspace/rust/hash-checker/target; fi'
