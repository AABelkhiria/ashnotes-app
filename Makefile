# Makefile for the Note App project

.DEFAULT_GOAL := build

# ==============================================================================
# Variables
# ==============================================================================

FRONTEND_DIR := frontend
BACKEND_DIR := backend

WEB_APP_BINARY_NAME := web-app
WEB_APP_COMPILED_BINARY_RELEASE := $(BACKEND_DIR)/target/release/$(WEB_APP_BINARY_NAME)

DESKTOP_APP_BINARY_NAME := ashnotes
DESKTOP_APP_COMPILED_BINARY_RELEASE := $(BACKEND_DIR)/target/release/$(DESKTOP_APP_BINARY_NAME)

# ==============================================================================
# High-Level Targets
# ==============================================================================

.PHONY: all
all: build ## Build all applications

.PHONY: build
build: build-web build-desktop ## Build both web and desktop applications

.PHONY: install
install: ## Install dependencies
	make install-frontend
	make install-backend

.PHONY: install-frontend
install-frontend: ## Install frontend dependencies
	@echo "Installing frontend dependencies..."
	cd $(FRONTEND_DIR) && npm install

.PHONY: install-backend
install-backend: ## Install backend dependencies
	@echo "Installing backend dependencies..."
	cargo install tauri-cli

# ==============================================================================
# Web Application Targets
# ==============================================================================

.PHONY: build-web
build-web: build-frontend build-web-app ## Build the complete web application

.PHONY: build-frontend
build-frontend:
	@echo "Building frontend for web..."
	cd $(FRONTEND_DIR) && npm run build

.PHONY: run-frontend
	@echo "Starting frontend development server..."
	cd $(FRONTEND_DIR) && npm run dev

.PHONY: build-web-app
build-web-app:
	@echo "Building web server..."
	cd $(BACKEND_DIR) && cargo build --release --bin web-app

.PHONY: run-web-app
run-web-app:
	@echo "Starting web server..."
	@# Check if the compiled debug binary exists
	@if [ -f "$(WEB_APP_COMPILED_BINARY_RELEASE)" ]; then \
		echo "--> Found compiled binary, running it directly."; \
		$(WEB_APP_COMPILED_BINARY_RELEASE); \
	else \
		echo "--> No compiled binary found, using 'cargo run' (will compile first)..."; \
		cd $(BACKEND_DIR) && cargo run --bin $(BINARY_NAME); \
	fi

.PHONY: run-web-app-bg
run-web-app-bg:
	@echo "Starting web server in the background..."
	cd $(BACKEND_DIR) && cargo run -p web-app &

# ==============================================================================
# Desktop Application Targets
# ==============================================================================

.PHONY: build-desktop
build-desktop: ## Build the desktop application
	@echo "Building desktop application..."
	cd $(BACKEND_DIR) && cargo tauri build

.PHONY: run-desktop
run-desktop: ## Run the desktop application in development mode
	@echo "Starting desktop application..."
	@# Check if the compiled debug binary exists
	@if [ -f "$(DESKTOP_APP_COMPILED_BINARY_RELEASE)" ]; then \
		echo "--> Found compiled binary, running it directly."; \
		$(DESKTOP_APP_COMPILED_BINARY_RELEASE); \
	else \
		echo "--> No compiled binary found, using 'cargo run' (will compile first)..."; \
		cd $(BACKEND_DIR) && cargo tauri dev; \
	fi

# ==============================================================================
# Shared Targets
# ==============================================================================

.PHONY: test
test: ## Run tests for the backend
	@echo "Running tests for backend..."
	cd $(BACKEND_DIR) && cargo test --workspace

.PHONY: clean
clean: ## Clean all build artifacts
	@echo "Cleaning project..."
	cd $(FRONTEND_DIR) && rm -rf build .svelte-kit node_modules
	cd $(BACKEND_DIR) && cargo clean

# ==============================================================================
# Help Target
# ==============================================================================

.PHONY: help
help: ## Show this help message
	@echo "Usage: make [target]"
	@echo ""
	@echo "Targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'
