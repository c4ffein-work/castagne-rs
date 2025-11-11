extends SceneTree

func _init():
	print("Testing Castagne GDScript syntax...")

	# Try to access the Castagne autoload
	if Castagne:
		print("✅ Castagne autoload accessible")
		print("Version info: ", Castagne.versionInfo)
	else:
		print("❌ Castagne autoload not accessible")

	quit()
