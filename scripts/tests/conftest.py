from __future__ import annotations

import json
import sys
from pathlib import Path
from typing import Dict, Iterable

import pytest


PROJECT_ROOT = Path(__file__).resolve().parents[2]
SCRIPTS_DIR = PROJECT_ROOT / "scripts"

# Ensure the telemetry scripts can be imported as modules during tests.
if str(SCRIPTS_DIR) not in sys.path:
    sys.path.insert(0, str(SCRIPTS_DIR))


@pytest.fixture
def telemetry_events() -> Iterable[Dict[str, object]]:
    """Representative telemetry events for unit tests."""
    return [
        {"type": "performance", "duration_ms": 120, "message": "scan_complete"},
        {"type": "error", "message": "network timeout"},
        {"type": "ui_state", "state": "overview"},
        {"type": "performance", "duration_ms": 80, "message": "scan_complete"},
    ]


@pytest.fixture
def telemetry_log_file(tmp_path: Path, telemetry_events: Iterable[Dict[str, object]]) -> Path:
    """Write telemetry events to a temporary log file (JSON lines)."""
    log_file = tmp_path / "telemetry.log"
    with log_file.open("w", encoding="utf-8") as fp:
        for event in telemetry_events:
            fp.write(json.dumps(event))
            fp.write("\n")
    return log_file


@pytest.fixture
def current_metrics_file(tmp_path: Path) -> Path:
    """Create a current metrics JSON file for regression comparisons."""
    metrics_file = tmp_path / "current.json"
    metrics = {
        "scan_duration_ms": 230,
        "memory_usage_mb": 512,
        "ui_render_ms": 48,
    }
    metrics_file.write_text(json.dumps(metrics), encoding="utf-8")
    return metrics_file


@pytest.fixture
def baseline_metrics_file(tmp_path: Path) -> Path:
    """Create a baseline metrics JSON file for regression comparisons."""
    metrics_file = tmp_path / "baseline.json"
    metrics = {
        "scan_duration_ms": 200,
        "memory_usage_mb": 520,
        "ui_render_ms": 50,
    }
    metrics_file.write_text(json.dumps(metrics), encoding="utf-8")
    return metrics_file
