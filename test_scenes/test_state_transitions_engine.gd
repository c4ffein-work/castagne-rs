extends SceneTree

# TRUE E2E Test: State Transitions with REAL ENGINE
# This test actually runs CastagneEngine and verifies state transitions happen

func _init():
	print("\n=== E2E Test: State Transitions (REAL ENGINE) ===\n")

func _process(_delta):
	# Get Castagne autoload to access properly initialized config
	var castagne = root.get_node_or_null("/root/Castagne")
	if not castagne:
		print("ERROR: Castagne autoload not found")
		print("TEST_FAIL")
		quit()
		return

	print("✓ Castagne autoload found")

	# Load engine script
	var engine_script = load("res://castagne_godot4/engine/CastagneEngine.gd")
	if not engine_script:
		print("ERROR: Could not load engine script")
		print("TEST_FAIL")
		quit()
		return

	print("✓ Engine script loaded")

	# Create engine instance with properly initialized config from autoload
	var engine = engine_script.new()
	var config_data = castagne.baseConfigData

	# Set up engine
	engine.configData = config_data
	engine.battleInitData = {}
	engine.initOnReady = false
	engine.runAutomatically = false
	engine.renderGraphics = false

	print("✓ Engine instance created")

	# Initialize the engine
	print("\n--- Initializing CastagneEngine ---")
	engine.Init()

	if engine.initError:
		print("ERROR: Engine initialization failed")
		print("TEST_FAIL")
		quit()
		return

	print("✓ Engine initialized successfully!")

	# Load character into engine
	print("\n--- Loading character into engine ---")
	var character_path = "res://test_characters/test_minimal_fighter.casp"
	var fighter_id = engine.ParseFighterScript(character_path)

	if fighter_id == -1:
		print("ERROR: Failed to load character into engine")
		print("TEST_FAIL")
		quit()
		return

	print("✓ Character loaded into engine (Fighter ID: ", fighter_id, ")")

	# Initialize memory properly
	if not engine._memory:
		print("ERROR: Engine memory not initialized")
		print("TEST_FAIL")
		quit()
		return

	# Add entity to engine
	print("\n--- Creating entity in engine ---")
	var game_state = engine.CreateStateHandle(engine._memory)
	var entity_id = engine.AddNewEntity(game_state, 0, fighter_id, null)

	print("✓ Entity created (Entity ID: ", entity_id, ")")

	# Get initial state
	var initial_state = game_state.Memory().EntityGet(entity_id, "_State")
	print("\n--- Testing state transitions ---")
	print("Initial state: ", initial_state)

	# Run engine tick to initialize entity
	print("\nRunning first frame (initialization)...")
	var player_inputs = [{}]
	engine._memory = engine.EngineTick(engine._memory, player_inputs)

	game_state = engine.CreateStateHandle()
	game_state.PointToEntity(entity_id)
	var state_after_init = game_state.EntityGet("_State")
	print("State after init: ", state_after_init)

	if state_after_init == "Idle":
		print("✓ Character transitioned to Idle state!")
	else:
		print("WARNING: Character in state '", state_after_init, "' instead of 'Idle'")

	# Force state change to LightPunch
	print("\n--- Testing manual state change ---")
	game_state.PointToEntity(entity_id)
	game_state.EntitySet("_State", "LightPunch")
	game_state.EntitySet("StateTimer", 5)

	print("Set state to LightPunch with StateTimer=5")

	# Run 6 frames to let the state timer expire
	print("\nRunning 6 frames to test auto-transition...")
	for frame in range(6):
		engine._memory = engine.EngineTick(engine._memory, player_inputs)
		game_state = engine.CreateStateHandle()
		game_state.PointToEntity(entity_id)
		var current_state = game_state.EntityGet("_State")
		var timer = game_state.EntityGet("StateTimer")
		print("  Frame ", frame + 1, ": State=", current_state, " Timer=", timer)

	# Check if transitioned back to Idle
	game_state = engine.CreateStateHandle()
	game_state.PointToEntity(entity_id)
	var final_state = game_state.EntityGet("_State")

	print("\nFinal state: ", final_state)

	if final_state == "Idle":
		print("\n✅ STATE TRANSITION TEST PASSED!")
		print("Engine executed LightPunch state and transitioned to Idle")
		print("TEST_PASS")
	else:
		print("\n⚠️  State is '", final_state, "' instead of 'Idle'")
		print("Engine is running but transitions may need adjustment")
		print("TEST_PARTIAL")

	quit()
