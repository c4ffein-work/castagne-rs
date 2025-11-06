# Running Comparison Tests from the CLI

This guide explains how to run comparison tests between Castagne (GDScript) and castagne-rs (Rust) from the command line.

## Quick Start

### 1. Setup Godot (one time)

```bash
./scripts/setup-godot.sh
```

This will download and install Godot headless in `.godot-bin/`. The script will:
- Detect your OS and architecture
- Download the appropriate Godot version (default: 4.2.2)
- Extract and configure it for CLI usage
- Check for existing system-wide Godot installations

**Alternative: Use your own Godot installation**

If you already have Godot installed:
```bash
# Option 1: Use system Godot (if in PATH)
# The scripts will auto-detect it

# Option 2: Set custom path
export GODOT_BIN=/path/to/your/godot
./scripts/run-tests.sh

# Option 3: Create symlink manually
mkdir -p .godot-bin
ln -s /path/to/your/godot .godot-bin/godot
```

### 2. Run the tests

```bash
./scripts/run-tests.sh
```

This will:
1. Build the castagne-rs Rust library (release mode)
2. Run all comparison tests via Godot headless
3. Display detailed results and summary

## Test Output

The test runner will show:

```
==========================================
  Castagne-RS Comparison Tests
==========================================

Running comparison tests...
✓ memory_global: PASSED
✓ memory_player: PASSED
✓ memory_entity: PASSED
✓ state_handle_point_to: PASSED
✓ state_handle_target_entity: PASSED

==========================================
  Summary
==========================================

Total tests:  5
Passed:       5
Failed:       0

✓ All tests passed!
```

## What Gets Tested?

The comparison tests verify that the Rust implementation behaves identically to the original GDScript implementation:

### Memory Module Tests

1. **Global Memory** (`test_memory_global`)
   - Global variable storage and retrieval
   - Data type handling (int, string, etc.)
   - Key existence checks

2. **Player Memory** (`test_memory_player`)
   - Player data management
   - HP and Meter variables
   - Multiple player support

3. **Entity Memory** (`test_memory_entity`)
   - Entity lifecycle (create, destroy)
   - Entity ID management
   - Entity-specific data storage

### StateHandle Tests

4. **Point To Operations** (`test_state_handle_point_to`)
   - Entity targeting
   - Automatic player ID resolution

5. **Target Entity Operations** (`test_state_handle_target`)
   - Target entity assignment
   - Target entity data retrieval

## Customizing Godot Version

To use a different Godot version:

```bash
GODOT_VERSION=4.3.0 ./scripts/setup-godot.sh
```

Supported versions: Godot 4.2+

## Troubleshooting

### Network Issues

If the download fails due to network restrictions:

1. **Download manually:**
   ```bash
   # Download from https://godotengine.org/download/linux/
   # Extract to .godot-bin/godot
   mkdir -p .godot-bin
   unzip Godot_v4.2.2-stable_linux.x86_64.zip -d .godot-bin
   mv .godot-bin/Godot_v4.2.2-stable_linux.x86_64 .godot-bin/godot
   chmod +x .godot-bin/godot
   ```

2. **Use Docker:**
   ```bash
   # Coming soon: Docker image with Godot pre-installed
   ```

### Build Issues

If the Rust build fails:

```bash
# Check Rust installation
rustc --version
cargo --version

# Clean and rebuild
cargo clean
cargo build --release
```

### Test Failures

If tests fail, check:

1. **Submodule is initialized:**
   ```bash
   git submodule update --init --recursive
   ```

2. **Castagne files are present:**
   ```bash
   ls castagne/engine/CastagneMemory.gd
   ```

3. **Library is built:**
   ```bash
   ls target/release/libcastagne_rs.so
   ```

## Adding New Tests

To add new comparison tests:

1. **Add test method to `src/test_runner.rs`:**
   ```rust
   fn test_my_feature(&mut self) -> bool {
       // Load GDScript
       let script = try_load::<GDScript>("res://castagne/engine/MyFeature.gd");
       // ... comparison logic ...
   }
   ```

2. **Register in `run_comparison_tests()`:**
   ```rust
   results.set("my_feature", self.test_my_feature());
   ```

3. **Run tests:**
   ```bash
   ./scripts/run-tests.sh
   ```

## CI/CD Integration

### GitHub Actions Example

```yaml
name: Comparison Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
        with:
          submodules: true

      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Setup Godot
        run: ./scripts/setup-godot.sh

      - name: Run Tests
        run: ./scripts/run-tests.sh
```

## Performance Benchmarking

The test runner also includes benchmarking capabilities:

```gdscript
# From Godot console or script
var runner = CastagneTestRunner.new()
var benchmark = runner.benchmark_comparison(10000)
print("Rust speedup: ", benchmark["speedup"], "x")
```

This will run both implementations multiple times and compare performance.

## Files Created

The testing setup creates:

```
castagne-rs/
├── .godot-bin/           # Downloaded Godot (gitignored)
│   └── godot             # Godot executable
├── scripts/
│   ├── setup-godot.sh    # Setup script
│   ├── run-tests.sh      # Test runner script
│   └── run_comparison_tests.gd  # GDScript test entry point
├── project.godot         # Godot project file
└── castagne_rs.gdextension  # GDExtension config
```

## Next Steps

- See [TESTING.md](TESTING.md) for comprehensive testing strategies
- See [TODO.md](TODO.md) for planned testing improvements
- Contribute new comparison tests!

## Support

If you encounter issues:
1. Check this guide's troubleshooting section
2. Review the [TESTING.md](TESTING.md) documentation
3. Check existing issues on GitHub
4. Open a new issue with your test output
