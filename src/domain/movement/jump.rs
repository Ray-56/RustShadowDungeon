//! Jump mechanics domain model
//!
//! Pure Rust logic for jump calculations, zero Bevy dependencies

/// Jump parameters configuration
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct JumpParams {
    /// Initial jump velocity (pixels per second, upward)
    pub initial_velocity: f32,
    /// Minimum jump height velocity (for short hops)
    pub min_velocity: f32,
    /// Gravity acceleration (pixels per second squared, downward)
    pub gravity: f32,
}

impl JumpParams {
    /// Create new jump parameters
    ///
    /// # Arguments
    /// * `initial_velocity` - Initial upward velocity when jumping
    /// * `min_velocity` - Minimum velocity for variable jump height
    /// * `gravity` - Downward acceleration
    #[must_use]
    pub const fn new(initial_velocity: f32, min_velocity: f32, gravity: f32) -> Self {
        Self { initial_velocity, min_velocity, gravity }
    }

    /// Default parameters for ~2 tile jump height
    ///
    /// Calculation:
    /// - Jump height: 32 pixels (2 tiles)
    /// - Using physics: v² = 2gh
    /// - v = sqrt(2 * gravity * height)
    #[must_use]
    pub fn default_params() -> Self {
        let gravity = 980.0_f32; // pixels/s² (close to real 9.8 m/s²)
        let jump_height = 32.0_f32; // 2 tiles
        let initial_velocity = (2.0_f32 * gravity * jump_height).sqrt();

        Self {
            initial_velocity,
            min_velocity: initial_velocity * 0.4, // 40% for short hops
            gravity,
        }
    }
}

/// Calculate if a jump can be initiated
///
/// # Arguments
/// * `is_grounded` - Is the character on the ground?
/// * `jump_pressed` - Is the jump button pressed this frame?
///
/// # Returns
/// `true` if jump should be initiated
#[must_use]
pub const fn can_jump(is_grounded: bool, jump_pressed: bool) -> bool {
    is_grounded && jump_pressed
}

/// Apply jump velocity
///
/// # Arguments
/// * `_current_velocity_y` - Current vertical velocity (unused, jump overrides)
/// * `params` - Jump parameters
///
/// # Returns
/// New vertical velocity after jump initiation
#[must_use]
pub const fn apply_jump(_current_velocity_y: f32, params: &JumpParams) -> f32 {
    // Override current velocity with jump velocity
    params.initial_velocity
}

/// Apply variable jump (early release)
///
/// When player releases jump button while ascending, reduce upward velocity
/// to create variable jump height.
///
/// # Arguments
/// * `current_velocity_y` - Current vertical velocity (positive = up)
/// * `params` - Jump parameters
///
/// # Returns
/// New velocity after early release
#[must_use]
pub fn apply_variable_jump(current_velocity_y: f32, params: &JumpParams) -> f32 {
    if current_velocity_y > params.min_velocity {
        // Cap velocity to minimum (creates shorter jump)
        params.min_velocity
    } else {
        // Already below minimum, don't change
        current_velocity_y
    }
}

/// Apply gravity to velocity
///
/// # Arguments
/// * `current_velocity_y` - Current vertical velocity
/// * `gravity` - Gravity acceleration (positive = downward)
/// * `delta_time` - Time elapsed since last frame (seconds)
///
/// # Returns
/// New velocity after gravity application
#[must_use]
pub fn apply_gravity(current_velocity_y: f32, gravity: f32, delta_time: f32) -> f32 {
    gravity.mul_add(-delta_time, current_velocity_y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_jump_when_grounded() {
        assert!(can_jump(true, true));
    }

    #[test]
    fn test_cannot_jump_in_air() {
        assert!(!can_jump(false, true));
    }

    #[test]
    fn test_cannot_jump_without_input() {
        assert!(!can_jump(true, false));
    }

    #[test]
    fn test_apply_jump_sets_velocity() {
        let params = JumpParams::default_params();
        let new_vel = apply_jump(0.0, &params);
        assert!(new_vel > 0.0);
        assert!((new_vel - params.initial_velocity).abs() < 0.01);
    }

    #[test]
    fn test_variable_jump_caps_velocity() {
        let params = JumpParams::default_params();
        let high_velocity = params.initial_velocity;
        let capped = apply_variable_jump(high_velocity, &params);
        assert_eq!(capped, params.min_velocity);
    }

    #[test]
    fn test_gravity_reduces_upward_velocity() {
        let initial = 100.0;
        let gravity = 980.0;
        let dt = 0.016; // ~60 FPS
        let new_vel = apply_gravity(initial, gravity, dt);
        assert!(new_vel < initial);
    }
}
