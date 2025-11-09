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

	# Files to parse - start with our simpler test files
	# Note: Baston example file requires full Castagne module system, so we skip it for now
	var test_files = [
		"test_character.casp",
		"test_character_advanced.casp",
		"test_character_complete.casp"
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

		# Save as JSON - create nested directory structure if needed
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
		quit(0)
	else:
		print("  ✗ Some files failed to generate")
		quit(1)

func serialize_character(character):
	# Convert the ParsedCharacter object to a plain dictionary
	# Parser returns: Character, Subentities, Variables, States, TransformedData
	var output = {}

	# Metadata
	output["metadata"] = {}
	if "Character" in character and character["Character"]:
		var meta = character["Character"]
		# The metadata is a dictionary with keys like Name, Author, Description, Skeleton
		for key in meta:
			output["metadata"][key.to_lower()] = meta[key]

	# Subentities
	output["subentities"] = {}
	if "Subentities" in character:
		for entity_name in character["Subentities"]:
			var entity = character["Subentities"][entity_name]
			output["subentities"][entity_name] = {}
			for key in entity:
				output["subentities"][entity_name][key.to_lower()] = entity[key]

	# Variables (note: capital V in parser output!)
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

	# States (note: capital S in parser output!)
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
			# Each phase contains an array of instructions
			if "Phases" in state:
				for phase_name in state["Phases"]:
					var instructions = state["Phases"][phase_name]
					output["states"][state_name]["Phases"][phase_name] = {
						"instruction_count": instructions.size(),
						"instructions": instructions  # Full instruction data for comparison
					}

	# TransformedData (this contains processed specblocks and other data)
	output["transformed_data"] = {}
	if "TransformedData" in character:
		output["transformed_data"] = character["TransformedData"].duplicate(true)

	return output
