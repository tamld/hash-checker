#!/usr/bin/env python3
"""
Telemetry Analysis Script
Purpose: Parse GUI telemetry logs and generate summary reports
Usage: python3 scripts/analyze_telemetry.py [log_file]
"""

import json
import sys
import os
from pathlib import Path
from datetime import datetime
from typing import Dict, List, Any

def parse_telemetry_log(log_file: Path) -> List[Dict[str, Any]]:
    """Parse telemetry log file and return structured data."""
    events = []
    
    if not log_file.exists():
        print(f"Warning: Log file {log_file} not found")
        return events
    
    try:
        with open(log_file, 'r') as f:
            for line_num, line in enumerate(f, 1):
                line = line.strip()
                if not line:
                    continue
                
                try:
                    event = json.loads(line)
                    event['_line_number'] = line_num
                    events.append(event)
                except json.JSONDecodeError as e:
                    print(f"Warning: Invalid JSON on line {line_num}: {e}")
                    continue
    except Exception as e:
        print(f"Error reading log file: {e}")
    
    return events

def analyze_events(events: List[Dict[str, Any]]) -> Dict[str, Any]:
    """Analyze telemetry events and generate summary."""
    if not events:
        return {"error": "No events found"}
    
    # Group events by type
    event_types = {}
    for event in events:
        event_type = event.get('type', 'unknown')
        if event_type not in event_types:
            event_types[event_type] = []
        event_types[event_type].append(event)
    
    # Calculate statistics
    total_events = len(events)
    unique_types = len(event_types)
    
    # Performance metrics
    performance_events = event_types.get('performance', [])
    avg_duration = 0
    if performance_events:
        durations = [e.get('duration_ms', 0) for e in performance_events if 'duration_ms' in e]
        if durations:
            avg_duration = sum(durations) / len(durations)
    
    # Error analysis
    error_events = event_types.get('error', [])
    error_count = len(error_events)
    
    # UI state transitions
    ui_events = event_types.get('ui_state', [])
    state_transitions = len(ui_events)
    
    return {
        "summary": {
            "total_events": total_events,
            "unique_event_types": unique_types,
            "error_count": error_count,
            "state_transitions": state_transitions,
            "avg_performance_ms": round(avg_duration, 2)
        },
        "event_types": {k: len(v) for k, v in event_types.items()},
        "errors": [e.get('message', 'Unknown error') for e in error_events[:5]],  # First 5 errors
        "performance_events": len(performance_events)
    }

def generate_report(analysis: Dict[str, Any], output_file: Path = None) -> str:
    """Generate markdown report from analysis."""
    if "error" in analysis:
        return f"# Telemetry Analysis Report\n\n**Error:** {analysis['error']}\n"
    
    summary = analysis["summary"]
    event_types = analysis["event_types"]
    errors = analysis.get("errors", [])
    
    report = f"""# Telemetry Analysis Report
Generated: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}

## Summary
- **Total Events:** {summary['total_events']}
- **Event Types:** {summary['unique_event_types']}
- **Errors:** {summary['error_count']}
- **State Transitions:** {summary['state_transitions']}
- **Avg Performance:** {summary['avg_performance_ms']}ms

## Event Types
"""
    
    for event_type, count in sorted(event_types.items()):
        report += f"- **{event_type}:** {count}\n"
    
    if errors:
        report += f"\n## Recent Errors\n"
        for error in errors:
            report += f"- {error}\n"
    
    report += f"\n## Performance Events\n- **Count:** {analysis.get('performance_events', 0)}\n"
    
    return report

def main():
    """Main function."""
    if len(sys.argv) < 2:
        print("Usage: python3 scripts/analyze_telemetry.py [log_file]")
        print("Example: python3 scripts/analyze_telemetry.py logs/gui-manifest/telemetry.log")
        sys.exit(1)
    
    log_file = Path(sys.argv[1])
    output_file = Path(sys.argv[2]) if len(sys.argv) > 2 else None
    
    print(f"Analyzing telemetry log: {log_file}")
    
    # Parse and analyze
    events = parse_telemetry_log(log_file)
    analysis = analyze_events(events)
    
    # Generate report
    report = generate_report(analysis, output_file)
    
    if output_file:
        output_file.write_text(report)
        print(f"Report written to: {output_file}")
    else:
        print(report)

if __name__ == "__main__":
    main()