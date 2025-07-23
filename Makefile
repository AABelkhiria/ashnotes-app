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

.PHONY: build
build: build-web-app build-desktop-app ## Build both web and desktop applications
	@echo "✅ Full project build complete."

.PHONY: install
install: install-frontend install-backend ## Install all dependencies
	@echo "✅ All dependencies installed."

.PHONY: clean
clean: clean-frontend clean-backend ## Clean all build artifacts
	@echo "✅ Full project clean complete."

# ==============================================================================
# Frontend Targets
# ==============================================================================

.PHONY: install-frontend
install-frontend: ## Install frontend dependencies
	@echo "Installing frontend dependencies..."
	npm install --prefix $(FRONTEND_DIR)

.PHONY: build-frontend
build-frontend: install-frontend ## Build the user interface
	@echo "Building frontend for web..."
	DEBUG_BUILD=$(DEBUG_BUILD) npm --prefix $(FRONTEND_DIR) run build

.PHONY: dev-frontend
dev-frontend: ## Run the frontend development server
	@echo "Starting frontend development server..."
	npm --prefix $(FRONTEND_DIR) run dev

.PHONY: clean-frontend
clean-frontend: ## Clean frontend build artifacts
	@echo "Cleaning frontend project..."
	rm -rf $(FRONTEND_DIR)/build $(FRONTEND_DIR)/.svelte-kit $(FRONTEND_DIR)/node_modules

# ==============================================================================
# Backend Targets
# ==============================================================================

.PHONY: install-backend
install-backend: ## Install backend dependencies
	@echo "Installing backend dependencies..."
	cargo install tauri-cli

.PHONY: build-backend
build-backend: build-web-app build-desktop-app ## Build both backend applications
	@echo "✅ Backend build complete."

.PHONY: test-backend
test-backend: ## Run tests for the backend
	@echo "Running tests for backend..."
	@cd $(BACKEND_DIR) && cargo test --workspace

.PHONY: clean-backend
clean-backend: ## Clean backend build artifacts
	@echo "Cleaning backend project..."
	@cd $(BACKEND_DIR) && cargo clean

# ==============================================================================
# Web Application Targets
# ==============================================================================

.PHONY: build-web-app-debug
build-web-app-debug:
	@echo "Building web server (debug)..."
	DEBUG_BUILD=true $(MAKE) build-frontend
	cd $(BACKEND_DIR) && cargo build --release --bin web-app

.PHONY: build-web-app
build-web-app: build-frontend
	@echo "Building web server..."
	cd $(BACKEND_DIR) && cargo build --release --bin web-app

.PHONY: run-web-app
run-web-app:
	@echo "Starting web server..."
	@if [ -f "$(WEB_APP_COMPILED_BINARY_RELEASE)" ]; then \
		echo "--> Found compiled binary, running it directly."; \
		$(WEB_APP_COMPILED_BINARY_RELEASE); \
	else \
		echo "--> No compiled binary found, using 'cargo run' (will compile first)..."; \
		cd $(BACKEND_DIR) && cargo run --bin $(WEB_APP_BINARY_NAME); \
	fi

# ==============================================================================
# Desktop Application Targets
# ==============================================================================

.PHONY: build-desktop-app-debug
build-desktop-app-debug: ## Build the desktop application with debug features
	@echo "Building desktop application with debug features..."
	cd $(BACKEND_DIR) && DEBUG_BUILD=true cargo tauri build

.PHONY: build-desktop-app
build-desktop-app: ## Build the desktop application
	@echo "Building desktop application..."
	cd $(BACKEND_DIR) && cargo tauri build

.PHONY: run-desktop-app
run-desktop-app: ## Run the desktop application in development mode
	@echo "Starting desktop application..."
	@if [ -f "$(DESKTOP_APP_COMPILED_BINARY_RELEASE)" ]; then \
		echo "--> Found compiled binary, running it directly."; \
		$(DESKTOP_APP_COMPILED_BINARY_RELEASE); \
	else \
		echo "--> No compiled binary found, using 'cargo tauri dev' (will compile first)..."; \
		cd $(BACKEND_DIR) && cargo tauri dev; \
	fi

# ==============================================================================
# Help Target
# ==============================================================================

.PHONY: help
help: ## Show this help message
	@echo "Usage: make [target]"
	@echo "--------------------"
	@echo "Available targets:"
	@grep -E '^[a-zA-Z0-9_-]+:.*?##' $(firstword $(MAKEFILE_LIST)) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-25s\033[0m %s\n", $$1, $$2}'
