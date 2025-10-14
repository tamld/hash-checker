# Legacy Cleanup Plan

_Updated: 2025-10-13_

## Python status
- No `.py` files remain; the project is Rust-only.
- If Python files appear in contributions, request a separate repo or a Rust port.

## CI fallback monitoring
- After two consecutive CI failures on the same platform, run `make ci-linux-local` and the Vagrant smoke tests.
- Document root cause in `docs/reports/<yyyy-mm-dd>-ci-fallback.md` to avoid repeat incidents.

## Periodic checklist
- [ ] Monthly: `git ls-files '*.py'` should return empty.
- [ ] Review scripts in `scripts/` and remove stale helpers.
- [ ] Ensure README/docs no longer reference legacy Python instructions.
