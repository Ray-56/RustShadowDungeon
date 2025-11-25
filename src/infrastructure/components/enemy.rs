//! Enemy components

use bevy::prelude::*;

/// Enemy marker component
#[derive(Component, Debug)]
pub struct Enemy;

/// Patrol behavior - enemy moves back and forth
#[derive(Component, Debug)]
pub struct PatrolBehavior {
    /// Left boundary of patrol area
    pub left_bound: f32,
    /// Right boundary of patrol area
    pub right_bound: f32,
    /// Current movement direction (1.0 = right, -1.0 = left)
    pub direction: f32,
    /// Movement speed (pixels per second)
    pub speed: f32,
}

impl PatrolBehavior {
    /// Create a new patrol behavior
    #[must_use]
    pub fn new(center_x: f32, patrol_range: f32, speed: f32) -> Self {
        Self {
            left_bound: center_x - patrol_range,
            right_bound: center_x + patrol_range,
            direction: 1.0, // Start moving right
            speed,
        }
    }
}
