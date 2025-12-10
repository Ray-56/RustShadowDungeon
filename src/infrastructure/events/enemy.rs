//! Enemy AI events
//!
//! Events for enemy AI system communication.
//!
//! 敌人 AI 系统事件定义

use bevy::ecs::message::Message;
use bevy::math::Vec2;
use bevy::prelude::*;

use crate::infrastructure::components::enemy::AttackType;

/// 敌人检测到玩家事件
///
/// Emitted when an enemy detects a player within detection range with line of sight.
/// 当敌人在检测范围内且有视线时检测到玩家时触发
#[derive(Event, Message, Debug, Clone)]
pub struct EnemyDetectedPlayer {
    /// 敌人实体
    pub enemy: Entity,
    /// 玩家实体
    pub player: Entity,
    /// 检测位置
    pub detection_position: Vec2,
    /// 距离
    pub distance: f32,
}

/// 敌人丢失目标事件
///
/// Emitted when an enemy loses its target (player out of range or lost sight).
/// 当敌人丢失目标时触发（玩家超出范围或失去视线）
#[derive(Event, Message, Debug, Clone)]
pub struct EnemyLostTarget {
    /// 敌人实体
    pub enemy: Entity,
    /// 丢失的目标实体
    pub lost_target: Entity,
    /// 丢失原因
    pub reason: TargetLossReason,
}

/// 目标丢失原因
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetLossReason {
    /// 超出脱战范围
    OutOfRange,
    /// 超出无视线时间
    LostSight,
    /// 目标死亡
    TargetDead,
}

/// 敌人攻击触发事件
///
/// Emitted when an enemy triggers an attack (at the hit frame of attack animation).
/// 当敌人触发攻击时触发（在攻击动画的判定帧）
#[derive(Event, Message, Debug, Clone)]
pub struct EnemyAttackTriggered {
    /// 敌人实体
    pub enemy: Entity,
    /// 目标实体
    pub target: Entity,
    /// 攻击伤害
    pub damage: f32,
    /// 攻击类型
    pub attack_type: AttackType,
    /// 攻击位置（用于伤害判定）
    pub attack_position: Vec2,
}
