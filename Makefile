# ai-ota — top-level task runner
#
# Usage:  make <target>
# List:   make help
#
# Targets are added as the corresponding features land. Anything documented in
# DEV.md that is not implemented yet is intentionally absent rather than faked.

SHELL := /bin/bash
CARGO ?= cargo

TARGET_DIR   := target
COVERAGE_DIR := $(TARGET_DIR)/coverage

.DEFAULT_GOAL := help

# ── Help ─────────────────────────────────────────────────────────────────────
.PHONY: help
help: ## Show this help
	@printf 'Usage:\n  make \033[36m<target>\033[0m\n\nTargets:\n'
	@grep -hE '^[a-zA-Z0-9_.-]+:.*?## ' $(MAKEFILE_LIST) \
		| sort \
		| awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-16s\033[0m %s\n", $$1, $$2}'

# ── Quality ──────────────────────────────────────────────────────────────────
.PHONY: fmt
fmt: ## Format the whole workspace
	$(CARGO) fmt --all

.PHONY: fmt-check
fmt-check: ## Check formatting without writing
	$(CARGO) fmt --all --check

.PHONY: clippy
clippy: ## Lint the workspace with clippy (-D warnings)
	$(CARGO) clippy --workspace --all-targets -- -D warnings

.PHONY: lint
lint: fmt-check clippy ## fmt-check + clippy

# ── Build ────────────────────────────────────────────────────────────────────
.PHONY: build
build: ## Build the workspace (debug)
	$(CARGO) build --workspace

.PHONY: build-release
build-release: ## Build the workspace (release)
	$(CARGO) build --workspace --release

# ── Test ─────────────────────────────────────────────────────────────────────
.PHONY: test
test: test-unit ## Default test entry point

.PHONY: test-unit
test-unit: ## Run unit + integration tests on the host
	$(CARGO) test --workspace

.PHONY: test-all
test-all: lint test-unit ## Everything runnable without hardware
	@echo "test-all: lint + host tests passed"
	@echo "note: property/emulated/HIL layers land with their features"

.PHONY: coverage
coverage: ## Coverage report (requires cargo-llvm-cov)
	$(CARGO) llvm-cov --workspace --html --output-dir $(COVERAGE_DIR)
	@echo "Coverage report: $(COVERAGE_DIR)/index.html"

# ── Environment ──────────────────────────────────────────────────────────────
.PHONY: doctor
doctor: ## Check the local toolchain and workspace
	@echo "cargo:     $$($(CARGO) --version)"
	@echo "rustc:     $$(rustc --version)"
	@echo "toolchain: $$(cat rust-toolchain.toml | grep -m1 channel || echo 'n/a')"
	@$(CARGO) metadata --format-version 1 >/dev/null && echo "workspace: OK"

.PHONY: clean
clean: ## Remove build artifacts
	$(CARGO) clean
