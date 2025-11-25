//! Movement domain logic module
//!
//! Pure Rust functions for player movement calculations.
//! Zero Bevy dependencies for maximum testability.

pub mod input;
pub mod jump;
pub mod state;
pub mod velocity;

pub use input::{Input, InputDirection};
pub use jump::{apply_jump, apply_variable_jump, can_jump, JumpParams};
pub use state::{transition_state, MovementState};
pub use velocity::{calculate_air_velocity, calculate_ground_velocity, Velocity};
