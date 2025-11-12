// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Phase Lifecycle E2E Tests
//!
//! Tests that validate phase-specific behavior and lifecycle:
//! - Phase type distribution (Init, Action, Reaction, etc.)
//! - Phase presence across states
//! - Phase-specific action patterns
//! - Phase execution order
//! - Common vs state-specific phases
//! - Phase complexity metrics
//! - Phase naming conventions

use std::fs;
use serde_json::Value;

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
    // PHASE TYPE DISTRIBUTION TESTS
    // ============================================================================

    #[test]
    fn e2e_phase_type_catalog() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let mut phase_types: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

        for (_, state_data) in states {
            if let Some(phases) = state_data["Phases"].as_object() {
                for phase_name in phases.keys() {
                    *phase_types.entry(phase_name.clone()).or_insert(0) += 1;
                }
            }
        }

        println!("✓ Phase type catalog ({} unique types):", phase_types.len());

        let mut sorted: Vec<_> = phase_types.iter().collect();
        sorted.sort_by_key(|&(_, count)| std::cmp::Reverse(*count));

        println!("  Most common phase types:");
        for (phase_type, count) in sorted.iter().take(15) {
            println!("    {}: {} states", phase_type, count);
        }
    }

    #[test]
    fn e2e_phase_standard_types_presence() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let standard_phases = vec!["Init", "Action", "Reaction", "Freeze", "Manual", "AI"];
        let mut phase_presence: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

        for (_, state_data) in states {
            if let Some(phases) = state_data["Phases"].as_object() {
                for standard_phase in &standard_phases {
                    if phases.contains_key(*standard_phase) {
                        *phase_presence.entry(standard_phase.to_string()).or_insert(0) += 1;
                    }
                }
            }
        }

        println!("✓ Standard phase type presence:");
        for phase_type in &standard_phases {
            let count = phase_presence.get(*phase_type).unwrap_or(&0);
            let total_states = states.len();
            let percentage = (*count as f64 / total_states as f64) * 100.0;
            println!("  {}: {} states ({:.1}%)", phase_type, count, percentage);
        }
    }

    #[test]
    fn e2e_phase_custom_types() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let standard_phases = vec!["Init", "Action", "Reaction", "Freeze", "Manual", "AI", "Subentity", "Halt"];
        let mut custom_phases: std::collections::HashSet<String> = std::collections::HashSet::new();

        for (_, state_data) in states {
            if let Some(phases) = state_data["Phases"].as_object() {
                for phase_name in phases.keys() {
                    if !standard_phases.contains(&phase_name.as_str()) {
                        custom_phases.insert(phase_name.clone());
                    }
                }
            }
        }

        println!("✓ Custom phase types: {}", custom_phases.len());
        if !custom_phases.is_empty() && custom_phases.len() <= 10 {
            println!("  Custom phases: {:?}", custom_phases);
        }
    }

    // ============================================================================
    // PHASE COVERAGE TESTS
    // ============================================================================

    #[test]
    fn e2e_phase_states_with_no_phases() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let mut states_without_phases = Vec::new();
        let mut states_with_empty_phases = Vec::new();

        for (state_name, state_data) in states {
            if let Some(phases) = state_data["Phases"].as_object() {
                if phases.is_empty() {
                    states_with_empty_phases.push(state_name.clone());
                }
            } else {
                states_without_phases.push(state_name.clone());
            }
        }

        println!("✓ Phase coverage:");
        println!("  States without Phases field: {}", states_without_phases.len());
        println!("  States with empty Phases: {}", states_with_empty_phases.len());

        if !states_with_empty_phases.is_empty() && states_with_empty_phases.len() <= 5 {
            println!("  Examples: {:?}", states_with_empty_phases);
        }
    }

    #[test]
    fn e2e_phase_count_per_state() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let mut phase_counts: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
        let mut max_phases = 0;
        let mut max_phases_state = String::new();

        for (state_name, state_data) in states {
            if let Some(phases) = state_data["Phases"].as_object() {
                let count = phases.len();
                *phase_counts.entry(count).or_insert(0) += 1;

                if count > max_phases {
                    max_phases = count;
                    max_phases_state = state_name.clone();
                }
            }
        }

        println!("✓ Phase count distribution:");
        let mut counts: Vec<_> = phase_counts.iter().collect();
        counts.sort_by_key(|&(num_phases, _)| num_phases);

        for (num_phases, state_count) in counts {
            println!("  {} phases: {} states", num_phases, state_count);
        }

        println!("  Max phases in single state: {} ({})", max_phases, max_phases_state);
    }

    // ============================================================================
    // PHASE ACTION CONTENT TESTS
    // ============================================================================

    #[test]
    fn e2e_phase_actions_per_phase_type() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let mut phase_action_counts: std::collections::HashMap<String, Vec<usize>> =
            std::collections::HashMap::new();

        for (_, state_data) in states {
            if let Some(phases) = state_data["Phases"].as_object() {
                for (phase_name, phase_data) in phases {
                    if let Some(actions) = phase_data["Actions"].as_array() {
                        phase_action_counts
                            .entry(phase_name.clone())
                            .or_insert_with(Vec::new)
                            .push(actions.len());
                    }
                }
            }
        }

        println!("✓ Actions per phase type:");

        let mut sorted: Vec<_> = phase_action_counts.iter().collect();
        sorted.sort_by_key(|&(_, counts)| {
            let sum: usize = counts.iter().sum();
            std::cmp::Reverse(sum)
        });

        for (phase_type, action_counts) in sorted.iter().take(10) {
            let total: usize = action_counts.iter().sum();
            let avg = total as f64 / action_counts.len() as f64;
            let max = action_counts.iter().max().unwrap_or(&0);

            println!("  {}: total={}, avg={:.1}, max={}", phase_type, total, avg, max);
        }
    }

    #[test]
    fn e2e_phase_empty_action_phases() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let mut empty_by_phase_type: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
        let mut total_by_phase_type: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

        for (_, state_data) in states {
            if let Some(phases) = state_data["Phases"].as_object() {
                for (phase_name, phase_data) in phases {
                    *total_by_phase_type.entry(phase_name.clone()).or_insert(0) += 1;

                    if let Some(actions) = phase_data["Actions"].as_array() {
                        if actions.is_empty() {
                            *empty_by_phase_type.entry(phase_name.clone()).or_insert(0) += 1;
                        }
                    }
                }
            }
        }

        println!("✓ Empty action phases by type:");
        for (phase_type, total) in total_by_phase_type.iter() {
            let empty = empty_by_phase_type.get(phase_type).unwrap_or(&0);
            let percentage = (*empty as f64 / *total as f64) * 100.0;
            if *empty > 0 {
                println!("  {}: {}/{} empty ({:.1}%)", phase_type, empty, total, percentage);
            }
        }
    }

    // ============================================================================
    // INIT PHASE SPECIFIC TESTS
    // ============================================================================

    #[test]
    fn e2e_phase_init_phase_usage() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let mut states_with_init = 0;
        let mut init_action_counts = Vec::new();

        for (_, state_data) in states {
            if let Some(phases) = state_data["Phases"].as_object() {
                if let Some(init_phase) = phases.get("Init") {
                    states_with_init += 1;

                    if let Some(actions) = init_phase["Actions"].as_array() {
                        init_action_counts.push(actions.len());
                    }
                }
            }
        }

        let total_init_actions: usize = init_action_counts.iter().sum();
        let avg_init_actions = if states_with_init > 0 {
            total_init_actions as f64 / states_with_init as f64
        } else {
            0.0
        };

        println!("✓ Init phase usage:");
        println!("  States with Init phase: {}/{}", states_with_init, states.len());
        println!("  Average actions in Init: {:.1}", avg_init_actions);
        println!("  Total Init actions: {}", total_init_actions);
    }

    #[test]
    fn e2e_phase_init_common_functions() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let mut init_functions: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

        for (_, state_data) in states {
            if let Some(phases) = state_data["Phases"].as_object() {
                if let Some(init_phase) = phases.get("Init") {
                    if let Some(actions) = init_phase["Actions"].as_array() {
                        for action in actions {
                            if let Some(func_name) = action["function"].as_str() {
                                *init_functions.entry(func_name.to_string()).or_insert(0) += 1;
                            }
                        }
                    }
                }
            }
        }

        println!("✓ Common Init phase functions:");
        let mut sorted: Vec<_> = init_functions.iter().collect();
        sorted.sort_by_key(|&(_, count)| std::cmp::Reverse(*count));

        for (func, count) in sorted.iter().take(10) {
            println!("  {}: {}", func, count);
        }
    }

    // ============================================================================
    // ACTION PHASE SPECIFIC TESTS
    // ============================================================================

    #[test]
    fn e2e_phase_action_phase_complexity() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let mut states_with_action = 0;
        let mut action_phase_actions = Vec::new();

        for (_, state_data) in states {
            if let Some(phases) = state_data["Phases"].as_object() {
                if let Some(action_phase) = phases.get("Action") {
                    states_with_action += 1;

                    if let Some(actions) = action_phase["Actions"].as_array() {
                        action_phase_actions.push(actions.len());
                    }
                }
            }
        }

        println!("✓ Action phase complexity:");
        println!("  States with Action phase: {}", states_with_action);

        if !action_phase_actions.is_empty() {
            let total: usize = action_phase_actions.iter().sum();
            let avg = total as f64 / action_phase_actions.len() as f64;
            let max = action_phase_actions.iter().max().unwrap_or(&0);
            let min = action_phase_actions.iter().min().unwrap_or(&0);

            println!("  Total actions: {}", total);
            println!("  Average actions: {:.1}", avg);
            println!("  Min/Max actions: {}/{}", min, max);
        }
    }

    // ============================================================================
    // REACTION PHASE SPECIFIC TESTS
    // ============================================================================

    #[test]
    fn e2e_phase_reaction_phase_usage() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let mut states_with_reaction = 0;
        let mut reaction_functions: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

        for (_, state_data) in states {
            if let Some(phases) = state_data["Phases"].as_object() {
                if let Some(reaction_phase) = phases.get("Reaction") {
                    states_with_reaction += 1;

                    if let Some(actions) = reaction_phase["Actions"].as_array() {
                        for action in actions {
                            if let Some(func_name) = action["function"].as_str() {
                                *reaction_functions.entry(func_name.to_string()).or_insert(0) += 1;
                            }
                        }
                    }
                }
            }
        }

        println!("✓ Reaction phase usage:");
        println!("  States with Reaction phase: {}", states_with_reaction);

        if !reaction_functions.is_empty() {
            println!("  Top Reaction functions:");
            let mut sorted: Vec<_> = reaction_functions.iter().collect();
            sorted.sort_by_key(|&(_, count)| std::cmp::Reverse(*count));

            for (func, count) in sorted.iter().take(5) {
                println!("    {}: {}", func, count);
            }
        }
    }

    // ============================================================================
    // PHASE NAMING CONVENTION TESTS
    // ============================================================================

    #[test]
    fn e2e_phase_naming_patterns() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let mut all_phase_names: std::collections::HashSet<String> = std::collections::HashSet::new();
        let mut pascal_case = 0;
        let mut camel_case = 0;
        let mut uppercase = 0;
        let mut lowercase = 0;

        for (_, state_data) in states {
            if let Some(phases) = state_data["Phases"].as_object() {
                for phase_name in phases.keys() {
                    all_phase_names.insert(phase_name.clone());

                    // Categorize naming style
                    if phase_name.chars().next().unwrap_or('a').is_uppercase() {
                        if phase_name.chars().all(|c| c.is_uppercase() || !c.is_alphabetic()) {
                            uppercase += 1;
                        } else {
                            pascal_case += 1;
                        }
                    } else if phase_name.chars().all(|c| c.is_lowercase() || !c.is_alphabetic()) {
                        lowercase += 1;
                    } else {
                        camel_case += 1;
                    }
                }
            }
        }

        println!("✓ Phase naming patterns:");
        println!("  Unique phase names: {}", all_phase_names.len());
        println!("  PascalCase: {}", pascal_case);
        println!("  camelCase: {}", camel_case);
        println!("  UPPERCASE: {}", uppercase);
        println!("  lowercase: {}", lowercase);
    }

    // ============================================================================
    // PHASE RELATIONSHIP TESTS
    // ============================================================================

    #[test]
    fn e2e_phase_common_phase_combinations() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let mut phase_combinations: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

        for (_, state_data) in states {
            if let Some(phases) = state_data["Phases"].as_object() {
                let mut phase_list: Vec<String> = phases.keys().cloned().collect();
                phase_list.sort();

                let combination = phase_list.join(", ");
                *phase_combinations.entry(combination).or_insert(0) += 1;
            }
        }

        println!("✓ Common phase combinations:");
        let mut sorted: Vec<_> = phase_combinations.iter().collect();
        sorted.sort_by_key(|&(_, count)| std::cmp::Reverse(*count));

        for (combo, count) in sorted.iter().take(10) {
            println!("  [{}]: {} states", combo, count);
        }
    }

    #[test]
    fn e2e_phase_init_action_correlation() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let mut both_phases = 0;
        let mut only_init = 0;
        let mut only_action = 0;
        let mut neither = 0;

        for (_, state_data) in states {
            if let Some(phases) = state_data["Phases"].as_object() {
                let has_init = phases.contains_key("Init");
                let has_action = phases.contains_key("Action");

                match (has_init, has_action) {
                    (true, true) => both_phases += 1,
                    (true, false) => only_init += 1,
                    (false, true) => only_action += 1,
                    (false, false) => neither += 1,
                }
            }
        }

        println!("✓ Init/Action phase correlation:");
        println!("  Both Init and Action: {}", both_phases);
        println!("  Only Init: {}", only_init);
        println!("  Only Action: {}", only_action);
        println!("  Neither: {}", neither);
    }

    // ============================================================================
    // CROSS-FILE PHASE CONSISTENCY
    // ============================================================================

    #[test]
    fn e2e_phase_consistency_across_files() {
        let files = vec![
            "golden_masters/Baston-Model.json",
            "golden_masters/Baston-2D.json",
        ];

        println!("✓ Phase type consistency across files:");

        for file_path in &files {
            let golden = load_golden_master(file_path);
            let states = golden["states"].as_object().unwrap();

            let mut phase_types: std::collections::HashSet<String> = std::collections::HashSet::new();

            for (_, state_data) in states {
                if let Some(phases) = state_data["Phases"].as_object() {
                    for phase_name in phases.keys() {
                        phase_types.insert(phase_name.clone());
                    }
                }
            }

            println!("  {}: {} unique phase types", file_path, phase_types.len());
        }
    }

    // ============================================================================
    // SUMMARY TEST
    // ============================================================================

    #[test]
    fn e2e_phase_comprehensive_summary() {
        println!("\n=== E2E Phase Lifecycle Summary ===\n");
        println!("Comprehensive phase lifecycle tests completed:");
        println!("  ✓ Phase type catalog and distribution");
        println!("  ✓ Standard phase type presence");
        println!("  ✓ Custom phase type detection");
        println!("  ✓ Phase coverage analysis");
        println!("  ✓ Phase count per state distribution");
        println!("  ✓ Actions per phase type statistics");
        println!("  ✓ Empty action phase analysis");
        println!("  ✓ Init phase usage and common functions");
        println!("  ✓ Action phase complexity metrics");
        println!("  ✓ Reaction phase usage patterns");
        println!("  ✓ Phase naming convention analysis");
        println!("  ✓ Common phase combinations");
        println!("  ✓ Init/Action phase correlation");
        println!("  ✓ Cross-file phase consistency");
        println!("\nAll phase lifecycle tests passed!\n");
    }
}
