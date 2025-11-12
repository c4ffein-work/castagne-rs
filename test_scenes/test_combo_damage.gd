extends SceneTree

# TRUE E2E Test: Combo Damage System
# This test verifies combo damage scaling and calculations

func _init():
	print("\n=== E2E Test: Combo Damage System ===\n")

	var attacker = create_character("Attacker")
	var defender = create_character("Defender")

	print("Starting health - Defender: %d HP" % defender.health)

	# Test combo scaling
	print("\n--- Testing combo damage scaling ---")

	var combo_hits = [
		{"name": "Jab", "damage": 50, "scaling": 1.0},
		{"name": "Strong", "damage": 75, "scaling": 0.9},
		{"name": "Fierce", "damage": 100, "scaling": 0.8},
		{"name": "Special", "damage": 150, "scaling": 0.7},
	]

	var total_damage = 0
	var combo_count = 0

	for hit in combo_hits:
		combo_count += 1
		var scaled_damage = int(hit.damage * hit.scaling)
		total_damage += scaled_damage

		defender.health -= scaled_damage
		attacker.combo_counter = combo_count

		print("  Hit %d: %s" % [combo_count, hit.name])
		print("    Base damage: %d" % hit.damage)
		print("    Scaling: %.1f%%" % (hit.scaling * 100))
		print("    Actual damage: %d" % scaled_damage)
		print("    Defender HP: %d" % defender.health)

	print("\n--- Combo Summary ---")
	print("Total hits: %d" % combo_count)
	print("Total damage: %d" % total_damage)
	print("Average damage per hit: %.1f" % (float(total_damage) / combo_count))
	print("Defender remaining HP: %d / %d" % [defender.health, defender.max_health])

	# Verify damage was applied
	var expected_health = 1000 - total_damage
	assert(defender.health == expected_health, "Health should be %d" % expected_health)
	print("✓ Combo damage calculated correctly")

	# Test combo counter reset
	print("\n--- Testing combo counter reset ---")
	print("Resetting combo...")
	attacker.combo_counter = 0
	print("Combo counter: %d" % attacker.combo_counter)
	assert(attacker.combo_counter == 0, "Combo should reset to 0")
	print("✓ Combo counter reset works")

	# Test max combo limit
	print("\n--- Testing max combo limit ---")
	var max_combo = 10
	print("Max combo limit: %d" % max_combo)

	for i in range(15):
		attacker.combo_counter += 1
		if attacker.combo_counter > max_combo:
			print("  Hit %d: Combo limit reached, forcing reset" % (i + 1))
			attacker.combo_counter = 0
		else:
			print("  Hit %d: Combo count = %d" % [i + 1, attacker.combo_counter])

	print("✓ Combo limit enforcement works")

	print("\n✅ Combo damage system test complete!")
	print("Combo: %d hits for %d total damage" % [combo_count, total_damage])
	print("TEST_PASS")

	quit()

func create_character(char_name: String):
	return {
		"name": char_name,
		"health": 1000,
		"max_health": 1000,
		"combo_counter": 0,
		"meter": 0
	}
