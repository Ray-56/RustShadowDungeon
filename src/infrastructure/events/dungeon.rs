//! Dungeon-related events
//!
//! Events for dungeon system communication.

use crate::infrastructure::components::dungeon::{DoorId, RoomId};
use bevy::ecs::message::Message;
use bevy::prelude::*;

/// Event published when player enters a room
///
/// 玩家进入房间事件
///
/// Published when player transitions into a new room.
#[derive(Message, Debug, Clone)]
pub struct RoomEntered {
    /// ID of the room that was entered
    pub room_id: RoomId,
    /// Position where player entered
    pub player_position: Vec2,
}

/// Event published when a room is cleared
///
/// 房间清理完成事件
///
/// Published when all enemies in a room are defeated.
#[derive(Message, Debug, Clone)]
pub struct RoomCleared {
    /// ID of the room that was cleared
    pub room_id: RoomId,
    /// Timestamp when room was cleared
    pub cleared_at: f64,
}

/// Event published when a door is unlocked
///
/// 门解锁事件
///
/// Published when a door transitions from locked to unlocked state.
#[derive(Message, Debug, Clone)]
pub struct DoorUnlocked {
    /// ID of the door that was unlocked
    pub door_id: DoorId,
    /// ID of the room containing this door
    pub room_id: RoomId,
}

/// Event published when player transitions between rooms
///
/// 房间过渡事件
///
/// Published when player successfully moves from one room to another.
#[derive(Message, Debug, Clone)]
pub struct RoomTransitioned {
    /// ID of the room player left
    pub from_room_id: RoomId,
    /// ID of the room player entered
    pub to_room_id: RoomId,
    /// Position where player appeared in the new room
    pub player_position: Vec2,
}

/// Event published when player dies in a dungeon room
///
/// 玩家在房间内死亡事件
///
/// Published when player dies while in a dungeon room.
#[derive(Message, Debug, Clone)]
pub struct PlayerDeathInRoom {
    /// ID of the room where player died
    pub room_id: RoomId,
    /// Position where player died
    pub death_position: Vec2,
}
