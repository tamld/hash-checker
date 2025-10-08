.PHONY: rust-test rust-build rust-gui-build rust-gui-smoke clean help dist-linux

PROJECT_ROOT := $(shell pwd)
DIST_DIR := dist

rust-test: ## Run Rust tests in Docker (cached target)
	./scripts/docker-rust-test.sh

rust-build: ## Build Rust release binary in Docker
	./scripts/docker-rust-build.sh

rust-gui-build: ## Build Rust GUI binary in Docker
	./scripts/docker-rust-gui-build.sh

rust-gui-smoke: ## Headless GUI smoke test via Vagrant (placeholder)
	./scripts/vagrant-gui-smoke.sh

clean: ## Remove build artifacts and Docker volumes
	rm -rf dist build
	docker volume prune -f

help: ## Show this help
	@grep -E '^[a-zA-Z_-]+:.*?##' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?##";} {printf "%-20s %s\n", $$1, $$2}'


dist-linux: rust-build rust-gui-build ## Build and package Linux artifacts via Docker
	rm -rf $(DIST_DIR)/linux
	mkdir -p $(DIST_DIR)/linux
	cp rust/hash-checker/target/release/hash-checker $(DIST_DIR)/linux/
	cp rust/hash-checker-gui/target/release/hash-checker-gui $(DIST_DIR)/linux/
	(cd $(DIST_DIR)/linux && sha256sum hash-checker hash-checker-gui > SHA256SUMS)
	tar -czf $(DIST_DIR)/hash-checker-linux.tar.gz -C $(DIST_DIR)/linux .


rust-build-temp: ## Build Rust binaries into /tmp/hash-checker-build (clean)
	TMP=/tmp/hash-checker-build && \
		rm -rf $$TMP && mkdir -p $$TMP && \
		docker run --rm -v "$(shell pwd):/workspace" -w /workspace/rust/hash-checker rust:1.83 bash -lc "export PATH=\"/usr/local/cargo/bin:$$PATH\"; cargo build --release" && \
		docker run --rm -v "$(shell pwd):/workspace" -w /workspace/rust/hash-checker-gui rust:1.83 bash -lc "apt-get update >/dev/null && apt-get install -y pkg-config libgtk-3-dev >/dev/null && export PATH=\"/usr/local/cargo/bin:$$PATH\"; cargo build --release" && \
		cp rust/hash-checker/target/release/hash-checker $$TMP/ && \
		cp rust/hash-checker-gui/target/release/hash-checker-gui $$TMP/ && \
		sha256sum $$TMP/hash-checker $$TMP/hash-checker-gui > $$TMP/SHA256SUMS

rust-build-host: ## Build Rust CLI on host (requires Rust toolchain)
	cargo build --release --manifest-path rust/hash-checker/Cargo.toml

rust-gui-build-host: ## Build Rust GUI on host (requires gtk deps)
	cargo build --release --manifest-path rust/hash-checker-gui/Cargo.toml

rust-gui-smoke-host: ## Run GUI smoke test on host
	cargo run --release --manifest-path rust/hash-checker-gui/Cargo.toml -- --smoke-test

sample-verify: ## Verify sample fixture against built CLI
	cargo run --manifest-path rust/hash-checker/Cargo.toml -- test-fixtures/sample.txt 260948c8a3f06f47c92b8fe2db23d696705bc5801d7af840141de0466a94e52e
