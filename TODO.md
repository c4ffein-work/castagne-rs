# Castagne-RS: TODO and Status

This document tracks what has been ported from the original Castagne GDScript engine to Rust, and what remains to be done.

## ✅ Completed Components

### Core Infrastructure
- [x] **CastagneMemory** - Full implementation with all methods (src/memory.rs)
  - Global, Player, and Entity memory management
  - Variable get/set/has operations
  - Entity lifecycle (AddEntity, RemoveEntity, IsEIDValid)
  - CopyFrom for rollback support
  - Comprehensive unit tests

- [x] **CastagneStateHandle** - Context-aware memory access (src/state_handle.rs)
  - Entity/Player/Global get/set/has/add operations
  - Target entity management
  - Flag operations (entity_has_flag, entity_set_flag)
  - Phase tracking
  - Point-to operations for entity/player navigation

- [x] **CastagneConfig** - Configuration and module management (src/config.rs)
  - Config data storage and access
  - Module registration system
  - Module slot management

- [x] **CastagneEngineCore** - Main engine loop (src/engine.rs)
  - Initialization system
  - Frame execution
  - Phase execution (AI, Input, Init, Action, Subentity, Physics, Reaction, Freeze)
  - Module callback orchestration

- [x] **CastagneModule Trait** - Module system interface (src/module.rs)
  - All phase callbacks defined
  - Variable management hooks
  - State transition callbacks

- [x] **CoreModule** - Essential module (src/modules/core_module.rs)
  - Core entity variables (_Flags, _State, _EID, _Player, etc.)
  - Core global variables (_FrameID, _ActiveEntities, etc.)
  - Variable initialization and reset system
  - Flag processing (FlagsNext -> Flags)
  - Comprehensive tests

### Testing Infrastructure
- [x] **CastagneTestRunner** - Comparison testing (src/test_runner.rs)
  - Loads and compares GDScript vs Rust implementations
  - Memory operation tests (global, player, entity)
  - Performance benchmarking
  - Exposed to Godot via #[func] attributes

- [x] **Documentation**
  - GDSCRIPT_INTEROP.md - Guide for calling GDScript from Rust
  - TESTING.md - Comprehensive testing guide
  - README.md - Project overview

---

## 🚧 Partially Implemented

### CastagneEngine
- ✅ Basic phase system
- ✅ Module callback orchestration
- ❌ Script execution (ExecuteCurrentFighterScript)
- ❌ Entity initialization (DoEntityInit)
- ❌ Frozen/AI entity separation
- ❌ Halt frame handling
- ❌ Input handling integration

### CastagneStateHandle
- ✅ Basic memory access
- ❌ Instanced data access (IDGlobalGet, IDPlayerGet, IDEntityGet)
- ❌ Engine reference
- ❌ ConfigData/FighterScripts accessors

---

## 🚧 Partially Implemented

### Critical Components

#### 1. **CastagneParser** (src/parser.rs) - **v0 STUB IMPLEMENTED**
**Original File: 2279 lines** - Now has basic structure with TODOs

✅ **What's implemented:**
- Basic struct and data types (ParsedCharacter, ParsedVariable, ParsedState, etc.)
- File opening and metadata parsing structure
- Error handling and logging
- Main parsing flow skeleton

❌ **What's missing (TODOs):**
- Actual .casp file parsing logic
- Specblock parsing
- Variable definition parsing
- State parsing
- Function registration
- Instruction execution (I, F, S, L, V, P, R branches)
- Code optimization
- Consider using parser combinator library (nom, pest)

#### 2. **CastagneInput** (src/input.rs) - **MOSTLY IMPLEMENTED**
✅ **What's implemented:**
- Input device types and management
- Device addition/removal
- Physical input types (Raw, Button, Axis, Stick, Combination, Any)
- Game input types (Direct, Multiple, Derived)
- SOCD handling
- Input schema creation (basic)
- Raw input to game input conversion
- Device polling structure

❌ **What's missing (TODOs):**
- Actual Godot InputMap integration (needs proper bindings)
- Full input schema creation (simplified version implemented)
- Derived input handling (Press/Release events)
- Complete directional input processing

#### 3. **CastagneGlobal** (src/global.rs) - **CORE IMPLEMENTED**
✅ **What's implemented:**
- All enums (HitConfirmed, StateType, VariableType, etc.)
- Helper functions (has_flag, set_flag, get_int, get_bool, etc.)
- Data fusion utilities (fuse_data_overwrite, fuse_data_no_overwrite, etc.)
- String parsing (split_string_to_array)
- Battle init data helpers
- Version info structure
- Logging functions

❌ **What's missing (TODOs):**
- Module loading system (skeleton only)
- Config file parsing
- Character metadata loading

#### 4. **CastagneNet** (src/net.rs) - **STUB IMPLEMENTED**
**Note:** Original GDScript is marked "not maintained until v0.8 cycle"

✅ **What's implemented:**
- Network sync status tracking
- Basic structure and data types
- Logging system
- Callback method signatures

❌ **What's missing (TODOs):**
- Network peer creation (host/join)
- Actual synchronization logic
- Rollback implementation (state save/load)
- Input delay calculation
- Frame tracking
- All core netcode features

### Module System

#### Core Modules to Port (in order of importance)

1. **CMFlow** - Battle flow control
   - Battle initialization
   - Win/loss conditions
   - Round management

2. **CMPhysics2D** - 2D physics
   - Movement
   - Collision detection
   - Hit/hurtbox management

3. **CMAttacks** - Attack system
   - Hit detection
   - Damage calculation
   - Block/counter logic

4. **CMGraphics2D** - 2D rendering
   - Sprite management
   - Animation control
   - Visual effects

5. **CMInput** - Input processing
   - Command detection
   - Input buffer
   - Special move detection

6. **CMAudio** - Audio system
   - SFX playback
   - Music management

### Nice-to-Have Modules

- CMGraphics3D - 3D rendering
- CMAI - AI system
- CMMenus - Menu system
- CMEditor - Editor integration

---

## 🎯 Implementation Priority

### Phase 1: Make it Functional (Current)
- ✅ Core memory system
- ✅ Basic module system
- ✅ Core module with variables
- ✅ Testing infrastructure
- ❌ **Next: Start on minimal parser**

### Phase 2: Core Gameplay
- [ ] Basic parser (minimal .casp support)
- [ ] CMFlow module (battle flow)
- [ ] CMPhysics2D basics (movement, position)
- [ ] CMInput basics (button presses)
- [ ] Script execution in engine

### Phase 3: Fighting Game Features
- [ ] Full parser support
- [ ] CMAttacks (hit detection, damage)
- [ ] CMGraphics2D (visual display)
- [ ] State transitions
- [ ] Complete CMPhysics2D

### Phase 4: Advanced Features
- [ ] Rollback netcode (CastagneNet)
- [ ] Advanced input (motion detection)
- [ ] Audio system
- [ ] AI system

---

## 📝 Implementation Notes

### Challenges

1. **Parser Complexity**
   - The parser is massive and domain-specific
   - Needs careful incremental porting
   - Consider modern Rust parsing tools

2. **Godot Type System**
   - Variant types need careful handling
   - Not Send + Sync (no threading)
   - Tests need Godot runtime

3. **Reference Management**
   - Used Rc<RefCell<>> instead of Arc<RwLock<>>
   - Not thread-safe, but matches GDScript behavior
   - Simpler than dealing with Send + Sync

### Design Decisions

1. **Module System**
   - Kept trait-based approach
   - Removed Send + Sync requirement (Godot types aren't thread-safe anyway)
   - Used Rc<RefCell<>> for module storage

2. **Variable System**
   - CoreModule manages variable initialization and reset
   - Flags: ResetEachFrame, NoInit, InheritToSubentity
   - Matches GDScript behavior closely

3. **Testing Strategy**
   - Unit tests for pure Rust code
   - CastagneTestRunner for integration tests (runs in Godot)
   - Comparison tests validate parity with GDScript

---

## 🚀 Getting Started (For Contributors)

### To Add a New Module

1. Create file in `src/modules/your_module.rs`
2. Implement `CastagneModule` trait
3. Register variables in constructor
4. Implement phase callbacks as needed
5. Add to `src/modules/mod.rs`
6. Register in engine initialization

### To Port a GDScript Module

1. Read the GDScript module thoroughly
2. Identify all variables and their flags
3. Port RegisterFunction calls to Rust (may need parser work)
4. Implement phase callbacks
5. Add tests via CastagneTestRunner

### To Test Your Changes

```bash
# Build the library
cargo build

# Run Rust unit tests (will fail for Godot types)
cargo test --lib

# For real testing, use CastagneTestRunner from Godot:
# 1. Build the GDExtension
# 2. Load in Godot
# 3. Create a test scene
# 4. Call CastagneTestRunner.run_comparison_tests()
```

---

## 📊 Port Completion Estimate

- **Core Infrastructure**: 80% complete
- **Module System**: 30% complete
- **Parser**: 15% complete (v0 stub with TODOs) 📝
- **Physics**: 0% complete
- **Graphics**: 0% complete
- **Input**: 65% complete (core logic done, needs Godot bindings) ✅
- **Network**: 10% complete (stub only) 📝
- **Global Utilities**: 80% complete ✅

**Overall**: ~35% complete

---

## 🎓 Learning Resources

If you're continuing this port:

1. **Read the GDScript source** - `castagne/` directory
2. **Study CastagneModule.gd** - Understanding the module pattern
3. **Read existing ported code** - See patterns in memory.rs, state_handle.rs
4. **godot-rust docs** - https://godot-rust.github.io/

---

## 💡 Suggestions for Next Steps

### Option A: Parser (Hardest, Highest Impact)
If you want the engine to actually run character files:
1. Start with a minimal parser that handles basic state definitions
2. Add function calling (Map, Set, Add, etc.)
3. Incrementally add more functions
4. Test with simple character files

### Option B: Physics Module (Easier, Still Useful)
If you want to see entities move:
1. Port CMPhysics2D basics
2. Add position, velocity variables
3. Implement basic movement
4. Test with manually-created entities

### Option C: More Modules (Gradual Progress)
Continue porting modules one by one:
1. CMFlow - battle flow
2. CMInput - input handling
3. Each module adds more functionality

---

## 🐛 Known Issues

1. **Tests require Godot runtime**
   - Can't use PackedStringArray, Variant in unit tests
   - Use CastagneTestRunner for integration tests

2. **No parser yet**
   - Can't load character files
   - Can't execute scripts
   - Entities must be created manually in code

3. **No function registry**
   - Parser would populate this
   - Functions need manual Rust implementation

4. **Testing Infrastructure**
   - Can we download godot somehow and run these from the cli or something?

---

## ✨ What Works Right Now

You can:
- Create a CastagneEngine
- Initialize memory
- Add entities manually
- Set variables on entities
- Run the phase system
- Test parity with GDScript using CastagneTestRunner

You cannot yet:
- Load character files (.casp)
- Execute character scripts
- Play an actual fight

---

## 🎮 Godot 4 GDScript Port (castagne_godot4/)

### Completed ✅
- All Godot 3→4 API compatibility fixes
- All 14 E2E tests passing
- Core engine fully functional
- Parser working correctly
- Character loading working
- State transitions working
- All combat scenarios working

### Todo

#### ✅ 1. Fix shader warning (COMPLETED)
Fixed the shader parameter deprecation warning in CMGraphics-SpritesheetVisualizer:
- ✅ Updated `shader_param` to `shader_parameter` (Godot 4 naming)
- ✅ Converted .tscn file to format=3 (Godot 4 format)
- ✅ Updated Vector2 to Vector2i for shader uniforms
- ✅ No more deprecation warnings!

#### ✅ 2. Port missing module functions (COMPLETED)
Implemented the following optional module functions:
- ✅ `AIInputTransition` - Sets `_AIAttackCancelWhiff` for AI input transitions
- ✅ `ModelSwitchFacing` - Flips model facing by multiplying `_FacingHModel` by -1
- ✅ No more "Parse function or Action func couldn't be found" warnings

#### ✅ 3. Start porting the full Castagne Editor UI (COMPLETED)
Created a functional Godot 4 editor foundation:
- ✅ Converted main CastagneEditor.tscn to format=3 (Godot 4)
- ✅ Updated CastagneEditor.gd with Godot 4 API compatibility
- ✅ Created CastagneEditorConfig.gd stub with proper Godot 4 APIs
- ✅ Created CEDocumentation.gd stub with proper Godot 4 APIs
- ✅ Main menu structure with all buttons and navigation working
- ✅ Config editor panel (basic stub)
- ✅ Documentation viewer panel (basic stub)
- ✅ All E2E tests still passing
- ✅ No errors or warnings on editor load
- Note: Character editor and other advanced features are stubs that can be expanded later

#### ✅ 4. Look for other potential improvements (COMPLETED)
Fixed additional Godot 4 compatibility issues:
- ✅ Fixed string multiplication in test_full_match.gd (changed `"=" * 50` to `"=".repeat(50)`)
- ✅ Fixed `.instance()` to `.instantiate()` in FightingUI.gd
- ✅ All E2E tests now passing including test_full_match
- Note: CastagneNet.gd still uses deprecated `network_peer` API, but this is marked as "not maintained until v0.8" and not currently used in tests

#### ✅ 5. Convert remaining scene files to Godot 4 format (COMPLETED)
Converted all remaining .tscn files from format=2 (Godot 3) to format=3 (Godot 4):
- ✅ CastagneEngine.tscn - Main engine scene
- ✅ Graphics2DRoot.tscn - Graphics root with shader material
- ✅ CMEditor.tscn - Editor module scene
- ✅ FightingUI.tscn - Fighting UI module scene
- ✅ CMAttacks-TypesBigWindow.tscn - Attack types window with complex UI
- ✅ Updated all deprecated properties: `rect_min_size` → `custom_minimum_size`, `pressed` → `button_pressed`, `align` → `horizontal_alignment`
- ✅ Added proper Godot 4 layout modes and anchors presets
- ✅ All E2E tests still passing after conversion

---

**Last Updated**: $(date)
**Primary Author**: Claude (AI Assistant)
**Original Castagne Engine**: https://github.com/panthavma/castagne
