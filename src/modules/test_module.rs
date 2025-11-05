// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Test module - a simple example module to demonstrate the system

use crate::module::CastagneModule;
use crate::state_handle::CastagneStateHandle;
use godot::prelude::*;

/// A simple test module
pub struct TestModule {
    name: String,
}

impl TestModule {
    pub fn new() -> Self {
        Self {
            name: "TestModule".to_string(),
        }
    }
}

impl CastagneModule for TestModule {
    fn module_name(&self) -> &str {
        &self.name
    }

    fn module_setup(&mut self) {
        godot_print!("TestModule: Setup called");
    }

    fn battle_init(&mut self, state_handle: &mut CastagneStateHandle, _battle_init_data: &std::collections::HashMap<String, godot::prelude::Variant>) {
        godot_print!("TestModule: Battle init called");

        // Example: Set a global variable
        state_handle.memory().borrow_mut().global_set("TestVariable", godot::prelude::Variant::from(42), true);
    }

    fn frame_start(&mut self, _state_handle: &mut CastagneStateHandle) {
        // Called every frame
    }

    fn action_phase_start(&mut self, _state_handle: &mut CastagneStateHandle) {
        // Called at the start of the action phase
    }
}
