// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Memory system for Castagne
//!
//! Provides three-tier storage: Global, Player, and Entity data.
//! This is the core state storage system that can be saved/restored for rollback.

use std::collections::HashMap;
use godot::prelude::*;

/// Type for memory values - can be integers, strings, booleans, etc.
/// In GDScript this is dynamically typed, in Rust we use a Variant
pub type MemoryValue = Variant;

/// The main memory structure holding all game state
#[derive(GodotClass)]
#[class(no_init)]
pub struct CastagneMemory {
    memory_global: HashMap<String, MemoryValue>,
    memory_players: Vec<HashMap<String, MemoryValue>>,
    pub(crate) memory_entities: Vec<Option<HashMap<String, MemoryValue>>>,
}

impl CastagneMemory {
    /// Create a new empty memory
    pub fn new() -> Self {
        Self {
            memory_global: HashMap::new(),
            memory_players: Vec::new(),
            memory_entities: Vec::new(),
        }
    }

    /// Initialize memory (currently a no-op, matches GDScript)
    pub fn init_memory(&mut self) {
        // No-op for now
    }

    /// Deep copy from another memory
    pub fn copy_from(&mut self, other: &CastagneMemory) {
        self.memory_global = other.memory_global.clone();
        self.memory_players = other.memory_players.clone();
        self.memory_entities = other.memory_entities.clone();
    }

    // ============================================================
    // Global Memory Access
    // ============================================================

    pub fn global_get(&self, key: &str) -> Option<MemoryValue> {
        if let Some(value) = self.memory_global.get(key) {
            Some(value.clone())
        } else {
            godot_error!("Memory Global Get: Key is undefined: {}", key);
            None
        }
    }

    pub fn global_set(&mut self, key: &str, value: MemoryValue, new_value: bool) {
        if new_value || self.memory_global.contains_key(key) {
            self.memory_global.insert(key.to_string(), value);
        } else {
            godot_error!("Memory Global Set: Key doesn't already exist: {}", key);
        }
    }

    pub fn global_has(&self, key: &str) -> bool {
        self.memory_global.contains_key(key)
    }

    // ============================================================
    // Player Memory Access
    // ============================================================

    pub fn add_player(&mut self) {
        self.memory_players.push(HashMap::new());
    }

    pub fn player_get(&self, pid: usize, key: &str) -> Option<MemoryValue> {
        if pid >= self.memory_players.len() {
            godot_error!("Memory Player Get ({}): PID is invalid! Key: {}", pid, key);
            return None;
        }

        if let Some(value) = self.memory_players[pid].get(key) {
            Some(value.clone())
        } else {
            godot_error!("Memory Player Get ({}): Key is undefined: {}", pid, key);
            None
        }
    }

    pub fn player_set(&mut self, pid: usize, key: &str, value: MemoryValue, new_value: bool) {
        if pid >= self.memory_players.len() {
            godot_error!("Memory Player Set ({}): PID is invalid! Key: {}", pid, key);
            return;
        }

        if new_value || self.memory_players[pid].contains_key(key) {
            self.memory_players[pid].insert(key.to_string(), value);
        } else {
            godot_error!("Memory Player Set ({}): Key doesn't already exist: {}", pid, key);
        }
    }

    pub fn player_has(&self, pid: usize, key: &str) -> bool {
        if pid >= self.memory_players.len() {
            return false;
        }
        self.memory_players[pid].contains_key(key)
    }

    // ============================================================
    // Entity Memory Access
    // ============================================================

    pub fn add_entity(&mut self) -> usize {
        self.memory_entities.push(Some(HashMap::new()));
        self.memory_entities.len() - 1
    }

    pub fn remove_entity(&mut self, eid: usize) {
        if eid < self.memory_entities.len() {
            self.memory_entities[eid] = None;
        }
    }

    pub fn is_eid_valid(&self, eid: i32) -> bool {
        if eid < 0 {
            return false;
        }
        let eid = eid as usize;
        eid < self.memory_entities.len() && self.memory_entities[eid].is_some()
    }

    pub fn entity_get(&self, eid: i32, key: &str) -> Option<MemoryValue> {
        if !self.is_eid_valid(eid) {
            godot_error!("Memory Entity Get ({}): EID is invalid! Key: {}", eid, key);
            return None;
        }

        let eid = eid as usize;
        if let Some(Some(entity_map)) = self.memory_entities.get(eid) {
            if let Some(value) = entity_map.get(key) {
                return Some(value.clone());
            }
        }

        godot_error!("Memory Entity Get ({}): Key is undefined: {}", eid, key);
        None
    }

    pub fn entity_set(&mut self, eid: i32, key: &str, value: MemoryValue, new_value: bool) {
        if !self.is_eid_valid(eid) {
            godot_error!("Memory Entity Set ({}): EID is invalid! Key: {}", eid, key);
            return;
        }

        let eid = eid as usize;
        if let Some(Some(entity_map)) = self.memory_entities.get_mut(eid) {
            if new_value || entity_map.contains_key(key) {
                entity_map.insert(key.to_string(), value);
            } else {
                godot_error!("Memory Entity Set ({}): Key doesn't already exist: {}", eid, key);
            }
        }
    }

    pub fn entity_has(&self, eid: i32, key: &str) -> bool {
        if !self.is_eid_valid(eid) {
            return false;
        }

        let eid = eid as usize;
        if let Some(Some(entity_map)) = self.memory_entities.get(eid) {
            return entity_map.contains_key(key);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_initialization() {
        let memory = CastagneMemory::new();
        assert!(!memory.global_has("NonExistent"));
    }

    #[test]
    fn test_global_memory_operations() {
        let mut memory = CastagneMemory::new();

        // Set a global value
        memory.global_set("TestKey", Variant::from(42), true);

        // Check it exists
        assert!(memory.global_has("TestKey"));

        // Get the value back
        let value = memory.global_get("TestKey");
        assert!(value.is_some());
        assert_eq!(value.unwrap().try_to::<i32>().unwrap(), 42);
    }

    #[test]
    fn test_player_memory() {
        let mut memory = CastagneMemory::new();

        // Add a player
        memory.add_player();

        // Set player data
        memory.player_set(0, "HP", Variant::from(100), true);

        // Check and retrieve
        assert!(memory.player_has(0, "HP"));
        let hp = memory.player_get(0, "HP").unwrap();
        assert_eq!(hp.try_to::<i32>().unwrap(), 100);
    }

    #[test]
    fn test_entity_lifecycle() {
        let mut memory = CastagneMemory::new();

        // Add entity
        let eid = memory.add_entity();
        assert!(memory.is_eid_valid(eid as i32));

        // Set entity data
        memory.entity_set(eid as i32, "Position", Variant::from(10), true);
        assert!(memory.entity_has(eid as i32, "Position"));

        // Remove entity
        memory.remove_entity(eid);
        assert!(!memory.is_eid_valid(eid as i32));
    }

    #[test]
    fn test_copy_from() {
        let mut memory1 = CastagneMemory::new();
        memory1.global_set("Test", Variant::from(42), true);

        let mut memory2 = CastagneMemory::new();
        memory2.copy_from(&memory1);

        assert!(memory2.global_has("Test"));
        assert_eq!(memory2.global_get("Test").unwrap().try_to::<i32>().unwrap(), 42);
    }

    #[test]
    fn test_invalid_entity_access() {
        let memory = CastagneMemory::new();

        // Accessing invalid entity should return None
        let result = memory.entity_get(-1, "Test");
        assert!(result.is_none());

        let result = memory.entity_get(999, "Test");
        assert!(result.is_none());
    }
}
