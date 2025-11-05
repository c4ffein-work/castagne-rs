// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! CMCore - Castagne Core Module
//!
//! Contains the most basic functions and flow of the engine.
//! Provides essential variable manipulation, math, and entity management.

use crate::module::CastagneModule;
use crate::state_handle::CastagneStateHandle;
use crate::memory::CastagneMemory;
use godot::prelude::*;
use std::collections::HashMap;

/// Core module providing essential functionality
pub struct CoreModule {
    /// Variables to initialize for each entity
    variables_entity_init: HashMap<String, Variant>,
    /// Variables to reset each frame
    variables_entity_reset: HashMap<String, Variant>,
    /// Variables to initialize globally
    variables_global_init: HashMap<String, Variant>,
}

impl CoreModule {
    pub fn new() -> Self {
        let mut module = Self {
            variables_entity_init: HashMap::new(),
            variables_entity_reset: HashMap::new(),
            variables_global_init: HashMap::new(),
        };

        // Register core entity variables
        module.register_variable_entity("_Flags", Variant::from(PackedStringArray::new()), true);
        module.register_variable_entity("_FlagsNext", Variant::from(PackedStringArray::new()), false);
        module.register_variable_entity("_State", Variant::from(""), false);
        module.register_variable_entity("_StateTransition", Variant::from(false), true);
        module.register_variable_entity("_EID", Variant::from(-1), false);
        module.register_variable_entity("_Player", Variant::from(-1), false);
        module.register_variable_entity("_TargetEID", Variant::from(-1), false);
        module.register_variable_entity("_FreezeFrames", Variant::from(0), false);
        module.register_variable_entity("_HaltFrames", Variant::from(0), false);
        module.register_variable_entity("_Entity", Variant::nil(), false);

        // Register global variables
        module.register_variable_global("_FrameID", Variant::from(0), false);
        module.register_variable_global("_TrueFrameID", Variant::from(0), false);
        module.register_variable_global("_ActiveEntities", Variant::from(PackedInt32Array::new()), false);
        module.register_variable_global("_ActiveFullEntities", Variant::from(PackedInt32Array::new()), false);
        module.register_variable_global("_ActiveSubentities", Variant::from(PackedInt32Array::new()), false);
        module.register_variable_global("_EntitiesToInit", Variant::from(PackedInt32Array::new()), false);
        module.register_variable_global("_SubentitiesToInit", Variant::from(PackedInt32Array::new()), false);
        module.register_variable_global("_EntitiesToDestroy", Variant::from(PackedInt32Array::new()), false);
        module.register_variable_global("_SkipFrame", Variant::from(false), false);

        module
    }

    fn register_variable_entity(&mut self, name: &str, default_value: Variant, reset_each_frame: bool) {
        self.variables_entity_init.insert(name.to_string(), default_value.clone());
        if reset_each_frame {
            self.variables_entity_reset.insert(name.to_string(), default_value);
        }
    }

    fn register_variable_global(&mut self, name: &str, default_value: Variant, reset_each_frame: bool) {
        self.variables_global_init.insert(name.to_string(), default_value.clone());
        // Global reset variables would go here if needed
    }
}

impl CastagneModule for CoreModule {
    fn module_name(&self) -> &str {
        "Core"
    }

    fn module_slot(&self) -> Option<&str> {
        Some("Core")
    }

    fn module_setup(&mut self) {
        godot_print!("Core Module: Setup");
    }

    fn copy_variables_global(&mut self, memory: &mut CastagneMemory) {
        for (key, value) in &self.variables_global_init {
            memory.global_set(key, value.clone(), true);
        }
    }

    fn copy_variables_entity(&mut self, state_handle: &mut CastagneStateHandle, new_entity: bool) {
        for (key, value) in &self.variables_entity_init {
            if new_entity {
                // Only set if doesn't exist
                if !state_handle.entity_has(key) {
                    state_handle.entity_set(key, value.clone());
                }
            } else {
                state_handle.entity_set(key, value.clone());
            }
        }
    }

    fn reset_variables(&mut self, state_handle: &mut CastagneStateHandle, eids: &[i32]) {
        for &eid in eids {
            state_handle.point_to_entity(eid);
            for (key, value) in &self.variables_entity_reset {
                state_handle.entity_set(key, value.clone());
            }
        }
    }

    fn action_phase_start(&mut self, state_handle: &mut CastagneStateHandle) {
        // Process flags for next frame
        // TODO: Implement flag processing
    }

    fn action_phase_start_entity(&mut self, state_handle: &mut CastagneStateHandle) {
        // Apply FlagsNext to Flags
        if let Some(flags_next) = state_handle.entity_get("_FlagsNext") {
            if let Ok(flags_next_array) = flags_next.try_to::<PackedStringArray>() {
                if let Some(current_flags) = state_handle.entity_get("_Flags") {
                    if let Ok(mut flags_array) = current_flags.try_to::<PackedStringArray>() {
                        // Merge FlagsNext into Flags
                        for i in 0..flags_next_array.len() {
                            if let Some(flag) = flags_next_array.get(i) {
                                flags_array.push(flag.to_string().as_str());
                            }
                        }
                        state_handle.entity_set("_Flags", Variant::from(flags_array));
                    }
                }
                // Clear FlagsNext
                state_handle.entity_set("_FlagsNext", Variant::from(PackedStringArray::new()));
            }
        }
    }
}

impl Default for CoreModule {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::CastagneMemory;
    use std::rc::Rc;
    use std::cell::RefCell;

    #[test]
    fn test_core_module_creation() {
        let module = CoreModule::new();
        assert_eq!(module.module_name(), "Core");
        assert_eq!(module.module_slot(), Some("Core"));
    }

    #[test]
    fn test_core_module_variables() {
        let mut module = CoreModule::new();
        let mut memory = CastagneMemory::new();

        // Initialize global variables
        module.copy_variables_global(&mut memory);

        // Check that global variables were set
        assert!(memory.global_has("_FrameID"));
        assert!(memory.global_has("_ActiveEntities"));
    }

    #[test]
    fn test_entity_variables() {
        let mut module = CoreModule::new();
        let memory = Rc::new(RefCell::new(CastagneMemory::new()));
        memory.borrow_mut().init_memory();

        // Add an entity
        let eid = memory.borrow_mut().add_entity();

        let mut state_handle = CastagneStateHandle::new(Rc::clone(&memory));
        state_handle.point_to_entity(eid as i32);

        // Copy entity variables
        module.copy_variables_entity(&mut state_handle, true);

        // Check that entity variables were set
        assert!(state_handle.entity_has("_Flags"));
        assert!(state_handle.entity_has("_State"));
        assert!(state_handle.entity_has("_EID"));
    }

    #[test]
    fn test_reset_variables() {
        let mut module = CoreModule::new();
        let memory = Rc::new(RefCell::new(CastagneMemory::new()));
        memory.borrow_mut().init_memory();

        let eid = memory.borrow_mut().add_entity();
        let mut state_handle = CastagneStateHandle::new(Rc::clone(&memory));
        state_handle.point_to_entity(eid as i32);

        // Initialize and then modify a reset variable
        module.copy_variables_entity(&mut state_handle, true);
        let mut flags = PackedStringArray::new();
        flags.push("TestFlag");
        state_handle.entity_set("_Flags", Variant::from(flags));

        // Reset variables
        module.reset_variables(&mut state_handle, &[eid as i32]);

        // Check that _Flags was reset (it has ResetEachFrame flag)
        if let Some(reset_flags) = state_handle.entity_get("_Flags") {
            if let Ok(flags_array) = reset_flags.try_to::<PackedStringArray>() {
                assert_eq!(flags_array.len(), 0, "Flags should be reset to empty array");
            }
        }
    }

    #[test]
    fn test_flags_next_processing() {
        let mut module = CoreModule::new();
        let memory = Rc::new(RefCell::new(CastagneMemory::new()));
        memory.borrow_mut().init_memory();

        let eid = memory.borrow_mut().add_entity();
        let mut state_handle = CastagneStateHandle::new(Rc::clone(&memory));
        state_handle.point_to_entity(eid as i32);

        module.copy_variables_entity(&mut state_handle, true);

        // Add a flag to FlagsNext
        let mut flags_next = PackedStringArray::new();
        flags_next.push("NextFrameFlag");
        state_handle.entity_set("_FlagsNext", Variant::from(flags_next));

        // Run action phase to process flags
        module.action_phase_start_entity(&mut state_handle);

        // Check that flag was moved to Flags
        if let Some(flags) = state_handle.entity_get("_Flags") {
            if let Ok(flags_array) = flags.try_to::<PackedStringArray>() {
                assert!(flags_array.len() > 0, "Flag should be in Flags array");
            }
        }

        // Check that FlagsNext was cleared
        if let Some(flags_next) = state_handle.entity_get("_FlagsNext") {
            if let Ok(flags_next_array) = flags_next.try_to::<PackedStringArray>() {
                assert_eq!(flags_next_array.len(), 0, "FlagsNext should be cleared");
            }
        }
    }
}
