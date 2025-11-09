extends SceneTree

func _init():
	print("Testing parser load...")
	var parser_script = load("res://castagne_godot4/engine/CastagneParser.gd")
	if parser_script:
		print("✓ Parser script loaded successfully!")
		var parser = parser_script.new()
		print("✓ Parser instantiated successfully!")
	else:
		print("✗ Failed to load parser script")
	quit()
