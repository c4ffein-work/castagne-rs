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

## ❌ Not Yet Ported

### Critical Components

#### 1. **CastagneParser** (castagne/engine/CastagneParser.gd) - **HIGHEST PRIORITY**
**File Size: 75KB** - This is the BIGGEST component!

The parser is responsible for:
- Loading and parsing .casp character files
- Function registration and lookup
- Script compilation
- State management
- Variable definition parsing
- Documentation generation

**Why it's hard:**
- Very large, complex file (2000+ lines)
- Domain-specific language parser
- Intricate state machine
- Many edge cases

**Suggested approach:**
- Start with a minimal parser that handles basic states and functions
- Add function registration incrementally
- Leave complex features (documentation, editor features) for later
- Consider using a parser combinator library (nom, pest)

#### 2. **CastagneInput** (castagne/engine/CastagneInput.gd)
Input management system:
- Device polling
- Input buffering
- Motion/button detection
- Input history for rollback

#### 3. **CastagneGlobal** (castagne/engine/CastagneGlobal.gd)
Global utilities:
- Error logging
- Data fusion utilities
- String parsing helpers
- Constants and enums

#### 4. **CastagneNet** (castagne/engine/CastagneNet.gd)
Rollback netcode:
- State saving/loading
- Input delay
- Rollback logic
- Network synchronization

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
- **Parser**: 0% complete ⚠️
- **Physics**: 0% complete
- **Graphics**: 0% complete
- **Input**: 0% complete
- **Network**: 0% complete

**Overall**: ~25% complete

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

**Last Updated**: $(date)
**Primary Author**: Claude (AI Assistant)
**Original Castagne Engine**: https://github.com/panthavma/castagne
