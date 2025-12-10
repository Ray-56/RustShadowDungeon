//! Dungeon system components
//!
//! Components for managing dungeon rooms, doors, and progression.

use bevy::prelude::*;

/// Unique identifier for a room in the dungeon
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RoomId(pub u32);

/// Unique identifier for a door in the dungeon
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DoorId(pub u32);

// Re-export RoomState from domain layer
pub use crate::domain::dungeon::progression::RoomState;

/// Component marking a room entity
///
/// Contains room metadata and state information.
#[derive(Component, Debug, Clone)]
pub struct Room {
    /// Unique identifier for this room
    pub room_id: RoomId,
    /// Current state of the room (from domain layer)
    pub state: crate::domain::dungeon::progression::RoomState,
    /// Spawn points for enemies in this room
    pub spawn_points: Vec<Vec2>,
}

/// Component marking a door entity
///
/// Represents a connection between two rooms.
#[derive(Component, Debug, Clone)]
pub struct Door {
    /// Unique identifier for this door
    pub door_id: DoorId,
    /// ID of the room this door connects to
    pub connected_room_id: RoomId,
    /// Current state of the door
    pub door_state: DoorState,
    /// Position where player enters when coming through this door
    pub entrance_position: Vec2,
}

/// State of a door
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DoorState {
    /// Door is locked (cannot be used)
    Locked,
    /// Door is unlocked (can be used to transition)
    Unlocked,
}

/// Component marking the dungeon manager entity
///
/// Tracks the current room and dungeon structure.
#[derive(Component, Debug, Clone)]
pub struct DungeonManager {
    /// ID of the currently active room
    pub current_room_id: RoomId,
    /// Graph structure representing room connections
    /// Maps RoomId -> Vec<(DoorId, connected_room_id)>
    pub room_graph: std::collections::HashMap<RoomId, Vec<(DoorId, RoomId)>>,
}

/// Component marking an enemy spawn point
///
/// Defines where enemies should spawn when a room is activated.
#[derive(Component, Debug, Clone)]
pub struct EnemySpawnPoint {
    /// Position where enemy should spawn
    pub position: Vec2,
    /// Type of enemy to spawn
    pub enemy_type: String,
    /// Whether to spawn when room is activated
    pub spawn_on_activate: bool,
}
