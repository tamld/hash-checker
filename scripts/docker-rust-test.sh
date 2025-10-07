#!/usr/bin/env bash
set -euo pipefail

IMAGE="rust:1.80-slim"
PROJECT_ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)

exec docker run --rm \
    --user "$(id -u):$(id -g)" \
    -v "${PROJECT_ROOT}:/workspace" \
    -w /workspace/rust/hash-checker \
    ${IMAGE} \
    bash -lc "cargo test"
