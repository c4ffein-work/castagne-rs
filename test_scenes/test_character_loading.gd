extends SceneTree

# TRUE E2E Test: Character Loading
# This test loads a character file using the REAL Castagne parser
# and verifies the parsed structure is correct

func _init():
	print("\n=== E2E Test: Character Loading (REAL PARSER) ===\n")

func _process(_delta):
	# Load the Castagne parser script
	var parser_script = load("res://castagne_godot4/engine/CastagneParser.gd")
	if not parser_script:
		print("ERROR: Could not load CastagneParser.gd")
		print("TEST_FAIL")
		quit()
		return

	print("✓ CastagneParser script loaded")

	# Load the config script (parser needs this for modules)
	var config_script = load("res://castagne_godot4/engine/CastagneConfig.gd")
	if not config_script:
		print("ERROR: Could not load CastagneConfig.gd")
		print("TEST_FAIL")
		quit()
		return

	print("✓ CastagneConfig script loaded")

	# Create parser and config instances
	var parser = parser_script.new()
	var config_data = config_script.new()

	print("✓ Parser and Config instances created")

	# Test with the simple test character
	var test_character_path = "res://test_character.casp"
	if not FileAccess.file_exists(test_character_path):
		print("ERROR: Test character not found at: ", test_character_path)
		print("TEST_FAIL")
		quit()
		return

	print("\n--- Parsing character file with REAL parser ---")
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
