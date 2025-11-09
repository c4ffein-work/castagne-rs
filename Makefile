.PHONY: godot-setup godot-check test build clean

# Check if Godot 4 is installed
godot-check:
	@which godot > /dev/null 2>&1 && godot --version | grep -q "^4\." || \
		(echo "❌ Godot 4 not found. Run 'make godot-setup' to install." && exit 1)
	@echo "✅ Godot 4 is installed"

# Install Godot 4
godot-setup:
	@echo "Installing Godot 4..."
	@mkdir -p ~/godot
	@cd ~/godot && \
		wget -q https://github.com/godotengine/godot/releases/download/4.2-stable/Godot_v4.2-stable_linux.x86_64.zip && \
		unzip -q Godot_v4.2-stable_linux.x86_64.zip && \
		chmod +x Godot_v4.2-stable_linux.x86_64 && \
		sudo ln -sf ~/godot/Godot_v4.2-stable_linux.x86_64 /usr/local/bin/godot
	@echo "✅ Godot 4 installed successfully"
	@godot --version

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
