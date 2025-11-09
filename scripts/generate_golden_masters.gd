extends SceneTree

# Generate golden master JSON files by parsing .casp files with the original Castagne parser
# Usage: godot --script scripts/generate_golden_masters.gd

func _init():
	print("===========================================")
	print("  Generating Golden Master Files")
	print("===========================================")
	print("")

	# Load the parser
	var parser_script = load("res://castagne/engine/CastagneParser.gd")
	if !parser_script:
		print("ERROR: Could not load Castagne parser!")
		quit(1)
		return

	# Create a minimal config (the parser needs this)
	var config_data = Node.new()
	config_data.set_script(load("res://castagne/engine/CastagneConfig.gd"))

	# Files to parse - using actual Castagne example files
	var test_files = [
		"castagne/examples/fighters/baston/Baston-Model.casp"
	]

	var success_count = 0
	var fail_count = 0

	for filename in test_files:
		print("Processing: %s" % filename)

		var parser = parser_script.new()
		var result = parser.CreateFullCharacter(filename, config_data, true)

		if result == null:
			print("  ✗ FAILED to parse %s" % filename)
			var errors = parser._errors if "_errors" in parser else []
			for err in errors:
				print("    Error: %s" % err)
			fail_count += 1
			continue

		# Convert to a serializable dictionary
		var output = serialize_character(result)

		# Save as JSON
		var json_filename = "golden_masters/" + filename.replace(".casp", ".json")
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
		quit(0)
	else:
		print("  ✗ Some files failed to generate")
		quit(1)

func serialize_character(character):
	# Convert the ParsedCharacter object to a plain dictionary
	var output = {}

	# Metadata
	output["metadata"] = {}
	if "Character" in character and character["Character"]:
		var meta = character["Character"]
		output["metadata"]["name"] = meta.get("name", "")
		output["metadata"]["author"] = meta.get("author", "")
		output["metadata"]["description"] = meta.get("description", "")
		if "skeleton" in meta:
			output["metadata"]["skeleton"] = meta["skeleton"]

	# Variables
	output["variables"] = {}
	if "variables" in character:
		for var_name in character["variables"]:
			var var_data = character["variables"][var_name]
			output["variables"][var_name] = {
				"value": var_data.get("value", ""),
				"type": var_data.get("type", ""),
				"mutability": var_data.get("mutability", "")
			}

	# States
	output["states"] = {}
	if "states" in character:
		for state_name in character["states"]:
			var state = character["states"][state_name]
			output["states"][state_name] = {
				"parent": state.get("parent", null),
				"type": state.get("type", "Normal"),
				"actions": {}
			}
			if "actions" in state:
				for phase in state["actions"]:
					var actions = state["actions"][phase]
					output["states"][state_name]["actions"][phase] = actions.size()

	# Specblocks
	output["specblocks"] = {}
	if "specblocks" in character:
		for block_name in character["specblocks"]:
			output["specblocks"][block_name] = character["specblocks"][block_name].duplicate()

	return output
