extends Node

# Generate golden master JSON files by parsing .casp files
# Runs as a proper scene so Castagne autoload is available

func _ready():
	print("===========================================")
	print("  Generating Golden Master Files")
	print("===========================================")
	print("")

	# Wait for Castagne autoload to initialize
	yield(get_tree(), "idle_frame")

	# Use the global Castagne config with all modules
	var config_data = Castagne.baseConfigData
	if !config_data:
		print("ERROR: Castagne baseConfigData not available!")
		get_tree().quit(1)
		return

	# Load the parser script
	var parser_script = load("res://castagne/engine/CastagneParser.gd")
	if !parser_script:
		print("ERROR: Could not load Castagne parser!")
		get_tree().quit(1)
		return

	# Files to parse
	var test_files = [
		"castagne/examples/fighters/baston/Baston-Model.casp",
		"castagne/examples/fighters/baston/Baston-2D.casp",
		"castagne/editor/tutorials/assets/TutorialBaston.casp"
	]

	var success_count = 0
	var fail_count = 0

	for filename in test_files:
		print("Processing: %s" % filename)

		# Create parser and add to tree (needed for onready variables)
		var parser = parser_script.new()
		add_child(parser)

		var result = parser.CreateFullCharacter(filename, config_data, true)

		# Clean up
		parser.queue_free()

		if result == null:
			print("  ✗ FAILED to parse %s" % filename)
			var errors = parser._errors if "_errors" in parser else []
			for err in errors:
				print("    Error: %s" % err)
			fail_count += 1
			continue

		# Convert to JSON
		var output = serialize_character(result)

		# Save file
		var json_filename = "golden_masters/" + filename.replace(".casp", ".json").get_file()
		var file = File.new()
		var dir = Directory.new()
		if !dir.dir_exists("golden_masters"):
			dir.make_dir("golden_masters")

		if file.open(json_filename, File.WRITE) != OK:
			print("  ✗ FAILED to write %s" % json_filename)
			fail_count += 1
			continue

		file.store_string(JSON.print(output, "  "))
		file.close()

		print("  ✓ Generated: %s" % json_filename)
		success_count += 1

	print("")
	print("===========================================")
	print("  Summary")
	print("===========================================")
	print("  Success: %d" % success_count)
	print("  Failed:  %d" % fail_count)
	print("")

	if fail_count == 0:
		print("  ✓ All golden masters generated successfully!")
		get_tree().quit(0)
	else:
		print("  ✗ Some files failed to generate")
		get_tree().quit(1)

func serialize_character(character):
	var output = {}

	# Metadata
	output["metadata"] = {}
	if "Character" in character and character["Character"]:
		for key in character["Character"]:
			output["metadata"][key.to_lower()] = character["Character"][key]

	# Subentities
	output["subentities"] = {}
	if "Subentities" in character:
		for entity_name in character["Subentities"]:
			output["subentities"][entity_name] = {}
			for key in character["Subentities"][entity_name]:
				output["subentities"][entity_name][key.to_lower()] = character["Subentities"][entity_name][key]

	# Variables
	output["variables"] = {}
	if "Variables" in character:
		for var_name in character["Variables"]:
			var var_data = character["Variables"][var_name]
			output["variables"][var_name] = {
				"Name": var_data.get("Name", var_name),
				"Value": str(var_data.get("Value", "")),
				"Type": var_data.get("Type", ""),
				"Subtype": var_data.get("Subtype", ""),
				"Mutability": var_data.get("Mutability", "")
			}

	# States
	output["states"] = {}
	if "States" in character:
		for state_name in character["States"]:
			var state = character["States"][state_name]
			output["states"][state_name] = {
				"Parent": state.get("Parent", null),
				"Type": state.get("Type", "Normal"),
				"TransitionFlags": state.get("TransitionFlags", []),
				"Phases": {}
			}
			if "Phases" in state:
				for phase_name in state["Phases"]:
					var instructions = state["Phases"][phase_name]
					output["states"][state_name]["Phases"][phase_name] = {
						"instruction_count": instructions.size(),
						"instructions": instructions
					}

	# TransformedData
	output["transformed_data"] = {}
	if "TransformedData" in character:
		output["transformed_data"] = character["TransformedData"].duplicate(true)

	return output
