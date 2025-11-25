//! ECS Resources - global shared state
//!
//! Resources store configuration and shared game state.

/// Animation assets and configuration
pub mod animation;
/// Movement system configuration
pub mod movement_config;
pub mod score;

pub use animation::PlayerAnimations;
pub use movement_config::MovementConfig;
pub use score::Score;
