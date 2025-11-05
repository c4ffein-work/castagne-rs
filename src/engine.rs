// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Main Castagne Engine
//!
//! Orchestrates the game loop and manages the flow between different phases.

use crate::config::CastagneConfig;
use crate::memory::CastagneMemory;
use crate::state_handle::CastagneStateHandle;
use godot::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// Phase names in the engine
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    AI,
    Input,
    Init,
    Action,
    Subentity,
    Physics,
    Reaction,
    Freeze,
}

impl Phase {
    pub fn as_str(&self) -> &'static str {
        match self {
            Phase::AI => "AI",
            Phase::Input => "Input",
            Phase::Init => "Init",
            Phase::Action => "Action",
            Phase::Subentity => "Subentity",
            Phase::Physics => "Physics",
            Phase::Reaction => "Reaction",
            Phase::Freeze => "Freeze",
        }
    }
}

/// Main Castagne Engine
pub struct CastagneEngineCore {
    config: CastagneConfig,
    memory: Rc<RefCell<CastagneMemory>>,
    instanced_data: HashMap<String, Variant>,
    init_error: bool,
    run_automatically: bool,
    render_graphics: bool,
}

impl CastagneEngineCore {
    /// Create a new engine with config
    pub fn new(config: CastagneConfig) -> Self {
        Self {
            config,
            memory: Rc::new(RefCell::new(CastagneMemory::new())),
            instanced_data: HashMap::new(),
            init_error: false,
            run_automatically: true,
            render_graphics: true,
        }
    }

    /// Initialize the engine
    pub fn init(&mut self, battle_init_data: HashMap<String, Variant>) -> Result<(), String> {
        godot_print!("Castagne Engine: Init Started");

        // Reset state
        self.memory = Rc::new(RefCell::new(CastagneMemory::new()));
        self.memory.borrow_mut().init_memory();
        self.init_error = false;

        // Initialize instanced data
        self.instanced_data.insert("Players".to_string(), Variant::nil());
        self.instanced_data.insert("ParsedFighters".to_string(), Variant::nil());
        self.instanced_data.insert("Entities".to_string(), Variant::nil());

        if self.init_error {
            return Err("Initialization failed at map init stage".to_string());
        }

        // Initialize modules
        let mut state_handle = CastagneStateHandle::new(Rc::clone(&self.memory));

        for module in self.config.get_modules() {
            let mut module_write = module.borrow_mut();
            module_write.copy_variables_global(&mut self.memory.borrow_mut());
            module_write.battle_init(&mut state_handle, &battle_init_data);
        }

        if self.init_error {
            return Err("Initialization failed at fighter init stage".to_string());
        }

        godot_print!("Castagne Engine: Init Complete");
        Ok(())
    }

    /// Execute a single frame
    pub fn execute_frame(&mut self) -> Result<(), String> {
        let mut state_handle = CastagneStateHandle::new(Rc::clone(&self.memory));

        // Frame pre-start
        for module in self.config.get_modules() {
            module
                .borrow_mut()
                .frame_pre_start(&mut state_handle);
        }

        // Frame start
        for module in self.config.get_modules() {
            module.borrow_mut().frame_start(&mut state_handle);
        }

        // Get active entities (stub - normally loaded from memory)
        let active_entities: Vec<i32> = vec![]; // TODO: Get from memory GlobalGet("_ActiveEntities")

        // Execute phases
        self.execute_phase(Phase::AI, &active_entities, &mut state_handle)?;
        self.execute_phase(Phase::Input, &active_entities, &mut state_handle)?;
        self.execute_phase(Phase::Init, &active_entities, &mut state_handle)?;
        self.execute_phase(Phase::Action, &active_entities, &mut state_handle)?;
        self.execute_phase(Phase::Physics, &active_entities, &mut state_handle)?;
        self.execute_phase(Phase::Reaction, &active_entities, &mut state_handle)?;

        // Frame end
        for module in self.config.get_modules() {
            module.borrow_mut().frame_end(&mut state_handle);
        }

        Ok(())
    }

    /// Execute a specific phase
    fn execute_phase(
        &mut self,
        phase: Phase,
        eids: &[i32],
        state_handle: &mut CastagneStateHandle,
    ) -> Result<(), String> {
        state_handle.set_phase(phase.as_str());

        // Phase Start (global)
        for module in self.config.get_modules() {
            let mut module_write = module.borrow_mut();
            match phase {
                Phase::AI => module_write.ai_phase_start(state_handle),
                Phase::Input => module_write.input_phase_start(state_handle),
                Phase::Init => module_write.init_phase_start(state_handle),
                Phase::Action => module_write.action_phase_start(state_handle),
                Phase::Subentity => module_write.subentity_phase_start(state_handle),
                Phase::Physics => module_write.physics_phase_start(state_handle),
                Phase::Reaction => module_write.reaction_phase_start(state_handle),
                Phase::Freeze => module_write.freeze_phase_start(state_handle),
            }
        }

        // Phase Start Entity (per entity)
        for &eid in eids {
            state_handle.point_to_entity(eid);
            for module in self.config.get_modules() {
                let mut module_write = module.borrow_mut();
                match phase {
                    Phase::AI => module_write.ai_phase_start_entity(state_handle),
                    Phase::Input => module_write.input_phase_start_entity(state_handle),
                    Phase::Init => module_write.init_phase_start_entity(state_handle),
                    Phase::Action => module_write.action_phase_start_entity(state_handle),
                    Phase::Subentity => module_write.subentity_phase_start_entity(state_handle),
                    Phase::Physics => module_write.physics_phase_start_entity(state_handle),
                    Phase::Reaction => module_write.reaction_phase_start_entity(state_handle),
                    Phase::Freeze => module_write.freeze_phase_start_entity(state_handle),
                }
            }
        }

        // TODO: Execute entity scripts here
        // self.execute_entity_scripts(eids, state_handle)?;

        // Phase End Entity (per entity)
        for &eid in eids {
            state_handle.point_to_entity(eid);
            for module in self.config.get_modules() {
                let mut module_write = module.borrow_mut();
                match phase {
                    Phase::AI => module_write.ai_phase_end_entity(state_handle),
                    Phase::Input => module_write.input_phase_end_entity(state_handle),
                    Phase::Init => module_write.init_phase_end_entity(state_handle),
                    Phase::Action => module_write.action_phase_end_entity(state_handle),
                    Phase::Subentity => module_write.subentity_phase_end_entity(state_handle),
                    Phase::Physics => module_write.physics_phase_end_entity(state_handle),
                    Phase::Reaction => module_write.reaction_phase_end_entity(state_handle),
                    Phase::Freeze => module_write.freeze_phase_end_entity(state_handle),
                }
            }
        }

        // Phase End (global)
        for module in self.config.get_modules() {
            let mut module_write = module.borrow_mut();
            match phase {
                Phase::AI => module_write.ai_phase_end(state_handle),
                Phase::Input => module_write.input_phase_end(state_handle),
                Phase::Init => module_write.init_phase_end(state_handle),
                Phase::Action => module_write.action_phase_end(state_handle),
                Phase::Subentity => module_write.subentity_phase_end(state_handle),
                Phase::Physics => module_write.physics_phase_end(state_handle),
                Phase::Reaction => module_write.reaction_phase_end(state_handle),
                Phase::Freeze => module_write.freeze_phase_end(state_handle),
            }
        }

        Ok(())
    }

    /// Get a reference to the memory
    pub fn memory(&self) -> Rc<RefCell<CastagneMemory>> {
        Rc::clone(&self.memory)
    }

    /// Get a reference to the config
    pub fn config(&self) -> &CastagneConfig {
        &self.config
    }

    /// Get mutable reference to instanced data
    pub fn instanced_data(&mut self) -> &mut HashMap<String, Variant> {
        &mut self.instanced_data
    }
}
