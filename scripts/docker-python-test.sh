#!/usr/bin/env bash
set -euo pipefail

IMAGE="python:3.11-slim"
PROJECT_ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)

exec docker run --rm \
    -v "${PROJECT_ROOT}:/workspace:ro" \
    -w /workspace/legacy/python \
    ${IMAGE} \
    bash -lc "pip install --no-cache-dir -r requirements-build.txt && python -m unittest discover tests"
