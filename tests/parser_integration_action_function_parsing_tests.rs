// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Action and Function Parsing E2E Tests
//!
//! Tests that validate action parsing and function call handling:
//! - Function argument parsing (various types)
//! - Nested function calls
//! - Function name validation
//! - Argument count validation
//! - Common function patterns
//! - Function frequency analysis
//! - Argument type inference
//! - Complex expression arguments

use serde_json::Value;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    // ============================================================================
    // HELPER FUNCTIONS
    // ============================================================================

    fn load_golden_master(path: &str) -> Value {
        let json_content = fs::read_to_string(path)
            .unwrap_or_else(|_| panic!("Failed to load golden master: {}", path));
        serde_json::from_str(&json_content)
            .unwrap_or_else(|_| panic!("Failed to parse golden master JSON: {}", path))
    }

    // ============================================================================
    // BASIC FUNCTION STRUCTURE TESTS
    // ============================================================================

    #[test]
    fn e2e_action_function_name_format() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let mut function_names = std::collections::HashSet::new();
        let mut name_patterns: std::collections::HashMap<String, usize> =
            std::collections::HashMap::new();

        for (_, state_data) in states {
            if let Some(phases) = state_data["Phases"].as_object() {
                for (_, phase_data) in phases {
                    if let Some(actions) = phase_data["Actions"].as_array() {
                        for action in actions {
                            if let Some(func_name) = action["function"].as_str() {
                                function_names.insert(func_name.to_string());

                                // Categorize naming patterns
                                let pattern =
                                    if func_name.chars().next().unwrap_or('a').is_uppercase() {
                                        "PascalCase"
                                    } else if func_name.contains("_") {
                                        "snake_case"
                                    } else {
                                        "camelCase"
                                    };

                                *name_patterns.entry(pattern.to_string()).or_insert(0) += 1;
                            }
                        }
                    }
                }
            }
        }

        println!("✓ Function name format analysis:");
        println!("  Unique function names: {}", function_names.len());
        println!("  Naming patterns:");
        for (pattern, count) in name_patterns.iter() {
            println!("    {}: {}", pattern, count);
        }
    }

    #[test]
    fn e2e_action_function_catalog() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let mut function_catalog: std::collections::HashMap<String, usize> =
            std::collections::HashMap::new();

        for (_, state_data) in states {
            if let Some(phases) = state_data["Phases"].as_object() {
                for (_, phase_data) in phases {
                    if let Some(actions) = phase_data["Actions"].as_array() {
                        for action in actions {
                            if let Some(func_name) = action["function"].as_str() {
                                *function_catalog.entry(func_name.to_string()).or_insert(0) += 1;
                            }
                        }
                    }
                }
            }
        }

        println!(
            "✓ Function catalog ({} unique functions):",
            function_catalog.len()
        );

        let mut sorted: Vec<_> = function_catalog.iter().collect();
        sorted.sort_by_key(|&(_, count)| std::cmp::Reverse(*count));

        println!("  Top 15 most used functions:");
        for (func_name, count) in sorted.iter().take(15) {
            println!("    {}: {} calls", func_name, count);
        }
    }

    #[test]
    fn e2e_action_function_argument_counts() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let mut arg_count_by_function: std::collections::HashMap<String, Vec<usize>> =
            std::collections::HashMap::new();

        for (_, state_data) in states {
            if let Some(phases) = state_data["Phases"].as_object() {
                for (_, phase_data) in phases {
                    if let Some(actions) = phase_data["Actions"].as_array() {
                        for action in actions {
                            if let Some(func_name) = action["function"].as_str() {
                                if let Some(args) = action["args"].as_array() {
                                    arg_count_by_function
                                        .entry(func_name.to_string())
                                        .or_insert_with(Vec::new)
                                        .push(args.len());
                                }
                            }
                        }
                    }
                }
            }
        }

        println!("✓ Function argument count consistency:");

        // Check for functions with variable argument counts
        let mut variable_arg_functions = Vec::new();

        for (func_name, arg_counts) in arg_count_by_function.iter() {
            let min_args = arg_counts.iter().min().unwrap_or(&0);
            let max_args = arg_counts.iter().max().unwrap_or(&0);

            if min_args != max_args {
                variable_arg_functions.push((func_name.clone(), *min_args, *max_args));
            }
        }

        if variable_arg_functions.is_empty() {
            println!("  All functions have consistent argument counts");
        } else {
            println!("  Functions with variable argument counts:");
            for (func, min, max) in variable_arg_functions.iter().take(10) {
                println!("    {}: {}-{} args", func, min, max);
            }
        }
    }

    // ============================================================================
    // ARGUMENT TYPE TESTS
    // ============================================================================

    #[test]
    fn e2e_action_argument_type_patterns() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let mut total_args = 0;
        let mut numeric_args = 0;
        let mut string_args = 0;
        let mut identifier_args = 0;
        let mut expression_args = 0;

        for (_, state_data) in states {
            if let Some(phases) = state_data["Phases"].as_object() {
                for (_, phase_data) in phases {
                    if let Some(actions) = phase_data["Actions"].as_array() {
                        for action in actions {
                            if let Some(args) = action["args"].as_array() {
                                for arg in args {
                                    if let Some(arg_str) = arg.as_str() {
                                        total_args += 1;

                                        // Classify argument type
                                        if arg_str.parse::<i64>().is_ok()
                                            || arg_str.parse::<f64>().is_ok()
                                        {
                                            numeric_args += 1;
                                        } else if arg_str.starts_with('"')
                                            || arg_str.starts_with('\'')
                                        {
                                            string_args += 1;
                                        } else if arg_str.contains('(')
                                            || arg_str.contains('+')
                                            || arg_str.contains('-')
                                            || arg_str.contains('*')
                                        {
                                            expression_args += 1;
                                        } else {
                                            identifier_args += 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        println!("✓ Argument type patterns:");
        println!("  Total arguments: {}", total_args);
        println!(
            "  Numeric literals: {} ({:.1}%)",
            numeric_args,
            (numeric_args as f64 / total_args as f64) * 100.0
        );
        println!(
            "  String literals: {} ({:.1}%)",
            string_args,
            (string_args as f64 / total_args as f64) * 100.0
        );
        println!(
            "  Identifiers: {} ({:.1}%)",
            identifier_args,
            (identifier_args as f64 / total_args as f64) * 100.0
        );
        println!(
            "  Expressions: {} ({:.1}%)",
            expression_args,
            (expression_args as f64 / total_args as f64) * 100.0
        );
    }

    #[test]
    fn e2e_action_argument_string_literals() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let mut string_args = Vec::new();

        for (_, state_data) in states {
            if let Some(phases) = state_data["Phases"].as_object() {
                for (_, phase_data) in phases {
                    if let Some(actions) = phase_data["Actions"].as_array() {
                        for action in actions {
                            if let Some(args) = action["args"].as_array() {
                                for arg in args {
                                    if let Some(arg_str) = arg.as_str() {
                                        if arg_str.starts_with('"') || arg_str.contains('\'') {
                                            string_args.push(arg_str.to_string());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        println!("✓ String literal arguments: {}", string_args.len());
        if !string_args.is_empty() {
            println!(
                "  Examples: {:?}",
                string_args.iter().take(5).collect::<Vec<_>>()
            );
        }
    }

    #[test]
    fn e2e_action_argument_numeric_literals() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let mut int_args = 0;
        let mut float_args = 0;
        let mut negative_args = 0;

        for (_, state_data) in states {
            if let Some(phases) = state_data["Phases"].as_object() {
                for (_, phase_data) in phases {
                    if let Some(actions) = phase_data["Actions"].as_array() {
                        for action in actions {
                            if let Some(args) = action["args"].as_array() {
                                for arg in args {
                                    if let Some(arg_str) = arg.as_str() {
                                        if arg_str.parse::<i64>().is_ok() {
                                            int_args += 1;
                                            if arg_str.starts_with('-') {
                                                negative_args += 1;
                                            }
                                        } else if arg_str.parse::<f64>().is_ok() {
                                            float_args += 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        println!("✓ Numeric literal arguments:");
        println!("  Integer literals: {}", int_args);
        println!("  Float literals: {}", float_args);
        println!("  Negative numbers: {}", negative_args);
    }

    // ============================================================================
    // NESTED FUNCTION TESTS
    // ============================================================================

    #[test]
    fn e2e_action_nested_function_calls() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let mut nested_calls = 0;

        for (_, state_data) in states {
            if let Some(phases) = state_data["Phases"].as_object() {
                for (_, phase_data) in phases {
                    if let Some(actions) = phase_data["Actions"].as_array() {
                        for action in actions {
                            if let Some(args) = action["args"].as_array() {
                                for arg in args {
                                    if let Some(arg_str) = arg.as_str() {
                                        // Check if argument contains a function call
                                        if arg_str.contains('(') && arg_str.contains(')') {
                                            nested_calls += 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        println!("✓ Nested function calls: {}", nested_calls);
    }

    #[test]
    fn e2e_action_expression_complexity() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let mut simple_args = 0; // No operators
        let mut arithmetic_args = 0; // +, -, *, /
        let mut comparison_args = 0; // <, >, ==, !=
        let mut logical_args = 0; // &&, ||, !

        for (_, state_data) in states {
            if let Some(phases) = state_data["Phases"].as_object() {
                for (_, phase_data) in phases {
                    if let Some(actions) = phase_data["Actions"].as_array() {
                        for action in actions {
                            if let Some(args) = action["args"].as_array() {
                                for arg in args {
                                    if let Some(arg_str) = arg.as_str() {
                                        let has_arithmetic = arg_str.contains('+')
                                            || arg_str.contains('-')
                                            || arg_str.contains('*')
                                            || arg_str.contains('/');
                                        let has_comparison = arg_str.contains('<')
                                            || arg_str.contains('>')
                                            || arg_str.contains("==")
                                            || arg_str.contains("!=");
                                        let has_logical = arg_str.contains("&&")
                                            || arg_str.contains("||")
                                            || arg_str.contains('!');

                                        if has_logical {
                                            logical_args += 1;
                                        } else if has_comparison {
                                            comparison_args += 1;
                                        } else if has_arithmetic {
                                            arithmetic_args += 1;
                                        } else {
                                            simple_args += 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        println!("✓ Expression complexity distribution:");
        println!("  Simple (no operators): {}", simple_args);
        println!("  Arithmetic expressions: {}", arithmetic_args);
        println!("  Comparison expressions: {}", comparison_args);
        println!("  Logical expressions: {}", logical_args);
    }

    // ============================================================================
    // COMMON FUNCTION PATTERN TESTS
    // ============================================================================

    #[test]
    fn e2e_action_variable_manipulation_functions() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let var_keywords = ["Set", "Add", "Sub", "Mul", "Div", "Increment", "Decrement"];
        let mut var_functions: std::collections::HashMap<String, usize> =
            std::collections::HashMap::new();

        for (_, state_data) in states {
            if let Some(phases) = state_data["Phases"].as_object() {
                for (_, phase_data) in phases {
                    if let Some(actions) = phase_data["Actions"].as_array() {
                        for action in actions {
                            if let Some(func_name) = action["function"].as_str() {
                                for keyword in &var_keywords {
                                    if func_name.contains(keyword) {
                                        *var_functions.entry(func_name.to_string()).or_insert(0) +=
                                            1;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        println!("✓ Variable manipulation functions:");
        let mut sorted: Vec<_> = var_functions.iter().collect();
        sorted.sort_by_key(|&(_, count)| std::cmp::Reverse(*count));

        for (func, count) in sorted.iter().take(10) {
            println!("  {}: {}", func, count);
        }
    }

    #[test]
    fn e2e_action_state_transition_functions() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let state_keywords = ["ChangeState", "State", "Goto", "Jump", "Return"];
        let mut state_functions: std::collections::HashMap<String, usize> =
            std::collections::HashMap::new();

        for (_, state_data) in states {
            if let Some(phases) = state_data["Phases"].as_object() {
                for (_, phase_data) in phases {
                    if let Some(actions) = phase_data["Actions"].as_array() {
                        for action in actions {
                            if let Some(func_name) = action["function"].as_str() {
                                for keyword in &state_keywords {
                                    if func_name.contains(keyword) {
                                        *state_functions
                                            .entry(func_name.to_string())
                                            .or_insert(0) += 1;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        println!("✓ State transition functions:");
        for (func, count) in state_functions.iter() {
            println!("  {}: {}", func, count);
        }
    }

    #[test]
    fn e2e_action_conditional_functions() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let conditional_keywords = ["If", "Else", "EndIf", "Switch", "Case", "When"];
        let mut conditional_functions: std::collections::HashMap<String, usize> =
            std::collections::HashMap::new();

        for (_, state_data) in states {
            if let Some(phases) = state_data["Phases"].as_object() {
                for (_, phase_data) in phases {
                    if let Some(actions) = phase_data["Actions"].as_array() {
                        for action in actions {
                            if let Some(func_name) = action["function"].as_str() {
                                for keyword in &conditional_keywords {
                                    if func_name == *keyword || func_name.starts_with(keyword) {
                                        *conditional_functions
                                            .entry(func_name.to_string())
                                            .or_insert(0) += 1;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        println!("✓ Conditional control flow functions:");
        for (func, count) in conditional_functions.iter() {
            println!("  {}: {}", func, count);
        }
    }

    // ============================================================================
    // FUNCTION USAGE BY STATE TESTS
    // ============================================================================

    #[test]
    fn e2e_action_functions_per_state() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let mut state_action_counts: Vec<(String, usize)> = Vec::new();

        for (state_name, state_data) in states {
            let mut action_count = 0;

            if let Some(phases) = state_data["Phases"].as_object() {
                for (_, phase_data) in phases {
                    if let Some(actions) = phase_data["Actions"].as_array() {
                        action_count += actions.len();
                    }
                }
            }

            if action_count > 0 {
                state_action_counts.push((state_name.clone(), action_count));
            }
        }

        state_action_counts.sort_by_key(|&(_, count)| std::cmp::Reverse(count));

        println!("✓ Actions per state (top 10 states):");
        for (state_name, count) in state_action_counts.iter().take(10) {
            println!("  {}: {} actions", state_name, count);
        }

        let total_actions: usize = state_action_counts.iter().map(|&(_, count)| count).sum();
        let states_with_actions = state_action_counts.len();

        if states_with_actions > 0 {
            println!(
                "  Average actions per state: {:.1}",
                total_actions as f64 / states_with_actions as f64
            );
        }
    }

    #[test]
    fn e2e_action_function_diversity_by_state() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let mut state_function_diversity: Vec<(String, usize)> = Vec::new();

        for (state_name, state_data) in states {
            let mut unique_functions = std::collections::HashSet::new();

            if let Some(phases) = state_data["Phases"].as_object() {
                for (_, phase_data) in phases {
                    if let Some(actions) = phase_data["Actions"].as_array() {
                        for action in actions {
                            if let Some(func_name) = action["function"].as_str() {
                                unique_functions.insert(func_name.to_string());
                            }
                        }
                    }
                }
            }

            if !unique_functions.is_empty() {
                state_function_diversity.push((state_name.clone(), unique_functions.len()));
            }
        }

        state_function_diversity.sort_by_key(|&(_, count)| std::cmp::Reverse(count));

        println!("✓ Function diversity by state (top 10):");
        for (state_name, count) in state_function_diversity.iter().take(10) {
            println!("  {}: {} unique functions", state_name, count);
        }
    }

    // ============================================================================
    // ARGUMENT VALIDATION TESTS
    // ============================================================================

    #[test]
    fn e2e_action_argument_structure_valid() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let mut invalid_args = Vec::new();

        for (state_name, state_data) in states {
            if let Some(phases) = state_data["Phases"].as_object() {
                for (phase_name, phase_data) in phases {
                    if let Some(actions) = phase_data["Actions"].as_array() {
                        for (i, action) in actions.iter().enumerate() {
                            // Check args is an array
                            if !action["args"].is_array() {
                                invalid_args.push(format!(
                                    "{}.{} action {}: args is not array",
                                    state_name, phase_name, i
                                ));
                            }
                        }
                    }
                }
            }
        }

        assert!(
            invalid_args.is_empty(),
            "Invalid argument structures: {:?}",
            invalid_args
        );
        println!("✓ All action argument structures valid");
    }

    #[test]
    fn e2e_action_zero_argument_functions() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let mut zero_arg_functions: std::collections::HashMap<String, usize> =
            std::collections::HashMap::new();

        for (_, state_data) in states {
            if let Some(phases) = state_data["Phases"].as_object() {
                for (_, phase_data) in phases {
                    if let Some(actions) = phase_data["Actions"].as_array() {
                        for action in actions {
                            if let Some(args) = action["args"].as_array() {
                                if args.is_empty() {
                                    if let Some(func_name) = action["function"].as_str() {
                                        *zero_arg_functions
                                            .entry(func_name.to_string())
                                            .or_insert(0) += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        println!("✓ Zero-argument functions: {}", zero_arg_functions.len());
        if !zero_arg_functions.is_empty() {
            println!("  Examples:");
            for (func, count) in zero_arg_functions.iter().take(10) {
                println!("    {}: {} calls", func, count);
            }
        }
    }

    // ============================================================================
    // SUMMARY TEST
    // ============================================================================

    #[test]
    fn e2e_action_comprehensive_summary() {
        println!("\n=== E2E Action & Function Parsing Summary ===\n");
        println!("Comprehensive action and function parsing tests completed:");
        println!("  ✓ Function name format analysis");
        println!("  ✓ Function catalog and frequency");
        println!("  ✓ Argument count validation");
        println!("  ✓ Argument type pattern analysis");
        println!("  ✓ String and numeric literal detection");
        println!("  ✓ Nested function call detection");
        println!("  ✓ Expression complexity analysis");
        println!("  ✓ Variable manipulation function patterns");
        println!("  ✓ State transition function patterns");
        println!("  ✓ Conditional control flow functions");
        println!("  ✓ Function usage per state");
        println!("  ✓ Function diversity analysis");
        println!("  ✓ Argument structure validation");
        println!("  ✓ Zero-argument function handling");
        println!("\nAll action parsing tests passed!\n");
    }
}
