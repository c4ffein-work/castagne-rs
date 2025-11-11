.PHONY: godot-setup godot-check test build clean

# Check if Godot 4 is installed
godot-check:
	@which godot > /dev/null 2>&1 && godot --version | grep -q "^4\." || \
		(echo "❌ Godot 4 not found. Run 'make godot-setup' to install." && exit 1)
	@echo "✅ Godot 4 is installed"

# Install Godot 4
godot-setup:
	@echo "Installing Godot 4.5..."
	@mkdir -p ~/godot
	@cd ~/godot && \
		wget -q https://github.com/godotengine/godot/releases/download/4.5-stable/Godot_v4.5-stable_linux.x86_64.zip && \
		unzip -o -q Godot_v4.5-stable_linux.x86_64.zip && \
		chmod +x Godot_v4.5-stable_linux.x86_64 && \
		sudo ln -sf ~/godot/Godot_v4.5-stable_linux.x86_64 /usr/local/bin/godot || \
		echo "⚠ Note: Could not create /usr/local/bin/godot symlink (requires sudo). Use ~/godot/Godot_v4.5-stable_linux.x86_64 directly."
	@echo "✅ Godot 4.5 installed successfully"
	@if command -v godot > /dev/null 2>&1; then godot --version; else ~/godot/Godot_v4.5-stable_linux.x86_64 --version; fi

# Build the Rust library
build:
	cargo build

# Run all tests (requires Godot 4)
test: godot-check
	cargo test
	godot --headless --script scripts/run_comparison_tests.gd

# Run just Rust tests (no Godot required)
test-rust:
	cargo test

# Run just Godot integration tests (requires Godot 4)
test-godot: godot-check
	godot --headless --script scripts/run_comparison_tests.gd

# Clean build artifacts
clean:
	cargo clean
