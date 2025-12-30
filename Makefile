.PHONY: help build run test clean check fmt clippy install dev watch release

help: ## Show this help message
	@echo 'Usage: make [target]'
	@echo ''
	@echo 'Available targets:'
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  %-15s %s\n", $$1, $$2}' $(MAKEFILE_LIST)

build: ## Build the project
	cargo build

run: ## Run the application
	cargo run

test: ## Run tests
	cargo test

clean: ## Clean build artifacts
	cargo clean
	rm -rf target/

check: ## Check code without building
	cargo check

fmt: ## Format code
	cargo fmt

clippy: ## Run clippy linter
	cargo clippy -- -D warnings

install: ## Install the application
	cargo install --path .

dev: ## Run in development mode with auto-reload
	cargo watch -x run

watch: dev ## Alias for dev

release: ## Build release version
	cargo build --release

setup: ## Setup development environment
	chmod +x scripts/setup.sh
	./scripts/setup.sh

all: fmt clippy test build ## Run all checks and build
