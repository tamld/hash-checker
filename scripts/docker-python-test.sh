#!/usr/bin/env bash
set -euo pipefail

IMAGE="python:3.11-slim"
PROJECT_ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)

exec docker run --rm \
    --user "$(id -u):$(id -g)" \
    -v "${PROJECT_ROOT}:/workspace:ro" \
    -w /workspace \
    ${IMAGE} \
    bash -lc "pip install -r requirements-build.txt && python -m unittest discover tests"
