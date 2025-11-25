use crate::domain::movement::MovementState;
use bevy::prelude::*;

/// Animation state component
#[derive(Component, Debug, Default)]
pub struct AnimationState {
    /// Timer for frame updates
    pub timer: Timer,
    /// Current frame index
    pub current_frame: usize,
    /// Current animation state (matches movement state)
    pub current_state: MovementState,
}

impl AnimationState {
    /// Create new animation state
    pub fn new(frame_duration: f32) -> Self {
        Self {
            timer: Timer::from_seconds(frame_duration, TimerMode::Repeating),
            current_frame: 0,
            current_state: MovementState::Idle,
        }
    }
}
