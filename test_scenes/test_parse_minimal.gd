extends SceneTree

func _init():
	print("\n=== Testing Minimal Fighter Parsing ===\n")

func _process(_delta):
	var parser_script = load("res://castagne_godot4/engine/CastagneParser.gd")
	var config_script = load("res://castagne_godot4/engine/CastagneConfig.gd")

	var parser = parser_script.new()
	var config_data = config_script.new()

	var result = parser.CreateFullCharacter("res://test_characters/test_minimal_fighter.casp", config_data, true)

	if result:
		print("Parser returned result!")
		print("Keys: ", result.keys())
		print("\nCharacter: ", result.get("Character", {}))
		print("\nVariables keys: ", result.get("Variables", {}).keys() if result.has("Variables") else "NO VARIABLES")
		print("Variables: ", result.get("Variables", {}))
		print("\nStates keys: ", result.get("States", {}).keys() if result.has("States") else "NO STATES")

		if "_errors" in parser and parser._errors.size() > 0:
			print("\nParser errors:")
			for err in parser._errors:
				print("  ", err)
	else:
		print("Parser returned null!")
		if "_errors" in parser:
			print("Errors:")
			for err in parser._errors:
				print("  ", err)

	quit()
