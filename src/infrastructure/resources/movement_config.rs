//! Movement configuration resource

use bevy::prelude::*;

use crate::domain::movement::JumpParams;

/// Movement configuration
#[derive(Resource, Debug, Clone)]
pub struct MovementConfig {
    /// Ground movement speed (pixels/second)
    pub ground_speed: f32,
    /// Air control factor (0.0 - 1.0)
    pub air_control_factor: f32,
    /// Jump parameters
    pub jump_params: JumpParams,
}

impl Default for MovementConfig {
    fn default() -> Self {
        Self {
            ground_speed: 48.0 * 4.5, // 4.5 tiles per second (faster response)
            air_control_factor: 0.8,  // Better air control (was 0.6)
            jump_params: JumpParams::default_params(),
        }
    }
}
