# GUI Screenshot Checklist

_Updated: 2025-10-13_

Goal: provide a consistent set of screenshots for README/docs whenever the GUI changes.

## Preparation
- Build the GUI in release mode: `cargo run --release --manifest-path rust/hash-checker-gui/Cargo.toml`.
- Toggle High contrast if you want to capture the dark theme variant.
- Capture at least 1440×900 using your platform shortcut (Cmd+Shift+4 on macOS, Snipping Tool on Windows).

## Required Shots
| Suggested filename | Content | Notes |
| --- | --- | --- |
| `gui-main.png` | Main screen with file picker, expected hash field, and Calculate button | Use default light theme, before entering a hash |
| `gui-match.png` | Verification success state (green message) | Show computed digest and success banner |
| `gui-mismatch.png` | Verification failure state (red message) | Include actual digest and error banner |
| `gui-algorithm.png` | Algorithm dropdown expanded | Focus on SHA-256 entry, keep other UI visible |
| `gui-high-contrast.png` | High contrast mode enabled | Highlight the toggle and dark color scheme |
| *(Optional)* `gui-theme-slate.png` | Slate theme preview | Capture after selecting the Slate theme (if updated) |

## Storage rules
- Save files in `docs/assets/` and keep the naming as listed above.
- Update README/docs references if filenames change.
- Record the update in `logs/assets/<yyyy-mm-dd>-screenshots.md` (include commit hash).

## Notes
- The CLI environment used by automation cannot capture GUI windows; screenshots must be taken on a workstation with a display.
- When themes change, refresh screenshots for each preset captured in docs to avoid confusing users.
- Capture order recommendation:
  1. `gui-main.png` (Soft Light theme, no hash entered).
  2. `gui-match.png` (Soft Light theme, matching hash message).
  3. `gui-mismatch.png` (Soft Light theme, mismatch message).
  4. `gui-algorithm.png` (Soft Light theme with dropdown open).
  5. `gui-high-contrast.png` (High Contrast theme toggled on).
  6. *(Optional)* `gui-theme-slate.png` (Slate theme overview, if we publish dark preview).
- After capturing, record the action in `logs/assets/<yyyy-mm-dd>-screenshots.md` with theme/version context.
