# Contributing to Hash Checker

Thanks for helping improve Hash Checker! This guide outlines the lightweight
process we follow so changes land smoothly and stay traceable.

## Workflow Overview

1. **Open an issue first.** Every bug fix or feature starts with a GitHub issue
   so decisions and evidence stay linked. Reference the issue number in branch
   names and commits. Use the templates under `.github/ISSUE_TEMPLATE/`.
2. **Create a topic branch.** Favour `feature/<descriptor>`, `fix/<descriptor>`,
   or `docs/<descriptor>` as documented in `.agents/AGENTS.yml`.
3. **Keep public docs high level.** Detailed runbooks, RCA notes, or secrets
   belong in the git-ignored `.agents/` workspace. Public files (`docs/`, `README.md`,
   etc.) should stay user-facing and free of sensitive information.
4. **Sync with existing guidance.** Review `docs/PLAN.md`, `docs/TASKS.md`,
   and `.agents/` notes relevant to your change before starting work.

## Development Environment

- Hash Checker targets **Rust 1.83+**. Make sure `rustup` or the Docker-based
  workflow uses that toolchain.
- The recommended local gate is:

  ```bash
  make ci-linux-local
  ```

  This runs `fmt`, `clippy`, unit tests (CLI + GUI), GUI smoke tests, and Docker
  integration checks. Capture the log (for example under `logs/ci-linux-<timestamp>.log`)
  and reference it in your PR description.
- For quick reference on other commands (GUI builds, packaging rehearsals, Vagrant
  smoke tests), see `docs/OPERATIONS.md`.

## Pull Request Expectations

- Include a concise summary plus links to issues (`Fixes #NN` when appropriate).
- Note any CI runs or manual verification you performed.
- Update documentation when behaviour changes:
  - Public docs stay under `docs/`.
  - Internal guardrails, lessons, or workflow tweaks go under `.agents/`.
- Screenshots or artefacts should follow the storage rules in
  `docs/GUI_SCREENSHOT.md` and related checklists.
- After merge, clean up topic branches (`git branch -d` and `git fetch --prune`).

## Code Style

- Follow `rustfmt` defaults (enforced by the CI gate).
- Treat `clippy` warnings as errors; fix or justify them within the PR.
- Prefer idiomatic Rust error handling and the existing logging conventions.

## Getting Help

If you get stuck, open a draft PR or start a discussion on the relevant GitHub
issue. Please include context (logs, screenshots, steps tried) so others can
assist quickly.

We appreciate every contribution—thanks for keeping Hash Checker healthy!
