# Project Status Overview

_Last updated: 2025-10-15_

This page summarises the current state. Refer to [`docs/PLAN.md`](PLAN.md) for
the roadmap and [`docs/TASKS.md`](TASKS.md) for near-term work; this overview
only highlights the key signals and where to dig deeper.

## Snapshot
- Feature polishing: PR #7 (`feature/gui-themes`) và PR #9 (`feature/gui-copy-prefix`) đã có đủ log test/QA; chờ bạn duyệt để gộp. Checklist và tài liệu được cập nhật trong `docs/TASKS.md` & `logs/qa/`.
- macOS universal DMG: script cục bộ vẫn tạo artefact; bước tự động hóa CI được theo dõi tại `docs/PLAN.md` §Phase 3.
- Documentation: README/CHANGELOG và bộ screenshot (`docs/assets/`) đã làm mới ngày 2025-10-15.
- SignPath onboarding: vẫn kẹt vì OSS subscription; theo dõi trong `.agents/project_state.yml` và `docs/security/SIGNPATH_CHECKLIST.md`.

## Maintenance cadence
- Monthly: dependency refresh (`deps-refresh.yml`) and cargo-dist maintenance.
- Quarterly: Vagrant smoke reminder + log archival (`docs/vagrant/VALIDATION_PLAYBOOK.md`).

## Looking ahead
- Near-term backlog comes directly from `docs/TASKS.md` (“Ready backlog”).
- Longer-term stretch goals remain in `docs/PLAN.md` §Phase 5.
