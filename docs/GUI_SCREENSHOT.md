# GUI Screenshot Checklist

_Updated: 2025-10-13_

Mục tiêu: tạo bộ ảnh mới cho README và tài liệu khi giao diện thay đổi.

## Chuẩn bị
- Build GUI trong chế độ release: `cargo run --release --manifest-path rust/hash-checker-gui/Cargo.toml`.
- Bật tuỳ chọn high contrast nếu muốn minh họa.
- Sử dụng phím `Cmd+Shift+4` (macOS) hoặc Snipping Tool (Windows) để chụp màn hình ở độ phân giải tối thiểu 1440×900.

## Ảnh cần có
| Tên file đề xuất | Nội dung | Lưu ý |
| --- | --- | --- |
| `gui-main.png` | Màn hình chính với form chọn file, trường expected hash, nút Calculate | Dùng theme sáng mặc định, chưa nhập hash |
| `gui-match.png` | Kết quả hash trùng khớp (status xanh) | Hiển thị digest vừa tính, highlight thông điệp Success |
| `gui-mismatch.png` | Kết quả hash không khớp (status đỏ) | Tô rõ thông báo Error, thể hiện digest thực tế |
| `gui-algorithm.png` | Danh sách thuật toán trong dropdown | Mở popup dropdown, giữ focus vào SHA-256 |
| `gui-high-contrast.png` | Giao diện với High contrast bật | Cho thấy toggle và phần nền tối |

## Quy trình lưu trữ
- Đặt tên file trong `docs/assets/` theo định dạng `gui-<context>.png`.
- Cập nhật README và các tài liệu liên quan (ví dụ `docs/GUI_DECISION.md`) nếu cần.
- Ghi log cập nhật ảnh vào `logs/assets/<yyyy-mm-dd>-screenshots.md` cùng commit hash.

## Ghi chú
- Hiện tại environment CLI không có GUI nên chưa thể chụp ảnh mới; checklist này dùng cho lần cập nhật gần nhất.
