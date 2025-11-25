//! Player ECS components
//!
//! Pure data structures for the ECS

use bevy::prelude::*;

use crate::domain::movement::{MovementState, Velocity as DomainVelocity};

/// Player marker component
#[derive(Component, Debug)]
pub struct Player;

/// Player input state
#[derive(Component, Debug, Default)]
pub struct InputState {
    /// Movement direction (-1.0 to 1.0)
    pub move_direction: f32,
    /// Jump button pressed this frame (triggered once)
    pub jump_pressed: bool,
    /// Jump button is being held down
    pub jump_held: bool,
    /// Jump buffer: time remaining for buffered jump
    pub jump_buffer_time: f32,
}

impl InputState {
    /// Buffer a jump input
    pub fn buffer_jump(&mut self) {
        self.jump_buffer_time = 0.1; // 100ms buffer
    }

    /// Check if has buffered jump
    #[must_use]
    pub fn has_buffered_jump(&self) -> bool {
        self.jump_buffer_time > 0.0
    }

    /// Consume buffered jump
    pub fn consume_jump_buffer(&mut self) {
        self.jump_buffer_time = 0.0;
    }

    /// Update buffer timer
    pub fn tick_buffer(&mut self, delta: f32) {
        self.jump_buffer_time = (self.jump_buffer_time - delta).max(0.0);
    }
}

/// Movement state component (wraps domain state)
#[derive(Component, Debug, Default)]
pub struct MovementStateComponent(pub MovementState);

/// Velocity component (wraps domain velocity)
#[derive(Component, Debug, Default)]
pub struct VelocityComponent(pub DomainVelocity);

/// Grounded state component
#[derive(Component, Debug, Default)]
pub struct GroundedState {
    /// Is player touching ground?
    pub is_grounded: bool,
    /// Coyote time: seconds since left ground (for late jumps)
    pub time_since_grounded: f32,
}

impl GroundedState {
    /// Check if can coyote jump (recently left ground)
    #[must_use]
    pub fn can_coyote_jump(&self) -> bool {
        !self.is_grounded && self.time_since_grounded < 0.15 // 150ms grace period
    }
}

/// Pixel snap marker - ensures entity snaps to pixel grid
#[derive(Component, Debug)]
pub struct PixelSnap;
