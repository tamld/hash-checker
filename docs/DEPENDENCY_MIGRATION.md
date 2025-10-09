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
