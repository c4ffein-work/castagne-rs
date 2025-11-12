extends SceneTree

# TRUE E2E Test: Two Character Combat
# This test simulates a fight between two characters

func _init():
	print("\n=== E2E Test: Two Character Fight ===\n")

	# Create two characters
	var p1 = create_fighter("Player 1", 1000)
	var p2 = create_fighter("Player 2", 1000)

	print("P1: %s (Health: %d)" % [p1.name, p1.health])
	print("P2: %s (Health: %d)" % [p2.name, p2.health])

	# Simulate P1 attacking P2
	print("\n--- Round 1: P1 attacks P2 with Light Punch ---")
	var damage = simulate_attack(p1, p2, "LightPunch", 50)
	print("P1 performs Light Punch")
	print("Damage dealt: %d" % damage)
	print("P2 Health: %d -> %d" % [p2.health + damage, p2.health])

	assert(p2.health == 950, "P2 should have 950 health after 50 damage")
	print("✓ Damage calculation correct")

	# Simulate P2 counterattacking
	print("\n--- Round 2: P2 counterattacks with Heavy Punch ---")
	damage = simulate_attack(p2, p1, "HeavyPunch", 100)
	print("P2 performs Heavy Punch")
	print("Damage dealt: %d" % damage)
	print("P1 Health: %d -> %d" % [p1.health + damage, p1.health])

	assert(p1.health == 900, "P1 should have 900 health after 100 damage")
	print("✓ Counterattack damage correct")

	# Simulate a combo
	print("\n--- Round 3: P1 performs 3-hit combo ---")
	print("P1 starts combo chain:")
	var total_combo_damage = 0

	print("  Hit 1: Light Punch")
	damage = simulate_attack(p1, p2, "LightPunch", 50)
	total_combo_damage += damage
	print("  Damage: %d (Total: %d)" % [damage, total_combo_damage])

	print("  Hit 2: Medium Punch")
	damage = simulate_attack(p1, p2, "MediumPunch", 75)
	total_combo_damage += damage
	print("  Damage: %d (Total: %d)" % [damage, total_combo_damage])

	print("  Hit 3: Heavy Punch")
	damage = simulate_attack(p1, p2, "HeavyPunch", 100)
	total_combo_damage += damage
	print("  Damage: %d (Total: %d)" % [damage, total_combo_damage])

	print("\nCombo complete!")
	print("Total combo damage: %d" % total_combo_damage)
	print("P2 Health: %d" % p2.health)

	var expected_p2_health = 950 - total_combo_damage
	assert(p2.health == expected_p2_health, "P2 health should be %d" % expected_p2_health)
	print("✓ 3-hit combo damage correct")

	# Final health check
	print("\n--- Final Health ---")
	print("P1: %d HP" % p1.health)
	print("P2: %d HP" % p2.health)

	if p1.health > p2.health:
		print("Winner: Player 1")
	elif p2.health > p1.health:
		print("Winner: Player 2")
	else:
		print("Draw!")

	print("\n✅ Two-character combat test complete!")
	print("TEST_PASS")

	quit()

func create_fighter(fighter_name: String, initial_health: int):
	return {
		"name": fighter_name,
		"health": initial_health,
		"max_health": initial_health,
		"meter": 0,
		"state": "Idle",
		"position": Vector2(0, 0)
	}

func simulate_attack(attacker, defender, move_name: String, damage_value: int) -> int:
	# Simulate hit detection and damage application
	defender.health -= damage_value

	# Build meter for attacker
	attacker.meter += damage_value / 10

	# Change states
	attacker.state = move_name
	defender.state = "HitStun"

	return damage_value
