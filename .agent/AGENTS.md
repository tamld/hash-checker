# Hash Checker Agent Guide

## Purpose
- Provide repo-specific guardrails and onboarding steps for agents working on the Hash Checker project.

## Sources of Truth
- Global MCP (core laws, guardrails, templates): `/Users/tamld/Documents/MCP-Server`.
- Repository documents: `README.md`, `PLAN.md`, `BACKLOG.md`, `GOALS.md`, `TASKS.md`, `tests/test_cases.md`.

## Startup Checklist
1. Run the Global bootstrap summary to refresh shared policies:
   - `bash /Users/tamld/Documents/MCP-Server/tools/bootstrap_orchestrator.sh --print-summary`
2. Read this guide, then review repo docs in order:
   - `README.md` → `PLAN.md` → `BACKLOG.md` → `TASKS.md` → `tests/test_cases.md`.
3. Record open TODO/backlog items from the same files before starting new work.

## Workflow & Communication
- `workflow.mode`: `light` (solo project; direct commits acceptable after local verification).
- Language: discuss in Vietnamese; write code, comments, commit messages in English.
- Default approach: minimal, reversible changes; keep plans short; add a short preamble before shell/tool calls.
- Follow Global guardrails (MAIN-PROTECT-001, CI-PATH-001, SECRET-SHIELD-001, GLOBAL-MEM-PR-001).

## Testing & Build Expectations
- Unit tests: `python -m unittest discover tests`.
- GUI smoke test: run `python -m hash_checker --gui` (or equivalent entry point) in a clean environment.
- Packaging guidance lives in the project README; prefer PyInstaller builds executed inside disposable VMs/containers per project policy.

## Reporting Template
When finishing a task, include at minimum:
- Sources scanned: [...]
- Laws/Policies summary: [...]
- TODO/Backlog found: [...]
- Conflicts: [none | details]
- Proposed resolution: [...]

## Escalation
- If repository instructions ever conflict with Global laws, pause and request human confirmation before proceeding (LAW-REFLECT-001).
