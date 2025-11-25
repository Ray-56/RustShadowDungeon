//! Health and damage components

use bevy::prelude::*;

/// Health component
#[derive(Component, Debug)]
pub struct Health {
    /// Current health
    pub current: i32,
    /// Maximum health
    pub max: i32,
}

impl Health {
    /// Create new health with max value
    #[must_use]
    pub const fn new(max: i32) -> Self {
        Self { current: max, max }
    }

    /// Check if alive
    #[must_use]
    pub const fn is_alive(&self) -> bool {
        self.current > 0
    }

    /// Take damage
    pub fn take_damage(&mut self, damage: i32) {
        self.current = (self.current - damage).max(0);
        info!("Took {} damage! Health: {}/{}", damage, self.current, self.max);
    }

    /// Heal
    pub fn heal(&mut self, amount: i32) {
        self.current = (self.current + amount).min(self.max);
    }

    /// Reset to max health
    pub fn reset(&mut self) {
        self.current = self.max;
    }
}

/// Invincibility timer (after taking damage)
#[derive(Component, Debug)]
pub struct InvincibilityTimer {
    /// Remaining invincibility time (seconds)
    pub remaining: f32,
}

impl InvincibilityTimer {
    /// Create new invincibility timer
    #[must_use]
    pub const fn new(duration: f32) -> Self {
        Self { remaining: duration }
    }

    /// Check if still invincible
    #[must_use]
    pub const fn is_active(&self) -> bool {
        self.remaining > 0.0
    }

    /// Update timer
    pub fn tick(&mut self, delta: f32) {
        self.remaining = (self.remaining - delta).max(0.0);
    }
}

