.PHONY: test-python test-rust test-all build-rust gui-smoke clean

PROJECT_ROOT := $(shell pwd)

python-test-image = python:3.11-slim
rust-image = rust:1.80-slim

python-test: ## Run Python unit tests in Docker (read-only)
	./scripts/docker-python-test.sh

rust-test: ## Run Rust tests in Docker (cached target)
	./scripts/docker-rust-test.sh

rust-build: ## Build Rust release binary in Docker
	./scripts/docker-rust-build.sh

python-build: ## Build Python PyInstaller artifact in Docker
	docker run --rm \
		--user "$(shell id -u):$(shell id -g)" \
		-v "$(PROJECT_ROOT):/workspace" \
		-w /workspace \
		python:3.11-slim \
		bash -lc "pip install -r requirements-build.txt && pyinstaller --name HashChecker --onefile src/hash_checker/__main__.py"

rust-gui-smoke: ## Headless GUI smoke test via Vagrant (placeholder)
	./scripts/vagrant-gui-smoke.sh

clean: ## Remove build artifacts and Docker volumes
	rm -rf dist build
	docker volume prune -f

help: ## Show this help
	@grep -E '^[a-zA-Z_-]+:.*?##' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?##";} {printf "%-20s %s\n", $$1, $$2}'
