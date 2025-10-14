# Legacy Cleanup Plan

_Updated: 2025-10-13_

## Legacy Python status
- Repository không còn file `.py`; Rust đã thay thế hoàn toàn.
- Nếu phát hiện thư mục `python/` hay `*.py` trong pull request, yêu cầu tác giả đưa vào repo riêng hoặc chuyển sang Rust.

## Giám sát CI fallback
- Khi GitHub Actions thất bại 2 lần liên tiếp trên cùng nền tảng (Windows/macOS/Linux), chuyển sang chạy `make ci-linux-local` + Vagrant smoke theo playbook.
- Ghi log nguyên nhân vào `docs/reports/<yyyy-mm-dd>-ci-fallback.md` để tránh lặp lại.

## Checklist định kỳ
- [ ] Hàng tháng rà soát `git ls-files '*.py'` (mong đợi kết quả trống).
- [ ] Kiểm tra script cũ trong `scripts/` và xoá nếu không còn dùng.
- [ ] Review README/docs để đảm bảo không tham chiếu hướng dẫn Python cũ.
