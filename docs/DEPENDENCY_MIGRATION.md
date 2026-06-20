# Dependency Migration Roadmap

This document tracks the plan to migrate away from unmaintained GTK3 bindings (`atk/gdk/gtk-sys`) and the `instant` crate while keeping packaging workflows healthy.

## Objectives (status as of 2025-10-28)
- Reduce reliance on GTK3 by moving to GTK4 (or egui-native replacements) once upstream support stabilises. *(In progress – portal flow remains default while GTK4-native prototype bakes.)*
- Remove the legacy `instant` crate from the dependency graph in favour of maintained timing abstractions. *(Completed via the eframe 0.33 / winit 0.30 upgrade shipped in v0.1.7.)*
- Ensure each migration step ships with packaging smoke tests for Linux/macOS/Windows. *(Ongoing – captured in CI checklists and release scripts.)*

## Milestones
1. **Investigation (original target 2025-11-15) – Status: Completed 2025-10-22**
   - Audit current usage of GTK3-specific APIs in `hash-checker-gui`.
   - Evaluate GTK4 support in `eframe/egui` or alternatives (winit-only pipeline).
   - Capture findings in an issue referencing this roadmap.

2. **Pilot migration (target 2025-12-15) – Status: Completed 2025-10-22**
   - Upgraded to `eframe 0.33` / `egui 0.33` (pulling `winit 0.30`) and confirmed `cargo tree` no longer lists `instant`.
   - Ran `make rust-gui-build` and manual macOS/Windows smoke tests to validate the new stack before tagging v0.1.7.

3. **GTK transition (target 2026-01-31) – Status: In progress**
   - Introduce feature-gated GTK4 backend or prefer pure egui winit pipeline if available.
   - Verify packaging (`cargo packager`) still succeeds for Debian/AppImage/NSIS/DMG locally before merging.

4. **Cleanup & monitoring (ongoing)**
   - Remove fallback GTK3 code once the GTK4 path is stable for all supported OSes.
   - Schedule quarterly reviews to ensure transitive dependencies do not reintroduce yanked crates.

## Operational Notes
- All migration PRs must reference this roadmap.
- If packaging fails during migration, fall back to local builds per `docs/OPERATIONS.md` before retrying CI.
- Update `docs/PLAN.md` and `docs/TASKS.md` when milestones are completed.
- Docker helpers now pull `rust:1.88`, matching the documented MSRV. Prefer the
  containerised builds when you need a clean environment; fall back to host
  builds only when debugging hardware-specific issues (e.g. macOS runners).

## 2025-10 Investigation Summary

### GTK surface
- The GUI crate keeps the portal backend (`rfd 0.15.4` with `default-features = false, features = ["xdg-portal", "async-std"]`) for the default build, avoiding GTK3/gtk-sys entirely.
- An opt-in `gtk4-native` feature now depends on `gtk4 0.10.1`, pulling `glib 0.21.3` and the `gtk4` 4.10+ stack to address RUSTSEC-2024-0429. `cargo tree --features gtk4-native -p hash-checker-gui -i glib` shows the new dependency edge ending at `glib v0.21.3`.
- GTK4 support in `rfd` is still pending upstream; native dialogs are implemented locally via `gtk4::FileDialog` behind the feature flag. Portal remains the recommended default path for packaged builds.
- Packaging considerations:
  - Flatpak / modern desktops already ship `xdg-desktop-portal`; older GTK-only environments will require installing `xdg-desktop-portal` + a backend (e.g. `xdg-desktop-portal-gtk`).
  - Our Docker CI images must include the portal services or skip GUI snapshot steps when unavailable. Document the requirement in release notes once we switch.

### Timing abstraction status
- `cargo tree --manifest-path rust/hash-checker-gui/Cargo.toml | rg instant` (run 2025-10-28) returns no matches. The eframe/winit upgrade removed the `instant` crate from both the GUI and CLI dependency graphs.
- Maintain a `cargo deny` check to alert the team if any dependency reintroduces `instant` or other unmaintained timing crates.
- The MSRV is now anchored at Rust 1.88 to support the upgraded egui stack; retain that baseline in CI and contributor guidance.

### Proposed staged roadmap
1. **Completed (v0.1.7)** – Upgraded to `rfd 0.15.4` with the portal features, introduced the `gtk4-native` option behind a feature flag, and refreshed CI images to include the required XDG portal services.
2. **Next (v0.1.8)** – Keep MSRV at 1.88, harden the GTK4-native spike, and document any API adjustments discovered while exercising the new feature flag across macOS/Linux/Windows packaging scripts.
3. **Follow-up** – Evaluate GTK4-native dialogs once upstream support stabilises; continue tracking details in `docs/GTK4_MIGRATION_PLAN.md` and migrate README/OPERATIONS guidance as functionality graduates from experimental to default.

### Open follow-up work
- Issue #33 tracks automation of GUI snapshot/telemetry harness (supports future GTK migration tests).
- Additional issues to open:
  - Track the upstream `rfd` GTK4 discussions and mirror any stable backend once released.
  - Capture telemetry requirements for GTK4-native runs so failures can be diagnosed quickly in CI.
- Update `.agents/project_state.yml` (migration.gtk_strategy) to reflect the completed dependency upgrades and the remaining portal vs GTK4 rollout decisions.

### Verification snapshot (2025-10-28)
- `cargo tree --manifest-path rust/hash-checker-gui/Cargo.toml | rg instant` → no matches.
- `cargo tree --manifest-path rust/hash-checker/Cargo.toml | rg instant` → no matches.
- `make rust-gui-build` chạy thành công trong Docker `rust:1.88` (2025-10-28 20:15 UTC). Các lỗi snapshot đã được Codex sửa thông qua việc tổng quát hóa `write_snapshot_json` và điều chỉnh borrow trong `process_snapshot`.
