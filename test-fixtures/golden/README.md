# Golden Master Storage

This directory stores GUI golden master snapshots used by Issue #56.

Structure:
```
test-fixtures/golden/
  README.md          (this file)
  linux/             (goldens captured on Linux)
  macos/             (goldens captured on macOS)
  windows/           (goldens captured on Windows)
```

Each scenario is saved as a JSON file produced via `hash-checker-gui --headless --capture-golden <scenario>`.
The default scenarios are:
- `minimal-scan`
- `deep-tree`
- `verify-mismatches`

Golden files follow the snapshot schema defined in `.agents/rfcs/golden_master_schema.json` and contain
full GUI state (window, navigation, widgets, telemetry, metadata).

Snapshots MUST NOT contain local repository paths; the CLI sanitizes demo data automatically.
