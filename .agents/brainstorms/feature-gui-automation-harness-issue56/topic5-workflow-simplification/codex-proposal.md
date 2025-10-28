# Workflow Simplification & Lessons Management - Codex's Proposal

**Date**: 2025-10-28  
**Author**: Codex  
**Confidence**: 50%  
**Status**: Draft (ready for discussion)

---

## Problem Understanding

- Có 15 lessons learned (12 `.md`, 3 `.yml`) rải trong `lessons_learned/`. Không có index, khó truy vết nhanh xem lesson nào proven, lesson nào còn giả thuyết.
- Cấu trúc tổng thể `.agents/` có nhiều thư mục nhỏ (brainstorms, workflows, lessons, records...), dễ gây “context switching” khi agent mới vào – tuy nhiên mỗi lesson hiện là một file độc lập, thuận tiện cho versioning và attach evidence.
- Mục tiêu người dùng: rõ ràng, minh bạch, truy vết được, hỗ trợ scripts (đào tạo/vận hành). Nếu gộp toàn bộ lesson vào một khối lớn sẽ khó merge, khó audit, và mâu thuẫn với lesson workflow (lesson chỉ sau khi chứng minh).

## Proposed Solution

### 1. Giữ lesson dạng file riêng nhưng bổ sung **SSOT Lesson Index**

- Tạo `lessons_index.yml` trong `.agents/lessons_learned/` với schema:

```yaml
lessons:
  - id: L-2025-10-26-01
    title: "Execute only when prerequisites done"
    status: proven | draft | needs_review
    evidence:
      - commit: 9660a5e
      - log: logs/ci-baseline-measurements.json
    owner: cursor
    tags: [execution, governance]
    location: .agents/lessons_learned/EXECUTION_TIME_ESTIMATION_LESSON.md
    summary: "Time estimates optimistic by 12x..."
```

- Scripts đào tạo chỉ cần đọc index để biết lesson nào proven, lấy đường dẫn gốc. Giữ tính truy vết (link thẳng tới file + commit). Có thể auto-generate bảng trong README hoặc metrics.

### 2. Tách kho kiến thức thành ba lớp rõ ràng

| Layer | Nội dung | Ví dụ thư mục |
|-------|----------|---------------|
| **Active** | tasks, locks, current-session handoff | `.agents/active/` (giữ như hiện tại) |
| **Knowledge Base** | lessons đã chứng minh, operating principles, workflows chuẩn hóa | `.agents/knowledge/` (mới) chứa `lessons_index.yml`, `lessons/`, `workflows/`, `principles.md` |
| **Archive** | handoff cũ, brainstorm đã kết thúc, tài liệu lỗi thời | `.agents/archive/` |

### 3. Lessons Refactor

- Di chuyển lesson đã proven vào `.agents/knowledge/lessons/` (mỗi file giữ nguyên). Các lesson chưa proven (ví dụ phân tích đang trial) ở `.agents/brainstorms/...` hoặc `.agents/archive/drafts/`.
- Dùng index để phân biệt trạng thái, gắn tag (governance, execution, telemetry...). Điều này hỗ trợ scripts training: ví dụ script filter lấy tất cả lesson thuộc governance để train AA mới.

### 4. Gộp thông tin phụ trợ

- `CORRECT_LESSON_CREATION_WORKFLOW.md` chuyển vào `knowledge/workflows.md` dưới mục “Lesson workflow”.
- `HYPOTHESIS_VALIDATION_METHODOLOGY.md` và các phân tích meta nên chuyển sang `knowledge/frameworks/` nếu proven, nếu chưa thì để trong `brainstorms/` với trạng thái `hypothesis`.
- Tạo script nhẹ `scripts/generate-lessons-report.py` đọc index, spit ra bảng cho Gemini tổng hợp.

## Pros / Cons của từng phương án

### Phương án A – Gộp tất cả lesson vào một file duy nhất

- **Pros**: tìm kiếm đơn giản, một file duy nhất.
- **Cons lớn**: merge conflict cao, khó enforce lesson workflow, dung lượng khổng lồ, mất khả năng audit từng lesson, khó attach bằng chứng riêng.

### Phương án B – Giữ file riêng + index (đề xuất)

- **Pros**: truy vết dễ (mỗi file), scripts đọc index nhanh, chia theo tag, dễ archive, ít conflict. Phù hợp với nguyên tắc “Reality > Hypothesis” (lesson proven mới vào index).  
- **Cons**: Cần duy trì index (cập nhật tay/automation). Cần script check đồng bộ `location`.

### Phương án C – Gộp theo chủ đề (1 file/ category)

- **Pros**: Giảm số file, conflict thấp hơn A.  
- **Cons**: Một lesson có thể thuộc nhiều chủ đề → lặp thông tin, tricky audit. Index vẫn cần.

## Behaviour với nội dung không đồng thuận

- Nếu team muốn gộp tất cả lesson thành một file duy nhất (Phương án A), mình vẫn tuân thủ kế hoạch khi được quyết định chung; song song sẽ log rủi ro (conflict, mất audit) vào discussion và chuẩn bị metric để chứng minh nếu vấn đề xảy ra (stop-the-line). Không tự ý chống đối, nhưng ghi lại hypothesis cần kiểm chứng.
- Nếu consensus chọn C (gộp theo category) mình đề xuất vẫn giữ index để tránh duplication; sẽ xây POC script để làm rõ lợi ích.

## Questions for Cursor & Gemini

1. Cursor: Bạn có sẵn sàng chuyển lesson proven vào layer `knowledge/` và dùng index không, để tránh duplication?  
2. Gemini: Bạn cần output định dạng nào từ index để phục vụ đào tạo/báo cáo?

---

**Ready for Discussion**: YES
