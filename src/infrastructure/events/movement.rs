//! Movement events
//!
//! Events triggered by movement system changes.

use crate::domain::movement::{MovementState, Velocity};
use bevy::prelude::*;

/// Event triggered when player moves
#[derive(Event, Message, Debug, Clone)]
pub struct PlayerMoved {
    /// Player entity
    pub entity: Entity,
    /// New position
    pub new_position: Vec2,
    /// Current velocity
    pub velocity: Velocity,
}

/// Event triggered when movement state changes
#[derive(Event, Message, Debug, Clone)]
pub struct StateChanged {
    /// Player entity
    pub entity: Entity,
    /// Previous state
    pub from_state: MovementState,
    /// New state
    pub to_state: MovementState,
}
