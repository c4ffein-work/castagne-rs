// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! State Handle - Context-aware wrapper around memory
//!
//! Provides convenient access to memory with automatic context (current entity, player, phase).

use crate::memory::{CastagneMemory, MemoryValue};
use godot::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;

/// State handle provides context-aware access to game state
pub struct CastagneStateHandle {
    memory: Rc<RefCell<CastagneMemory>>,
    eid: i32,
    target_eid: i32,
    pid: i32,
    phase: String,
}

impl CastagneStateHandle {
    /// Create a new state handle
    pub fn new(memory: Rc<RefCell<CastagneMemory>>) -> Self {
        Self {
            memory,
            eid: -1,
            target_eid: -1,
            pid: -1,
            phase: "None".to_string(),
        }
    }

    /// Clone this state handle (shallow copy, shares memory reference)
    pub fn clone_handle(&self) -> Self {
        Self {
            memory: Rc::clone(&self.memory),
            eid: self.eid,
            target_eid: self.target_eid,
            pid: self.pid,
            phase: self.phase.clone(),
        }
    }

    // ============================================================
    // Context Management
    // ============================================================

    /// Point to a specific entity
    pub fn point_to_entity(&mut self, eid: i32) -> bool {
        self.eid = eid;
        let valid_eid = self.memory.borrow().is_eid_valid(eid);

        if !valid_eid {
            return false;
        }

        if eid >= 0 {
            // Get the player from the entity
            if let Some(player_variant) = self.entity_get("_Player") {
                if let Ok(player) = player_variant.try_to::<i32>() {
                    self.point_to_player(player);
                }
            }
        } else {
            self.pid = -1;
        }

        self.recall_target_entity();
        true
    }

    /// Point to a specific player
    pub fn point_to_player(&mut self, pid: i32) {
        self.pid = pid;
    }

    /// Get current player ID
    pub fn get_player(&self) -> i32 {
        self.pid
    }

    /// Point to the player's main entity
    pub fn point_to_player_main_entity(&mut self, pid: i32) -> bool {
        // Extract the value first to avoid borrow conflicts
        let main_entity_opt = self.memory.borrow().player_get(pid as usize, "MainEntity");

        if let Some(main_entity) = main_entity_opt {
            if let Ok(eid) = main_entity.try_to::<i32>() {
                return self.point_to_entity(eid);
            }
        }
        false
    }

    /// Get current entity ID
    pub fn get_entity_id(&self) -> i32 {
        self.eid
    }

    /// Get target entity ID
    pub fn get_target_eid(&self) -> i32 {
        self.target_eid
    }

    /// Set the target entity
    pub fn set_target_entity(&mut self, target_eid: i32) {
        let mem = self.memory.borrow();
        let entity_count = mem.memory_entities.len() as i32;
        drop(mem);

        let mut final_eid = target_eid;
        if target_eid < 0 || target_eid >= entity_count {
            final_eid = self.eid;
        }

        self.target_eid = final_eid;

        if self.entity_has("_TargetEID") {
            self.entity_set("_TargetEID", Variant::from(final_eid));
        }
    }

    /// Recall the target entity from memory
    pub fn recall_target_entity(&mut self) {
        if self.entity_has("_TargetEID") {
            if let Some(target) = self.entity_get("_TargetEID") {
                if let Ok(eid) = target.try_to::<i32>() {
                    self.set_target_entity(eid);
                }
            }
        }
    }

    /// Get current phase
    pub fn get_phase(&self) -> &str {
        &self.phase
    }

    /// Set current phase
    pub fn set_phase(&mut self, phase: &str) {
        self.phase = phase.to_string();
    }

    // ============================================================
    // Global Memory Access
    // ============================================================

    pub fn global_get(&self, key: &str) -> Option<MemoryValue> {
        self.memory.borrow().global_get(key)
    }

    pub fn global_set(&mut self, key: &str, value: MemoryValue) {
        self.memory.borrow_mut().global_set(key, value, false);
    }

    pub fn global_has(&self, key: &str) -> bool {
        self.memory.borrow().global_has(key)
    }

    pub fn global_add(&mut self, key: &str, value: i32) {
        if let Some(current) = self.global_get(key) {
            if let Ok(current_val) = current.try_to::<i32>() {
                self.global_set(key, Variant::from(current_val + value));
            }
        }
    }

    // ============================================================
    // Player Memory Access (uses current pid)
    // ============================================================

    pub fn player_get(&self, key: &str) -> Option<MemoryValue> {
        if self.pid < 0 {
            return None;
        }
        self.memory.borrow().player_get(self.pid as usize, key)
    }

    pub fn player_set(&mut self, key: &str, value: MemoryValue) {
        if self.pid < 0 {
            return;
        }
        self.memory.borrow_mut().player_set(self.pid as usize, key, value, false);
    }

    pub fn player_has(&self, key: &str) -> bool {
        if self.pid < 0 {
            return false;
        }
        self.memory.borrow().player_has(self.pid as usize, key)
    }

    pub fn player_add(&mut self, key: &str, value: i32) {
        if let Some(current) = self.player_get(key) {
            if let Ok(current_val) = current.try_to::<i32>() {
                self.player_set(key, Variant::from(current_val + value));
            }
        }
    }

    // ============================================================
    // Entity Memory Access (uses current eid)
    // ============================================================

    pub fn entity_get(&self, key: &str) -> Option<MemoryValue> {
        self.memory.borrow().entity_get(self.eid, key)
    }

    pub fn entity_set(&mut self, key: &str, value: MemoryValue) {
        self.memory.borrow_mut().entity_set(self.eid, key, value, false);
    }

    pub fn entity_has(&self, key: &str) -> bool {
        self.memory.borrow().entity_has(self.eid, key)
    }

    pub fn entity_add(&mut self, key: &str, value: i32) {
        if let Some(current) = self.entity_get(key) {
            if let Ok(current_val) = current.try_to::<i32>() {
                self.entity_set(key, Variant::from(current_val + value));
            }
        }
    }

    // ============================================================
    // Target Entity Memory Access
    // ============================================================

    pub fn target_entity_get(&self, key: &str) -> Option<MemoryValue> {
        self.memory.borrow().entity_get(self.target_eid, key)
    }

    pub fn target_entity_set(&mut self, key: &str, value: MemoryValue) {
        self.memory.borrow_mut().entity_set(self.target_eid, key, value, false);
    }

    pub fn target_entity_has(&self, key: &str) -> bool {
        self.memory.borrow().entity_has(self.target_eid, key)
    }

    pub fn target_entity_add(&mut self, key: &str, value: i32) {
        if let Some(current) = self.target_entity_get(key) {
            if let Ok(current_val) = current.try_to::<i32>() {
                self.target_entity_set(key, Variant::from(current_val + value));
            }
        }
    }

    // ============================================================
    // Flags (entity has a _Flags array)
    // ============================================================

    pub fn entity_has_flag(&self, flag: &str) -> bool {
        if let Some(flags_variant) = self.entity_get("_Flags") {
            if let Ok(flags) = flags_variant.try_to::<PackedStringArray>() {
                let flag_gstring = GString::from(flag);
                for i in 0..flags.len() {
                    if let Some(item) = flags.get(i) {
                        if item == flag_gstring {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    pub fn entity_set_flag(&mut self, flag: &str, active: bool) {
        if let Some(flags_variant) = self.entity_get("_Flags") {
            if let Ok(mut flags) = flags_variant.try_to::<PackedStringArray>() {
                let flag_gstring = GString::from(flag);
                if active {
                    // Add flag if not present
                    let mut found = false;
                    for i in 0..flags.len() {
                        if let Some(item) = flags.get(i) {
                            if item == flag_gstring {
                                found = true;
                                break;
                            }
                        }
                    }
                    if !found {
                        flags.push(flag);
                    }
                } else {
                    // Remove flag
                    let mut new_flags = PackedStringArray::new();
                    for i in 0..flags.len() {
                        if let Some(item) = flags.get(i) {
                            if item != flag_gstring {
                                new_flags.push(item.to_string().as_str());
                            }
                        }
                    }
                    flags = new_flags;
                }
                self.entity_set("_Flags", Variant::from(flags));
            }
        }
    }

    // ============================================================
    // Direct memory access (for advanced use)
    // ============================================================

    pub fn memory(&self) -> Rc<RefCell<CastagneMemory>> {
        Rc::clone(&self.memory)
    }
}
