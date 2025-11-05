// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Configuration and module management for Castagne

use crate::module::CastagneModule;
use godot::prelude::*;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

/// Configuration data for the Castagne engine
pub struct CastagneConfig {
    config_data: HashMap<String, Variant>,
    default_config_data: HashMap<String, Variant>,
    modules: Vec<Rc<RefCell<dyn CastagneModule>>>,
    module_slots: HashMap<String, usize>, // slot name -> module index
}

impl CastagneConfig {
    /// Create a new empty config
    pub fn new() -> Self {
        Self {
            config_data: HashMap::new(),
            default_config_data: HashMap::new(),
            modules: Vec::new(),
            module_slots: HashMap::new(),
        }
    }

    // ============================================================
    // Config Data Access
    // ============================================================

    /// Get a config value
    pub fn get(&self, key: &str) -> Option<Variant> {
        self.config_data.get(key).cloned()
    }

    /// Get a config value or return a default
    pub fn get_or(&self, key: &str, default: Variant) -> Variant {
        self.config_data.get(key).cloned().unwrap_or(default)
    }

    /// Set a config value
    pub fn set(&mut self, key: &str, value: Variant, new_value: bool) {
        if new_value || self.config_data.contains_key(key) {
            self.config_data.insert(key.to_string(), value);
        } else {
            godot_error!("Config Set: Key doesn't already exist: {}", key);
        }
    }

    /// Check if a config key exists
    pub fn has(&self, key: &str) -> bool {
        self.config_data.contains_key(key)
    }

    /// Get all config keys
    pub fn get_config_keys(&self) -> Vec<String> {
        self.config_data.keys().cloned().collect()
    }

    // ============================================================
    // Module Management
    // ============================================================

    /// Register a module
    pub fn register_module(&mut self, module: Rc<RefCell<dyn CastagneModule>>) {
        let module_borrow = module.borrow();
        let module_name = module_borrow.module_name().to_string();
        let module_slot = module_borrow.module_slot().map(|s| s.to_string());
        drop(module_borrow);

        godot_print!("Registering module: {}", module_name);

        let module_index = self.modules.len();
        self.modules.push(module);

        // Register slot if present
        if let Some(slot) = module_slot {
            self.module_slots.insert(slot, module_index);
        }
    }

    /// Get all modules
    pub fn get_modules(&self) -> &Vec<Rc<RefCell<dyn CastagneModule>>> {
        &self.modules
    }

    /// Get a module by slot name
    pub fn get_module_slot(&self, slot: &str) -> Option<Rc<RefCell<dyn CastagneModule>>> {
        self.module_slots
            .get(slot)
            .and_then(|&idx| self.modules.get(idx))
            .cloned()
    }

    /// Initialize all modules
    pub fn init_modules(&mut self) {
        for module in &self.modules {
            let mut module_borrow = module.borrow_mut();
            module_borrow.module_setup();
            module_borrow.on_module_registration(&self.config_data);
        }
    }

    // ============================================================
    // Config File Loading (stub for now)
    // ============================================================

    /// Load config from a JSON file
    pub fn load_from_config_file(&mut self, _path: &str) -> Result<(), String> {
        // TODO: Implement JSON loading
        godot_warn!("Config file loading not yet implemented");
        Ok(())
    }

    /// Save config to a JSON file
    pub fn save_config_file(&self, _path: &str) -> Result<(), String> {
        // TODO: Implement JSON saving
        godot_warn!("Config file saving not yet implemented");
        Ok(())
    }
}

impl Default for CastagneConfig {
    fn default() -> Self {
        Self::new()
    }
}
