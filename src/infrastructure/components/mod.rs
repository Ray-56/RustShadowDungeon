//! ECS Components - pure data structures
//!
//! Components follow Bevy ECS best practices: pure data, no behavior.

/// Animation state components
pub mod animation;
/// Camera follow components
pub mod camera;
pub mod collectible;
pub mod combat;
pub mod enemy;
pub mod health;
pub mod player;
/// UI components
pub mod ui;

pub use animation::AnimationState;
pub use camera::{CameraFollow, ScreenShake};
pub use collectible::Coin;
pub use combat::{
    AttackAnimation, Combo, Fireball, HitBox, HurtBox, Invincibility, Lifetime, Particle, Skill,
    Stats,
};
pub use enemy::{Enemy, EnemyId, EnemyType, PatrolBehavior};
pub use health::{Health, InvincibilityTimer};
pub use player::{
    GroundedState, InputState, MovementStateComponent, PixelSnap, Player, VelocityComponent, MP,
};
pub use ui::DamageNumber;
