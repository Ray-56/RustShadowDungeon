use crate::domain::combat::{ComboState, DamageResult};
/// Combat-related events
/// 战斗相关事件
use bevy::prelude::*;

// Re-export Message as Event for backwards compatibility
use bevy::ecs::message::Message;

/// Event published when damage is dealt to an entity
///
/// 伤害造成事件
///
/// Published by collision_detection_system when HitBox hits HurtBox.
/// Consumed by apply_damage_system to update Health component.
#[derive(Message, Debug, Clone)]
pub struct DamageDealt {
    /// Entity that was damaged
    pub target: Entity,
    /// Entity that caused the damage (attacker)
    pub source: Entity,
    /// Damage calculation result
    pub result: DamageResult,
    /// Position where damage occurred (for feedback effects)
    pub position: Vec2,
}

/// Event published when combo is extended
///
/// 连击延续事件
///
/// Published by combo_system when player successfully extends combo chain.
#[derive(Message, Debug, Clone)]
pub struct ComboExtended {
    /// Player entity
    pub player: Entity,
    /// New combo count (1, 2, or 3)
    pub combo_count: u32,
    /// New combo state
    pub new_state: ComboState,
}

/// Reason why combo was reset
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComboResetReason {
    /// Combo window expired (timeout)
    Timeout,
    /// Combo chain completed (reached end)
    Completed,
    /// Player was interrupted (hit/stunned)
    Interrupted,
}

/// Event published when combo is reset
///
/// 连击重置事件
///
/// Published by combo_system when combo window expires or combo completes.
#[derive(Message, Debug, Clone)]
pub struct ComboReset {
    /// Player entity
    pub player: Entity,
    /// Final combo count before reset
    pub final_combo_count: u32,
    /// Reason for reset
    pub reason: ComboResetReason,
}

/// Event published when an enemy is defeated
///
/// 敌人死亡事件
///
/// Published by death_system when enemy Health reaches 0.
#[derive(Message, Debug, Clone)]
pub struct EnemyDefeated {
    /// Enemy entity that was defeated
    pub enemy: Entity,
    /// Position where enemy died (for effects/loot)
    pub position: Vec2,
    /// Optional: Entity that dealt killing blow
    pub killed_by: Option<Entity>,
}

/// Event published when a skill is activated
///
/// 技能激活事件
///
/// Published by skill_input_system when player triggers a skill.
#[derive(Message, Debug, Clone)]
pub struct SkillActivated {
    /// Player entity that activated skill
    pub player: Entity,
    /// Skill ID (e.g., "fireball")
    pub skill_id: String,
    /// Target position for skill (e.g., projectile direction)
    pub target_position: Vec2,
}

/// Event published when invincibility frames start
///
/// 无敌帧开始事件
#[derive(Message, Debug, Clone)]
pub struct InvincibilityStarted {
    /// Entity that became invincible
    pub entity: Entity,
    /// Duration of invincibility (seconds)
    pub duration: f32,
}

/// Event published when invincibility frames end
///
/// 无敌帧结束事件
#[derive(Message, Debug, Clone)]
pub struct InvincibilityEnded {
    /// Entity that lost invincibility
    pub entity: Entity,
}

/// Event published when a HitBox is spawned
///
/// HitBox 生成事件
///
/// Published by attack systems when creating attack hitboxes.
#[derive(Message, Debug, Clone)]
pub struct HitBoxSpawned {
    /// HitBox entity
    pub hitbox: Entity,
    /// Owner entity (e.g., player or enemy)
    pub owner: Entity,
    /// Base damage of the hitbox
    pub damage: f32,
}
