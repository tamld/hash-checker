.PHONY: test-python test-rust test-all rust-test rust-build python-test python-build gui-smoke clean help

PROJECT_ROOT := $(shell pwd)

python-test: ## Run legacy Python unit tests in Docker (read-only)
	./scripts/docker-python-test.sh

rust-test: ## Run Rust tests in Docker (cached target)
	./scripts/docker-rust-test.sh

rust-build: ## Build Rust release binary in Docker
	./scripts/docker-rust-build.sh

rust-gui-build: ## Build Rust GUI binary in Docker
	./scripts/docker-rust-gui-build.sh

python-build: ## Build legacy Python PyInstaller artifact in Docker
	docker run --rm \
		--user "$(shell id -u):$(shell id -g)" \
		-v "$(PROJECT_ROOT):/workspace" \
		-w /workspace/legacy/python \
		python:3.11-slim \
		bash -lc "pip install -r requirements-build.txt && pyinstaller --name HashCheckerLegacy --onefile src/main.py"

rust-gui-smoke: ## Headless GUI smoke test via Vagrant (placeholder)
	./scripts/vagrant-gui-smoke.sh

clean: ## Remove build artifacts and Docker volumes
	rm -rf dist build legacy/python/dist legacy/python/build
	docker volume prune -f

help: ## Show this help
	@grep -E '^[a-zA-Z_-]+:.*?##' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?##";} {printf "%-20s %s\n", $$1, $$2}'
