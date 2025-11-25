//! Input domain model
//!
//! Pure Rust data structures for input representation.

/// Input direction for movement
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputDirection {
    /// Move left
    Left,
    /// Move right
    Right,
    /// No horizontal movement
    #[default]
    None,
}

impl InputDirection {
    /// Convert to velocity factor (-1.0, 0.0, 1.0)
    #[must_use]
    pub const fn to_velocity_factor(&self) -> f32 {
        match self {
            Self::Left => -1.0,
            Self::Right => 1.0,
            Self::None => 0.0,
        }
    }
}

/// Input state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Input {
    /// Movement direction
    pub direction: InputDirection,
}

impl Input {
    /// Create new input with direction
    #[must_use]
    pub const fn new(direction: InputDirection) -> Self {
        Self { direction }
    }
}
