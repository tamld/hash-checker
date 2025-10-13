# GUI Screenshot Checklist

_Updated: 2025-10-13_

Mục tiêu: tạo bộ ảnh mới cho README và tài liệu khi giao diện thay đổi.

## Chuẩn bị
- Build GUI trong chế độ release: `cargo run --release --manifest-path rust/hash-checker-gui/Cargo.toml`.
- Bật tuỳ chọn high contrast nếu muốn minh họa.
- Sử dụng phím `Cmd+Shift+4` (macOS) hoặc Snipping Tool (Windows) để chụp màn hình ở độ phân giải tối thiểu 1440×900.

## Ảnh cần có
1. **Main view** – giao diện chính khi nhập file và hash.
2. **Match result** – ví dụ hash trùng khớp (màu trạng thái xanh).
3. **Mismatch result** – ví dụ hash không trùng (màu đỏ).
4. **Algorithm dropdown** – minh họa danh sách thuật toán.

## Quy trình lưu trữ
- Đặt tên file trong `docs/assets/` theo định dạng `gui-<context>.png`.
- Cập nhật README và các tài liệu liên quan (ví dụ `docs/GUI_DECISION.md`) nếu cần.
- Ghi log cập nhật ảnh vào `logs/assets/<yyyy-mm-dd>-screenshots.md` cùng commit hash.

## Ghi chú
- Hiện tại environment CLI không có GUI nên chưa thể chụp ảnh mới; checklist này dùng cho lần cập nhật gần nhất.
