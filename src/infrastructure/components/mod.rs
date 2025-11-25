//! ECS Components - pure data structures
//!
//! Components follow Bevy ECS best practices: pure data, no behavior.

/// Animation state components
pub mod animation;
/// Camera follow components
pub mod camera;
pub mod collectible;
pub mod enemy;
pub mod health;
pub mod player;

pub use animation::AnimationState;
pub use camera::CameraFollow;
pub use collectible::Coin;
pub use enemy::{Enemy, PatrolBehavior};
pub use health::{Health, InvincibilityTimer};
pub use player::{
    GroundedState, InputState, MovementStateComponent, PixelSnap, Player, VelocityComponent,
};
