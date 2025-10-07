# Hash Checker Agent Guide

## Purpose
- Provide repo-specific guardrails and onboarding steps for agents working on the Hash Checker project.

## Sources of Truth
- Global MCP (core laws, guardrails, templates): refer to the secured MCP workspace configured via the `GLOBAL_MCP` environment variable.
- Repository documents: `README.md`, `docs/PLAN.md`, `docs/BACKLOG.md`, `docs/GOALS.md`, `docs/TASKS.md`, `docs/SECURITY_ROADMAP.md`, `tests/test_cases.md`.

## Startup Checklist
1. Run the Global bootstrap summary to refresh shared policies:
   - `bash "$GLOBAL_MCP"/tools/bootstrap_orchestrator.sh --print-summary`
2. Read this guide, then review repo docs in order:
   - `README.md` → `docs/PLAN.md` → `docs/BACKLOG.md` → `docs/TASKS.md` → `tests/test_cases.md`.
3. Record open TODO/backlog items from the same files before starting new work.

## Workflow & Communication
- `workflow.mode`: `light` (solo project; direct commits acceptable after local verification).
- Migration note: Python prototype is frozen; new work targets the Rust codebase per `docs/PLAN.md`.
- Language: discuss in Vietnamese; write code, comments, commit messages in English.
- Default approach: minimal, reversible changes; keep plans short; add a short preamble before shell/tool calls.
- Follow Global guardrails (MAIN-PROTECT-001, CI-PATH-001, SECRET-SHIELD-001, GLOBAL-MEM-PR-001).

## Testing & Build Expectations
- Unit tests: `python -m unittest discover tests` (legacy prototype) and `cargo test` inside `rust/hash-checker`.
- GUI smoke test: run `python -m hash_checker --gui` (legacy) until the Rust GUI is implemented.
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
