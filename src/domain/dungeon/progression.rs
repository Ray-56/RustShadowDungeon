//! Room progression and state management logic
//!
//! Pure functions for determining room state, enemy spawning, and door unlocking.
//! Zero Bevy dependencies - can be tested in isolation.

/// State of a room (domain model)
///
/// This enum represents the state of a room in the dungeon system.
/// It is used by domain logic functions and is re-exported by the infrastructure layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoomState {
    /// Room is active (currently loaded)
    Active,
    /// Room has been cleared (all enemies defeated)
    Cleared,
    /// Room has not been cleared yet
    Uncleared,
}

// Re-export for infrastructure layer compatibility
pub use crate::infrastructure::components::dungeon::{DoorState, RoomId};

/// Determines if enemies should spawn based on room state
///
/// # Arguments
/// * `room_state` - Current state of the room
///
/// # Returns
/// `true` if enemies should spawn, `false` otherwise
pub fn should_spawn_enemies(room_state: RoomState) -> bool {
    matches!(room_state, RoomState::Uncleared)
}

/// Checks if a room should be marked as cleared
///
/// # Arguments
/// * `enemy_count` - Number of enemies currently alive in the room
///
/// # Returns
/// `true` if room should be marked as cleared (no enemies remaining)
pub fn check_room_cleared(enemy_count: usize) -> bool {
    enemy_count == 0
}

/// Determines if doors should be unlocked based on room state
///
/// # Arguments
/// * `room_state` - Current state of the room
///
/// # Returns
/// `true` if doors should be unlocked, `false` otherwise
pub fn should_unlock_doors(room_state: RoomState) -> bool {
    matches!(room_state, RoomState::Cleared)
}

/// Marks a room as cleared
///
/// # Arguments
/// * `room_state` - Current state of the room
///
/// # Returns
/// New room state (Cleared if was Uncleared, otherwise unchanged)
pub fn mark_room_cleared(room_state: RoomState) -> RoomState {
    match room_state {
        RoomState::Uncleared => RoomState::Cleared,
        other => other,
    }
}

/// Checks if a room has been cleared
///
/// # Arguments
/// * `room_state` - Current state of the room
///
/// # Returns
/// `true` if room is cleared, `false` otherwise
pub fn is_room_cleared(room_state: RoomState) -> bool {
    matches!(room_state, RoomState::Cleared)
}

/// Gets the target room entrance position based on door connection
///
/// # Arguments
/// * `door_entrance_position` - Position where player should appear when entering through this door
///
/// # Returns
/// Entrance position in the target room
pub fn get_target_room_entrance(door_entrance_position: bevy::math::Vec2) -> bevy::math::Vec2 {
    door_entrance_position
}

/// Checks if player can transition to a target room
///
/// # Arguments
/// * `door_state` - State of the door (locked/unlocked)
///
/// # Returns
/// `true` if transition is allowed, `false` otherwise
pub fn can_transition_to_room(
    door_state: crate::infrastructure::components::dungeon::DoorState,
) -> bool {
    matches!(door_state, crate::infrastructure::components::dungeon::DoorState::Unlocked)
}
