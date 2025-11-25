//! Velocity domain model
//!
//! Pure Rust data structure, zero Bevy dependencies

use super::input::InputDirection;

/// Movement velocity in pixels per second
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Velocity {
    /// Horizontal velocity (pixels/second)
    pub x: f32,
    /// Vertical velocity (pixels/second)
    pub y: f32,
}

impl Velocity {
    /// Create a new velocity
    #[must_use]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Zero velocity
    #[must_use]
    pub const fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    /// Calculate velocity magnitude
    #[must_use]
    pub fn magnitude(&self) -> f32 {
        self.x.hypot(self.y)
    }

    /// Normalize velocity to unit vector
    #[must_use]
    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag > 0.0 {
            Self { x: self.x / mag, y: self.y / mag }
        } else {
            Self::zero()
        }
    }
}

impl Default for Velocity {
    fn default() -> Self {
        Self::zero()
    }
}

/// Calculate ground movement velocity
///
/// # Arguments
/// * `current` - Current velocity
/// * `direction` - Input direction
/// * `speed` - Target movement speed
#[must_use]
pub fn calculate_ground_velocity(
    current: Velocity,
    direction: InputDirection,
    speed: f32,
) -> Velocity {
    Velocity {
        x: direction.to_velocity_factor() * speed,
        y: current.y, // Preserve vertical velocity (gravity)
    }
}

/// Calculate air movement velocity
///
/// Applies air control to horizontal velocity while preserving vertical velocity.
///
/// # Arguments
/// * `current` - Current velocity
/// * `direction` - Input direction
/// * `ground_speed` - Base ground speed
/// * `air_speed_factor` - Multiplier for speed in air (0.0-1.0)
#[must_use]
pub fn calculate_air_velocity(
    current: Velocity,
    direction: InputDirection,
    ground_speed: f32,
    air_speed_factor: f32,
) -> Velocity {
    let target_x = direction.to_velocity_factor() * ground_speed * air_speed_factor;

    // Responsiveness factor (0.3 = 30% per frame/update)
    // Ideally this should depend on delta time, but for fixed update it's fine.
    // Using the tuned value from previous implementation: 0.3
    let responsiveness = 0.3;

    Velocity {
        x: current.x.mul_add(1.0 - responsiveness, target_x * responsiveness),
        y: current.y, // Preserve vertical velocity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_velocity_magnitude() {
        let v = Velocity::new(3.0, 4.0);
        assert_eq!(v.magnitude(), 5.0);
    }

    #[test]
    fn test_velocity_normalize() {
        let v = Velocity::new(3.0, 4.0);
        let normalized = v.normalize();
        assert!((normalized.magnitude() - 1.0).abs() < 0.001);
    }
}
