# Testing Guide for Castagne-RS

This document explains how to test both the original GDScript Castagne engine and the Rust port.

## 📋 Table of Contents
1. [GDScript Testing](#gdscript-testing)
2. [Rust Testing](#rust-testing)
3. [Integration Testing](#integration-testing)
4. [CI/CD Setup](#cicd-setup)

---

## 🎮 GDScript Testing

### Recommended Framework: GUT (Godot Unit Test)

**Why GUT?**
- Mature and stable
- Simple to set up
- Good for comparing implementations
- Works great with CI/CD
- Active community support

### Installation

**Option 1: Godot Asset Library**
1. Open Godot Editor
2. Go to AssetLib tab
3. Search for "GUT"
4. Download and install

**Option 2: Manual Installation**
```bash
cd castagne/
git clone https://github.com/bitwes/Gut.git addons/gut
```

### Creating Tests

Create a `tests/` directory in the Castagne submodule:

```
castagne/
├── engine/
│   ├── CastagneMemory.gd
│   └── ...
└── tests/
    ├── test_memory.gd
    ├── test_state_handle.gd
    └── test_engine.gd
```

#### Example: test_memory.gd

```gdscript
extends GutTest

var Memory = load("res://engine/CastagneMemory.gd")
var memory

func before_each():
    memory = Memory.new()
    memory.InitMemory()

func after_each():
    memory.free()

func test_global_set_and_get():
    memory.GlobalSet("TestKey", 42, true)
    assert_eq(memory.GlobalGet("TestKey"), 42, "Should store and retrieve value")

func test_global_has():
    memory.GlobalSet("Exists", true, true)
    assert_true(memory.GlobalHas("Exists"), "Should return true for existing key")
    assert_false(memory.GlobalHas("DoesNotExist"), "Should return false for missing key")

func test_player_memory():
    memory.AddPlayer()
    memory.PlayerSet(0, "HP", 100, true)
    assert_eq(memory.PlayerGet(0, "HP"), 100, "Player HP should be 100")
    assert_true(memory.PlayerHas(0, "HP"), "Should have HP key")

func test_entity_lifecycle():
    var eid = memory.AddEntity()
    assert_true(memory.IsEIDValid(eid), "New entity should be valid")

    memory.EntitySet(eid, "Position", Vector2(10, 20), true)
    assert_eq(memory.EntityGet(eid, "Position"), Vector2(10, 20), "Should store position")

    memory.RemoveEntity(eid)
    assert_false(memory.IsEIDValid(eid), "Removed entity should be invalid")

func test_copy_from():
    var memory2 = Memory.new()
    memory.GlobalSet("Test", 123, true)

    memory2.CopyFrom(memory)
    assert_eq(memory2.GlobalGet("Test"), 123, "Should copy data")

    memory2.free()
```

### Running GDScript Tests

**In Godot Editor:**
1. Open GUT panel (bottom of editor)
2. Select tests directory
3. Click "Run All"

**Command Line (Headless):**
```bash
# From the castagne directory
godot --headless -s addons/gut/gut_cmdln.gd -gtest=tests/

# Or with specific options:
godot --headless -s addons/gut/gut_cmdln.gd \
    -gtest=tests/ \
    -gexit \
    -glog=2  # Verbosity level
```

---

## 🦀 Rust Testing

The Rust port includes unit tests using Rust's built-in test framework.

### Running Rust Tests

```bash
cd castagne-rs/

# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_global_memory_operations

# Run tests in a specific module
cargo test memory::tests
```

### Example Test Output

```
running 6 tests
test memory::tests::test_memory_initialization ... ok
test memory::tests::test_global_memory_operations ... ok
test memory::tests::test_player_memory ... ok
test memory::tests::test_entity_lifecycle ... ok
test memory::tests::test_copy_from ... ok
test memory::tests::test_invalid_entity_access ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### Writing Rust Tests

Tests are included at the bottom of each module file:

```rust
// src/memory.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_your_feature() {
        let mut memory = CastagneMemory::new();
        memory.global_set("Test", Variant::from(42), true);

        assert!(memory.global_has("Test"));
        assert_eq!(
            memory.global_get("Test").unwrap().try_to::<i32>().unwrap(),
            42
        );
    }
}
```

### Adding Tests for New Modules

When you port a new module, add tests:

```rust
// src/my_module.rs

pub struct MyModule {
    // ...
}

impl MyModule {
    // implementation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_feature() {
        let module = MyModule::new();
        // Test your feature
    }
}
```

---

## 🔄 Integration Testing

Test that the Rust port behaves identically to GDScript.

### Approach 1: Behavior Verification

Create GDScript tests that document expected behavior, then verify Rust matches:

**GDScript test (documents behavior):**
```gdscript
func test_memory_global_operations_behavior():
    var memory = CastagneMemory.new()

    # Test 1: New keys require new_value flag
    memory.GlobalSet("Key1", 10, true)  # Should work
    assert_eq(memory.GlobalGet("Key1"), 10)

    # Test 2: Existing keys can be updated without flag
    memory.GlobalSet("Key1", 20, false)  # Should work
    assert_eq(memory.GlobalGet("Key1"), 20)

    # Test 3: Non-existent key without flag should error (just logs)
    # memory.GlobalSet("Key2", 30, false)  # Would error
```

**Rust test (verifies same behavior):**
```rust
#[test]
fn test_memory_global_operations_behavior() {
    let mut memory = CastagneMemory::new();

    // Test 1: New keys require new_value flag
    memory.global_set("Key1", Variant::from(10), true);
    assert_eq!(memory.global_get("Key1").unwrap().try_to::<i32>().unwrap(), 10);

    // Test 2: Existing keys can be updated without flag
    memory.global_set("Key1", Variant::from(20), false);
    assert_eq!(memory.global_get("Key1").unwrap().try_to::<i32>().unwrap(), 20);
}
```

### Approach 2: Snapshot Testing

1. Run GDScript version, capture state
2. Run Rust version with same inputs
3. Compare outputs

```gdscript
# test_snapshot.gd
func test_create_snapshot():
    var memory = CastagneMemory.new()
    memory.GlobalSet("HP", 100, true)
    memory.AddPlayer()
    memory.PlayerSet(0, "Name", "Player1", true)

    # Export state
    var snapshot = {
        "global": memory._memoryGlobal,
        "players": memory._memoryPlayers,
        "entities": memory._memoryEntities
    }

    var file = FileAccess.open("res://tests/snapshots/memory_snapshot.json", FileAccess.WRITE)
    file.store_string(JSON.stringify(snapshot))
```

---

## 🚀 CI/CD Setup

### GitHub Actions Example

Create `.github/workflows/test.yml`:

```yaml
name: Tests

on: [push, pull_request]

jobs:
  rust-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          submodules: true

      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Run Rust tests
        run: |
          cd castagne-rs
          cargo test --verbose

  gdscript-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          submodules: true

      - name: Download Godot
        run: |
          wget https://github.com/godotengine/godot/releases/download/4.3-stable/Godot_v4.3-stable_linux.x86_64.zip
          unzip Godot_v4.3-stable_linux.x86_64.zip
          chmod +x Godot_v4.3-stable_linux.x86_64

      - name: Run GDScript tests
        run: |
          cd castagne
          ../Godot_v4.3-stable_linux.x86_64 --headless -s addons/gut/gut_cmdln.gd -gtest=tests/ -gexit
```

---

## 📊 Testing Strategy Summary

### For the Original Castagne (GDScript)
✅ Use **GUT** for unit tests
✅ Test each module independently
✅ Document expected behaviors
✅ Create regression tests for bugs

### For the Rust Port
✅ Use **cargo test** for unit tests
✅ Mirror GDScript test structure
✅ Test memory management carefully
✅ Test Godot integration separately

### For Both
✅ Test edge cases (invalid IDs, null values)
✅ Test performance critical paths
✅ Test state serialization (for rollback)
✅ Create integration tests for full scenarios

---

## 🎯 Next Steps

1. **Set up GUT** in the castagne submodule
2. **Write baseline tests** for CastagneMemory
3. **Expand Rust tests** as you port more modules
4. **Set up CI/CD** to run tests automatically
5. **Create comparison tests** to verify parity

Would you like help setting up any of these testing frameworks?
