use crate::domain::movement::MovementState;
use bevy::prelude::*;
use std::collections::HashMap;

/// Animation configuration and assets
#[derive(Resource, Debug, Default)]
pub struct PlayerAnimations {
    /// Map state to list of texture handles (frames)
    pub animations: HashMap<MovementState, Vec<Handle<Image>>>,
    /// Frame duration in seconds
    pub frame_duration: f32,
}

impl PlayerAnimations {
    /// Add animation frames for a state
    pub fn add(&mut self, state: MovementState, handles: Vec<Handle<Image>>) {
        self.animations.insert(state, handles);
    }

    /// Get frames for a state
    pub fn get_frames(&self, state: MovementState) -> Option<&Vec<Handle<Image>>> {
        self.animations.get(&state)
    }
}
