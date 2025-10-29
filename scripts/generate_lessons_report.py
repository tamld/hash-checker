#!/usr/bin/env python3
"""Generate lessons report (stub)."""
import json
from pathlib import Path

try:
    import yaml
except ImportError:
    raise SystemExit('Missing dependency: PyYAML')

INDEX_PATH = Path('.agents/knowledge/lessons_index.yml')
OUTPUT_MD = Path('.agents/knowledge/LESSONS_REPORT.md')
OUTPUT_JSON = Path('.agents/knowledge/lessons_metrics.json')

def main():
    data = yaml.safe_load(INDEX_PATH.read_text())
    lessons = data.get('lessons', [])

    lines_md = ['# Lessons Report', '', f'Total lessons: {len(lessons)}', '']
    for item in lessons:
        lesson_id = item.get('id', 'unknown')
        title = item.get('title', 'Untitled')
        status = item.get('status', 'unknown')
        lines_md.append(f'- **{lesson_id}** — {title} ({status})')
    lines_md.append('')
    OUTPUT_MD.write_text('\n'.join(lines_md))

    metrics = {'total_lessons': len(lessons), 'status_breakdown': {}}
    for item in lessons:
        status = item.get('status', 'unknown')
        metrics['status_breakdown'][status] = metrics['status_breakdown'].get(status, 0) + 1
    OUTPUT_JSON.write_text(json.dumps(metrics, indent=2))

    print('Generated lesson report stubs.')

if __name__ == '__main__':
    main()
