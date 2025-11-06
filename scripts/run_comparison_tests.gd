extends SceneTree

# CLI script to run Castagne-RS comparison tests
# Usage: godot --headless --script scripts/run_comparison_tests.gd

func _init():
	print("===========================================")
	print("  Castagne-RS Comparison Tests")
	print("===========================================")
	print("")

	# Create test runner instance
	var test_runner = CastagneTestRunner.new()

	# Run all comparison tests
	var results = test_runner.run_comparison_tests()

	print("")
	print("===========================================")
	print("  Detailed Results")
	print("===========================================")
	print("")

	# Display detailed results
	var passed = 0
	var failed = 0
	var test_names = []

	for test_name in results.keys():
		test_names.append(test_name)
		var result = results[test_name]
		var status = "✓ PASS" if result else "✗ FAIL"
		var status_plain = "PASS" if result else "FAIL"

		# Try to use color if supported, fallback to plain text
		if result:
			print("  ✓ %s: PASSED" % test_name)
			passed += 1
		else:
			print("  ✗ %s: FAILED" % test_name)
			failed += 1

	print("")
	print("===========================================")
	print("  Summary")
	print("===========================================")
	print("")
	print("  Total tests:  %d" % (passed + failed))
	print("  Passed:       %d" % passed)
	print("  Failed:       %d" % failed)
	print("")

	if failed == 0:
		print("  ✓ All tests passed!")
		print("")
		quit(0)
	else:
		print("  ✗ Some tests failed")
		print("")
		quit(1)
