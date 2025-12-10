//! ECS Events - system-to-system communication
//!
//! Events enable decoupled communication between systems.

pub mod combat;
pub mod dungeon;
pub mod enemy;
/// Loot and inventory events
pub mod loot;
pub mod movement;
/// Boss events
pub mod boss;

pub use combat::{
    ComboExtended, ComboReset, ComboResetReason, DamageDealt, EnemyDefeated, HitBoxSpawned,
    InvincibilityEnded, InvincibilityStarted, SkillActivated,
};
pub use enemy::{EnemyAttackTriggered, EnemyDetectedPlayer, EnemyLostTarget, TargetLossReason};
pub use loot::{InventoryFull, ItemDropped, ItemPickedUp, ItemStacked};
pub use movement::{PlayerMoved, StateChanged};
pub use boss::{BossDefeated, BossEncounterStarted, BossPhaseTransition};
