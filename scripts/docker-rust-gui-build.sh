#!/usr/bin/env bash
set -euo pipefail

IMAGE="rust:1.83"
PROJECT_ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)

exec docker run --rm \
    -v "${PROJECT_ROOT}:/workspace" \
    -w /workspace/rust/hash-checker-gui \
    ${IMAGE} \
    bash -lc 'apt-get update >/dev/null && apt-get install -y pkg-config libgtk-3-dev > /dev/null && export PATH="/usr/local/cargo/bin:$PATH"; cargo build --release'
