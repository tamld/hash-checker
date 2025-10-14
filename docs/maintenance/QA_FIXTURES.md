# QA Fixtures Guide

_Updated: 2025-10-13_

## Current inventory
- `test-fixtures/sample.txt` is used by existing CLI/GUI smoke tests.

## Update workflow
1. Add new fixtures under `test-fixtures/` with descriptive names (e.g. `sample-10mb-large.bin`).
2. Compute reference digests and document them in this file.
3. Extend or add tests that consume the new fixtures (integration, GUI smoke, etc.).
4. Run `make ci-linux-local` before committing.

## Reference hashes
| File | SHA256 |
| --- | --- |
| sample.txt | 260948c8a3f06f47c92b8fe2db23d696705bc5801d7af840141de0466a94e52e |

## Logging
- When fixtures change, create `docs/reports/<yyyy-mm-dd>-fixtures.md` summarising the update and link it in the PR.
