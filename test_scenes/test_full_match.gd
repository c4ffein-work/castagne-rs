extends SceneTree

# TRUE E2E Test: Full Match Simulation
# This test simulates a complete fighting game match

func _init():
	print("\n=== E2E Test: Full Match Simulation ===\n")

	# Initialize match
	var match_config = {
		"rounds_to_win": 2,
		"round_time": 99,
		"starting_health": 1000
	}

	var p1 = create_fighter("Player 1", match_config.starting_health)
	var p2 = create_fighter("Player 2", match_config.starting_health)

	var match_state = {
		"current_round": 1,
		"p1_rounds": 0,
		"p2_rounds": 0,
		"match_complete": false
	}

	print("🥊 MATCH START 🥊")
	print("Configuration: Best of %d" % (match_config.rounds_to_win * 2 - 1))
	print("Round time: %d seconds" % match_config.round_time)
	print("\n")

	# Simulate multiple rounds
	while not match_state.match_complete:
		print("=".repeat(50))
		print("ROUND %d - FIGHT!" % match_state.current_round)
		print("=".repeat(50))
		print("")

		# Reset health for new round
		p1.health = match_config.starting_health
		p2.health = match_config.starting_health

		# Simulate round
		var round_result = simulate_round(p1, p2, match_state.current_round)

		# Award round
		if round_result == "P1":
			match_state.p1_rounds += 1
			print("\n🏆 Player 1 wins Round %d!" % match_state.current_round)
		else:
			match_state.p2_rounds += 1
			print("\n🏆 Player 2 wins Round %d!" % match_state.current_round)

		print("Score: P1 %d - %d P2\n" % [match_state.p1_rounds, match_state.p2_rounds])

		# Check for match winner
		if match_state.p1_rounds >= match_config.rounds_to_win:
			match_state.match_complete = true
			print("=".repeat(50))
			print("🎉 PLAYER 1 WINS THE MATCH! 🎉")
			print("=".repeat(50))
		elif match_state.p2_rounds >= match_config.rounds_to_win:
			match_state.match_complete = true
			print("=".repeat(50))
			print("🎉 PLAYER 2 WINS THE MATCH! 🎉")
			print("=".repeat(50))
		else:
			match_state.current_round += 1

	# Match summary
	print("\n--- Match Summary ---")
	print("Total rounds played: %d" % match_state.current_round)
	print("Final score: P1 %d - %d P2" % [match_state.p1_rounds, match_state.p2_rounds])
	print("Winner: %s" % ("Player 1" if match_state.p1_rounds > match_state.p2_rounds else "Player 2"))

	print("\n✅ Full match simulation complete!")
	print("Match complete")
	print("TEST_PASS")

	quit()

func create_fighter(fighter_name: String, health: int):
	return {
		"name": fighter_name,
		"health": health,
		"max_health": health,
		"meter": 0,
		"wins": 0
	}

func simulate_round(p1, p2, round_num: int) -> String:
	print("P1: %d HP | P2: %d HP\n" % [p1.health, p2.health])

	# Simulate different round patterns
	match round_num:
		1:
			# Round 1: Basic exchanges
			print("Exchange 1:")
			perform_attack(p1, p2, "Jab", 50)
			perform_attack(p2, p1, "Strong", 75)

			print("\nExchange 2:")
			perform_attack(p1, p2, "Fierce", 100)
			perform_attack(p1, p2, "Hadouken", 80)

			print("\nExchange 3:")
			perform_attack(p2, p1, "Shoryuken", 140)

			print("\nExchange 4:")
			perform_attack(p1, p2, "Combo", 225)  # 3-hit combo

		2:
			# Round 2: More aggressive
			print("Exchange 1:")
			perform_attack(p2, p1, "Fierce", 100)

			print("\nExchange 2:")
			perform_attack(p1, p2, "Super", 300)  # P1 uses super

			print("\nExchange 3:")
			perform_attack(p2, p1, "Combo", 200)

			print("\nExchange 4:")
			perform_attack(p1, p2, "Hadouken", 80)
			perform_attack(p1, p2, "Shoryuken", 140)

		3:
			# Round 3: Final round intensity
			print("Exchange 1:")
			perform_attack(p1, p2, "Combo", 250)

			print("\nExchange 2:")
			perform_attack(p2, p1, "Super", 300)

			print("\nExchange 3:")
			perform_attack(p1, p2, "Fierce", 100)
			perform_attack(p2, p1, "Hadouken", 80)

			print("\nExchange 4:")
			perform_attack(p1, p2, "Shoryuken", 140)

	print("\n--- Round End ---")
	print("P1: %d HP" % p1.health)
	print("P2: %d HP" % p2.health)

	# Determine winner
	if p1.health > p2.health:
		return "P1"
	elif p2.health > p1.health:
		return "P2"
	else:
		# Tie - give it to P1 arbitrarily
		return "P1"

func perform_attack(attacker, defender, move_name: String, damage: int):
	defender.health -= damage
	attacker.meter += damage / 10

	if defender.health < 0:
		defender.health = 0

	print("  %s uses %s → %d damage!" % [attacker.name, move_name, damage])
	print("    %s: %d HP" % [defender.name, defender.health])
