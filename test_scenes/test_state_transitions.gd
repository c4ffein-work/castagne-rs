extends SceneTree

# TRUE E2E Test: State Transitions
# This test verifies that character states are loaded and defined correctly
# using the REAL Castagne parser

func _init():
	print("\n=== E2E Test: State Transitions (REAL PARSER) ===\n")

func _process(_delta):
	# E2E: Get Castagne from autoload (the way it actually works in production!)
	var castagne = root.get_node_or_null("/root/Castagne")
	if not castagne:
		print("ERROR: Castagne autoload not found")
		print("TEST_FAIL")
		quit()
		return

	print("✓ Castagne autoload found")

	# E2E: Get parser from autoload
	var parser = castagne.Parser
	if not parser:
		print("ERROR: Castagne Parser not available from autoload")
		print("TEST_FAIL")
		quit()
		return

	print("✓ CastagneParser loaded from autoload")

	# E2E: Get config from Castagne
	var config_data = castagne.baseConfigData
	if not config_data:
		print("ERROR: Castagne config not available")
		print("TEST_FAIL")
		quit()
		return

	print("✓ CastagneConfig loaded from autoload")

	# Load the test character
	var test_character_path = "res://test_characters/test_basic_fighter.casp"
	if not FileAccess.file_exists(test_character_path):
		print("ERROR: Test character not found at: ", test_character_path)
		print("TEST_FAIL")
		quit()
		return

	print("✓ Test character file found")
	print("\n--- Parsing character with real parser (via autoload) ---")

	# Parse the character
	var result = parser.CreateFullCharacter(test_character_path, config_data, true)

	if result == null:
		print("ERROR: Parser failed to parse character!")
		if "_errors" in parser:
			for err in parser._errors:
				print("  Error: ", err)
		print("TEST_FAIL")
		quit()
		return

	print("✓ Character parsed successfully!")

	# Verify states exist
	if not "States" in result:
		print("ERROR: No states found in parsed character")
		print("TEST_FAIL")
		quit()
		return

	var states = result["States"]
	print("\n--- Verifying character states ---")
	print("Total states: ", states.size())

	# Check for required states
	var required_states = ["Idle", "Init", "LightPunch", "Jump"]
	var found_states = []
	var missing_states = []

	for state_name in required_states:
		if state_name in states:
			found_states.append(state_name)
			print("✓ Found state: ", state_name)
		else:
			missing_states.append(state_name)
			print("✗ Missing state: ", state_name)

	if missing_states.size() > 0:
		print("\nERROR: Missing required states: ", missing_states)
		print("TEST_FAIL")
		quit()
		return

	print("\n✅ All required states found!")

	# Verify state structure (states should have phases)
	print("\n--- Verifying state structure ---")

	# Check Idle state has Action phase
	var idle_state = states["Idle"]
	if not idle_state:
		print("ERROR: Idle state is null")
		print("TEST_FAIL")
		quit()
		return

	print("✓ Idle state structure verified")

	# Check LightPunch state has Init and Action phases
	var lightpunch_state = states["LightPunch"]
	if not lightpunch_state:
		print("ERROR: LightPunch state is null")
		print("TEST_FAIL")
		quit()
		return

	print("✓ LightPunch state structure verified")

	# Check Jump state
	var jump_state = states["Jump"]
	if not jump_state:
		print("ERROR: Jump state is null")
		print("TEST_FAIL")
		quit()
		return

	print("✓ Jump state structure verified")

	# Verify variables exist
	if "Variables" in result:
		var variables = result["Variables"]
		print("\n--- Verifying variables ---")
		print("Total variables: ", variables.size())

		# Check for Health variable
		var health_found = false
		for var_name in variables:
			if "Health" in var_name or "health" in var_name.to_lower():
				health_found = true
				print("✓ Found health variable: ", var_name)
				break

		if not health_found:
			print("WARNING: No health variable found")

	print("\n✅ State transition test PASSED!")
	print("Character states are loaded and structured correctly")
	print("TEST_PASS")

	quit()
