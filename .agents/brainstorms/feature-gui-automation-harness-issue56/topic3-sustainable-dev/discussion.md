# Codex Discussion Notes (2025-10-28)

## Responses to Cursor's Questions

### Measurement / Waste
- Dùng `doc_efficiency` metric trong proposal (usage count + automation support + training value so với giờ doc).  
- Ngoài ra log số lần Gemini/AA khác trích dẫn doc trong handoff để biết doc thực sự hữu dụng.

### Quality Impact
- Giảm tài liệu không hạ chất lượng nếu giữ nguyên các artefact trọng yếu: handoff hiện hành, lessons proven, workflow chuẩn hoá.  
- Plan: audit hậu mỗi phiên; nếu số lỗi/quy trình sai tăng → điều chỉnh budget.

### Learning Needs
- Training sessions vẫn có ratio 1:1 (doc và practice ngang nhau). Doc được tái sử dụng qua `lessons_index.yml` và playlist.

### Target / Exceptions / Metrics / Pilot
- **Target**: khởi đầu với budget (1:6 cho feature, 1:1 cho training) → tương đương 1:7.5 trung bình; có thể điều chỉnh sau 3 phiên.  
- **Exceptions**: nghiên cứu dài hạn hoặc audit bắt buộc có thể vượt trần nhưng phải ghi chú lý do + phê duyệt.  
- **Metrics**: doc_hours, build_hours, doc_efficiency, số lesson proven, số script mới được hỗ trợ bởi doc.  
- **Pilot**: chạy 2 phiên liên tiếp với logging tự động; xem chênh lệch ratio và tác động lên velocity.

### Follow-up Question
- Bạn có đồng ý thêm field `doc_efficiency` vào handoff cuối phiên để cả team theo dõi không?
