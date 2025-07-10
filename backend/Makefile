.PHONY: build-desktop run-backend

build-desktop:
	@echo "Building the desktop app..."
	@echo "This will also build the frontend as configured in tauri.conf.json."
	cd desktop-app && cargo tauri build
	@echo "Desktop app build finished."

run-backend:
	cargo run --package web-app