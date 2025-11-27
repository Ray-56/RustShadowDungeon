//! Health and damage components

use bevy::prelude::*;

/// Health component (updated for M2 Combat System)
///
/// # Breaking Change
/// Changed from i32 to f32 to match the combat damage calculation system
#[derive(Component, Debug, Clone)]
pub struct Health {
    /// Current health (f32 for precise damage calculation)
    pub current: f32,
    /// Maximum health
    pub max: f32,
}

impl Health {
    /// Create new health with max value
    #[must_use]
    pub fn new(max: f32) -> Self {
        Self { current: max, max }
    }

    /// Check if alive (backwards compatibility)
    #[must_use]
    pub fn is_alive(&self) -> bool {
        self.current > 0.0
    }

    /// Check if dead (M2 Combat System)
    #[must_use]
    pub fn is_dead(&self) -> bool {
        self.current <= 0.0
    }

    /// Take damage (updated to f32)
    ///
    /// # Arguments
    /// * `amount` - Damage amount (will be clamped to ensure health doesn't go negative)
    pub fn take_damage(&mut self, amount: f32) {
        self.current = (self.current - amount).max(0.0);
        info!("Took {:.1} damage! Health: {:.1}/{:.1}", amount, self.current, self.max);
    }

    /// Heal
    pub fn heal(&mut self, amount: f32) {
        self.current = (self.current + amount).min(self.max);
    }

    /// Reset to max health
    pub fn reset(&mut self) {
        self.current = self.max;
    }

    /// Get health percentage (0.0 to 1.0)
    #[must_use]
    pub fn health_percentage(&self) -> f32 {
        if self.max <= 0.0 {
            0.0
        } else {
            (self.current / self.max).clamp(0.0, 1.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_new() {
        let health = Health::new(100.0);
        assert_eq!(health.current, 100.0);
        assert_eq!(health.max, 100.0);
        assert!(health.is_alive());
        assert!(!health.is_dead());
    }

    #[test]
    fn test_health_take_damage() {
        let mut health = Health::new(100.0);
        health.take_damage(30.0);
        assert_eq!(health.current, 70.0);
        assert!(health.is_alive());
    }

    #[test]
    fn test_health_take_damage_death() {
        let mut health = Health::new(100.0);
        health.take_damage(150.0);
        assert_eq!(health.current, 0.0);
        assert!(!health.is_alive());
        assert!(health.is_dead());
    }

    #[test]
    fn test_health_heal() {
        let mut health = Health::new(100.0);
        health.take_damage(50.0);
        health.heal(30.0);
        assert_eq!(health.current, 80.0);
    }

    #[test]
    fn test_health_heal_cap() {
        let mut health = Health::new(100.0);
        health.heal(50.0); // Should not exceed max
        assert_eq!(health.current, 100.0);
    }

    #[test]
    fn test_health_percentage() {
        let health = Health::new(100.0);
        assert_eq!(health.health_percentage(), 1.0);

        let mut health2 = Health::new(100.0);
        health2.take_damage(50.0);
        assert_eq!(health2.health_percentage(), 0.5);

        let mut health3 = Health::new(100.0);
        health3.take_damage(100.0);
        assert_eq!(health3.health_percentage(), 0.0);
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
