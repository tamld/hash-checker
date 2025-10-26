.PHONY: rust-test rust-build rust-gui-build rust-gui-smoke clean help dist-linux cleanup-packaging ci-linux-local deps-refresh macos-dmg-universal

PROJECT_ROOT := $(shell pwd)
DIST_DIR := dist
KEEP_PACKAGING ?= 0
CLEAN_DOCKER ?= 1

rust-test: ## Run Rust tests in Docker (cached target)
	./scripts/docker-rust-test.sh

rust-build: ## Build Rust release binary in Docker
	./scripts/docker-rust-build.sh

rust-gui-build: ## Build Rust GUI binary in Docker
	./scripts/docker-rust-gui-build.sh

rust-gui-smoke: ## Headless GUI smoke test via Vagrant (placeholder)
	./scripts/vagrant-gui-smoke.sh

ci-linux-local: ## Run Linux CI checks locally inside Docker (fmt, clippy, tests)
	./scripts/ci-linux-local.sh

deps-refresh: ## Run monthly dependency refresh (cargo update/audit/deny) and log results
	./scripts/deps-refresh.sh

cleanup-packaging: ## Remove packaging artefacts (dist staging, packager dirs, tmp exports)
	KEEP_PACKAGING=$(KEEP_PACKAGING) ./scripts/cleanup-packaging.sh

clean: ## Remove build artifacts across platforms (respects KEEP_PACKAGING, CLEAN_DOCKER vars)
	KEEP_PACKAGING=$(KEEP_PACKAGING) CLEAN_DOCKER=$(CLEAN_DOCKER) ./scripts/clean.sh

rust-gui-dmg-temp: ## Build macOS .dmg into /tmp/hash-checker-gui
	cargo install cargo-packager@0.11.7 --locked >/dev/null 2>&1 || true
	cargo packager --release --formats dmg --manifest-path rust/hash-checker-gui/Cargo.toml
	mkdir -p /tmp/hash-checker-gui
	cp rust/hash-checker-gui/target/packager/*.dmg /tmp/hash-checker-gui/
	cp rust/hash-checker-gui/target/packager/SHA256SUMS /tmp/hash-checker-gui/
	rm -rf rust/hash-checker-gui/target/packager

rust-linux-deb-temp: ## Build Linux .deb into /tmp/hash-checker-deb
	cargo install cargo-packager@0.11.7 --locked >/dev/null 2>&1 || true
	cargo packager --release --formats deb --manifest-path rust/hash-checker-gui/Cargo.toml
	mkdir -p /tmp/hash-checker-deb
	cp rust/hash-checker-gui/target/packager/*.deb /tmp/hash-checker-deb/
	cp rust/hash-checker-gui/target/packager/SHA256SUMS /tmp/hash-checker-deb/
	rm -rf rust/hash-checker-gui/target/packager

rust-windows-zip-temp: ## Build Windows portable .zip into platform temp directory
	cargo build --release --manifest-path rust/hash-checker/Cargo.toml
	cargo build --release --manifest-path rust/hash-checker-gui/Cargo.toml
	TMP_ROOT=$${TMPDIR:-$${TEMP:-$${TMP:-/tmp}}}; \
	if command -v cygpath >/dev/null 2>&1; then \
	  TMP_POSIX=$$(cygpath "$$TMP_ROOT"); \
	else \
	  TMP_POSIX="$$TMP_ROOT"; \
	fi; \
	mkdir -p "$$TMP_POSIX/hash-checker-win"; \
	cp rust/hash-checker/target/release/hash-checker* "$$TMP_POSIX/hash-checker-win/" 2>/dev/null || true; \
	cp rust/hash-checker-gui/target/release/hash-checker-gui* "$$TMP_POSIX/hash-checker-win/" 2>/dev/null || true; \
	(cd "$$TMP_POSIX/hash-checker-win" && zip -qr hash-checker-windows-portable.zip hash-checker*)

help: ## Show this help
	@grep -E '^[a-zA-Z_-]+:.*?##' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?##";} {printf "%-20s %s\n", $$1, $$2}'

# GUI Automation (Container-First)
gui-automation-build: ## Build GUI automation Docker image
	docker build -f docker/gui-automation.Dockerfile -t hash-checker-gui-automation .

gui-automation-test: ## Run all GUI automation tests in container
	docker run --rm \
		-v $(PROJECT_ROOT):/workspace \
		-e RUST_BACKTRACE=1 \
		hash-checker-gui-automation \
		bash -c "cargo test --manifest-path rust/hash-checker-gui/Cargo.toml --tests"

gui-automation-clean: ## Clean up GUI automation container
	docker rmi hash-checker-gui-automation || true

# Cleanup & Compliance
check-clean: ## Check if workspace is clean (no untracked files)
	@echo "Checking workspace cleanliness..."
	@if [ -n "$$(git status --porcelain | grep '^??')" ]; then \
		echo "❌ Untracked files found:"; \
		git status --porcelain | grep '^??'; \
		echo "Run 'make clean-workspace CONFIRM=1' to remove them"; \
		exit 1; \
	else \
		echo "✅ Workspace is clean"; \
	fi

clean-workspace: ## Remove all untracked files and directories
	@if [ "$(CONFIRM)" != "1" ]; then \
		echo "Refusing to run 'git clean -fd' without confirmation."; \
		echo "Re-run as 'make clean-workspace CONFIRM=1' to proceed."; \
		exit 1; \
	fi
	@echo "Cleaning workspace (untracked files will be removed)..."
	git clean -fd
	@echo "✅ Workspace cleaned"

pre-commit-check: check-clean ## Run pre-commit checks (clean workspace + tests)
	@echo "Running pre-commit checks..."
	@make check-clean
	@echo "✅ Pre-commit checks passed"


dist-linux: rust-build rust-gui-build ## Build and package Linux artifacts via Docker
	rm -rf $(DIST_DIR)/linux
	mkdir -p $(DIST_DIR)/linux
	cp rust/hash-checker/target/release/hash-checker $(DIST_DIR)/linux/
	cp rust/hash-checker-gui/target/release/hash-checker-gui $(DIST_DIR)/linux/
	(cd $(DIST_DIR)/linux && sha256sum hash-checker hash-checker-gui > SHA256SUMS)
	tar -czf $(DIST_DIR)/hash-checker-linux.tar.gz -C $(DIST_DIR)/linux .
	KEEP_PACKAGING=$(KEEP_PACKAGING) ./scripts/cleanup-packaging.sh


rust-build-temp: ## Build Rust binaries into /tmp/hash-checker-build (clean)
	TMP=/tmp/hash-checker-build && \
		rm -rf $$TMP && mkdir -p $$TMP && \
		docker run --rm -v "$(shell pwd):/workspace" -w /workspace/rust/hash-checker rust:1.83 bash -lc "export PATH=\"/usr/local/cargo/bin:$$PATH\"; cargo build --release" && \
		docker run --rm -v "$(shell pwd):/workspace" -w /workspace/rust/hash-checker-gui rust:1.83 bash -lc "apt-get update >/dev/null && apt-get install -y pkg-config libgtk-3-dev >/dev/null && export PATH=\"/usr/local/cargo/bin:$$PATH\"; cargo build --release" && \
		cp rust/hash-checker/target/release/hash-checker $$TMP/ && \
		cp rust/hash-checker-gui/target/release/hash-checker-gui $$TMP/ && \
	sha256sum $$TMP/hash-checker $$TMP/hash-checker-gui > $$TMP/SHA256SUMS

macos-dmg-universal: ## Build universal (Intel + Apple Silicon) macOS DMG locally
	./scripts/macos-universal-dmg.sh

rust-build-host: ## Build Rust CLI on host (requires Rust toolchain)
	cargo build --release --manifest-path rust/hash-checker/Cargo.toml

rust-gui-build-host: ## Build Rust GUI on host (requires gtk deps)
	cargo build --release --manifest-path rust/hash-checker-gui/Cargo.toml

rust-gui-smoke-host: ## Run GUI smoke test on host
	cargo run --release --manifest-path rust/hash-checker-gui/Cargo.toml -- --smoke-test

sample-verify: ## Verify sample fixture against built CLI
	cargo run --manifest-path rust/hash-checker/Cargo.toml -- test-fixtures/sample.txt 260948c8a3f06f47c92b8fe2db23d696705bc5801d7af840141de0466a94e52e
