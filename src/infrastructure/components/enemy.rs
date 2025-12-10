//! Enemy components

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Enemy marker component
#[derive(Component, Debug)]
pub struct Enemy;

/// Enemy ID for identification
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EnemyId(pub u32);

/// Enemy type enum
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EnemyType {
    /// Slime enemy (basic enemy type)
    Slime,
}

/// 攻击类型
///
/// Attack type for enemy attacks
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AttackType {
    /// 近战攻击
    Melee,
    /// 远程攻击
    Ranged,
}

/// Patrol behavior - enemy moves back and forth
#[derive(Component, Debug)]
pub struct PatrolBehavior {
    /// Left boundary of patrol area
    pub left_bound: f32,
    /// Right boundary of patrol area
    pub right_bound: f32,
    /// Current movement direction (1.0 = right, -1.0 = left)
    pub direction: f32,
    /// Movement speed (pixels per second)
    pub speed: f32,
}

impl PatrolBehavior {
    /// Create a new patrol behavior
    #[must_use]
    pub fn new(center_x: f32, patrol_range: f32, speed: f32) -> Self {
        Self {
            left_bound: center_x - patrol_range,
            right_bound: center_x + patrol_range,
            direction: 1.0, // Start moving right
            speed,
        }
    }
}

/// Hit flash effect for visual feedback when enemy takes damage
///
/// 受伤闪烁效果，用于敌人受伤时的视觉反馈
#[derive(Component, Debug)]
pub struct HitFlash {
    /// Remaining duration of the flash effect (seconds)
    pub duration: f32,
    /// Original sprite color (to restore after flash)
    pub original_color: Color,
}

impl HitFlash {
    /// Create a new hit flash effect
    pub fn new(duration: f32, original_color: Color) -> Self {
        Self { duration, original_color }
    }
}

/// Death animation component for enemy death visual feedback
///
/// 死亡动画组件，用于敌人死亡时的视觉反馈
#[derive(Component, Debug)]
pub struct DeathAnimation {
    /// Remaining duration of death animation (seconds)
    pub duration: f32,
    /// Initial scale (for shrink effect)
    pub initial_scale: Vec3,
    /// Target scale (usually 0.0 for shrink to nothing)
    pub target_scale: Vec3,
}

impl DeathAnimation {
    /// Create a new death animation
    pub fn new(duration: f32, initial_scale: Vec3) -> Self {
        Self { duration, initial_scale, target_scale: Vec3::ZERO }
    }
}

// ============================================================================
// Enemy AI Components (004-enemy-ai)
// ============================================================================

use crate::domain::enemy::ai::AIState;

/// 敌人 AI 组件 - 存储 AI 状态和配置
///
/// Enemy AI component - stores AI state and configuration.
/// 敌人 AI 组件，存储当前状态和状态历史
#[derive(Component, Debug, Clone)]
pub struct EnemyAI {
    /// 当前 AI 状态
    pub state: AIState,
    /// 前一状态（用于状态转换逻辑）
    pub previous_state: AIState,
    /// 状态持续时间（秒）
    pub state_timer: f32,
    /// 状态进入时间（用于计算持续时间）
    pub state_entered_at: f32,
}

impl EnemyAI {
    /// Create a new EnemyAI component with initial Patrol state
    pub fn new() -> Self {
        Self {
            state: AIState::Patrol,
            previous_state: AIState::Patrol,
            state_timer: 0.0,
            state_entered_at: 0.0,
        }
    }
}

impl Default for EnemyAI {
    fn default() -> Self {
        Self::new()
    }
}

/// 感知组件 - 存储感知范围和当前感知状态
///
/// Perception component - stores perception range and current perception state.
/// 感知组件，存储感知范围和当前感知状态
#[derive(Component, Debug, Clone)]
pub struct Perception {
    /// 检测范围（像素）
    pub detection_range: f32,
    /// 攻击范围（像素）
    pub attack_range: f32,
    /// 脱战范围（像素，超过此范围放弃追逐）
    pub aggro_drop_range: f32,
    /// 当前目标位置（如果检测到）
    pub target_position: Option<Vec2>,
    /// 是否有视线（Line of Sight，使用简化的射线检测检查障碍物碰撞）
    pub has_line_of_sight: bool,
    /// 上次检测时间（用于节流）
    pub last_check_time: f32,
    /// 检测间隔（秒，用于性能优化，默认 0.1 秒，对应每 3-5 帧检测一次）
    pub check_interval: f32,
}

impl Perception {
    /// Create a new Perception component with default values
    pub fn new(detection_range: f32, attack_range: f32, aggro_drop_range: f32) -> Self {
        Self {
            detection_range,
            attack_range,
            aggro_drop_range,
            target_position: None,
            has_line_of_sight: false,
            last_check_time: 0.0,
            check_interval: 0.1, // ~3-5 frames at 60 FPS
        }
    }
}

/// 仇恨目标组件 - 存储当前目标和仇恨信息
///
/// Aggro target component - stores current target and aggro information.
/// 仇恨目标组件，存储当前目标和仇恨信息
#[derive(Component, Debug, Clone)]
pub struct AggroTarget {
    /// 当前目标实体
    pub current_target: Option<Entity>,
    /// 仇恨值（未来扩展，当前简化实现）
    pub aggro_value: f32,
    /// 最后看到目标的位置
    pub last_seen_position: Option<Vec2>,
    /// 自上次看到目标后的时间（秒）
    pub time_since_last_seen: f32,
    /// 最大无视线时间（秒，超过此时间放弃目标）
    pub max_time_without_sight: f32,
}

impl AggroTarget {
    /// Create a new AggroTarget component
    pub fn new(max_time_without_sight: f32) -> Self {
        Self {
            current_target: None,
            aggro_value: 0.0,
            last_seen_position: None,
            time_since_last_seen: 0.0,
            max_time_without_sight,
        }
    }
}

impl Default for AggroTarget {
    fn default() -> Self {
        Self::new(5.0) // Default 5 seconds
    }
}

/// 巡逻配置组件 - 存储巡逻行为参数
///
/// Patrol configuration component - stores patrol behavior parameters.
/// 巡逻配置组件，存储巡逻行为参数
#[derive(Component, Debug, Clone)]
pub struct PatrolConfig {
    /// 生成位置（巡逻中心）
    pub spawn_position: Vec2,
    /// 巡逻半径（像素）
    pub patrol_radius: f32,
    /// 到达目标点后等待时间（秒）
    pub wait_time: f32,
    /// 当前巡逻目标位置
    pub current_target: Option<Vec2>,
    /// 等待计时器（秒）
    pub wait_timer: f32,
    /// 是否使用预设路径点（false = 随机点巡逻，true = 预设路径点巡逻）
    pub use_waypoints: bool,
    /// 预设路径点列表（如果使用预设路径点模式）
    pub waypoints: Vec<Vec2>,
    /// 当前路径点索引（用于预设路径点模式）
    pub current_waypoint_index: usize,
}

impl PatrolConfig {
    /// Create a new PatrolConfig with random point patrol mode
    pub fn new_random(spawn_position: Vec2, patrol_radius: f32, wait_time: f32) -> Self {
        Self {
            spawn_position,
            patrol_radius,
            wait_time,
            current_target: None,
            wait_timer: 0.0,
            use_waypoints: false,
            waypoints: Vec::new(),
            current_waypoint_index: 0,
        }
    }

    /// Create a new PatrolConfig with waypoint patrol mode
    pub fn new_waypoint(spawn_position: Vec2, waypoints: Vec<Vec2>, wait_time: f32) -> Self {
        Self {
            spawn_position,
            patrol_radius: 0.0, // Not used in waypoint mode
            wait_time,
            current_target: None,
            wait_timer: 0.0,
            use_waypoints: true,
            waypoints,
            current_waypoint_index: 0,
        }
    }
}

/// 攻击配置组件 - 存储攻击行为参数
///
/// Attack configuration component - stores attack behavior parameters.
/// 攻击配置组件，存储攻击行为参数
#[derive(Component, Debug, Clone)]
pub struct AttackConfig {
    /// 攻击范围（像素）
    pub attack_range: f32,
    /// 攻击冷却时间（秒，从配置文件 `assets/data/enemies.ron` 中加载，每个敌人类型独立配置）
    pub attack_cooldown: f32,
    /// 当前冷却时间（秒）
    pub current_cooldown: f32,
    /// 攻击伤害
    pub attack_damage: f32,
    /// 攻击类型
    pub attack_type: AttackType,
    /// 攻击动画持续时间（秒）
    pub attack_animation_duration: f32,
    /// 攻击判定帧时间（秒，相对于攻击动画开始的时间，从配置文件中指定）
    pub attack_hit_frame: f32,
}

impl AttackConfig {
    /// Create a new AttackConfig
    pub fn new(
        attack_range: f32,
        attack_cooldown: f32,
        attack_damage: f32,
        attack_type: AttackType,
        attack_animation_duration: f32,
        attack_hit_frame: f32,
    ) -> Self {
        Self {
            attack_range,
            attack_cooldown,
            current_cooldown: 0.0,
            attack_damage,
            attack_type,
            attack_animation_duration,
            attack_hit_frame,
        }
    }

    /// Check if attack cooldown is ready
    pub fn is_cooldown_ready(&self) -> bool {
        self.current_cooldown <= 0.0
    }
}
