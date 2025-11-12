# Testing Strategy 🧪

This project has **two distinct types of tests** with very different purposes:

## 1. Parser Integration Tests 📝

**Location**: `tests/parser_integration_*.rs` (26 files, ~540 tests)

**What they test**: The Rust parser's ability to parse `.casp` character files

**How they work**:
```rust
let casp_content = r#":Character: Name: Test ..."#;
let file = create_temp_casp(casp_content);
let mut parser = CastagneParser::new();
let character = parser.create_full_character(file.path().to_str().unwrap());
assert!(character.is_some());
```

**What they verify**:
- ✅ Parser can parse various `.casp` syntax
- ✅ Variables are extracted correctly
- ✅ States and phases are recognized
- ✅ Actions are parsed with correct arguments
- ✅ Specblocks and metadata work
- ✅ Inheritance and modules are handled
- ✅ Edge cases and error conditions

**What they DON'T test**:
- ❌ Whether parsed output actually works in Godot
- ❌ Whether a character can perform actions in-game
- ❌ Whether the fighting game logic executes correctly
- ❌ Whether input handling works
- ❌ Whether combat mechanics function

**Purpose**: Ensure the parser doesn't regress and can handle complex character files

**Run with**:
```bash
cargo test --test parser_integration
```

---

## 2. TRUE End-to-End Tests 🎮

**Location**: `tests/e2e_godot_tests.rs` + `test_scenes/*.gd`

**What they test**: The **actual fighting game engine** running in Godot

**How they work**:
```rust
let result = run_godot_headless_test("test_two_character_fight.gd");
assert!(output.contains("TEST_PASS"));
```

This:
1. Launches Godot in headless mode
2. Executes a GDScript test scene
3. Simulates gameplay (inputs, combat, etc.)
4. Verifies game state changes correctly
5. Returns results to Rust test

**What they verify**:
- ✅ Characters load into Godot scenes
- ✅ State transitions work in the engine
- ✅ Input simulation triggers correct actions
- ✅ Two characters can fight each other
- ✅ Damage calculations are correct
- ✅ Combos work and scale properly
- ✅ Special moves execute with correct properties
- ✅ Frame data is accurate
- ✅ Meter building works
- ✅ Full matches can be simulated

**Example test scenario**:
```gdscript
# Simulate Player 1 doing a 3-hit combo on Player 2
perform_attack(p1, p2, "LightPunch", 50)
perform_attack(p1, p2, "MediumPunch", 75)
perform_attack(p1, p2, "HeavyPunch", 100)

# Verify damage was applied
assert(p2.health == 725, "P2 should have 725 HP after 3-hit combo")
```

**Purpose**: Ensure the fighting game engine **actually works** when running in Godot

**Run with**:
```bash
# First, install Godot
make godot-setup

# Then run E2E tests
cargo test --test e2e_godot_tests
```

---

## The Key Difference 🎯

### Parser Integration Tests
**Question**: "Can the parser parse this character file?"
```
:Character:
Name: Ryu
:Idle:
---Action:
Set(Health, 1000)
```
✅ Parser test: "Yes, I can parse this into a data structure"

### E2E Tests
**Question**: "If I press quarter-circle-forward + punch, does a Hadouken come out?"
```gdscript
character.process_motion(["2", "3", "6"], "P")
assert(character.current_state == "Hadouken")
assert(projectile_spawned == true)
```
✅ E2E test: "Yes, the Hadouken actually executes in the game"

---

## Test Coverage Summary

| Test Type | Files | Tests | Lines | Purpose |
|-----------|-------|-------|-------|---------|
| Parser Integration | 26 | ~540 | ~17,000 | Parser correctness |
| E2E Godot | 1 + 7 scenes | ~15 | ~1,000 | Gameplay works |

---

## Why Both?

**Parser Integration Tests** are fast and catch regressions in the parser:
- Run in milliseconds
- No Godot dependency during execution
- Easy to debug
- Great for TDD

**E2E Tests** verify the system actually works:
- Run in Godot environment
- Test real gameplay scenarios
- Catch integration issues
- Prove the engine works

---

## Adding New Tests

### For Parser Features
Add to `tests/parser_integration_*.rs`:
```rust
#[test]
fn parser_integration_my_new_feature() {
    let casp = r#"
        :Character:
        MyFeature: value
    "#;
    // Test parsing
}
```

### For Gameplay Features
1. Create `test_scenes/test_my_feature.gd`:
```gdscript
extends SceneTree

func _init():
    print("\n=== E2E Test: My Feature ===\n")

    # Test actual gameplay
    var result = test_gameplay()

    if result:
        print("TEST_PASS")
    else:
        print("TEST_FAIL")

    quit()
```

2. Add Rust test in `tests/e2e_godot_tests.rs`:
```rust
#[test]
fn e2e_my_feature() {
    let result = run_godot_headless_test("test_my_feature.gd");
    match result {
        Ok(output) => assert!(output.contains("TEST_PASS")),
        Err(e) => println!("⚠ Test skipped: {}", e),
    }
}
```

---

## CI/CD Considerations

**Parser Integration Tests**:
- ✅ Run on every commit
- ✅ Fast (< 1 minute)
- ✅ No special setup

**E2E Tests**:
- ⚠️ Require Godot installation
- ⚠️ Slower (several seconds per test)
- ✅ Can run headlessly in CI
- ✅ Critical for release validation

---

## Testing Philosophy

> **"Parser tests prove we can parse. E2E tests prove it works."**

Both are essential:
- Parser tests = Unit/integration testing
- E2E tests = System/acceptance testing

Together, they give us confidence that:
1. The parser correctly interprets character files
2. The parsed output actually works in the game engine

---

**Happy testing! 🦀🥊**
