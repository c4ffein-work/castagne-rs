// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! CastagneNet - Rollback netcode and online match management
//!
//! This module manages network synchronization and rollback for online matches.
//! NOTE: The original GDScript version states "not maintained until v0.8 cycle"
//! so this is a minimal stub implementation with TODOs.

use godot::prelude::*;

/// Network sync status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkSyncStatus {
    Off,
    Starting,
    Ready,
    Stopping,
}

/// CastagneNet - Network and rollback management
///
/// This struct serves as an interface between CastagneEngine, the rollback system,
/// and the network server. It manages:
/// - State saving/loading for rollback
/// - Input delay
/// - Rollback logic
/// - Network synchronization
///
/// NOTE: This is a stub implementation! The original is marked as unmaintained.
pub struct CastagneNet {
    server_ip: String,
    server_port: u16,
    net_log: String,
    nb_peers: usize,
    network_sync_status: NetworkSyncStatus,
}

impl CastagneNet {
    /// Create a new network manager
    pub fn new() -> Self {
        Self {
            server_ip: "127.0.0.1".to_string(),
            server_port: 6442,
            net_log: String::new(),
            nb_peers: 0,
            network_sync_status: NetworkSyncStatus::Off,
        }
    }

    /// Set server IP address
    pub fn set_server_ip(&mut self, ip: String) {
        self.server_ip = ip;
    }

    /// Set server port
    pub fn set_server_port(&mut self, port: u16) {
        self.server_port = port;
    }

    /// Get current network sync status
    pub fn get_network_sync_status(&self) -> NetworkSyncStatus {
        self.network_sync_status
    }

    /// Get number of connected peers
    pub fn get_nb_peers(&self) -> usize {
        self.nb_peers
    }

    // -------------------------------------------------------------------------
    // Network setup/teardown

    /// Host a server
    pub fn host(&mut self) {
        // TODO: Implement server hosting
        // In GDScript this creates a NetworkedMultiplayerENet server
        // and sets get_tree().network_peer
        self.log("Hosting Server (TODO: Not implemented)");
        self.nb_peers += 1;
        self.network_sync_status = NetworkSyncStatus::Starting;
    }

    /// Join a server
    pub fn join(&mut self) {
        // TODO: Implement client connection
        // In GDScript this creates a NetworkedMultiplayerENet client
        // and connects to server_ip:server_port
        self.log(&format!(
            "Joining Server at {}:{} (TODO: Not implemented)",
            self.server_ip, self.server_port
        ));
        self.network_sync_status = NetworkSyncStatus::Starting;
    }

    /// Disconnect from network
    pub fn disconnect(&mut self) {
        // TODO: Implement disconnection
        // Should stop sync manager and close network peer
        self.log("Disconnecting (TODO: Not implemented)");
        self.network_sync_status = NetworkSyncStatus::Stopping;
        self.nb_peers = 0;
    }

    /// Start synchronization
    pub fn start_sync(&mut self) {
        // TODO: Implement sync start
        // Should start the SyncManager if we're the server
        self.log("Starting Sync (TODO: Not implemented)");
        self.network_sync_status = NetworkSyncStatus::Ready;
    }

    /// Stop synchronization
    pub fn stop_sync(&mut self) {
        // TODO: Implement sync stop
        // Should stop the SyncManager and logging
        self.log("Stopping Sync (TODO: Not implemented)");
        self.network_sync_status = NetworkSyncStatus::Off;
    }

    /// Start a network match
    pub fn start_network_match(&mut self) {
        // TODO: Implement network match start
        // This should:
        // 1. Set battle init data to online mode
        // 2. Start sync
        // 3. Wait for sync to be ready
        // 4. Spawn the engine instance
        self.log("Starting Network Match (TODO: Not implemented)");
    }

    /// Start logging network events
    pub fn start_logging(&mut self) {
        // TODO: Implement network logging
        // Should log to user://network_logs/<timestamp>-<peer_id>.log
        self.log("Starting Logging (TODO: Not implemented)");
    }

    // -------------------------------------------------------------------------
    // Peer management (callbacks from Godot networking)

    /// Called when a peer connects
    pub fn on_network_peer_connected(&mut self, peer_id: i32) {
        // TODO: Add peer to sync manager
        self.log(&format!("Network Peer Connected: {} (TODO: Not implemented)", peer_id));
        self.nb_peers += 1;
    }

    /// Called when a peer disconnects
    pub fn on_network_peer_disconnected(&mut self, peer_id: i32) {
        // TODO: Remove peer from sync manager
        // TODO: Stop logging
        self.log(&format!("Network Peer Disconnected: {} (TODO: Not implemented)", peer_id));
        if self.nb_peers > 0 {
            self.nb_peers -= 1;
        }
    }

    /// Called when server disconnects
    pub fn on_server_disconnected(&mut self) {
        // TODO: Handle server disconnection
        self.log("Server Disconnected (TODO: Not implemented)");
        self.on_network_peer_disconnected(1);
    }

    // -------------------------------------------------------------------------
    // Sync manager callbacks

    /// Called when sync starts
    pub fn on_sync_manager_sync_started(&mut self) {
        self.log("SyncManager Sync Started");
        self.network_sync_status = NetworkSyncStatus::Ready;
    }

    /// Called when sync stops
    pub fn on_sync_manager_sync_stopped(&mut self) {
        self.log("SyncManager Sync Stopped");
        self.network_sync_status = NetworkSyncStatus::Off;
    }

    /// Called when sync is lost
    pub fn on_sync_manager_sync_lost(&mut self) {
        self.log("SyncManager Sync Lost");
    }

    /// Called when sync is regained
    pub fn on_sync_manager_sync_regained(&mut self) {
        self.log("SyncManager Sync Regained");
    }

    /// Called on fatal sync error
    pub fn on_sync_manager_sync_error(&mut self, msg: &str) {
        self.error(&format!("SyncManager Fatal Sync Error! {}", msg));
        // TODO: Stop logging, close connection, clear peers
        self.network_sync_status = NetworkSyncStatus::Off;
    }

    // -------------------------------------------------------------------------
    // Rollback functionality (TODO: These are the core netcode features)

    /// Save state for rollback
    pub fn save_state(&self) -> Vec<u8> {
        // TODO: Implement state saving
        // Should serialize all game state (memory, entities, etc.)
        // Returns serialized state as bytes
        Vec::new()
    }

    /// Load state for rollback
    pub fn load_state(&mut self, _state: &[u8]) {
        // TODO: Implement state loading
        // Should deserialize and restore game state
    }

    /// Get current frame number
    pub fn get_current_frame(&self) -> i32 {
        // TODO: Implement frame tracking
        0
    }

    /// Get input delay (frames)
    pub fn get_input_delay(&self) -> i32 {
        // TODO: Implement input delay calculation
        // This is important for netcode feel
        0
    }

    /// Perform rollback to a specific frame
    pub fn rollback_to_frame(&mut self, _frame: i32) {
        // TODO: Implement rollback
        // 1. Load state from that frame
        // 2. Re-simulate up to current frame
        // 3. Apply corrected inputs
    }

    // -------------------------------------------------------------------------
    // Logging

    fn log(&mut self, message: &str) {
        let log_message = format!("[Net] {}", message);
        godot_print!("{}", log_message);
        self.net_log.push_str(&log_message);
        self.net_log.push('\n');
    }

    fn error(&mut self, message: &str) {
        let log_message = format!("[Net] {}", message);
        godot_error!("{}", log_message);
        self.net_log.push_str(&log_message);
        self.net_log.push('\n');
    }

    /// Get the full network log
    pub fn get_net_log(&self) -> &str {
        &self.net_log
    }

    /// Clear the network log
    pub fn clear_net_log(&mut self) {
        self.net_log.clear();
    }
}

impl Default for CastagneNet {
    fn default() -> Self {
        Self::new()
    }
}
