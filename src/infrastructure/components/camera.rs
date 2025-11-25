use bevy::prelude::*;

/// Component to make entity follow the player
#[derive(Component, Debug, Clone)]
pub struct CameraFollow {
    /// Offset from the target
    pub offset: Vec3,
    /// Smoothness factor (0.0-1.0)
    pub smoothness: f32,
}

impl Default for CameraFollow {
    fn default() -> Self {
        Self { offset: Vec3::new(0.0, 0.0, 0.0), smoothness: 0.1 }
    }
}
