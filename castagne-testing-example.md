# Testing Castagne with GUT

## Setup

1. Install GUT from the Godot Asset Library
2. Create a `tests/` directory in your project
3. Create test scripts that extend `GutTest`

## Example Test Structure

```
castagne/
├── engine/
│   ├── CastagneMemory.gd
│   ├── CastagneEngine.gd
│   └── ...
└── tests/
    ├── test_castagne_memory.gd
    ├── test_castagne_state_handle.gd
    ├── test_castagne_engine.gd
    └── test_modules.gd
```

## Example Tests

### test_castagne_memory.gd
```gdscript
extends GutTest

var memory

func before_each():
    memory = load("res://engine/CastagneMemory.gd").new()
    memory.InitMemory()

func after_each():
    memory.free()

func test_global_memory_operations():
    # Test setting a new value
    memory.GlobalSet("TestVar", 42, true)
    assert_eq(memory.GlobalGet("TestVar"), 42, "Should store integer")

    # Test has check
    assert_true(memory.GlobalHas("TestVar"), "Should have TestVar")
    assert_false(memory.GlobalHas("NonExistent"), "Should not have NonExistent")

func test_player_memory():
    memory.AddPlayer()
    memory.PlayerSet(0, "HP", 100, true)
    assert_eq(memory.PlayerGet(0, "HP"), 100, "Player HP should be 100")

func test_entity_lifecycle():
    var eid = memory.AddEntity()
    assert_true(memory.IsEIDValid(eid), "Entity should be valid")

    memory.EntitySet(eid, "Position", Vector2(10, 20), true)
    assert_eq(memory.EntityGet(eid, "Position"), Vector2(10, 20))

    memory.RemoveEntity(eid)
    assert_false(memory.IsEIDValid(eid), "Entity should be invalid after removal")

func test_entity_flags():
    var eid = memory.AddEntity()
    memory.EntitySet(eid, "_Flags", [], true)

    # This would require StateHandle, showing integration testing
    pass
```

### test_castagne_state_handle.gd
```gdscript
extends GutTest

var memory
var state_handle
var engine

func before_each():
    engine = load("res://engine/CastagneEngine.gd").new()
    memory = engine.CreateMemory()
    state_handle = engine.CreateStateHandle(memory)

func after_each():
    engine.free()

func test_entity_context():
    var eid = memory.AddEntity()
    memory.EntitySet(eid, "_Player", 0, true)

    assert_true(state_handle.PointToEntity(eid), "Should point to entity")
    assert_eq(state_handle.GetEntityID(), eid, "Should return correct entity ID")

func test_phase_tracking():
    state_handle.SetPhase("Action")
    assert_eq(state_handle.GetPhase(), "Action", "Should track phase")

func test_memory_convenience_methods():
    memory.GlobalSet("GameSpeed", 60, true)
    assert_eq(state_handle.GlobalGet("GameSpeed"), 60, "Should access global memory")
```

### test_castagne_engine.gd
```gdscript
extends GutTest

var engine
var config

func before_each():
    engine = load("res://engine/CastagneEngine.gd").new()
    config = load("res://engine/CastagneConfig.gd").new()
    engine.configData = config

func after_each():
    engine.free()

func test_engine_initialization():
    engine.battleInitData = {}
    engine.Init()
    assert_false(engine.initError, "Engine should initialize without errors")

func test_frame_execution():
    engine.Init()
    var initial_frame = engine._memory.GlobalGet("_FrameID") if engine._memory.GlobalHas("_FrameID") else 0

    # Execute one frame
    var result = engine.ExecuteFrame()
    assert_not_null(result, "Frame should execute")
```

### test_modules.gd
```gdscript
extends GutTest

var module
var state_handle

func before_each():
    module = load("res://modules/general/CMFunctions.gd").new()
    var engine = load("res://engine/CastagneEngine.gd").new()
    var memory = engine.CreateMemory()
    state_handle = engine.CreateStateHandle(memory)

func test_module_registration():
    module.ModuleSetup()
    assert_eq(module.moduleName, "Functions", "Module name should be Functions")

func test_abs_function():
    # Setup entity with variable
    var eid = state_handle.Memory().AddEntity()
    state_handle.PointToEntity(eid)
    state_handle.EntitySet("TestVar", -42, true)

    # Call Abs function
    module.Abs(["TestVar"], state_handle)

    assert_eq(state_handle.EntityGet("TestVar"), 42, "Should compute absolute value")
```

## Running Tests

### In Godot Editor
1. Go to the GUT panel (Bottom panel)
2. Click "Run All Tests"
3. View results in the panel

### Command Line (CI/CD)
```bash
# Using GUT's command line runner
godot --headless -s addons/gut/gut_cmdln.gd -gtest=tests/
```

## Comparing GDScript vs Rust Implementation

You could create parallel tests:

```gdscript
# test_rust_vs_gdscript.gd
extends GutTest

func test_memory_parity():
    # Test GDScript version
    var gd_memory = load("res://engine/CastagneMemory.gd").new()
    gd_memory.GlobalSet("Test", 42, true)
    var gd_result = gd_memory.GlobalGet("Test")

    # Test Rust version (if exposed via GDExtension)
    var rust_engine = CastagneEngine.new()  # Your Rust wrapper
    # Compare behaviors...
```

## Benefits for Your Port

1. **Verify original behavior** - Understand exactly how GDScript version works
2. **Regression testing** - Ensure your Rust port matches GDScript behavior
3. **Documentation** - Tests serve as executable documentation
4. **Refactoring safety** - Catch breaking changes

## Recommendation

I'd suggest **GUT** for your use case because:
- ✅ Simpler to set up
- ✅ Better for comparing implementations
- ✅ Easier to run in CI/CD
- ✅ More straightforward syntax for this type of testing

Would you like me to set up a basic test suite for the original Castagne GDScript code?
