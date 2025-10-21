# Dependency Migration Roadmap

This document tracks the plan to migrate away from unmaintained GTK3 bindings (`atk/gdk/gtk-sys`) and the `instant` crate while keeping packaging workflows healthy.

## Objectives
- Reduce reliance on GTK3 by moving to GTK4 (or egui-native replacements) once upstream support stabilises.
- Remove `instant` in favour of `std::time` or `web-time` abstractions that are actively maintained.
- Ensure each migration step ships with packaging smoke tests for Linux/macOS/Windows.

## Milestones
1. **Investigation (by 2025-11-15)**
   - Audit current usage of GTK3-specific APIs in `hash-checker-gui`.
   - Evaluate GTK4 support in `eframe/egui` or alternatives (winit-only pipeline).
   - Capture findings in an issue referencing this roadmap.

2. **Pilot migration (target 2025-12-15)**
   - Prototype branch replacing `instant` with `web-time` (already a transitive dependency).
   - Run `make ci-linux-local` and manual macOS/Windows smoke tests; attach logs to the PR.

3. **GTK transition (target 2026-01-31)**
   - Introduce feature-gated GTK4 backend or prefer pure egui winit pipeline if available.
   - Verify packaging (`cargo packager`) still succeeds for Debian/AppImage/NSIS/DMG locally before merging.

4. **Cleanup & monitoring (ongoing)**
   - Remove fallback GTK3 code once the GTK4 path is stable for all supported OSes.
   - Schedule quarterly reviews to ensure transitive dependencies do not reintroduce yanked crates.

## Operational Notes
- All migration PRs must reference this roadmap.
- If packaging fails during migration, fall back to local builds per `docs/OPERATIONS.md` before retrying CI.
- Update `docs/PLAN.md` and `docs/TASKS.md` when milestones are completed.

## 2025-10 Investigation Summary

### GTK3 dependency surface
- The GUI crate does not call GTK APIs directly; dependencies arrive via `rfd 0.11.4` whose default feature set enables `gtk3`, `glib-sys`, and `gobject-sys` on Linux/BSD. `cargo tree --target x86_64-unknown-linux-gnu -i gtk-sys` confirms the only inbound edge is `hash-checker-gui → rfd → gtk-sys` (with `gtk-sys` pulling `gdk-sys`/`atk-sys`).
- Upstream `rfd` v0.15.4 flips the default features to `xdg-portal` + `async-std`, keeping `gtk3` available but opt-in. Migrating to ≥0.15 allows us to drop GTK entirely by disabling the `gtk3` feature and enabling `xdg-portal`.
- GTK4 support in `rfd` is tracked upstream but not yet stabilised. If native GTK4 dialogs are required, they will arrive through the `gtk-sys` 0.18+ family once the crate exposes a `gtk4` feature flag. Until then, the recommended path is to adopt `xdg-portal` and depend on the host portal service.
- Packaging considerations:
  - Flatpak / modern desktops already ship `xdg-desktop-portal`; older GTK-only environments will require installing `xdg-desktop-portal` + a backend (e.g. `xdg-desktop-portal-gtk`).
  - Our Docker CI images must include the portal services or skip GUI snapshot steps when unavailable. Document the requirement in release notes once we switch.

### `instant` crate evaluation
- `instant` enters the dependency graph through `winit 0.28.7` (transitively via `eframe 0.24.1`). Desktop targets consume `std::time::Instant`, but the crate remains to support WASM builds.
- `winit 0.30.x` replaced `instant` with `std::time` on native platforms and `web-time` on the web. However, current `eframe` stable requires `winit 0.28`; upgrading to `eframe` ≥0.33 adopts `winit 0.30` but raises the MSRV to Rust 1.88 and brings API changes (renderer modularization, egui 0.33 APIs).
- Interim option: override `cargo` features to disable GTK3 while keeping existing `eframe` + `instant`; replacing `instant` before an `eframe` upgrade would require forking `winit`, so the practical path is:
  1. Bump toolchain to Rust 1.88 and upgrade `eframe`/`egui` to 0.33 (pulls `winit 0.30`, removing `instant`).
  2. Validate that `egui_glow` + packaging scripts still work across macOS/Linux/Windows.
- `instant` stays acceptable in the short term; flag the crate in `cargo deny` to catch regressions after the upgrade.

### Proposed staged roadmap
1. **Short term (v0.1.7)** – Update to `rfd` ≥0.15.4 with features `default-features = false, features = ["xdg-portal"]` to remove GTK runtime dependencies. Update Docker/CI images to include `xdg-desktop-portal`, document requirement in README.
2. **Medium term (v0.1.8)** – Raise MSRV to 1.88, upgrade `eframe`/`egui`/`winit`. Record API adjustments (e.g. `AppCreator` changes) and verify packaging scripts.
3. **Follow-up** – Evaluate GTK4-native dialogs once upstream exposes stable support; otherwise remain on portal route.

### Open follow-up work
- Issue #33 tracks automation of GUI snapshot/telemetry harness (supports future GTK migration tests).
- Additional issues to open:
  - Upgrade `rfd` and migrate to XDG portal backend (drops gtk-sys).
  - Plan `eframe`/`egui` upgrade & MSRV bump (remove `instant`).
- Update `.agents/project_state.yml` (migration.gtk_strategy) to reflect spike completion and next steps.
