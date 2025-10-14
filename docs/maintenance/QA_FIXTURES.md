# QA Fixtures Guide

_Updated: 2025-10-13_

## Hiện trạng
- Thư mục `test-fixtures/` hiện chỉ chứa `sample.txt` dùng cho smoke/unit test.
- Fixtures được dùng bởi các bài test Rust (xem `rust/hash-checker/tests/`).

## Quy trình cập nhật
1. Tạo fixture mới trong `test-fixtures/` với tên mô tả: `sample-<size>-<purpose>.ext`.
2. Ghi hash chính thức vào `docs/maintenance/QA_FIXTURES.md` để tiện so sánh.
3. Cập nhật bài test tương ứng (CLI hoặc GUI) để sử dụng fixture mới.
4. Chạy `make ci-linux-local` trước khi commit.

## Hash tham chiếu
| File | SHA256 |
| --- | --- |
| sample.txt | 260948c8a3f06f47c92b8fe2db23d696705bc5801d7af840141de0466a94e52e |

## Ghi log
- Khi thêm/chỉnh fixture, tạo file log `docs/reports/<yyyy-mm-dd>-fixtures.md` mô tả thay đổi.
