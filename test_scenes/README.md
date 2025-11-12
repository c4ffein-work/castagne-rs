# TRUE E2E Test Scenes 🎮

This directory contains **real end-to-end tests** that run Godot headlessly and test the fighting game engine with actual game logic.

## What Makes These TRUE E2E Tests?

Unlike parser integration tests, these tests:

1. **Launch Godot** in headless mode
2. **Execute GDScript** test scenarios
3. **Simulate gameplay** (inputs, combat, state transitions)
4. **Verify game state** changes correctly
5. **Test real scenarios** a player would encounter

## Test Scenes

### Basic Infrastructure
- `test_character_loading.gd` - Verifies characters load into Godot
- `test_rust_parser_integration.gd` - Tests Rust parser integration (TODO)

### State & Input
- `test_state_transitions.gd` - Tests state machine transitions
- `test_input_simulation.gd` - Tests input handling (buttons, motions, charge)

### Combat
- `test_two_character_fight.gd` - Two characters fighting each other
- `test_combo_damage.gd` - Combo system and damage scaling
- `test_special_moves.gd` - Special move execution and properties

### Frame Data
- `test_frame_data.gd` - Frame data accuracy (startup, active, recovery)
- `test_hitbox_collision.gd` - Hitbox/hurtbox collision (TODO)

### Game Scenarios
- `test_full_match.gd` - Complete match simulation (best of 3)
- `test_meter_system.gd` - Meter building and resource management (TODO)

## Running E2E Tests

### Prerequisites

```bash
# Install Godot 4.5
make godot-setup
```

### Run All E2E Tests

```bash
cargo test --test e2e_godot_tests
```

### Run Individual Test

```bash
godot --headless --script test_scenes/test_character_loading.gd --quit
```

## How It Works

1. **Rust test** (`tests/e2e_godot_tests.rs`) invokes Godot headless
2. **Godot** executes the GDScript test scene
3. **Test scene** prints results to stdout
4. **Rust test** parses stdout and asserts on results

## Test Output

Tests print `TEST_PASS` or `TEST_FAIL` to indicate results:

```
=== E2E Test: Character Loading ===

✓ CastagneParser loaded successfully
✓ Character file loaded successfully
✓ Found :Character: section
✓ Found :Variables: section
✓ Found state definitions

Character loaded successfully!
TEST_PASS
```

## Adding New Tests

1. Create a new `.gd` file in `test_scenes/`
2. Extend `SceneTree` (for headless execution)
3. Implement test logic in `_init()`
4. Print `TEST_PASS` or `TEST_FAIL`
5. Call `quit()` to exit
6. Add corresponding Rust test in `tests/e2e_godot_tests.rs`

Example:

```gdscript
extends SceneTree

func _init():
    print("\n=== E2E Test: My Feature ===\n")

    # Test logic here
    var result = test_my_feature()

    if result:
        print("✓ Feature works!")
        print("TEST_PASS")
    else:
        print("✗ Feature failed!")
        print("TEST_FAIL")

    quit()
```

## Philosophy

**These are REAL E2E tests** because they:
- Actually run the game engine
- Test integration of all components
- Simulate real gameplay scenarios
- Run in an environment close to production (Godot runtime)

This is the difference between:
- ❌ Parser integration tests: "Can we parse this .casp file?"
- ✅ E2E tests: "Does a Hadouken work when a player presses QCF+P?"

---

**Let's go! 🥊🦀**
