// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Module system for Castagne
//!
//! Modules extend the engine with new functionality and can hook into
//! various phases of the game loop.

use crate::state_handle::CastagneStateHandle;
use crate::memory::CastagneMemory;
use godot::prelude::*;
use std::collections::HashMap;

/// Base trait for all Castagne modules
pub trait CastagneModule: Send + Sync {
    /// Get the module name
    fn module_name(&self) -> &str;

    /// Get the module slot (for base system modules like Input, Physics, etc.)
    fn module_slot(&self) -> Option<&str> {
        None
    }

    // ============================================================
    // Lifecycle Callbacks
    // ============================================================

    /// Called when the program starts, on loading
    fn module_setup(&mut self) {}

    /// Called when a config data registers this module
    fn on_module_registration(&mut self, _config_data: &HashMap<String, Variant>) {}

    /// Called at the beginning of a battle
    fn battle_init(&mut self, _state_handle: &mut CastagneStateHandle, _battle_init_data: &HashMap<String, Variant>) {}

    /// Called at the beginning of a battle (late phase)
    fn battle_init_late(&mut self, _state_handle: &mut CastagneStateHandle, _battle_init_data: &HashMap<String, Variant>) {}

    /// Called at the beginning of each frame, before choosing the loop
    fn frame_pre_start(&mut self, _state_handle: &mut CastagneStateHandle) {}

    /// Called at the beginning of each frame
    fn frame_start(&mut self, _state_handle: &mut CastagneStateHandle) {}

    /// Called at the end of each frame
    fn frame_end(&mut self, _state_handle: &mut CastagneStateHandle) {}

    // ============================================================
    // Phase Callbacks
    // ============================================================
    // Each phase can have Start/StartEntity/EndEntity/End callbacks
    // The engine will call them in this order:
    // 1. PhaseStart (global)
    // 2. PhaseStartEntity (per entity)
    // 3. Entity script execution
    // 4. PhaseEndEntity (per entity)
    // 5. PhaseEnd (global)
    // ============================================================

    // AI Phase
    fn ai_phase_start(&mut self, _state_handle: &mut CastagneStateHandle) {}
    fn ai_phase_start_entity(&mut self, _state_handle: &mut CastagneStateHandle) {}
    fn ai_phase_end_entity(&mut self, _state_handle: &mut CastagneStateHandle) {}
    fn ai_phase_end(&mut self, _state_handle: &mut CastagneStateHandle) {}

    // Input Phase
    fn input_phase_start(&mut self, _state_handle: &mut CastagneStateHandle) {}
    fn input_phase_start_entity(&mut self, _state_handle: &mut CastagneStateHandle) {}
    fn input_phase_end_entity(&mut self, _state_handle: &mut CastagneStateHandle) {}
    fn input_phase_end(&mut self, _state_handle: &mut CastagneStateHandle) {}

    // Init Phase
    fn init_phase_start(&mut self, _state_handle: &mut CastagneStateHandle) {}
    fn init_phase_start_entity(&mut self, _state_handle: &mut CastagneStateHandle) {}
    fn init_phase_end_entity(&mut self, _state_handle: &mut CastagneStateHandle) {}
    fn init_phase_end(&mut self, _state_handle: &mut CastagneStateHandle) {}

    // Action Phase
    fn action_phase_start(&mut self, _state_handle: &mut CastagneStateHandle) {}
    fn action_phase_start_entity(&mut self, _state_handle: &mut CastagneStateHandle) {}
    fn action_phase_end_entity(&mut self, _state_handle: &mut CastagneStateHandle) {}
    fn action_phase_end(&mut self, _state_handle: &mut CastagneStateHandle) {}

    // Subentity Phase (lightweight phase for subentities)
    fn subentity_phase_start(&mut self, state_handle: &mut CastagneStateHandle) {
        self.action_phase_start(state_handle)
    }
    fn subentity_phase_start_entity(&mut self, state_handle: &mut CastagneStateHandle) {
        self.action_phase_start_entity(state_handle)
    }
    fn subentity_phase_end_entity(&mut self, state_handle: &mut CastagneStateHandle) {
        self.action_phase_end_entity(state_handle)
    }
    fn subentity_phase_end(&mut self, state_handle: &mut CastagneStateHandle) {
        self.action_phase_end(state_handle)
    }

    // Physics Phase
    fn physics_phase_start(&mut self, _state_handle: &mut CastagneStateHandle) {}
    fn physics_phase_start_entity(&mut self, _state_handle: &mut CastagneStateHandle) {}
    fn physics_phase_end_entity(&mut self, _state_handle: &mut CastagneStateHandle) {}
    fn physics_phase_end(&mut self, _state_handle: &mut CastagneStateHandle) {}

    // Reaction Phase
    fn reaction_phase_start(&mut self, _state_handle: &mut CastagneStateHandle) {}
    fn reaction_phase_start_entity(&mut self, _state_handle: &mut CastagneStateHandle) {}
    fn reaction_phase_end_entity(&mut self, _state_handle: &mut CastagneStateHandle) {}
    fn reaction_phase_end(&mut self, _state_handle: &mut CastagneStateHandle) {}

    // Freeze Phase (when entities are frozen)
    fn freeze_phase_start(&mut self, _state_handle: &mut CastagneStateHandle) {}
    fn freeze_phase_start_entity(&mut self, _state_handle: &mut CastagneStateHandle) {}
    fn freeze_phase_end_entity(&mut self, _state_handle: &mut CastagneStateHandle) {}
    fn freeze_phase_end(&mut self, _state_handle: &mut CastagneStateHandle) {}

    // ============================================================
    // Other Callbacks
    // ============================================================

    /// Called when an entity transitions state
    fn on_state_transition_entity(
        &mut self,
        _state_handle: &mut CastagneStateHandle,
        _previous_state: &str,
        _new_state: &str,
    ) {}

    /// Called on each graphical frame to keep the display up to date
    fn update_graphics(&mut self, _state_handle: &mut CastagneStateHandle) {}

    // ============================================================
    // Variable Management
    // ============================================================

    /// Copy global variables to memory
    fn copy_variables_global(&mut self, _memory: &mut CastagneMemory) {}

    /// Copy player variables to memory
    fn copy_variables_player(&mut self, _memory: &mut CastagneMemory, _pid: usize) {}

    /// Copy entity variables to memory
    fn copy_variables_entity(&mut self, _state_handle: &mut CastagneStateHandle, _new_entity: bool) {}

    /// Reset per-frame variables
    fn reset_variables(&mut self, _state_handle: &mut CastagneStateHandle, _eids: &[i32]) {}
}

/// Helper struct to store module variables
#[derive(Clone)]
pub struct ModuleVariable {
    pub name: String,
    pub default_value: Variant,
    pub flags: Vec<String>,
    pub reset_each_frame: bool,
}

/// Helper struct to store module functions (for the parser/script system)
#[derive(Clone)]
pub struct ModuleFunction {
    pub name: String,
    pub arg_count: Vec<usize>, // e.g., [2, 3] means accepts 2 or 3 arguments
    pub description: String,
}
