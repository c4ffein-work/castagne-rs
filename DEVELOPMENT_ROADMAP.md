# Castagne-RS Development Roadmap

## Quick Reference

### Current Status
- **Overall Progress**: 35% complete (4,032 lines)
- **Build Status**: ✅ Compiles successfully
- **Test Status**: 9/18 unit tests pass (expect Godot runtime failures)
- **Primary Focus**: Parser development (40% complete)

### Development Quick Links

| Component | Status | Files | Priority |
|-----------|--------|-------|----------|
| Parser | 40% | parser.rs (866) | 🔴 HIGH |
| Memory | 100% | memory.rs (266) | ✅ DONE |
| StateHandle | 100% | state_handle.rs (315) | ✅ DONE |
| Engine | 80% | engine.rs (233) | 🟡 MEDIUM |
| Modules | 30% | core_module.rs (256) | 🔴 HIGH |
| Input | 65% | input.rs (546) | 🟡 MEDIUM |
| Global | 80% | global.rs (398) | ✅ MOSTLY DONE |
| Network | 10% | net.rs (269) | 🔵 LOW |

---

## Phase 1: Parser Completion (Next 2-3 Sessions)

### What Parser Can Do Now
```
✓ Load .casp files
✓ Parse metadata (:Character: block)
✓ Parse variables (:Variables: block)
✓ Parse states and phases
✓ Parse function calls with complex arguments
✗ Parse specblocks (constants)
✗ Handle skeleton inheritance
✗ Execute parsed scripts
```

### Phase 1 Tasks (Estimated 200-400 lines total)

#### Task 1.1: Specblock Parsing (80 lines, 1-2 hours)
**File**: `src/parser.rs::parse_specblocks()`

**What**: Parse `:SpecblockName:` blocks that define constants

**Implementation**:
```rust
fn parse_specblocks(&mut self, _file_id: usize) -> HashMap<String, String> {
    let mut specblocks = HashMap::new();
    // Find all :BlockName: sections (not special blocks)
    // Parse key: value pairs inside them
    // Store in specblock_defines
    specblocks
}
```

**Test**: Add `test_parse_specblocks()` unit test

**Impact**: Characters can define and use constants

---

#### Task 1.2: Skeleton Inheritance (100 lines, 2-3 hours)
**Files**: `src/parser.rs::parse_full_file()`, `src/parser.rs::open_file()`

**What**: Load parent character files recursively

**Implementation**:
1. Modify `parse_full_file()` after metadata parsing:
   ```rust
   // If metadata has skeleton, load that file first
   if let Some(skeleton_path) = &self.metadata.skeleton {
       self.open_file(&skeleton_path);
       self.parse_full_file(); // Recursively parse skeleton
   }
   ```

2. Merge parsed data from parent with child

**Impact**: Code reuse between characters

---

#### Task 1.3: Type Conversion (80 lines, 2-3 hours)
**File**: `src/parser.rs` (new: `parse_value()` method)

**What**: Convert variable values from strings to actual types

**Implementation**:
```rust
fn parse_value(&self, value_str: &str, var_type: &VariableType) -> Variant {
    match var_type {
        VariableType::Int => Variant::from(value_str.parse::<i32>().unwrap_or(0)),
        VariableType::Str => Variant::from(value_str),
        VariableType::Bool => Variant::from(value_str == "true" || value_str == "1"),
        VariableType::Vec2 => {
            // Parse "x, y" format
            // Return Variant::from(Vector2::new(x, y))
        },
        // ... other types
    }
}
```

**Update**: Modify `ParsedVariable` to use typed values instead of strings

**Impact**: Proper type safety for variables

---

### Phase 1 Success Criteria
- [x] All parser tests pass (already passing)
- [ ] Specblock parsing works
- [ ] Skeleton loading works
- [ ] Type conversion accurate
- [ ] Can parse test_character_advanced.casp completely

### Phase 1 Timeline
- Specblock: 1-2 hours
- Skeleton: 2-3 hours
- Types: 2-3 hours
- **Total**: 5-8 hours = 1-2 development sessions

---

## Phase 2: Script Execution (Next 3-4 Sessions)

### What's Needed
Currently parser can **load** scripts but can't **execute** them.

### Phase 2 Tasks

#### Task 2.1: Instruction Routing (150 lines)
**Files**: `src/parser.rs`, new: `src/executor.rs`

**What**: Execute parsed action instructions during engine phases

**Implementation**:
```rust
pub struct CastagneExecutor {
    memory: Rc<RefCell<CastagneMemory>>,
    parser: CastagneParser,
}

impl CastagneExecutor {
    fn execute_action(&self, action: &ParsedAction, state_handle: &mut CastagneStateHandle) {
        match action.instruction.as_str() {
            "Set" => self.exec_set(&action.args, state_handle),
            "Add" => self.exec_add(&action.args, state_handle),
            "If" => self.exec_if(&action.args, state_handle),
            // ... route to appropriate handler
        }
    }
}
```

**Connect**: Hook into engine phases:
```rust
// In engine.rs action_phase_start
for entity_id in active_entities {
    state_handle.point_to_entity(entity_id);
    if let Some(state_actions) = current_state.actions.get("Action") {
        for action in state_actions {
            executor.execute_action(action, &mut state_handle);
        }
    }
}
```

---

#### Task 2.2: Core Instructions (200 lines)
**File**: `src/executor.rs`

**What**: Implement essential instruction types

**Instructions to Implement**:
1. **Set** - Set variable: `Set(Variable, Value)`
2. **Add** - Add to variable: `Add(Variable, Amount)`
3. **If/Else/EndIf** - Conditional execution
4. **ChangeState** - Change entity state: `ChangeState(StateName)`
5. **CallFunction** - Call registered function

**Example**:
```rust
fn exec_set(&self, args: &[String], state_handle: &mut CastagneStateHandle) {
    if args.len() < 2 { return; }
    let var_name = &args[0];
    let value = self.resolve_value(&args[1], state_handle);
    state_handle.entity_set(var_name, value);
}
```

---

### Phase 2 Success Criteria
- [ ] Scripts load and execute without panic
- [ ] Variables change during execution
- [ ] Test character runs its scripts
- [ ] State transitions work

### Phase 2 Timeline: 6-8 hours (1-2 sessions)

---

## Phase 3: Physics Module (Next 2-3 Sessions)

### What's Needed
Entities need to **move** and have **positions**.

### Task 3.1: CMPhysics2D Basics (200 lines)
**File**: `src/modules/physics_module.rs` (new)

**Variables to Register**:
```rust
fn new() -> Self {
    let mut this = Self::default();
    
    // Entity variables
    this.register_variable_entity("Position", Vec2::new(0.0, 0.0));
    this.register_variable_entity("Velocity", Vec2::new(0.0, 0.0));
    this.register_variable_entity("Acceleration", Vec2::new(0.0, 0.0));
    
    // Global variables
    this.register_variable_global("Gravity", Variant::from(10.0));
    
    this
}
```

**Core Logic**:
```rust
fn physics_phase_start_entity(&mut self, state_handle: &mut CastagneStateHandle) {
    // Get position, velocity, acceleration
    let mut pos = state_handle.entity_get("Position").unwrap();
    let vel = state_handle.entity_get("Velocity").unwrap();
    let gravity = state_handle.global_get("Gravity").unwrap();
    
    // Apply gravity: vel.y += gravity
    // Apply velocity: pos += vel
    
    // Set new values
    state_handle.entity_set("Position", pos);
    state_handle.entity_set("Velocity", vel);
}
```

### Phase 3 Success Criteria
- [ ] Entities have Position variable
- [ ] Entities move when Velocity is set
- [ ] Gravity affects vertical movement

### Phase 3 Timeline: 4-6 hours (1 session)

---

## Quick Win: Add to-do List

### Immediate (This Week)
1. [ ] Complete parser specblock support
2. [ ] Test with test_character_advanced.casp
3. [ ] Document specblock syntax

### Short Term (Next Week)
1. [ ] Implement skeleton inheritance
2. [ ] Add type conversion
3. [ ] Create script executor
4. [ ] Test script execution

### Medium Term (2-3 Weeks)
1. [ ] Port basic physics module
2. [ ] Test entity movement
3. [ ] Add more core instructions
4. [ ] Create simple test game

---

## Testing Strategy

### For Each Task
1. **Unit Tests** (Pure Rust)
   ```bash
   cargo test --lib
   ```
   - Test parsing logic
   - Test type conversion
   - Test instruction routing

2. **Integration Tests** (Requires Godot)
   ```bash
   ./scripts/run-tests.sh
   ```
   - Test parsing + execution
   - Compare vs GDScript
   - Verify parity

3. **Manual Testing**
   - Load test_character.casp
   - Verify parsed output
   - Check execution results

### Test Files to Use
- `test_character.casp` - Simple (17 lines)
- `test_character_advanced.casp` - Complex (52 lines)

---

## Common Patterns to Use

### Adding a Parser Feature
```rust
fn parse_new_feature(&mut self) {
    // 1. Find section start
    // 2. Parse content
    // 3. Validate
    // 4. Store in data structure
    
    // 5. Add unit test:
    // #[test]
    // fn test_new_feature() {
    //     let mut parser = CastagneParser::new();
    //     // Set up test data
    //     parser.parse_new_feature();
    //     // Assert results
    // }
}
```

### Adding Module Variables
```rust
fn register_variable_entity(&mut self, name: &str, value: Variant) {
    self.variables.insert(name.to_string(), (
        value,
        VariableFlags {
            reset_each_frame: false,
            no_init: false,
            inherit_to_subentity: false,
        }
    ));
}
```

### Executing Instructions
```rust
fn execute_instruction(&self, instruction: &str, args: &[String], handle: &mut StateHandle) {
    match instruction {
        "Set" => {
            let var_name = &args[0];
            let value = self.resolve_value(&args[1], handle);
            handle.entity_set(var_name, value);
        },
        // ... more cases
    }
}
```

---

## Resources

### Documentation Files
- `/home/user/castagne-rs/TODO.md` - Comprehensive status (500+ lines)
- `/home/user/castagne-rs/TESTING.md` - Testing guide
- `/home/user/castagne-rs/TESTING_CLI.md` - CLI testing guide
- `/home/user/castagne-rs/README.md` - Project overview

### Example Code to Study
- `src/memory.rs` - Memory implementation (well-commented)
- `src/state_handle.rs` - Context wrapper pattern
- `src/modules/core_module.rs` - Module implementation
- `src/parser.rs` - Parser structure and patterns

### Original GDScript Reference
- `castagne/engine/CastagneParser.gd` - Original parser (2,279 lines)
- `castagne/engine/CastagneMemory.gd` - Original memory system
- `castagne/modules/` - Original game modules

---

## Success Checklist

### To Unlock Character Loading
- [ ] Parser completes specblock parsing
- [ ] Parser handles skeleton inheritance
- [ ] Parser does type conversion
- [ ] Parser produces valid ParsedCharacter

### To Unlock Script Execution
- [ ] Executor routes instructions
- [ ] Basic instructions implemented (Set, Add, If)
- [ ] Instructions modify entity state
- [ ] State transitions work

### To Unlock Gameplay
- [ ] Physics module provides movement
- [ ] Entities have position/velocity
- [ ] Gravity works correctly
- [ ] Manual test shows visible movement

---

## Troubleshooting

### "Parser tests fail"
- Check file exists at path
- Verify file format matches expected
- Use `parser.logs_active = true` for debug output

### "Godot runtime tests fail"
- This is expected without Godot
- Use integration tests instead
- Run: `./scripts/run-tests.sh`

### "Module callback not called"
- Check module registered in engine
- Check phase name matches
- Verify state_handle passed correctly

---

## Next Steps

1. **Pick a phase** based on your interests
2. **Read the relevant section** above
3. **Start with Task 1** of your chosen phase
4. **Use existing patterns** as templates
5. **Test thoroughly** before moving to next task
6. **Commit frequently** with clear messages

---

**Last Updated**: Nov 6, 2025  
**Recommended First Task**: Parser Phase 1.1 (Specblock Parsing)  
**Estimated Time to Playable**: 10-15 hours development work

