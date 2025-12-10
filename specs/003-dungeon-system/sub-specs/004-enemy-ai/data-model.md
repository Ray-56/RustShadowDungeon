# Data Model: Enemy AI System

**Feature**: 004-enemy-ai  
**Date**: 2025-01-27  
**Status**: Complete

## Overview

本文档定义敌人 AI 系统的所有数据结构和实体。遵循 DDD 架构原则：领域层使用纯 Rust 类型（零 Bevy 依赖），基础设施层使用 Bevy ECS 组件和资源。

## Domain Layer (Pure Rust)

### AIState

AI 状态枚举，用于领域逻辑判断。

```rust
// src/domain/enemy/ai.rs

/// AI 状态枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AIState {
    /// 巡逻状态 - 敌人在无目标时移动
    Patrol,
    /// 追逐状态 - 敌人发现目标并追逐
    Chase,
    /// 攻击状态 - 敌人在攻击范围内执行攻击
    Attack,
    /// 返回状态 - 敌人脱战后返回初始位置
    Return,
}
```

**用途**: 领域层函数使用此枚举判断 AI 状态，决定状态转换。

### Direction

方向向量，用于计算移动方向。

```rust
// src/domain/enemy/ai.rs

use bevy::math::Vec2;

/// 方向向量类型别名
pub type Direction = Vec2;

/// 计算追逐方向
pub fn calculate_chase_direction(
    enemy_pos: Vec2,
    target_pos: Vec2,
) -> Direction {
    (target_pos - enemy_pos).normalize()
}
```

**用途**: 领域层函数计算敌人移动方向。

### Position

位置坐标，用于领域层计算。

```rust
// src/domain/enemy/ai.rs

use bevy::math::Vec2;

/// 位置类型别名
pub type Position = Vec2;
```

**用途**: 领域层函数计算距离、方向等。

## Infrastructure Layer (Bevy ECS)

### Components (Pure Data Structures)

#### EnemyAI

敌人 AI 核心组件，存储当前状态和状态历史。

```rust
// src/infrastructure/components/enemy.rs

use bevy::prelude::*;
use crate::domain::enemy::ai::AIState;

/// 敌人 AI 组件 - 存储 AI 状态和配置
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
```

**字段说明**:
- `state`: 当前 AI 状态（Patrol, Chase, Attack, Return）
- `previous_state`: 前一状态（用于状态转换逻辑）
- `state_timer`: 状态持续时间（秒）
- `state_entered_at`: 状态进入时间（用于计算持续时间）

**生命周期**: 敌人生成时创建，敌人死亡时删除。

#### Perception

感知组件，存储感知范围和当前感知状态。

```rust
// src/infrastructure/components/enemy.rs

use bevy::prelude::*;
use bevy::math::Vec2;

/// 感知组件 - 存储感知范围和当前感知状态
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
    /// 是否有视线（Line of Sight）
    pub has_line_of_sight: bool,
    /// 上次检测时间（用于节流）
    pub last_check_time: f32,
    /// 检测间隔（秒，用于性能优化）
    pub check_interval: f32,
}
```

**字段说明**:
- `detection_range`: 检测范围（像素），敌人可以检测到玩家的最大距离
- `attack_range`: 攻击范围（像素），敌人可以攻击的最大距离
- `aggro_drop_range`: 脱战范围（像素），超过此范围放弃追逐
- `target_position`: 当前目标位置（如果检测到）
- `has_line_of_sight`: 是否有视线（Line of Sight，使用简化的射线检测检查障碍物碰撞）
- `last_check_time`: 上次检测时间（用于节流）
- `check_interval`: 检测间隔（秒，用于性能优化，默认 0.1 秒，对应每 3-5 帧检测一次）

#### AggroTarget

仇恨目标组件，存储当前目标和仇恨信息。

```rust
// src/infrastructure/components/enemy.rs

use bevy::prelude::*;
use bevy::math::Vec2;

/// 仇恨目标组件 - 存储当前目标和仇恨信息
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
```

**字段说明**:
- `current_target`: 当前目标实体（通常是玩家）
- `aggro_value`: 仇恨值（未来扩展，当前简化实现）
- `last_seen_position`: 最后看到目标的位置（用于丢失目标时的处理）
- `time_since_last_seen`: 自上次看到目标后的时间（秒）
- `max_time_without_sight`: 最大无视线时间（秒，超过此时间放弃目标）

#### PatrolConfig

巡逻配置组件，存储巡逻行为参数。

```rust
// src/infrastructure/components/enemy.rs

use bevy::prelude::*;
use bevy::math::Vec2;

/// 巡逻配置组件 - 存储巡逻行为参数
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
    /// 是否使用预设路径点（false = 随机点）
    pub use_waypoints: bool,
    /// 预设路径点列表（如果使用）
    pub waypoints: Vec<Vec2>,
    /// 当前路径点索引
    pub current_waypoint_index: usize,
}
```

**字段说明**:
- `spawn_position`: 生成位置（巡逻中心）
- `patrol_radius`: 巡逻半径（像素）
- `wait_time`: 到达目标点后等待时间（秒）
- `current_target`: 当前巡逻目标位置
- `wait_timer`: 等待计时器（秒）
- `use_waypoints`: 是否使用预设路径点（false = 随机点巡逻，true = 预设路径点巡逻）
- `waypoints`: 预设路径点列表（如果使用预设路径点模式）
- `current_waypoint_index`: 当前路径点索引（用于预设路径点模式）

#### AttackConfig

攻击配置组件，存储攻击行为参数。

```rust
// src/infrastructure/components/enemy.rs

use bevy::prelude::*;

/// 攻击类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttackType {
    /// 近战攻击
    Melee,
    /// 远程攻击
    Ranged,
}

/// 攻击配置组件 - 存储攻击行为参数
#[derive(Component, Debug, Clone)]
pub struct AttackConfig {
    /// 攻击范围（像素）
    pub attack_range: f32,
    /// 攻击冷却时间（秒）
    pub attack_cooldown: f32,
    /// 当前冷却时间（秒）
    pub current_cooldown: f32,
    /// 攻击伤害
    pub attack_damage: f32,
    /// 攻击类型
    pub attack_type: AttackType,
    /// 攻击动画持续时间（秒）
    pub attack_animation_duration: f32,
    /// 攻击判定帧（攻击动画中的有效帧）
    pub attack_hit_frame: f32,
}
```

**字段说明**:
- `attack_range`: 攻击范围（像素）
- `attack_cooldown`: 攻击冷却时间（秒，从配置文件 `assets/data/enemies.ron` 中加载，每个敌人类型独立配置）
- `current_cooldown`: 当前冷却时间（秒）
- `attack_damage`: 攻击伤害
- `attack_type`: 攻击类型（Melee, Ranged）
- `attack_animation_duration`: 攻击动画持续时间（秒）
- `attack_hit_frame`: 攻击判定帧时间（秒，相对于攻击动画开始的时间，从配置文件中指定）

### Resources (Global State)

#### AIConfig

AI 全局配置资源（可选）。

```rust
// src/infrastructure/resources/enemy.rs

use bevy::prelude::*;

/// AI 全局配置资源（可选）
#[derive(Resource, Debug)]
pub struct AIConfig {
    /// 默认检测范围（像素）
    pub default_detection_range: f32,
    /// 默认攻击范围（像素）
    pub default_attack_range: f32,
    /// 默认脱战范围（像素）
    pub default_aggro_drop_range: f32,
    /// 默认感知检查间隔（秒）
    pub default_perception_check_interval: f32,
    /// 是否启用调试可视化
    pub debug_visualization: bool,
}
```

**字段说明**:
- `default_detection_range`: 默认检测范围（像素）
- `default_attack_range`: 默认攻击范围（像素）
- `default_aggro_drop_range`: 默认脱战范围（像素）
- `default_perception_check_interval`: 默认感知检查间隔（秒）
- `debug_visualization`: 是否启用调试可视化

**用途**: 提供全局 AI 配置，可以在运行时调整。

### Events (Inter-System Communication)

#### EnemyDetectedPlayer

敌人检测到玩家事件。

```rust
// src/infrastructure/events/enemy.rs

use bevy::prelude::*;
use bevy::math::Vec2;

/// 敌人检测到玩家事件
#[derive(Event, Debug, Clone)]
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
```

**触发时机**: 敌人感知系统检测到玩家在检测范围内且有视线时。

**监听系统**: AI 状态机系统监听此事件，切换到 Chase 状态。

#### EnemyLostTarget

敌人丢失目标事件。

```rust
// src/infrastructure/events/enemy.rs

use bevy::prelude::*;

/// 敌人丢失目标事件
#[derive(Event, Debug, Clone)]
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
```

**触发时机**: 敌人感知系统检测到目标超出脱战范围或失去视线超过最大时间时。

**监听系统**: AI 状态机系统监听此事件，切换到 Return 或 Patrol 状态。

#### EnemyAttackTriggered

敌人攻击触发事件。

```rust
// src/infrastructure/events/enemy.rs

use bevy::prelude::*;
use crate::infrastructure::components::enemy::AttackType;

/// 敌人攻击触发事件
#[derive(Event, Debug, Clone)]
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
```

**触发时机**: 敌人进入 Attack 状态且攻击冷却完成时。

**监听系统**: 战斗系统监听此事件，处理伤害计算和判定。

## Configuration Files (RON Format)

### Enemy AI Configuration Schema

```ron
// assets/data/enemies.ron

(
    enemies: [
        (
            enemy_type: "goblin",
            ai_config: (
                detection_range: 200.0,
                attack_range: 32.0,
                aggro_drop_range: 400.0,
                patrol_radius: 100.0,
                wait_time: 2.0,
                use_waypoints: false,  // false = 随机点巡逻，true = 预设路径点
                waypoints: [],  // 预设路径点列表（如果 use_waypoints = true）
            ),
            attack_config: (
                attack_range: 32.0,
                attack_cooldown: 1.5,  // 每个敌人类型独立配置
                attack_damage: 10.0,
                attack_type: "melee",
                attack_hit_frame: 0.2,  // 攻击判定帧时间（秒，相对于动画开始）
            ),
        ),
        (
            enemy_type: "archer",
            ai_config: (
                detection_range: 300.0,
                attack_range: 250.0,
                aggro_drop_range: 500.0,
                patrol_radius: 80.0,
                wait_time: 1.5,
            ),
            attack_config: (
                attack_range: 250.0,
                attack_cooldown: 2.0,
                attack_damage: 8.0,
                attack_type: "ranged",
            ),
        ),
    ],
)
```

**字段说明**:
- `enemy_type`: 敌人类型 ID（字符串）
- `ai_config`: AI 配置
  - `detection_range`: 检测范围（f32，像素）
  - `attack_range`: 攻击范围（f32，像素）
  - `aggro_drop_range`: 脱战范围（f32，像素）
  - `patrol_radius`: 巡逻半径（f32，像素，用于随机点巡逻）
  - `wait_time`: 等待时间（f32，秒，到达目标点后等待时间）
  - `use_waypoints`: 是否使用预设路径点（bool，false = 随机点巡逻，true = 预设路径点巡逻）
  - `waypoints`: 预设路径点列表（Vec<[f32; 2]>，如果 use_waypoints = true）
- `attack_config`: 攻击配置
  - `attack_range`: 攻击范围（f32，像素）
  - `attack_cooldown`: 攻击冷却（f32，秒，每个敌人类型独立配置）
  - `attack_damage`: 攻击伤害（f32）
  - `attack_type`: 攻击类型（字符串，"melee" 或 "ranged"）
  - `attack_hit_frame`: 攻击判定帧时间（f32，秒，相对于攻击动画开始的时间）

## State Transitions

### AI State Machine

```
Patrol (初始状态)
    ↓ (检测到玩家)
Chase
    ↓ (玩家进入攻击范围)
Attack
    ↓ (攻击完成)
Chase (如果玩家仍在范围内)
    ↓ (玩家离开脱战范围或失去视线)
Return
    ↓ (返回原位)
Patrol
```

**状态转换规则**:
- **Patrol → Chase**: 检测到玩家在检测范围内且有视线
- **Chase → Attack**: 玩家进入攻击范围且攻击冷却完成
- **Attack → Chase**: 攻击完成，玩家仍在范围内但不在攻击范围
- **Chase → Return**: 玩家超出脱战范围或失去视线超过最大时间
- **Return → Patrol**: 返回生成位置
- **任何状态 → Patrol**: 目标死亡

### State Transition Conditions

```rust
// Domain layer functions

// Patrol → Chase
pub fn should_transition_to_chase(
    distance: f32,
    detection_range: f32,
    has_line_of_sight: bool,
) -> bool {
    distance <= detection_range && has_line_of_sight
}

// Chase → Attack
pub fn should_transition_to_attack(
    distance: f32,
    attack_range: f32,
    cooldown_ready: bool,
) -> bool {
    distance <= attack_range && cooldown_ready
}

// Chase → Return
pub fn should_drop_aggro(
    distance: f32,
    aggro_drop_range: f32,
    time_since_last_seen: f32,
    max_time_without_sight: f32,
) -> bool {
    distance > aggro_drop_range || time_since_last_seen > max_time_without_sight
}
```

## Relationships

### Entity Relationships

```
Enemy (1) ──→ (1) EnemyAI
Enemy (1) ──→ (1) Perception
Enemy (1) ──→ (1) AggroTarget
Enemy (1) ──→ (1) PatrolConfig (optional)
Enemy (1) ──→ (1) AttackConfig
AggroTarget (N) ──→ (1) Player (current_target)
```

### Component Dependencies

- `EnemyAI` requires `Perception` and `AggroTarget`
- `PatrolConfig` is optional (only for enemies with patrol behavior)
- `AttackConfig` is required for all enemies

## Validation Rules

### AI Configuration Validation

- `detection_range` must be > 0
- `attack_range` must be > 0 and <= `detection_range`
- `aggro_drop_range` must be >= `detection_range`
- `patrol_radius` must be > 0 (if patrol enabled)
- `attack_cooldown` must be > 0
- `attack_damage` must be >= 0

### State Transition Validation

- State transitions must follow the state machine rules
- Cannot transition from Attack to Patrol directly (must go through Return)
- Cannot transition to Attack if cooldown not ready

## Data Flow

### Perception Flow

```
1. perception_system runs (throttled, every 3-5 frames)
2. For each enemy:
   - Calculate distance to player
   - Check line of sight (simplified raycast)
   - Update Perception component
   - If detected: Emit EnemyDetectedPlayer event
   - If lost: Emit EnemyLostTarget event
```

### State Machine Flow

```
1. ai_state_machine_system runs (every frame)
2. For each enemy:
   - Check current state
   - Evaluate transition conditions (call domain functions)
   - If condition met: Transition state
   - Update state timer and history
```

### Attack Flow

```
1. attack_system runs (every frame)
2. For enemies in Attack state:
   - Check if attack animation complete
   - If hit frame reached: Emit EnemyAttackTriggered event
   - Update attack cooldown
   - After attack: Transition back to Chase or Return
```

