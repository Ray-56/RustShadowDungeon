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
/// Particle pool resource for object pooling
pub mod particle_pool;
/// Skill database resource for loading skill configurations
pub mod skill_database;

// Dungeon system resources (M3)
/// Dungeon session and configuration resources
pub mod dungeon;

// Enemy AI system resources (M3)
/// Enemy AI configuration resources
pub mod enemy;

// Loot and inventory system resources (M3)
/// Loot and inventory configuration resources
pub mod loot;

// Boss encounter system resources (M3)
/// Boss configuration resources
pub mod boss_config;

pub use animation::PlayerAnimations;
pub use movement_config::MovementConfig;
pub use score::Score;

// Combat resources
pub use combat_config::CombatConfig;
pub use hitfreeze::HitfreezeTimer;
pub use particle_pool::ParticlePool;
pub use skill_database::{SkillData, SkillDatabase};

// Dungeon resources
pub use dungeon::{DoorConfig, DungeonConfig, DungeonSession, RoomConfig};

// Enemy AI resources
pub use enemy::AIConfig;

// Loot and inventory resources
pub use loot::{LootConfig, PickupConfig, PickupMode};

// Boss encounter resources
pub use boss_config::{BossConfig, BossDefinition};
