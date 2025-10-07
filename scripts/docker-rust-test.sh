#!/usr/bin/env bash
set -euo pipefail

IMAGE="rust:1.80"
PROJECT_ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)

exec docker run --rm \
    -v "${PROJECT_ROOT}:/workspace" \
    -w /workspace/rust/hash-checker \
    ${IMAGE} \
    bash -lc 'export PATH="/usr/local/cargo/bin:$PATH"; cargo test'
