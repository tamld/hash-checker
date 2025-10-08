# Hash Checker Agent Guide

## Purpose
- Provide repo-specific guardrails and onboarding steps for agents working on the Hash Checker project.

## Sources of Truth
- Global MCP (core laws, guardrails, templates): refer to the secured MCP workspace configured via the `GLOBAL_MCP` environment variable.
- Repository documents: `README.md`, `docs/PLAN.md`, `docs/BACKLOG.md`, `docs/GOALS.md`, `docs/TASKS.md`, `docs/SECURITY_ROADMAP.md`, `docs/GUI_DECISION.md`, `docs/GUI_MVP_DESIGN.md`.

## Startup Checklist
1. Run the Global bootstrap summary to refresh shared policies:
   - `bash "$GLOBAL_MCP"/tools/bootstrap_orchestrator.sh --print-summary`
2. Read this guide, then review repo docs in order:
   - `README.md` → `docs/PLAN.md` → `docs/BACKLOG.md` → `docs/TASKS.md`.
3. Record open TODO/backlog items from the same files before starting new work.

## Workflow & Communication
- `workflow.mode`: `light` (solo project; direct commits acceptable after local verification).
- Stack note: Project is Rust-only; all runtime and tooling changes should target the Rust crates and supporting infrastructure.
- Commit to GitHub when tasks or milestones are completed to avoid large divergence; for major changes prefer opening a PR and merge it for clear tracking.
- Language: discuss in Vietnamese; write code, comments, commit messages in English.
- Default approach: minimal, reversible changes; keep plans short; add a short preamble before shell/tool calls.
- Workspace scope: restrict file edits and artefacts to the repository root (`local-scripts/hash-checker/`); do not introduce files elsewhere on the host.
- Build rule: when builds are required, invoke `make` targets that stage outputs under `/tmp` and avoid creating additional files or directories outside the project workspace.
- Issue hygiene: when addressing bugs or tech debt, document the root cause, update relevant roadmap/backlog entries, keep commits focused, and only close issues/PRs after CI (including Vagrant smoke where applicable) is green.
- Follow Global guardrails (MAIN-PROTECT-001, CI-PATH-001, SECRET-SHIELD-001, GLOBAL-MEM-PR-001).

## Testing & Build Expectations
- Unit tests: `make rust-test` (Dockerized). Use `make rust-gui-build` for GUI build validation when required.
- GUI smoke test: run `make rust-gui-smoke` (headless Vagrant VM) to exercise the Rust GUI in isolation.
- Packaging guidance lives in the project README; use the `cargo-packager` flow (`cargo packager --release --formats <fmt>`) for installers.

## Reporting Template
When finishing a task, include at minimum:
- Sources scanned: [...]
- Laws/Policies summary: [...]
- TODO/Backlog found: [...]
- Conflicts: [none | details]
- Proposed resolution: [...]

## Escalation
- If repository instructions ever conflict with Global laws, pause and request human confirmation before proceeding (LAW-REFLECT-001).
