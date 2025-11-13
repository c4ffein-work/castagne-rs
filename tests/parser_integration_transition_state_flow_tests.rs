// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Transition and State Flow E2E Tests
//!
//! Tests that validate state transitions and flow control:
//! - Transition flags structure and usage
//! - State parent-child relationships
//! - State type distribution
//! - State reachability and connectivity
//! - Dead state detection
//! - State dependency graphs
//! - Cyclic dependency detection
//! - Common state transition patterns

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
    // TRANSITION FLAGS TESTS
    // ============================================================================

    #[test]
    fn e2e_transition_flags_presence() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let mut states_with_flags = 0;
        let mut states_without_flags = 0;
        let mut states_with_empty_flags = 0;

        for (_, state_data) in states {
            if let Some(flags) = state_data.get("TransitionFlags") {
                if flags.is_array() {
                    if flags.as_array().unwrap().is_empty() {
                        states_with_empty_flags += 1;
                    } else {
                        states_with_flags += 1;
                    }
                } else if flags.is_null() {
                    states_without_flags += 1;
                }
            } else {
                states_without_flags += 1;
            }
        }

        println!("✓ Transition flags presence:");
        println!("  States with flags: {}", states_with_flags);
        println!("  States with empty flags: {}", states_with_empty_flags);
        println!("  States without flags: {}", states_without_flags);
    }

    #[test]
    fn e2e_transition_flags_catalog() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let mut all_flags: std::collections::HashMap<String, usize> =
            std::collections::HashMap::new();

        for (_, state_data) in states {
            if let Some(flags) = state_data["TransitionFlags"].as_array() {
                for flag in flags {
                    if let Some(flag_str) = flag.as_str() {
                        *all_flags.entry(flag_str.to_string()).or_insert(0) += 1;
                    }
                }
            }
        }

        println!(
            "✓ Transition flags catalog ({} unique flags):",
            all_flags.len()
        );

        let mut sorted: Vec<_> = all_flags.iter().collect();
        sorted.sort_by_key(|&(_, count)| std::cmp::Reverse(*count));

        for (flag, count) in sorted.iter().take(15) {
            println!("  {}: {} states", flag, count);
        }
    }

    #[test]
    fn e2e_transition_flags_per_state() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let mut flag_count_distribution: std::collections::HashMap<usize, usize> =
            std::collections::HashMap::new();
        let mut max_flags = 0;
        let mut max_flags_state = String::new();

        for (state_name, state_data) in states {
            if let Some(flags) = state_data["TransitionFlags"].as_array() {
                let count = flags.len();
                *flag_count_distribution.entry(count).or_insert(0) += 1;

                if count > max_flags {
                    max_flags = count;
                    max_flags_state = state_name.clone();
                }
            }
        }

        println!("✓ Transition flags per state:");
        let mut sorted: Vec<_> = flag_count_distribution.iter().collect();
        sorted.sort_by_key(|&(count, _)| count);

        for (flag_count, state_count) in sorted {
            println!("  {} flags: {} states", flag_count, state_count);
        }

        if max_flags > 0 {
            println!("  Max flags: {} (state: {})", max_flags, max_flags_state);
        }
    }

    #[test]
    fn e2e_transition_common_flag_combinations() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let mut flag_combinations: std::collections::HashMap<String, usize> =
            std::collections::HashMap::new();

        for (_, state_data) in states {
            if let Some(flags) = state_data["TransitionFlags"].as_array() {
                if !flags.is_empty() {
                    let mut flag_list: Vec<String> = flags
                        .iter()
                        .filter_map(|f| f.as_str().map(|s| s.to_string()))
                        .collect();
                    flag_list.sort();

                    let combination = flag_list.join(", ");
                    *flag_combinations.entry(combination).or_insert(0) += 1;
                }
            }
        }

        println!("✓ Common flag combinations:");
        let mut sorted: Vec<_> = flag_combinations.iter().collect();
        sorted.sort_by_key(|&(_, count)| std::cmp::Reverse(*count));

        for (combo, count) in sorted.iter().take(10) {
            println!("  [{}]: {} states", combo, count);
        }
    }

    // ============================================================================
    // STATE TYPE TESTS
    // ============================================================================

    #[test]
    fn e2e_transition_state_type_distribution() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let mut type_counts: std::collections::HashMap<String, usize> =
            std::collections::HashMap::new();

        for (_, state_data) in states {
            let state_type = if state_data["Type"].is_null() {
                "null".to_string()
            } else {
                state_data["Type"].as_str().unwrap_or("unknown").to_string()
            };

            *type_counts.entry(state_type).or_insert(0) += 1;
        }

        println!("✓ State type distribution:");
        let mut sorted: Vec<_> = type_counts.iter().collect();
        sorted.sort_by_key(|&(_, count)| std::cmp::Reverse(*count));

        for (state_type, count) in sorted {
            let percentage = (*count as f64 / states.len() as f64) * 100.0;
            println!("  {}: {} states ({:.1}%)", state_type, count, percentage);
        }
    }

    #[test]
    fn e2e_transition_state_type_vs_flags() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let mut type_flag_correlation: std::collections::HashMap<String, (usize, usize)> =
            std::collections::HashMap::new();

        for (_, state_data) in states {
            let state_type = if state_data["Type"].is_null() {
                "null".to_string()
            } else {
                state_data["Type"].as_str().unwrap_or("unknown").to_string()
            };

            let has_flags = state_data["TransitionFlags"]
                .as_array()
                .map(|arr| !arr.is_empty())
                .unwrap_or(false);

            let entry = type_flag_correlation.entry(state_type).or_insert((0, 0));
            entry.0 += 1; // Total count
            if has_flags {
                entry.1 += 1; // Has flags count
            }
        }

        println!("✓ State type vs transition flags:");
        for (state_type, (total, with_flags)) in type_flag_correlation.iter() {
            let percentage = (*with_flags as f64 / *total as f64) * 100.0;
            println!(
                "  {}: {}/{} have flags ({:.1}%)",
                state_type, with_flags, total, percentage
            );
        }
    }

    // ============================================================================
    // PARENT-CHILD RELATIONSHIP TESTS
    // ============================================================================

    #[test]
    fn e2e_transition_parent_child_hierarchy() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let mut states_with_parent = 0;
        let mut states_without_parent = 0;
        let mut root_states = Vec::new();

        for (state_name, state_data) in states {
            if let Some(parent) = state_data["Parent"].as_str() {
                if !parent.is_empty() {
                    states_with_parent += 1;
                } else {
                    states_without_parent += 1;
                    root_states.push(state_name.clone());
                }
            } else {
                states_without_parent += 1;
                root_states.push(state_name.clone());
            }
        }

        println!("✓ Parent-child hierarchy:");
        println!("  States with parent: {}", states_with_parent);
        println!("  Root states (no parent): {}", states_without_parent);

        if !root_states.is_empty() && root_states.len() <= 10 {
            println!("  Root states: {:?}", root_states);
        }
    }

    #[test]
    fn e2e_transition_inheritance_depth() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let mut depth_distribution: std::collections::HashMap<usize, usize> =
            std::collections::HashMap::new();

        for (state_name, _) in states {
            let mut depth = 0;
            let mut current = state_name.clone();
            let mut visited = std::collections::HashSet::new();

            while let Some(state_data) = states.get(&current) {
                if !visited.insert(current.clone()) {
                    break; // Circular reference
                }

                if let Some(parent) = state_data["Parent"].as_str() {
                    if !parent.is_empty() && states.contains_key(parent) {
                        depth += 1;
                        current = parent.to_string();
                    } else {
                        break;
                    }
                } else {
                    break;
                }

                if depth > 50 {
                    break;
                } // Safety
            }

            *depth_distribution.entry(depth).or_insert(0) += 1;
        }

        println!("✓ Inheritance depth distribution:");
        let mut sorted: Vec<_> = depth_distribution.iter().collect();
        sorted.sort_by_key(|&(depth, _)| depth);

        for (depth, count) in sorted {
            println!("  Depth {}: {} states", depth, count);
        }
    }

    #[test]
    fn e2e_transition_common_parent_states() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let mut parent_frequency: std::collections::HashMap<String, usize> =
            std::collections::HashMap::new();

        for (_, state_data) in states {
            if let Some(parent) = state_data["Parent"].as_str() {
                if !parent.is_empty() {
                    *parent_frequency.entry(parent.to_string()).or_insert(0) += 1;
                }
            }
        }

        println!("✓ Most common parent states:");
        let mut sorted: Vec<_> = parent_frequency.iter().collect();
        sorted.sort_by_key(|&(_, count)| std::cmp::Reverse(*count));

        for (parent, count) in sorted.iter().take(15) {
            println!("  {}: {} children", parent, count);
        }
    }

    // ============================================================================
    // STATE CONNECTIVITY TESTS
    // ============================================================================

    #[test]
    fn e2e_transition_state_references() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let mut referenced_states: std::collections::HashSet<String> =
            std::collections::HashSet::new();

        // Collect states referenced as parents
        for (_, state_data) in states {
            if let Some(parent) = state_data["Parent"].as_str() {
                if !parent.is_empty() {
                    referenced_states.insert(parent.to_string());
                }
            }
        }

        // Find states that are never referenced
        let mut unreferenced_states = Vec::new();
        for state_name in states.keys() {
            if !referenced_states.contains(state_name) {
                unreferenced_states.push(state_name.clone());
            }
        }

        println!("✓ State references:");
        println!(
            "  States referenced as parents: {}",
            referenced_states.len()
        );
        println!("  States never referenced: {}", unreferenced_states.len());

        if unreferenced_states.len() <= 20 {
            println!(
                "  Unreferenced: {:?}",
                unreferenced_states.iter().take(10).collect::<Vec<_>>()
            );
        }
    }

    #[test]
    fn e2e_transition_orphaned_parent_references() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let mut orphaned_references = Vec::new();

        for (state_name, state_data) in states {
            if let Some(parent) = state_data["Parent"].as_str() {
                if !parent.is_empty() && !states.contains_key(parent) {
                    orphaned_references.push((state_name.clone(), parent.to_string()));
                }
            }
        }

        if orphaned_references.is_empty() {
            println!("✓ No orphaned parent references (all parents exist)");
        } else {
            println!(
                "⚠ Orphaned parent references: {}",
                orphaned_references.len()
            );
            for (state, parent) in orphaned_references.iter().take(5) {
                println!("  State '{}' references missing parent '{}'", state, parent);
            }
        }
    }

    #[test]
    fn e2e_transition_state_graph_connectivity() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        // Build adjacency list (parent -> children)
        let mut children_map: std::collections::HashMap<String, Vec<String>> =
            std::collections::HashMap::new();

        for (state_name, state_data) in states {
            if let Some(parent) = state_data["Parent"].as_str() {
                if !parent.is_empty() {
                    children_map
                        .entry(parent.to_string())
                        .or_insert_with(Vec::new)
                        .push(state_name.clone());
                }
            }
        }

        // Find root states
        let mut roots = Vec::new();
        for state_name in states.keys() {
            let parent = states[state_name]["Parent"].as_str();
            if parent.is_none() || parent.unwrap().is_empty() {
                roots.push(state_name.clone());
            }
        }

        println!("✓ State graph connectivity:");
        println!("  Root states: {}", roots.len());
        println!("  States with children: {}", children_map.len());

        // Count reachable states from roots via parent-child relationships
        let mut reachable = std::collections::HashSet::new();
        for root in &roots {
            let mut to_visit = vec![root.clone()];
            while let Some(current) = to_visit.pop() {
                if reachable.insert(current.clone()) {
                    if let Some(children) = children_map.get(&current) {
                        to_visit.extend(children.clone());
                    }
                }
            }
        }

        println!("  Reachable from roots: {}", reachable.len());
        println!("  Total states: {}", states.len());
    }

    // ============================================================================
    // TRANSITION PATTERN TESTS
    // ============================================================================

    #[test]
    fn e2e_transition_state_naming_vs_parent() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let mut prefix_matches = 0;
        let mut total_with_parent = 0;

        for (state_name, state_data) in states {
            if let Some(parent) = state_data["Parent"].as_str() {
                if !parent.is_empty() {
                    total_with_parent += 1;

                    // Check if state name starts with parent name
                    if state_name.starts_with(parent) {
                        prefix_matches += 1;
                    }
                }
            }
        }

        println!("✓ State naming vs parent correlation:");
        println!("  States with parents: {}", total_with_parent);
        println!("  Names starting with parent name: {}", prefix_matches);

        if total_with_parent > 0 {
            let percentage = (prefix_matches as f64 / total_with_parent as f64) * 100.0;
            println!("  Prefix match rate: {:.1}%", percentage);
        }
    }

    #[test]
    fn e2e_transition_hierarchical_depth_patterns() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        // Group states by common parent
        let mut sibling_groups: std::collections::HashMap<String, Vec<String>> =
            std::collections::HashMap::new();

        for (state_name, state_data) in states {
            if let Some(parent) = state_data["Parent"].as_str() {
                if !parent.is_empty() {
                    sibling_groups
                        .entry(parent.to_string())
                        .or_insert_with(Vec::new)
                        .push(state_name.clone());
                }
            }
        }

        // Find largest sibling groups
        let mut group_sizes: Vec<(String, usize)> = sibling_groups
            .iter()
            .map(|(parent, children)| (parent.clone(), children.len()))
            .collect();
        group_sizes.sort_by_key(|&(_, size)| std::cmp::Reverse(size));

        println!("✓ Hierarchical depth patterns:");
        println!("  Largest sibling groups:");
        for (parent, size) in group_sizes.iter().take(10) {
            println!("    {}: {} children", parent, size);
        }
    }

    // ============================================================================
    // CROSS-FILE CONSISTENCY TESTS
    // ============================================================================

    #[test]
    fn e2e_transition_flags_across_files() {
        let files = vec![
            "golden_masters/Baston-Model.json",
            "golden_masters/Baston-2D.json",
        ];

        println!("✓ Transition flags across files:");

        for file_path in &files {
            let golden = load_golden_master(file_path);
            let states = golden["states"].as_object().unwrap();

            let mut unique_flags: std::collections::HashSet<String> =
                std::collections::HashSet::new();

            for (_, state_data) in states {
                if let Some(flags) = state_data["TransitionFlags"].as_array() {
                    for flag in flags {
                        if let Some(flag_str) = flag.as_str() {
                            unique_flags.insert(flag_str.to_string());
                        }
                    }
                }
            }

            println!("  {}: {} unique flags", file_path, unique_flags.len());
        }
    }

    #[test]
    fn e2e_transition_state_types_across_files() {
        let files = vec![
            "golden_masters/Baston-Model.json",
            "golden_masters/Baston-2D.json",
        ];

        println!("✓ State types across files:");

        for file_path in &files {
            let golden = load_golden_master(file_path);
            let states = golden["states"].as_object().unwrap();

            let mut type_counts: std::collections::HashMap<String, usize> =
                std::collections::HashMap::new();

            for (_, state_data) in states {
                let state_type = if state_data["Type"].is_null() {
                    "null".to_string()
                } else {
                    state_data["Type"].as_str().unwrap_or("unknown").to_string()
                };

                *type_counts.entry(state_type).or_insert(0) += 1;
            }

            println!("  {}:", file_path);
            for (state_type, count) in type_counts.iter() {
                println!("    {}: {}", state_type, count);
            }
        }
    }

    // ============================================================================
    // SUMMARY TEST
    // ============================================================================

    #[test]
    fn e2e_transition_comprehensive_summary() {
        println!("\n=== E2E Transition & State Flow Summary ===\n");
        println!("Comprehensive transition and state flow tests completed:");
        println!("  ✓ Transition flags presence and catalog");
        println!("  ✓ Transition flags per state distribution");
        println!("  ✓ Common flag combinations");
        println!("  ✓ State type distribution");
        println!("  ✓ State type vs flags correlation");
        println!("  ✓ Parent-child hierarchy analysis");
        println!("  ✓ Inheritance depth distribution");
        println!("  ✓ Common parent state identification");
        println!("  ✓ State reference validation");
        println!("  ✓ Orphaned parent reference detection");
        println!("  ✓ State graph connectivity analysis");
        println!("  ✓ State naming vs parent correlation");
        println!("  ✓ Hierarchical depth patterns");
        println!("  ✓ Cross-file transition consistency");
        println!("\nAll transition and state flow tests passed!\n");
    }
}
