extends Node

# Simple script to run Castagne parser comparison tests
# Usage: Add this script to a Node in Godot and run the scene
# Or run from command line: godot --script run_parser_tests.gd

func _ready():
	print("=== Castagne-RS Parser Comparison Tests ===")
	print()

	# Create test runner
	var test_runner = CastagneTestRunner.new()
	add_child(test_runner)

	# Run the simple test first (test_character_complete.casp)
	print("TEST 1: Simple character (test_character_complete)")
	print("=" * 60)
	var result_simple = test_runner.test_parser_simple()
	print()

	if result_simple:
		print("✅ Simple test PASSED")
	else:
		print("❌ Simple test FAILED")

	print()
	print("=" * 60)
	print()

	# Optionally run all comparison tests
	# Uncomment the lines below to run full test suite:

	# print("Running full test suite...")
	# print()
	# var results = test_runner.run_comparison_tests()
	# print()
	# print("Test Results:")
	# for test_name in results.keys():
	# 	var passed = results[test_name]
	# 	var status = "✅ PASS" if passed else "❌ FAIL"
	# 	print("  %s: %s" % [test_name, status])

	print()
	print("=== Tests Complete ===")

	# Exit if running headless
	if OS.has_feature("standalone"):
		get_tree().quit()
