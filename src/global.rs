// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! CastagneGlobal - Global utilities and helpers
//!
//! This module provides global utility functions, constants, and helpers
//! that are used throughout the Castagne engine.

use godot::prelude::*;
use std::collections::HashMap;

/// Hit confirmation types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HitConfirmed {
    None,
    Block,
    Hit,
    Clash,
}

/// State types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateType {
    Normal,
    BaseState,
    Helper,
    Special,
    Specblock,
}

/// Variable mutability
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VariableMutability {
    Variable,
    Define,
    Internal,
}

/// Variable types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VariableType {
    Int,
    Str,
    Var,
    Vec2,
    Vec3,
    Box,
    Bool,
}

/// Memory stacks
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryStack {
    Global,
    Player,
    Entity,
}

/// Module slots (for module priority/ordering)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModuleSlot {
    Core,
    Physics,
    Attacks,
    Graphics,
    Flow,
    Editor,
    Input,
    AI,
    Custom1,
    Custom2,
    Custom3,
}

/// Input device types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputDeviceType {
    Empty,
    Keyboard,
    Controller,
    AI,
}

/// Game modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameMode {
    Battle,
    Training,
    Editor,
}

/// Physics spaces
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicsSpace {
    World,
    Absolute,
    Entity,
}

/// Proration scale constant
pub const PRORATION_SCALE: i32 = 1000;

/// Version information
#[derive(Debug, Clone)]
pub struct VersionInfo {
    pub version: Option<String>,
    pub version_name: String,
    pub tldr: String,
    pub changelog: String,
    pub branch: String,
}

impl Default for VersionInfo {
    fn default() -> Self {
        Self {
            version: Some("0.1.0".to_string()),
            version_name: "Castagne-RS Experimental".to_string(),
            tldr: "Rust port of Castagne".to_string(),
            changelog: "Initial port".to_string(),
            branch: "main".to_string(),
        }
    }
}

/// CastagneGlobal - Global utilities and state
///
/// This struct provides utility functions and manages global state
/// similar to the autoload singleton in GDScript.
pub struct CastagneGlobal {
    version_info: VersionInfo,
}

impl CastagneGlobal {
    /// Create a new CastagneGlobal instance
    pub fn new() -> Self {
        Self {
            version_info: VersionInfo::default(),
        }
    }

    /// Get version information
    pub fn get_version_info(&self) -> &VersionInfo {
        &self.version_info
    }

    /// Log a message
    pub fn log(&self, message: &str) {
        godot_print!("[Castagne] {}", message);
    }

    /// Log an error
    pub fn error(&self, message: &str) {
        godot_error!("[Castagne] ! {}", message);
    }

    // -------------------------------------------------------------------------
    // Helper functions (matching GDScript implementation)

    /// Check if entity has a flag
    pub fn has_flag(flag_name: &str, flags: &[String]) -> bool {
        flags.contains(&flag_name.to_string())
    }

    /// Set a flag (add if not present)
    pub fn set_flag(flag_name: &str, flags: &mut Vec<String>) {
        if !Self::has_flag(flag_name, flags) {
            flags.push(flag_name.to_string());
        }
    }

    /// Unset a flag (remove if present)
    pub fn unset_flag(flag_name: &str, flags: &mut Vec<String>) {
        flags.retain(|f| f != flag_name);
    }

    /// Get integer value from string or variable
    pub fn get_int(value: &str, variables: &HashMap<String, i32>) -> i32 {
        // Try to parse as integer first
        if let Ok(int_val) = value.parse::<i32>() {
            return int_val;
        }

        // Otherwise look it up as a variable
        if let Some(&var_val) = variables.get(value) {
            return var_val;
        }

        godot_error!("GetInt: not a correct value: {}", value);
        0
    }

    /// Get string value
    pub fn get_str(value: &str) -> String {
        value.to_string()
    }

    /// Get string from variable or literal
    pub fn get_str_var(value: &str, variables: &HashMap<String, String>) -> String {
        if let Some(var_val) = variables.get(value) {
            var_val.clone()
        } else {
            value.to_string()
        }
    }

    /// Get boolean value
    pub fn get_bool(value: &str, variables: &HashMap<String, i32>) -> bool {
        Self::get_int(value, variables) > 0
    }

    // -------------------------------------------------------------------------
    // Data fusion utilities

    /// Fuse data with overwrite (additionalDict overwrites baseDict)
    pub fn fuse_data_overwrite<T: Clone>(base_dict: &mut HashMap<String, T>, additional_dict: &HashMap<String, T>) {
        for (key, value) in additional_dict {
            base_dict.insert(key.clone(), value.clone());
        }
    }

    /// Fuse data without overwrite (only add keys that don't exist)
    pub fn fuse_data_no_overwrite<T: Clone>(base_dict: &mut HashMap<String, T>, additional_dict: &HashMap<String, T>) {
        for (key, value) in additional_dict {
            if !base_dict.contains_key(key) {
                base_dict.insert(key.clone(), value.clone());
            }
        }
    }

    /// Fuse data with prefix move (move existing keys with prefix, then add new)
    pub fn fuse_data_move_with_prefix<T: Clone>(
        base_dict: &mut HashMap<String, T>,
        additional_dict: &HashMap<String, T>,
        prefix: &str,
    ) {
        for (key, value) in additional_dict {
            // Find all keys that need to be moved
            let mut key_name = key.clone();
            let mut keys_to_move = Vec::new();

            while base_dict.contains_key(&key_name) {
                keys_to_move.push(key_name.clone());
                key_name = format!("{}{}", prefix, key_name);
            }

            // Move keys in reverse order
            for k in keys_to_move.iter().rev() {
                if let Some(v) = base_dict.get(k).cloned() {
                    base_dict.insert(format!("{}{}", prefix, k), v);
                }
            }

            // Insert new value
            base_dict.insert(key.clone(), value.clone());
        }
    }

    /// Check if two HashMaps are equal (deep comparison)
    pub fn are_dictionaries_equal<T: PartialEq>(a: &HashMap<String, T>, b: &HashMap<String, T>) -> bool {
        if a.len() != b.len() {
            return false;
        }

        for (key, value_a) in a {
            match b.get(key) {
                Some(value_b) if value_a == value_b => continue,
                _ => return false,
            }
        }

        true
    }

    /// Check if two arrays are equal
    pub fn are_arrays_equal<T: PartialEq>(a: &[T], b: &[T]) -> bool {
        if a.len() != b.len() {
            return false;
        }

        a.iter().zip(b.iter()).all(|(a_val, b_val)| a_val == b_val)
    }

    /// Split string to array (comma-separated by default)
    pub fn split_string_to_array(string_to_separate: &str, separator: &str) -> Vec<String> {
        if string_to_separate.is_empty() {
            return Vec::new();
        }

        string_to_separate
            .split(separator)
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }

    // -------------------------------------------------------------------------
    // Battle init data helpers

    /// Get player data from battle init data
    pub fn battle_init_data_get_player(
        battle_init_data: &HashMap<String, HashMap<String, String>>,
        pid: usize,
    ) -> Option<HashMap<String, String>> {
        // TODO: Implement proper battle init data structure
        // This is a simplified version
        battle_init_data.get(&format!("player_{}", pid)).cloned()
    }

    /// Get entity data from battle init data
    pub fn battle_init_data_get_entity(
        battle_init_data: &HashMap<String, HashMap<String, String>>,
        pid: usize,
        eid: usize,
    ) -> Option<HashMap<String, String>> {
        // TODO: Implement proper battle init data structure
        battle_init_data.get(&format!("player_{}_entity_{}", pid, eid)).cloned()
    }

    /// Get value from battle init data
    pub fn battle_init_data_get_value(
        battle_init_data: &HashMap<String, HashMap<String, String>>,
        value_name: &str,
        pid: Option<usize>,
        eid: Option<usize>,
    ) -> Option<String> {
        match (pid, eid) {
            (None, None) => {
                // Global value
                battle_init_data.get("global").and_then(|g| g.get(value_name).cloned())
            }
            (Some(pid), None) => {
                // Player value
                Self::battle_init_data_get_player(battle_init_data, pid)
                    .and_then(|p| p.get(value_name).cloned())
            }
            (Some(pid), Some(eid)) => {
                // Entity value
                Self::battle_init_data_get_entity(battle_init_data, pid, eid)
                    .and_then(|e| e.get(value_name).cloned())
            }
            (None, Some(_)) => {
                // Invalid: can't have EID without PID
                None
            }
        }
    }
}

impl Default for CastagneGlobal {
    fn default() -> Self {
        Self::new()
    }
}

// -----------------------------------------------------------------------------
// Module loading (TODO: Implement when module system is more complete)

/// Module loader
pub struct ModuleLoader {
    modules_loaded: HashMap<String, String>, // path -> name
}

impl ModuleLoader {
    pub fn new() -> Self {
        Self {
            modules_loaded: HashMap::new(),
        }
    }

    /// Load modules from a config key
    pub fn load_modules(&mut self, _base_path: &str) -> Result<Vec<String>, String> {
        // TODO: Implement module loading
        // This would:
        // 1. Parse module list from config
        // 2. Load each module (either .tscn or .gd)
        // 3. Call ModuleSetup() on each
        // 4. Return list of loaded module names
        Ok(Vec::new())
    }

    /// Load a single module
    pub fn load_single_module(&mut self, _module_path: &str) -> Result<String, String> {
        // TODO: Implement single module loading
        Ok(String::new())
    }

    /// Get list of loaded modules
    pub fn get_loaded_modules(&self) -> &HashMap<String, String> {
        &self.modules_loaded
    }
}

impl Default for ModuleLoader {
    fn default() -> Self {
        Self::new()
    }
}
