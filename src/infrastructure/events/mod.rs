//! ECS Events - system-to-system communication
//!
//! Events enable decoupled communication between systems.

pub mod combat;
pub mod movement;

pub use combat::{
    ComboExtended, ComboReset, ComboResetReason, DamageDealt, EnemyDefeated, HitBoxSpawned,
    InvincibilityEnded, InvincibilityStarted, SkillActivated,
};
pub use movement::{PlayerMoved, StateChanged};
