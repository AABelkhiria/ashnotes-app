# Makefile for the Note App project

.DEFAULT_GOAL := help

# ==============================================================================
# Variables
# ==============================================================================

FRONTEND_DIR := frontend
BACKEND_DIR := backend

# ==============================================================================
# High-Level Targets
# ==============================================================================

.PHONY: all
all: build ## Build both the frontend and backend

.PHONY: build
build: build-frontend build-backend ## Build both the frontend and backend for production

.PHONY: run
run: run-backend-bg run-frontend ## Run both the frontend and backend development servers concurrently

.PHONY: install
install: frontend/install ## Install frontend dependencies

.PHONY: check
check: frontend/check ## Run static checks on the frontend code

.PHONY: test
test: backend/test ## Run tests for the backend

.PHONY: clean
clean: frontend/clean backend/clean ## Clean the build artifacts for both frontend and backend

# ==============================================================================
# Frontend Targets
# ==============================================================================

.PHONY: build-frontend
build-frontend:
	@echo "Building frontend..."
	@$(MAKE) frontend/build

.PHONY: run-frontend
run-frontend:
	@echo "Starting frontend development server..."
	@$(MAKE) frontend/dev

.PHONY: frontend/install
frontend/install:
	@echo "Installing frontend dependencies..."
	cd $(FRONTEND_DIR) && npm install

.PHONY: frontend/dev
frontend/dev:
	cd $(FRONTEND_DIR) && npm run dev

.PHONY: frontend/build
frontend/build:
	cd $(FRONTEND_DIR) && npm run build

.PHONY: frontend/check
frontend/check:
	@echo "Running checks on frontend..."
	cd $(FRONTEND_DIR) && npm run check

.PHONY: frontend/clean
frontend/clean:
	@echo "Cleaning frontend artifacts..."
	cd $(FRONTEND_DIR) && rm -rf build .svelte-kit node_modules

# ==============================================================================
# Backend Targets
# ==============================================================================

.PHONY: build-backend
build-backend:
	@echo "Building backend..."
	@$(MAKE) backend/build

.PHONY: run-backend
run-backend:
	@echo "Starting backend development server..."
	@$(MAKE) backend/run

.PHONY: run-backend-bg
run-backend-bg:
	@echo "Starting backend development server in the background..."
	@$(MAKE) backend/run &

.PHONY: backend/run
backend/run:
	cd $(BACKEND_DIR) && cargo run

.PHONY: backend/build
backend/build:
	cd $(BACKEND_DIR) && cargo build --release

.PHONY: backend/test
backend/test:
	@echo "Running tests for backend..."
	cd $(BACKEND_DIR) && cargo test

.PHONY: backend/clean
backend/clean:
	@echo "Cleaning backend artifacts..."
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