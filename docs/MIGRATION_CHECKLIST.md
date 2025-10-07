# Python → Rust Migration Checklist

- [x] Announce migration scope and timelines to collaborators.
- [x] Relocate legacy Python code to `legacy/python` and update documentation.
- [x] Provide containerized scripts (`make python-test`, `make rust-test`) to keep host clean.
- [x] Scaffold Rust crate and replicate CLI features.
- [x] Port hashing unit/integration tests to Rust.
- [ ] Tag the last Python release for archival reference.
- [ ] Deprecate Python packaging artifacts once Rust GUI ships.
- [ ] Update installers/release notes when Rust app reaches parity.

Keep this file updated as phases complete.
