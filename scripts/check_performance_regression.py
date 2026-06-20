#!/usr/bin/env python3
"""
Performance Regression Check Script
Purpose: Compare current performance metrics against baseline golden master
Usage:
    python3 scripts/check_performance_regression.py CURRENT BASELINE
    python3 scripts/check_performance_regression.py CURRENT BASELINE --tolerance 0.05 --output logs/performance/report.md
"""

import argparse
import json
import sys
from datetime import datetime
from pathlib import Path
from typing import Any, Dict

def load_metrics(file_path: Path) -> Dict[str, Any]:
    """Load performance metrics from JSON file."""
    if not file_path.exists():
        raise FileNotFoundError(f"Metrics file not found: {file_path}")
    
    with open(file_path, 'r') as f:
        return json.load(f)

def compare_metrics(current: Dict[str, Any], baseline: Dict[str, Any], tolerance: float = 0.1) -> Dict[str, Any]:
    """Compare current metrics against baseline with tolerance."""
    results = {
        "timestamp": datetime.now().isoformat(),
        "tolerance": tolerance,
        "comparisons": [],
        "regressions": [],
        "improvements": [],
        "summary": {
            "total_metrics": 0,
            "regressions": 0,
            "improvements": 0,
            "within_tolerance": 0
        }
    }
    
    # Compare each metric
    for metric_name, current_value in current.items():
        if metric_name not in baseline:
            results["comparisons"].append({
                "metric": metric_name,
                "status": "new",
                "current": current_value,
                "baseline": None,
                "change_percent": None
            })
            continue
        
        baseline_value = baseline[metric_name]
        results["summary"]["total_metrics"] += 1
        
        # Calculate percentage change
        if baseline_value == 0:
            change_percent = float('inf') if current_value > 0 else 0
        else:
            change_percent = ((current_value - baseline_value) / baseline_value) * 100
        
        # Determine status
        if abs(change_percent) <= tolerance * 100:
            status = "within_tolerance"
            results["summary"]["within_tolerance"] += 1
        elif change_percent > tolerance * 100:
            status = "regression"
            results["summary"]["regressions"] += 1
            results["regressions"].append({
                "metric": metric_name,
                "current": current_value,
                "baseline": baseline_value,
                "change_percent": round(change_percent, 2)
            })
        else:
            status = "improvement"
            results["summary"]["improvements"] += 1
            results["improvements"].append({
                "metric": metric_name,
                "current": current_value,
                "baseline": baseline_value,
                "change_percent": round(change_percent, 2)
            })
        
        results["comparisons"].append({
            "metric": metric_name,
            "status": status,
            "current": current_value,
            "baseline": baseline_value,
            "change_percent": round(change_percent, 2)
        })
    
    return results

def generate_markdown_report(results: Dict[str, Any]) -> str:
    """Generate markdown report from comparison results."""
    summary = results["summary"]
    
    report = f"""# Performance Regression Report
Generated: {results['timestamp']}
Tolerance: {results['tolerance'] * 100}%

## Summary
- **Total Metrics:** {summary['total_metrics']}
- **Regressions:** {summary['regressions']}
- **Improvements:** {summary['improvements']}
- **Within Tolerance:** {summary['within_tolerance']}

## Status
"""
    
    if summary['regressions'] == 0:
        report += "✅ **No regressions detected**\n\n"
    else:
        report += f"❌ **{summary['regressions']} regressions detected**\n\n"
    
    # Regressions
    if results['regressions']:
        report += "## Regressions\n"
        for reg in results['regressions']:
            report += f"- **{reg['metric']}:** {reg['current']} (was {reg['baseline']}, +{reg['change_percent']}%)\n"
        report += "\n"
    
    # Improvements
    if results['improvements']:
        report += "## Improvements\n"
        for imp in results['improvements']:
            report += f"- **{imp['metric']}:** {imp['current']} (was {imp['baseline']}, {imp['change_percent']}%)\n"
        report += "\n"
    
    # All comparisons
    report += "## All Metrics\n"
    report += "| Metric | Current | Baseline | Change | Status |\n"
    report += "|--------|---------|----------|--------|--------|\n"
    
    for comp in results['comparisons']:
        status_emoji = {
            'within_tolerance': '✅',
            'regression': '❌',
            'improvement': '🚀',
            'new': '🆕'
        }.get(comp['status'], '❓')
        
        change_str = f"{comp['change_percent']}%" if comp['change_percent'] is not None else "N/A"
        report += f"| {comp['metric']} | {comp['current']} | {comp['baseline'] or 'N/A'} | {change_str} | {status_emoji} |\n"
    
    return report

def main():
    """Main function."""
    parser = argparse.ArgumentParser(description="Compare performance metrics against a baseline.")
    parser.add_argument("current", type=Path, help="Path to JSON file with current metrics")
    parser.add_argument("baseline", type=Path, help="Path to JSON file with baseline metrics")
    parser.add_argument(
        "--tolerance",
        type=float,
        default=0.1,
        help="Allowed percentage delta before flagging regression (default: 0.1 => 10%%)",
    )
    parser.add_argument(
        "--output",
        type=Path,
        help="Optional output file for the markdown report (default: logs/performance/performance_regression_report.md)",
    )
    args = parser.parse_args()
    
    current_file = args.current
    baseline_file = args.baseline
    tolerance = args.tolerance
    output_file = args.output or Path("logs/performance/performance_regression_report.md")
    
    try:
        # Load metrics
        current = load_metrics(current_file)
        baseline = load_metrics(baseline_file)
        
        # Compare
        results = compare_metrics(current, baseline, tolerance)
        
        # Generate report
        report = generate_markdown_report(results)
        
        # Output
        output_file.parent.mkdir(parents=True, exist_ok=True)
        output_file.write_text(report)
        print(f"Performance regression report written to: {output_file}")
        
        # Exit with error code if regressions found
        if results["summary"]["regressions"] > 0:
            print(f"❌ {results['summary']['regressions']} performance regressions detected!")
            sys.exit(1)
        else:
            print("✅ No performance regressions detected")
            sys.exit(0)
            
    except Exception as e:
        print(f"Error: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()
