# Makefile for the Note App project

.DEFAULT_GOAL := build

# ==============================================================================
# Variables
# ==============================================================================

FRONTEND_DIR := frontend
BACKEND_DIR := backend

# ==============================================================================
# High-Level Targets
# ==============================================================================

.PHONY: all
all: build ## Build all applications

.PHONY: build
build: build-web build-desktop ## Build both web and desktop applications

.PHONY: install
install: ## Install frontend dependencies
	@echo "Installing frontend dependencies..."
	npm install

# ==============================================================================
# Web Application Targets
# ==============================================================================

.PHONY: build-web
build-web: build-frontend build-web-server ## Build the complete web application

.PHONY: run-web
run-web: run-web-server-bg run-frontend ## Run the web application

.PHONY: build-frontend
build-frontend:
	@echo "Building frontend for web..."
	cd $(FRONTEND_DIR) && npm run build

.PHONY: run-frontend
	@echo "Starting frontend development server..."
	cd $(FRONTEND_DIR) && npm run dev

.PHONY: build-web-server
build-web-server:
	@echo "Building web server..."
	cd $(BACKEND_DIR) && cargo build --release -p web-app

.PHONY: run-web-server
run-web-server:
	@echo "Starting web server..."
	cd $(BACKEND_DIR) && cargo run -p web-app

.PHONY: run-web-server-bg
run-web-server-bg:
	@echo "Starting web server in the background..."
	cd $(BACKEND_DIR) && cargo run -p web-app &

# ==============================================================================
# Desktop Application Targets
# ==============================================================================

.PHONY: build-desktop
build-desktop: ## Build the desktop application
	@echo "Building desktop application..."
	cd $(BACKEND_DIR)/desktop-app && cargo tauri build

.PHONY: run-desktop
run-desktop: ## Run the desktop application in development mode
	@echo "Starting desktop application..."
	cd $(BACKEND_DIR)/desktop-app && cargo tauri dev

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