# .agents/ Runbook

Updated: 2025-10-19

## Purpose
- Đây là **nguồn sự thật** cho vận hành nội bộ: lưu checklist, bằng chứng CI, bài học, luật.
- Tất cả guardrail kỹ thuật, log, quyết định chi tiết phải nằm ở đây (không để trong `docs/`).
- Khi có thay đổi quan trọng, cập nhật YAML/JSON tương ứng rồi ghi nhận trong `lessons.yml`.

## File catalogue
- `AGENTS.yml`: Guardrail tổng quát (laws, ngôn ngữ, nhịp workflow, policy branch/PR/issue).
- `ci_guidelines.yml`: Runbook CI, run xanh gần nhất, failure playbook, chính sách branch/PR/issue.
- `lessons.yml`: Nhật ký sự cố/thành công kèm checklist phòng tránh (có run ID và bằng chứng).
- `build_log.yml`: Thông tin build/DMG, test đã chạy, artefact lưu ở đâu.
- `gui_manifest_todo.yml`, `ui_states_manifest.yml`: Backlog & trạng thái UI nội bộ.
- `project_state.yml`, `session_workflow.yml`: Theo dõi tiến độ, phiên làm việc.
- `docs_inventory.yml`: Mapping tài liệu public vs nội bộ.
- `templates/`: CARE spec, handoff, branch_progress skeletons.
- `scripts/`: Automations như `validate_handoff.sh` (kiểm tra handoff LL-014).
- `metrics_log.yml`: Nhật ký latency handoff, CI trạng thái, vòng lặp Codex.

## Quy tắc cập nhật
1. **Trước khi mở PR hoặc merge**: chạy `make ci-linux-local`, cập nhật run ID trong `ci_guidelines.yml` (hoặc lessons nếu là bug fix).
2. **Sau mỗi sự cố CI**: thêm mục mới vào `lessons.yml` (trigger, impact, evidence, checklist).
3. **Khi claim task mới**: sao chép `templates/branch_progress_template.yml` thành `.agents/branch_progress.yml`, điền `session_label`, cập nhật `related_sessions`, rồi thêm/điều chỉnh dòng tương ứng trong `metrics_log.yml`.
4. **Chi tiết kỹ thuật / kế hoạch** chỉ ghi trong `.agents/`; `docs/` chỉ giữ whitebook cho người đọc bên ngoài.
5. **Ngôn ngữ**: YAML/JSON trong `.agents/` dùng tiếng Anh cho trường dữ liệu, nhưng phần giải thích có thể kèm tiếng Việt khi cần; mọi trao đổi với người dùng vẫn dùng tiếng Việt.
6. **Chứng cứ**: luôn dẫn đường dẫn chính xác (log, ảnh, run URL) để kiểm chứng.
