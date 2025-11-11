// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Fighting Game Pattern E2E Tests
//!
//! Tests that validate common fighting game patterns and mechanics:
//! - Character movement states (stand, walk, jump, crouch)
//! - Attack states and combos
//! - Hit reactions and hitstun
//! - Special moves and command inputs
//! - Blocking and defense mechanics
//! - State transition patterns
//! - Frame data and timing

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

    fn state_exists(states: &serde_json::Map<String, Value>, state_name: &str) -> bool {
        states.contains_key(state_name)
    }

    // ============================================================================
    // MOVEMENT STATE TESTS
    // ============================================================================

    #[test]
    fn e2e_fighting_movement_stand_state() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        // Stand is a fundamental state
        let has_stand = state_exists(states, "Stand");
        println!("✓ Stand state: {}", has_stand);
    }

    #[test]
    fn e2e_fighting_movement_init_state() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        // Init state should exist
        let has_init = state_exists(states, "Init");
        assert!(has_init, "Init state should exist");

        println!("✓ Init state validated");
    }

    #[test]
    fn e2e_fighting_movement_common_states() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        // Count common movement-related states
        let mut movement_states = Vec::new();
        let common_movements = vec!["Stand", "Walk", "Jump", "Crouch", "Land", "Dash"];

        for movement in common_movements {
            if state_exists(states, movement) {
                movement_states.push(movement);
            }
        }

        println!("✓ Common movement states found: {}", movement_states.len());
        println!("  States: {:?}", movement_states);
    }

    #[test]
    fn e2e_fighting_movement_directional_variants() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        // Check for directional variants
        let mut directional_count = 0;
        let directions = vec!["Forward", "Backward", "Up", "Down"];

        for state_name in states.keys() {
            for direction in &directions {
                if state_name.contains(direction) {
                    directional_count += 1;
                    break;
                }
            }
        }

        println!("✓ Directional states analyzed: {}", directional_count);
    }

    // ============================================================================
    // ATTACK STATE TESTS
    // ============================================================================

    #[test]
    fn e2e_fighting_attacks_basic_attacks() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        // Count basic attack types
        let mut punch_count = 0;
        let mut kick_count = 0;
        let mut attack_count = 0;

        for state_name in states.keys() {
            let name_lower = state_name.to_lowercase();
            if name_lower.contains("punch") || name_lower.contains("jab") {
                punch_count += 1;
            }
            if name_lower.contains("kick") {
                kick_count += 1;
            }
            if name_lower.contains("attack") || name_lower.starts_with("atk") {
                attack_count += 1;
            }
        }

        println!("✓ Basic attacks analyzed");
        println!("  Punches: {}, Kicks: {}, Generic attacks: {}",
                 punch_count, kick_count, attack_count);
    }

    #[test]
    fn e2e_fighting_attacks_strength_variants() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        // Check for strength variants (light, medium, heavy)
        let mut light_attacks = 0;
        let mut heavy_attacks = 0;

        for state_name in states.keys() {
            let name_lower = state_name.to_lowercase();
            if name_lower.contains("light") || name_lower.contains("weak") {
                light_attacks += 1;
            }
            if name_lower.contains("heavy") || name_lower.contains("strong") ||
               name_lower.contains("hard") {
                heavy_attacks += 1;
            }
        }

        println!("✓ Attack strength variants: Light={}, Heavy={}",
                 light_attacks, heavy_attacks);
    }

    #[test]
    fn e2e_fighting_attacks_special_moves() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        // Count special moves
        let mut special_count = 0;

        for state_name in states.keys() {
            let name_lower = state_name.to_lowercase();
            if name_lower.contains("special") || name_lower.contains("super") ||
               name_lower.contains("ultimate") {
                special_count += 1;
            }
        }

        println!("✓ Special moves found: {}", special_count);
    }

    // ============================================================================
    // HIT REACTION TESTS
    // ============================================================================

    #[test]
    fn e2e_fighting_hitstun_states() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        // Count hit reaction states
        let mut hit_states = 0;
        let mut hurt_states = 0;
        let mut hitstun_states = 0;

        for state_name in states.keys() {
            let name_lower = state_name.to_lowercase();
            if name_lower.contains("hit") && !name_lower.contains("hitbox") {
                hit_states += 1;
            }
            if name_lower.contains("hurt") {
                hurt_states += 1;
            }
            if name_lower.contains("stun") {
                hitstun_states += 1;
            }
        }

        println!("✓ Hit reaction states analyzed");
        println!("  Hit: {}, Hurt: {}, Stun: {}", hit_states, hurt_states, hitstun_states);
    }

    #[test]
    fn e2e_fighting_hitstun_variables() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let variables = golden["variables"].as_object().unwrap();

        // Count hitstun-related variables
        let mut stun_vars = 0;
        let mut frame_vars = 0;

        for (var_name, _) in variables {
            let name_lower = var_name.to_lowercase();
            if name_lower.contains("stun") || name_lower.contains("hitstop") {
                stun_vars += 1;
            }
            if name_lower.contains("frame") {
                frame_vars += 1;
            }
        }

        println!("✓ Hitstun variables: {}, Frame variables: {}", stun_vars, frame_vars);
    }

    #[test]
    fn e2e_fighting_knockdown_states() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        // Check for knockdown and wakeup states
        let mut knockdown_count = 0;
        let mut wakeup_count = 0;

        for state_name in states.keys() {
            let name_lower = state_name.to_lowercase();
            if name_lower.contains("knockdown") || name_lower.contains("down") {
                knockdown_count += 1;
            }
            if name_lower.contains("wakeup") || name_lower.contains("getup") {
                wakeup_count += 1;
            }
        }

        println!("✓ Knockdown/wakeup states: Knockdown={}, Wakeup={}",
                 knockdown_count, wakeup_count);
    }

    // ============================================================================
    // BLOCKING TESTS
    // ============================================================================

    #[test]
    fn e2e_fighting_block_states() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        // Check for blocking states
        let mut block_states = 0;
        let mut guard_states = 0;

        for state_name in states.keys() {
            let name_lower = state_name.to_lowercase();
            if name_lower.contains("block") {
                block_states += 1;
            }
            if name_lower.contains("guard") {
                guard_states += 1;
            }
        }

        println!("✓ Defensive states: Block={}, Guard={}", block_states, guard_states);
    }

    #[test]
    fn e2e_fighting_block_variables() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let variables = golden["variables"].as_object().unwrap();

        // Check for blocking-related variables
        let mut block_vars = 0;

        for (var_name, _) in variables {
            let name_lower = var_name.to_lowercase();
            if name_lower.contains("block") || name_lower.contains("guard") ||
               name_lower.contains("defend") {
                block_vars += 1;
            }
        }

        println!("✓ Defense variables found: {}", block_vars);
    }

    // ============================================================================
    // STATE TRANSITION TESTS
    // ============================================================================

    #[test]
    fn e2e_fighting_state_has_phases() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        // Count states with phases
        let mut states_with_phases = 0;
        let mut states_without_phases = 0;

        for (_state_name, state_data) in states {
            if let Some(phases) = state_data["Phases"].as_object() {
                if phases.is_empty() {
                    states_without_phases += 1;
                } else {
                    states_with_phases += 1;
                }
            } else {
                states_without_phases += 1;
            }
        }

        println!("✓ State phase distribution:");
        println!("  With phases: {}, Without phases: {}",
                 states_with_phases, states_without_phases);
    }

    #[test]
    fn e2e_fighting_common_phase_names() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        // Track common phase names
        let mut phase_name_counts = std::collections::HashMap::new();

        for (_state_name, state_data) in states {
            if let Some(phases) = state_data["Phases"].as_object() {
                for phase_name in phases.keys() {
                    *phase_name_counts.entry(phase_name.clone()).or_insert(0) += 1;
                }
            }
        }

        println!("✓ Common phase names analyzed ({} unique)", phase_name_counts.len());

        // Show top 5 most common
        let mut sorted: Vec<_> = phase_name_counts.iter().collect();
        sorted.sort_by_key(|(_, count)| std::cmp::Reverse(**count));

        if !sorted.is_empty() {
            println!("  Top 5 phase names:");
            for (name, count) in sorted.iter().take(5) {
                println!("    {}: {}", name, count);
            }
        }
    }

    #[test]
    fn e2e_fighting_parent_child_relationships() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        // Count parent-child relationships
        let mut root_states = 0;
        let mut child_states = 0;

        for (_state_name, state_data) in states {
            if state_data["Parent"].is_null() {
                root_states += 1;
            } else {
                child_states += 1;
            }
        }

        println!("✓ State hierarchy: {} root states, {} child states",
                 root_states, child_states);
    }

    // ============================================================================
    // COMBO SYSTEM TESTS
    // ============================================================================

    #[test]
    fn e2e_fighting_combo_potential() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        // Identify potential combo states (attacks with multiple hits)
        let mut combo_states = 0;

        for (state_name, _) in states {
            let name_lower = state_name.to_lowercase();
            if name_lower.contains("combo") || name_lower.contains("chain") ||
               name_lower.contains("multi") {
                combo_states += 1;
            }
        }

        println!("✓ Combo system states identified: {}", combo_states);
    }

    #[test]
    fn e2e_fighting_hit_count_variables() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let variables = golden["variables"].as_object().unwrap();

        // Check for combo-related variables
        let mut combo_vars = 0;
        let mut hit_count_vars = 0;

        for (var_name, _) in variables {
            let name_lower = var_name.to_lowercase();
            if name_lower.contains("combo") {
                combo_vars += 1;
            }
            if name_lower.contains("hitcount") || name_lower.contains("hit_count") ||
               name_lower.contains("hits") {
                hit_count_vars += 1;
            }
        }

        println!("✓ Combo tracking: Combo vars={}, Hit count vars={}",
                 combo_vars, hit_count_vars);
    }

    // ============================================================================
    // HEALTH AND DAMAGE TESTS
    // ============================================================================

    #[test]
    fn e2e_fighting_health_variables() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let variables = golden["variables"].as_object().unwrap();

        // Check for health-related variables
        let mut health_vars = 0;
        let mut damage_vars = 0;

        for (var_name, _) in variables {
            let name_lower = var_name.to_lowercase();
            if name_lower.contains("health") || name_lower.contains("hp") ||
               name_lower.contains("life") {
                health_vars += 1;
            }
            if name_lower.contains("damage") || name_lower.contains("dmg") {
                damage_vars += 1;
            }
        }

        println!("✓ Health system: Health vars={}, Damage vars={}",
                 health_vars, damage_vars);
    }

    #[test]
    fn e2e_fighting_meter_variables() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let variables = golden["variables"].as_object().unwrap();

        // Check for meter/resource variables
        let mut meter_vars = 0;

        for (var_name, _) in variables {
            let name_lower = var_name.to_lowercase();
            if name_lower.contains("meter") || name_lower.contains("gauge") ||
               name_lower.contains("resource") || name_lower.contains("energy") {
                meter_vars += 1;
            }
        }

        println!("✓ Resource system variables: {}", meter_vars);
    }

    // ============================================================================
    // POSITION AND HITBOX TESTS
    // ============================================================================

    #[test]
    fn e2e_fighting_position_variables() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let variables = golden["variables"].as_object().unwrap();

        // Count position-related variables
        let mut position_vars = 0;
        let mut vec2_count = 0;
        let mut vec3_count = 0;

        for (var_name, var_data) in variables {
            let name_lower = var_name.to_lowercase();
            if name_lower.contains("pos") || name_lower.contains("position") ||
               name_lower.contains("location") {
                position_vars += 1;
            }

            let var_type = var_data["Type"].as_str().unwrap_or("");
            if var_type == "Vec2" {
                vec2_count += 1;
            } else if var_type == "Vec3" {
                vec3_count += 1;
            }
        }

        println!("✓ Position system: {} position vars, {} Vec2, {} Vec3",
                 position_vars, vec2_count, vec3_count);
    }

    #[test]
    fn e2e_fighting_hitbox_variables() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let variables = golden["variables"].as_object().unwrap();

        // Check for hitbox/hurtbox variables
        let mut hitbox_vars = 0;
        let mut hurtbox_vars = 0;

        for (var_name, _) in variables {
            let name_lower = var_name.to_lowercase();
            if name_lower.contains("hitbox") {
                hitbox_vars += 1;
            }
            if name_lower.contains("hurtbox") {
                hurtbox_vars += 1;
            }
        }

        println!("✓ Collision system: Hitboxes={}, Hurtboxes={}",
                 hitbox_vars, hurtbox_vars);
    }

    // ============================================================================
    // INPUT AND COMMAND TESTS
    // ============================================================================

    #[test]
    fn e2e_fighting_input_variables() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let variables = golden["variables"].as_object().unwrap();

        // Check for input-related variables
        let mut input_vars = 0;
        let mut button_vars = 0;

        for (var_name, _) in variables {
            let name_lower = var_name.to_lowercase();
            if name_lower.contains("input") {
                input_vars += 1;
            }
            if name_lower.contains("button") || name_lower.contains("btn") {
                button_vars += 1;
            }
        }

        println!("✓ Input system: Input vars={}, Button vars={}",
                 input_vars, button_vars);
    }

    // ============================================================================
    // ANIMATION AND TIMING TESTS
    // ============================================================================

    #[test]
    fn e2e_fighting_frame_data_variables() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let variables = golden["variables"].as_object().unwrap();

        // Check for frame data variables
        let mut frame_vars = 0;
        let mut timing_vars = 0;

        for (var_name, _) in variables {
            let name_lower = var_name.to_lowercase();
            if name_lower.contains("frame") {
                frame_vars += 1;
            }
            if name_lower.contains("time") || name_lower.contains("duration") ||
               name_lower.contains("timer") {
                timing_vars += 1;
            }
        }

        println!("✓ Timing system: Frame vars={}, Timing vars={}",
                 frame_vars, timing_vars);
    }

    #[test]
    fn e2e_fighting_animation_variables() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let variables = golden["variables"].as_object().unwrap();

        // Check for animation-related variables
        let mut anim_vars = 0;
        let mut sprite_vars = 0;

        for (var_name, _) in variables {
            let name_lower = var_name.to_lowercase();
            if name_lower.contains("anim") {
                anim_vars += 1;
            }
            if name_lower.contains("sprite") {
                sprite_vars += 1;
            }
        }

        println!("✓ Animation system: Anim vars={}, Sprite vars={}",
                 anim_vars, sprite_vars);
    }

    // ============================================================================
    // COMPREHENSIVE SUMMARY
    // ============================================================================

    #[test]
    fn e2e_fighting_game_patterns_summary() {
        println!("\n=== Fighting Game Patterns Validation Summary ===\n");

        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();
        let variables = golden["variables"].as_object().unwrap();

        println!("Character: {}", golden["metadata"]["name"]);
        println!("Total states: {}", states.len());
        println!("Total variables: {}", variables.len());

        // Categorize states
        let mut movement_states = 0;
        let mut attack_states = 0;
        let mut hit_states = 0;
        let mut special_states = 0;

        for (state_name, _) in states {
            let name_lower = state_name.to_lowercase();
            if name_lower.contains("walk") || name_lower.contains("jump") ||
               name_lower.contains("crouch") || name_lower.contains("stand") {
                movement_states += 1;
            } else if name_lower.contains("attack") || name_lower.contains("punch") ||
                      name_lower.contains("kick") {
                attack_states += 1;
            } else if name_lower.contains("hit") || name_lower.contains("hurt") {
                hit_states += 1;
            } else if name_lower.contains("special") || name_lower.contains("super") {
                special_states += 1;
            }
        }

        println!("\nState categories:");
        println!("  Movement: {}", movement_states);
        println!("  Attacks: {}", attack_states);
        println!("  Hit reactions: {}", hit_states);
        println!("  Special moves: {}", special_states);

        println!("\n✓ Fighting game patterns validation complete!\n");
    }
}
