.PHONY: help fmt clippy test build clean dev docker-build docker-test docker-clippy

help: ## Show this help message
	@echo "CPM Development Commands:"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'

fmt: ## Format code with rustfmt
	cargo fmt --all

clippy: ## Run clippy linter
	cargo clippy --all-targets --all-features -- -D warnings

test: ## Run tests
	cargo test

build: ## Build the project
	cargo build

clean: ## Clean build artifacts
	cargo clean

dev: fmt clippy test build ## Run all development checks

check: ## Run fmt check and clippy
	cargo fmt --all -- --check
	cargo clippy --all-targets --all-features -- -D warnings

doc: ## Generate documentation
	cargo doc --open

docker-build: ## Build CPM with Docker
	docker build -t cpm:latest .

docker-test: ## Run tests with Docker
	docker-compose up cpm-test

docker-clippy: ## Run clippy with Docker
	docker-compose up cpm-clippy

docker-dev: ## Start development environment with Docker
	docker-compose up cpm
