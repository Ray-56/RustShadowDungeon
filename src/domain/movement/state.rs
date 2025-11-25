//! Movement state domain model
//!
//! Pure Rust enums and functions, zero Bevy dependencies

/// Player movement states
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum MovementState {
    /// Standing still, no input
    #[default]
    Idle,
    /// Walking on ground
    Walking,
    /// Jumping (ascending)
    Jumping,
    /// Falling (descending)
    Falling,
}

impl MovementState {
    /// Check if player is on ground
    #[must_use]
    pub const fn is_grounded(self) -> bool {
        matches!(self, Self::Idle | Self::Walking)
    }

    /// Check if player is in air
    #[must_use]
    pub const fn is_airborne(self) -> bool {
        matches!(self, Self::Jumping | Self::Falling)
    }
}

/// State transition logic (pure function)
///
/// # Arguments
/// * `current_state` - Current movement state
/// * `is_grounded` - Is player touching ground?
/// * `has_move_input` - Is player pressing movement keys?
/// * `has_jump_input` - Is player pressing jump key?
/// * `velocity_y` - Vertical velocity (positive = up)
///
/// # Returns
/// New movement state based on inputs
#[must_use]
pub fn transition_state(
    current_state: MovementState,
    is_grounded: bool,
    has_move_input: bool,
    has_jump_input: bool,
    velocity_y: f32,
) -> MovementState {
    match current_state {
        MovementState::Idle => {
            if is_grounded && has_jump_input {
                MovementState::Jumping
            } else if !is_grounded {
                if velocity_y > 0.0 {
                    MovementState::Jumping
                } else {
                    MovementState::Falling
                }
            } else if has_move_input {
                MovementState::Walking
            } else {
                MovementState::Idle
            }
        },
        MovementState::Walking => {
            if is_grounded && has_jump_input {
                MovementState::Jumping
            } else if !is_grounded {
                if velocity_y > 0.0 {
                    MovementState::Jumping
                } else {
                    MovementState::Falling
                }
            } else if !has_move_input {
                MovementState::Idle
            } else {
                MovementState::Walking
            }
        },
        MovementState::Jumping => {
            if is_grounded {
                if has_move_input {
                    MovementState::Walking
                } else {
                    MovementState::Idle
                }
            } else if velocity_y <= 0.0 {
                MovementState::Falling
            } else {
                MovementState::Jumping
            }
        },
        MovementState::Falling => {
            if is_grounded {
                if has_move_input {
                    MovementState::Walking
                } else {
                    MovementState::Idle
                }
            } else {
                MovementState::Falling
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_idle_to_walking() {
        let state = transition_state(MovementState::Idle, true, true, false, 0.0);
        assert_eq!(state, MovementState::Walking);
    }

    #[test]
    fn test_walking_to_idle() {
        let state = transition_state(MovementState::Walking, true, false, false, 0.0);
        assert_eq!(state, MovementState::Idle);
    }

    #[test]
    fn test_idle_to_falling() {
        let state = transition_state(MovementState::Idle, false, false, false, -1.0);
        assert_eq!(state, MovementState::Falling);
    }
}
