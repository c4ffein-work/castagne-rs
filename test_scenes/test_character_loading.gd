extends SceneTree

# TRUE E2E Test: Character Loading
# This test loads a character file using the REAL Castagne parser
# and verifies the parsed structure is correct

func _init():
	print("\n=== E2E Test: Character Loading (REAL PARSER) ===\n")

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

	# Get config from Castagne
	var config_data = castagne.baseConfigData
	if not config_data:
		print("ERROR: Castagne config not available")
		print("TEST_FAIL")
		quit()
		return

	print("✓ CastagneConfig loaded from autoload")

	# Test with the minimal test character (no skeleton)
	var test_character_path = "res://test_character_minimal.casp"
	if not FileAccess.file_exists(test_character_path):
		print("ERROR: Test character not found at: ", test_character_path)
		print("TEST_FAIL")
		quit()
		return

	print("✓ Test character file found")
	print("\n--- Parsing character file with REAL parser (via autoload) ---")
	print("File: ", test_character_path)

	# ACTUALLY PARSE THE CHARACTER (not just read the file!)
	var result = parser.CreateFullCharacter(test_character_path, config_data, true)

	if result == null:
		print("ERROR: Parser failed to parse character!")
		print("Parser errors:")
		if "_errors" in parser:
			for err in parser._errors:
				print("  - ", err)
		print("TEST_FAIL")
		quit()
		return

	print("✓ Character parsed successfully!")

	# Verify the parsed structure contains expected data
	print("\n--- Verifying parsed character structure ---")

	if not "Character" in result:
		print("ERROR: Missing 'Character' metadata in result")
		print("TEST_FAIL")
		quit()
		return

	print("✓ Character metadata found")
	var metadata = result["Character"]
	if "Name" in metadata:
		print("  Character Name: ", metadata["Name"])
	if "Author" in metadata:
		print("  Author: ", metadata["Author"])

	if not "Variables" in result:
		print("ERROR: Missing 'Variables' in result")
		print("TEST_FAIL")
		quit()
		return

	print("✓ Variables found (", result["Variables"].size(), " variable(s))")
	# List some variables
	var var_count = 0
	for var_name in result["Variables"]:
		if var_count < 3:  # Show first 3 variables
			print("  - ", var_name)
		var_count += 1

	if not "States" in result:
		print("ERROR: Missing 'States' in result")
		print("TEST_FAIL")
		quit()
		return

	print("✓ States found (", result["States"].size(), " state(s))")
	# List states
	var state_count = 0
	for state_name in result["States"]:
		if state_count < 3:  # Show first 3 states
			print("  - ", state_name)
		state_count += 1

	# Verify the Idle state exists (should be in every character)
	if "Idle" in result["States"]:
		print("✓ Found required 'Idle' state")
		var idle_state = result["States"]["Idle"]
		if "Init" in idle_state or "Action" in idle_state:
			print("✓ Idle state has phases (Init/Action)")
	else:
		print("WARNING: No 'Idle' state found")

	print("\n✅ Character parsing test PASSED!")
	print("The parser successfully loaded and parsed a .casp file")
	print("TEST_PASS")

	quit()
