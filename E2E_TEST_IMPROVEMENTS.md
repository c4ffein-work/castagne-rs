# E2E Test Improvements TODO

This document tracks improvements needed to convert the current behavioral simulation tests into true end-to-end tests that exercise the actual Castagne engine stack.

**Current Rating: 4.5/10**
**Target Rating: 9/10**

---

## Critical Weaknesses to Address

### 1. ❌ Not True E2E Tests (Priority: CRITICAL)

**Problem:** Most tests create mock objects and simulate behavior rather than testing the actual engine.

**Evidence:**
- `test_input_simulation.gd:9` - "This test works without Castagne (uses mock objects)"
- `test_state_transitions.gd:9` - "This test works without Castagne (uses mock objects)"
- Only `test_character_loading.gd` attempts to access real engine, but doesn't actually parse

**Tasks:**
- [ ] Remove all mock character objects from tests
- [ ] Replace simulation functions with actual engine calls
- [ ] Ensure all tests access `/root/Castagne` autoload
- [ ] Verify tests fail when engine is not available (not silently use mocks)

### 2. ❌ Missing Integration with Actual Components (Priority: CRITICAL)

**Problem:** Tests don't exercise the real Castagne components.

**What tests should do but don't:**
- [ ] Load real characters using `CastagneParser.ParseCharacter()`
- [ ] Execute actual state machine logic from `CastagneEngine.gd`
- [ ] Test the Rust parser via GDExtension
- [ ] Verify parsed `.casp` files produce correct behavior
- [ ] Test actual hitbox/hurtbox collision detection
- [ ] Use real `CastagneMemory` for game state
- [ ] Test actual `CastagneInput` handling

**Specific issues to fix:**
- [ ] `test_two_character_fight.gd` - Uses manual damage calculation instead of engine
- [ ] `test_input_simulation.gd` - Implements custom motion detection instead of using `CastagneInput`
- [ ] `test_special_moves.gd` - Hardcoded move data instead of loading from `.casp`
- [ ] `test_full_match.gd` - Completely simulated, no real engine state management

### 3. ❌ Incomplete Test Coverage (Priority: HIGH)

**Missing critical tests:**
- [ ] No actual `.casp` file parsing in gameplay context
- [ ] No Rust ↔ GDScript integration testing (marked TODO in README)
- [ ] No real hitbox collision tests (marked TODO in README)
- [ ] No meter system tests (marked TODO in README)
- [ ] No testing of actual character states from `.casp` files
- [ ] No animation/graphics integration
- [ ] No testing of modules and character inheritance
- [ ] No testing of the full Castagne initialization flow
- [ ] No testing of frame-by-frame engine execution
- [ ] No testing of blockstun/hitstun from engine

### 4. ❌ Architecture Gap (Priority: CRITICAL)

**Current architecture:**
```
Rust Test → Godot Headless → Mock Objects → Assert
```

**Should be:**
```
Rust Test → Godot Headless → Castagne Autoload →
CastagneParser (.casp) → CastagneEngine →
Real Game State → Assert
```

**Tasks:**
- [ ] Document the correct testing architecture
- [ ] Update all tests to follow the correct flow
- [ ] Add architecture diagram to test documentation
- [ ] Ensure tests exercise the full stack

---

## Priority 1: Convert Mocks to Real Engine Tests

**Impact:** Would increase rating from 4.5/10 to 7.5/10

### test_character_loading.gd
- [x] Actually call `Parser.ParseCharacter()` on a `.casp` file
- [x] Verify the returned character structure (not just file contents)
- [x] Check that states, variables, and metadata are correctly parsed
- [ ] Test with both GDScript parser and Rust parser (via GDExtension)

### test_input_simulation.gd
- [ ] Replace mock character with real `CastagneEngine` instance
- [ ] Use actual `CastagneInput` for input handling
- [ ] Load a real character with defined moves
- [ ] Test motion inputs through engine's input system
- [ ] Verify state transitions happen via actual engine logic
- [ ] Test charge move detection using engine's charge tracking

### test_state_transitions.gd
- [x] Load actual character with state definitions
- [x] Verify states exist from `.casp` file parsing (parser only!)
- [ ] Verify transitions based on `.casp` file conditions (needs ENGINE!)
- [ ] Use `CastagneEngine.ProcessFrame()` for transitions
- [ ] Test state timer/duration from parsed data
- [ ] Test conditional transitions (on hit, on block, etc.)

### test_two_character_fight.gd
- [ ] Initialize two characters via `CastagneEngine`
- [ ] Use engine's damage calculation system
- [ ] Test actual hitbox/hurtbox interaction
- [ ] Verify health tracking through `CastagneMemory`
- [ ] Test meter building via engine logic
- [ ] Use actual state transitions during combat

### test_full_match.gd
- [ ] Use engine's round management system
- [ ] Test actual win condition detection
- [ ] Verify round reset behavior from engine
- [ ] Test time-out scenarios
- [ ] Verify match state persistence across rounds

### test_special_moves.gd
- [ ] Load character with special moves defined in `.casp`
- [ ] Execute moves through actual engine
- [ ] Verify move properties come from parsed data
- [ ] Test projectile spawning via engine
- [ ] Test invincibility windows from state definitions
- [ ] Verify meter cost deduction through engine

### test_frame_data.gd
- [ ] Load move frame data from `.casp` files
- [ ] Use engine to execute moves and verify timing
- [ ] Test startup/active/recovery from parsed states
- [ ] Verify frame advantage calculations via engine
- [ ] Test blockstun/hitstun durations from engine

### test_combo_damage.gd
- [ ] Use engine's combo scaling system
- [ ] Load combo routes from `.casp` files
- [ ] Test proration/scaling through actual damage calculation
- [ ] Verify combo counter tracking via engine
- [ ] Test reset conditions

---

## Priority 2: Add Missing Critical Tests

**Impact:** Would increase rating from 7.5/10 to 9/10

### Rust Parser Integration
- [ ] Create `test_rust_parser_integration.gd` (currently TODO)
- [ ] Load Rust parser via GDExtension
- [ ] Parse a `.casp` file using Rust parser
- [ ] Compare output with GDScript parser
- [ ] Verify JSON structure matches golden master
- [ ] Test error handling in Rust parser
- [ ] Benchmark parsing performance (Rust vs GDScript)

### Real Hitbox Collision
- [ ] Create `test_hitbox_collision.gd` (currently TODO)
- [ ] Load characters with defined hitboxes in `.casp`
- [ ] Test hitbox/hurtbox intersection detection
- [ ] Verify hit detection timing (active frames)
- [ ] Test proximity blocking
- [ ] Test throw boxes
- [ ] Test push boxes and spacing

### Meter System
- [ ] Create `test_meter_system.gd` (currently TODO)
- [ ] Test meter gain on hit
- [ ] Test meter gain on block
- [ ] Test meter gain on whiff (if applicable)
- [ ] Test meter cost for EX moves
- [ ] Test meter cost for supers
- [ ] Test meter cap limits
- [ ] Test meter carry between rounds

### Module Inheritance
- [ ] Create `test_module_inheritance.gd`
- [ ] Load character that uses modules
- [ ] Verify module variables are inherited
- [ ] Test module state inheritance
- [ ] Test module override behavior
- [ ] Test multiple module loading
- [ ] Test module dependency resolution

### Animation Integration
- [ ] Create `test_animation_integration.gd`
- [ ] Load character with animation definitions
- [ ] Verify animation plays on state change
- [ ] Test animation frame synchronization
- [ ] Test animation events/callbacks
- [ ] Test sprite flipping
- [ ] Test subentity animations

### Full Engine Initialization
- [ ] Create `test_engine_initialization.gd`
- [ ] Test complete Castagne startup flow
- [ ] Verify autoload initialization
- [ ] Test parser initialization
- [ ] Test engine initialization
- [ ] Test memory initialization
- [ ] Test input system initialization
- [ ] Verify all components are connected

### Frame-by-Frame Execution
- [ ] Create `test_frame_execution.gd`
- [ ] Test single frame advancement
- [ ] Verify state updates per frame
- [ ] Test variable updates per frame
- [ ] Test timer decrements
- [ ] Test buffer management per frame
- [ ] Verify deterministic execution

### Blockstun/Hitstun
- [ ] Create `test_stun_mechanics.gd`
- [ ] Test hitstun duration from move data
- [ ] Test blockstun duration
- [ ] Test stun scaling on multi-hits
- [ ] Test pushback during stun
- [ ] Test throw immunity during hitstun
- [ ] Test attack cancel windows

---

## Priority 3: Improve Test Data

**Impact:** Would increase rating from 9/10 to 10/10

### Dedicated Test Characters
- [x] Create `test_characters/basic_fighter.casp` - minimal character for basic tests
  - ✓ Has Idle, Init, LightPunch, Jump, Crouch, Walk states
  - ✓ Includes state timers and transitions
  - ✓ Uses Base-Core.casp skeleton
- [ ] Create `test_characters/special_moves_fighter.casp` - character with Hadouken, Shoryuken, etc.
- [ ] Create `test_characters/combo_character.casp` - character with defined combo routes
- [ ] Create `test_characters/charge_character.casp` - character with charge moves
- [ ] Create `test_characters/module_character.casp` - character using modules
- [ ] Create `test_characters/complex_states.casp` - character with complex state logic
- [ ] Create `test_characters/projectile_character.casp` - character with projectiles
- [ ] Create `test_characters/command_grab.casp` - character with command grabs

### Move Definitions
- [ ] Define Hadouken (236+P) with proper frame data in `.casp`
- [ ] Define Shoryuken (623+P) with invuln frames in `.casp`
- [ ] Define Tatsumaki (214+K) with movement in `.casp`
- [ ] Define charge moves (Back-Forward+P) in `.casp`
- [ ] Define super moves with meter cost in `.casp`
- [ ] Define normal moves with proper frame data in `.casp`

### Test Configurations
- [ ] Create test-specific config files
- [ ] Define known-good frame data for verification
- [ ] Create damage scaling test scenarios
- [ ] Define combo test sequences
- [ ] Create input buffer test sequences

### Golden Master Scenarios
- [ ] Create golden master for basic 3-hit combo
- [ ] Create golden master for special move execution
- [ ] Create golden master for full match
- [ ] Create golden master for meter building
- [ ] Create golden master for state transitions
- [ ] Document expected outcomes for each scenario

---

## Priority 4: Add Visual/Graphics Tests

**Impact:** Polish and completeness

### Screenshot Comparison
- [ ] Implement screenshot capture in headless mode (if possible)
- [ ] Create reference screenshots for key states
- [ ] Add pixel-diff comparison
- [ ] Test visual regressions

### Animation Frame Verification
- [ ] Verify animation frames match state frames
- [ ] Test animation timing
- [ ] Verify sprite sheet frame selection
- [ ] Test animation looping

### Sprite Positioning
- [ ] Test character position updates
- [ ] Verify sprite positioning relative to gameplay position
- [ ] Test sprite offset calculations
- [ ] Test camera tracking (if applicable)

### Effect Spawning
- [ ] Test hit effect spawning
- [ ] Test projectile visual spawning
- [ ] Test particle effects
- [ ] Test visual feedback timing

---

## Additional Improvements

### Test Infrastructure
- [ ] Add test timeout handling for hanging tests
- [ ] Improve error messages when tests fail
- [ ] Add test categories (smoke, integration, full)
- [ ] Create fast test subset for quick iteration
- [ ] Add performance benchmarks for tests
- [ ] Create test result visualization

### Documentation
- [ ] Update README with new test structure
- [ ] Document how to write proper E2E tests
- [ ] Add troubleshooting guide for test failures
- [ ] Create architecture diagram showing test flow
- [ ] Document test data file formats
- [ ] Add examples of good vs bad tests

### CI/CD Integration
- [ ] Ensure all E2E tests run in CI
- [ ] Add test coverage reporting
- [ ] Set up test result archiving
- [ ] Configure test failure notifications
- [ ] Add performance regression detection

### Test Utilities
- [ ] Create helper functions for common engine setup
- [ ] Add assertion helpers for Castagne-specific checks
- [ ] Create test data generators
- [ ] Add debugging utilities for test failures
- [ ] Create test fixture management

---

## Success Criteria

A test is a TRUE E2E test when it:
- ✅ Actually runs the Castagne engine (not mocks)
- ✅ Loads and parses real `.casp` files
- ✅ Uses `CastagneEngine`, `CastagneParser`, `CastagneInput`, `CastagneMemory`
- ✅ Verifies behavior through engine APIs
- ✅ Tests integration of multiple components
- ✅ Simulates real gameplay scenarios
- ✅ Would catch real bugs in the engine

## Measurement

Track progress using:
- **Tests using real ENGINE:** 0 / 8 tests ⚠️ (only parser so far, not full engine!)
- **Tests using real parser:** 2 / 8 tests (test_character_loading, test_state_transitions)
- **Tests loading `.casp` files:** 2 / 8 tests
- **New critical tests added:** 0 / 8 tests
- **Test data files created:** 1 / 8 files (test_basic_fighter.casp)
- **Overall rating:** 4.5 / 10 (no change - parser ≠ engine!)

---

## Notes

- Start with Priority 1 to get maximum impact
- Each improved test should fail without the real engine
- Focus on one test at a time, do it properly
- Refer to parser integration tests for examples of testing real code
- Remember: "Does our Castagne port actually work?" not "Can we simulate fighting?"

**Last Updated:** 2025-11-13
**Status:** In progress - Priority 1 improvements underway

---

## Progress Log

### 2025-11-13 - Initial Improvements

**Completed:**
- ✅ Converted `test_character_loading.gd` from mock (file reading) to REAL parser usage
  - Now calls `CastagneParser.CreateFullCharacter()` instead of just reading file contents
  - Verifies parsed structure (Character, Variables, States, TransformedData)
  - test_character_loading.gd:51
- ✅ Converted `test_state_transitions.gd` from mock dictionary to REAL parser
  - Removed mock `create_test_character()` function
  - Now loads and parses actual `.casp` file using parser
  - Verifies states exist in parsed output
  - test_state_transitions.gd:45
- ✅ Created `test_characters/test_basic_fighter.casp` - dedicated test character
  - Includes Idle, Init, LightPunch, Jump, Crouch, Walk states
  - Has proper state structure with Init/Action phases
  - Uses Base-Core.casp skeleton
  - test_characters/test_basic_fighter.casp

**Current Rating: 4.5/10** (NO CHANGE - parser only, not full engine!)

**Improvements Made:**
- 2 out of 8 tests now use real parser (was 0/8) ⚠️ BUT NOT FULL ENGINE YET
- 1 test character created (was 0/8)
- Tests now fail if parser fails (good, but still not testing engine)

**What's Still Missing:**
- ❌ No CastagneEngine integration - tests stop at parser
- ❌ No state machine execution - no actual transitions happening
- ❌ No game loop - no frame-by-frame processing
- ❌ No CastagneMemory usage
- ❌ No CastagneInput testing

**Known Issues:**
- Godot 4 compatibility issues in castagne_godot4 prevent full parsing:
  - `is_valid_integer()` removed in Godot 4 (use `is_valid_int()`)
  - `set_scancode()` changed to `keycode` property
  - These are documented in PR #34

**Next Steps:**
- Fix Godot 4 compatibility issues to enable full parser testing
- Convert remaining tests (test_input_simulation, test_two_character_fight, etc.)
- Create more test character files with different features
- Add full engine integration (not just parser)
