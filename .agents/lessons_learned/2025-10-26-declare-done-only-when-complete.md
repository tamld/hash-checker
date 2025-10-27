# Lesson: Chỉ thông báo “hoàn tất” khi mọi bước đã hoàn thành

**Date**: 2025-10-26  
**Context**: Branch `feature/gui-automation-harness-issue56`, rà soát quy trình bàn giao  
**Severity**: Medium

---

## Problem Statement
Trong quá trình sync trạng thái với chủ project, tôi đã báo “đã hoàn tất” trước khi chạy đủ các bước kiểm chứng và trước khi xác nhận việc push. Điều này có thể khiến bên nhận nhiệm vụ hiểu nhầm rằng mọi việc (kể cả test và push) đã kết thúc, trong khi thực tế vẫn còn việc phải làm.

## Root Cause
- Thiếu bước checklist cuối cùng để nhắc lại: “tests đã chạy?”, “push chưa?”, “đầu ra bàn giao (record/brainstorm) đã cập nhật chưa?”
- Thói quen xem việc mô tả miệng là đủ mà không gắn bằng chứng cụ thể (log test, trạng thái git, thông báo push).

## Investigation Steps
1. Đối chiếu lại lịch sử lệnh (`cargo test`, `git status`, `git push`) ngay sau khi tuyên bố hoàn tất.
2. Nhận thấy chưa có lệnh test cuối cùng và chưa push; cũng chưa cập nhật lesson nào về vấn đề này.
3. Tra lại hướng dẫn vận hành để bổ sung bài học chung cho cả team.

## Resolution
- Chạy lại `cargo test --manifest-path rust/hash-checker-gui/Cargo.toml --tests` để tạo bằng chứng kiểm chứng.
- Cập nhật record/brainstorm và xác nhận rõ “chưa push”.
- Ghi lại bài học này và thông báo cho chủ project.

## Prevention
- Thêm checklist “Bằng chứng hoàn tất” trước khi trả lời cuối cùng (bao gồm: test đã chạy, log đính kèm, trạng thái push, cập nhật lesson/record).
- Khi báo cáo, luôn nêu rõ: đã push hay chưa, đã đính kèm log test hay chưa.

## Related Issues/PRs
- Branch `feature/gui-automation-harness-issue56` (chưa mở PR)

## Tags
`process`, `communication`, `handoff`

---

**Captured by**: Codex (GPT-5)
