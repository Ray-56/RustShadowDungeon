//! Enemy components

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Enemy marker component
#[derive(Component, Debug)]
pub struct Enemy;

/// Enemy ID for identification
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EnemyId(pub u32);

/// Enemy type enum
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EnemyType {
    /// Slime enemy (basic enemy type)
    Slime,
}

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

/// Hit flash effect for visual feedback when enemy takes damage
///
/// 受伤闪烁效果，用于敌人受伤时的视觉反馈
#[derive(Component, Debug)]
pub struct HitFlash {
    /// Remaining duration of the flash effect (seconds)
    pub duration: f32,
    /// Original sprite color (to restore after flash)
    pub original_color: Color,
}

impl HitFlash {
    /// Create a new hit flash effect
    pub fn new(duration: f32, original_color: Color) -> Self {
        Self {
            duration,
            original_color,
        }
    }
}

/// Death animation component for enemy death visual feedback
///
/// 死亡动画组件，用于敌人死亡时的视觉反馈
#[derive(Component, Debug)]
pub struct DeathAnimation {
    /// Remaining duration of death animation (seconds)
    pub duration: f32,
    /// Initial scale (for shrink effect)
    pub initial_scale: Vec3,
    /// Target scale (usually 0.0 for shrink to nothing)
    pub target_scale: Vec3,
}

impl DeathAnimation {
    /// Create a new death animation
    pub fn new(duration: f32, initial_scale: Vec3) -> Self {
        Self {
            duration,
            initial_scale,
            target_scale: Vec3::ZERO,
        }
    }
}
