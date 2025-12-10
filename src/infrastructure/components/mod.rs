//! ECS Components - pure data structures
//!
//! Components follow Bevy ECS best practices: pure data, no behavior.

/// Animation state components
pub mod animation;
/// Camera follow components
pub mod camera;
pub mod collectible;
pub mod combat;
pub mod dungeon;
pub mod enemy;
pub mod health;
/// Loot and inventory components
pub mod loot;
pub mod obstacle;
pub mod player;
/// UI components
pub mod ui;
/// Boss components
pub mod boss;

pub use animation::AnimationState;
pub use camera::{CameraFollow, ScreenShake};
pub use collectible::Coin;
pub use combat::{
    AttackAnimation, Combo, Fireball, HitBox, HurtBox, Invincibility, Lifetime, Particle, Skill,
    Stats,
};
pub use dungeon::{
    Door, DoorId, DoorState, DungeonManager, EnemySpawnPoint, Room, RoomId, RoomState,
};
pub use enemy::{
    AggroTarget, AttackConfig, AttackType, Enemy, EnemyAI, EnemyId, EnemyType, PatrolBehavior,
    PatrolConfig, Perception,
};
pub use health::{Health, InvincibilityTimer};
pub use loot::{
    InventoryComponent, InventorySlot, ItemDefinitionAsset, ItemId, LootTableAsset,
    LootTableEntryAsset, WorldItem,
};
pub use obstacle::{Obstacle, ObstacleCollider};
pub use player::{
    GroundedState, InputState, MovementStateComponent, PixelSnap, Player, VelocityComponent, MP,
};
pub use ui::DamageNumber;
pub use boss::{Boss, BossController, BossId, BossSkill, Telegraph};
