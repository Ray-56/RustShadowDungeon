//! ECS Resources - global shared state
//!
//! Resources store configuration and shared game state.

/// Animation assets and configuration
pub mod animation;
/// Movement system configuration
pub mod movement_config;
pub mod score;

// Combat system resources (M2)
/// Combat system configuration (damage, hitfreeze, screen shake, etc.)
pub mod combat_config;
/// Hitfreeze timer resource for global hit stop control
pub mod hitfreeze;
/// Skill database resource for loading skill configurations
pub mod skill_database;
/// Particle pool resource for object pooling
pub mod particle_pool;

pub use animation::PlayerAnimations;
pub use movement_config::MovementConfig;
pub use score::Score;

// Combat resources
pub use combat_config::CombatConfig;
pub use hitfreeze::HitfreezeTimer;
pub use skill_database::{SkillData, SkillDatabase};
pub use particle_pool::ParticlePool;
