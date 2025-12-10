# Phase 1 数据模型：Boss 遭遇战系统

**Created**: 2025-12-03
**Feature**: 006-boss-encounter
**Purpose**: 定义 ECS 组件、资源、事件和领域模型

---

## 数据模型概述

本文档定义 Boss 遭遇战系统的所有数据结构，严格遵循 DDD（领域驱动设计）架构和 Bevy ECS 模式。数据分为三层：

1. **领域层（Domain Layer）**: 纯 Rust 数据结构，无 Bevy 依赖
2. **基础设施层（Infrastructure Layer）**: Bevy ECS 组件、资源、事件
3. **配置层（Configuration Layer）**: RON 文件配置

---

## 1. 领域层数据模型（Domain Layer）

### 1.1 BossPhase（Boss 阶段定义）

**位置**: `src/domain/boss/phase.rs`

**用途**: 定义 Boss 的单个阶段属性

```rust
/// Boss 阶段定义（纯数据结构）
#[derive(Debug, Clone, PartialEq)]
pub struct BossPhase {
    pub phase_index: usize,                    // 阶段索引（0-based）
    pub health_threshold: f32,                 // 血量阈值（0.0-1.0）
    pub invulnerability_duration: f32,         // 无敌时间（秒，1-2秒）
    pub skill_ids: Vec<String>,                // 可用技能 ID 列表
    pub attack_frequency: f32,                 // 攻击频率（每秒攻击次数）
    pub move_speed: f32,                       // 移动速度（像素/秒）
}

impl BossPhase {
    /// 检查是否达到此阶段阈值
    pub fn is_threshold_reached(&self, health_percentage: f32) -> bool {
        health_percentage <= self.health_threshold
    }
}
```

---

### 1.2 PhaseTransition（阶段转换逻辑）

**位置**: `src/domain/boss/phase_transition.rs`

**用途**: 阶段转换的纯逻辑函数

```rust
use super::BossPhase;

/// 阶段转换结果
#[derive(Debug, Clone, PartialEq)]
pub enum PhaseTransitionResult {
    NoTransition,           // 无转换
    Transition {
        from_phase: usize,
        to_phase: usize,
    },
}

/// 检查是否需要阶段转换
pub fn check_phase_transition(
    current_phase: usize,
    health_percentage: f32,
    phases: &[BossPhase],
    is_locked: bool,
) -> PhaseTransitionResult {
    if is_locked {
        return PhaseTransitionResult::NoTransition;
    }
    
    // 查找下一个阶段
    if current_phase + 1 >= phases.len() {
        return PhaseTransitionResult::NoTransition;
    }
    
    let next_phase = &phases[current_phase + 1];
    if next_phase.is_threshold_reached(health_percentage) {
        PhaseTransitionResult::Transition {
            from_phase: current_phase,
            to_phase: current_phase + 1,
        }
    } else {
        PhaseTransitionResult::NoTransition
    }
}
```

---

### 1.3 TelegraphArea（预警区域计算）

**位置**: `src/domain/boss/telegraph.rs`

**用途**: 预警区域的几何计算（纯数学）

```rust
/// 预警区域形状
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TelegraphShape {
    Circle { radius: f32 },
    Rectangle { width: f32, height: f32 },
    Sector { radius: f32, angle: f32 },  // 扇形（角度制）
}

/// 预警区域定义
#[derive(Debug, Clone, PartialEq)]
pub struct TelegraphArea {
    pub shape: TelegraphShape,
    pub center: (f32, f32),  // 中心点坐标
    pub rotation: f32,       // 旋转角度（弧度）
}

impl TelegraphArea {
    /// 检查点是否在预警区域内
    pub fn contains_point(&self, point: (f32, f32)) -> bool {
        // 实现基于形状的点包含检测
        match self.shape {
            TelegraphShape::Circle { radius } => {
                let dx = point.0 - self.center.0;
                let dy = point.1 - self.center.1;
                (dx * dx + dy * dy).sqrt() <= radius
            }
            TelegraphShape::Rectangle { width, height } => {
                // 考虑旋转的矩形检测
                // ... 实现细节
                false
            }
            TelegraphShape::Sector { .. } => {
                // 扇形检测
                // ... 实现细节
                false
            }
        }
    }
}
```

---

### 1.4 SkillPriority（技能优先级队列）

**位置**: `src/domain/boss/skill_priority.rs`

**用途**: 技能选择的优先级逻辑

```rust
/// 技能优先级
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SkillPriority {
    Low = 1,
    Medium = 2,
    High = 3,
    Critical = 4,
}

/// 技能冷却状态
#[derive(Debug, Clone)]
pub struct SkillCooldown {
    pub skill_id: String,
    pub cooldown_remaining: f32,  // 剩余冷却时间（秒）
    pub priority: SkillPriority,
}

impl SkillCooldown {
    /// 检查技能是否可用
    pub fn is_available(&self) -> bool {
        self.cooldown_remaining <= 0.0
    }
}

/// 选择下一个可用技能（考虑优先级和随机性）
pub fn select_next_skill(
    available_skills: &[SkillCooldown],
    randomness_factor: f32,  // 0.0 = 完全按优先级，1.0 = 完全随机
) -> Option<String> {
    let available: Vec<_> = available_skills
        .iter()
        .filter(|s| s.is_available())
        .collect();
    
    if available.is_empty() {
        return None;
    }
    
    // 简单的优先级加权随机选择
    // 高优先级技能有更高概率被选中，但不保证
    // ... 实现细节
    
    available.first().map(|s| s.skill_id.clone())
}
```

---

## 2. 基础设施层数据模型（Infrastructure Layer）

### 2.1 ECS 组件（Components）

#### Boss（Boss 标记）

**位置**: `src/infrastructure/components/boss.rs`

**用途**: 标记 Boss 实体

```rust
use bevy::prelude::*;

/// Boss 组件（标记组件）
#[derive(Component, Debug)]
pub struct Boss;
```

---

#### BossController（Boss 控制器）

**位置**: `src/infrastructure/components/boss.rs`

**用途**: Boss 的核心控制组件

```rust
use crate::domain::boss::BossPhase;

/// Boss 控制器组件
#[derive(Component, Debug, Clone)]
pub struct BossController {
    pub current_phase: usize,                    // 当前阶段索引
    pub phases: Vec<BossPhase>,                  // 所有阶段配置
    pub health_lock: bool,                       // 血量锁定标志
    pub transition_start_time: Option<f32>,      // 阶段转换开始时间（秒）
    pub is_transitioning: bool,                  // 是否正在转换阶段
}

impl BossController {
    /// 创建新的 Boss 控制器
    pub fn new(phases: Vec<BossPhase>) -> Self {
        Self {
            current_phase: 0,
            phases,
            health_lock: false,
            transition_start_time: None,
            is_transitioning: false,
        }
    }
    
    /// 获取当前阶段
    pub fn current_phase_config(&self) -> Option<&BossPhase> {
        self.phases.get(self.current_phase)
    }
    
    /// 获取下一阶段阈值
    pub fn get_next_phase_threshold(&self) -> Option<f32> {
        if self.current_phase + 1 < self.phases.len() {
            Some(self.phases[self.current_phase + 1].health_threshold)
        } else {
            None
        }
    }
}
```

---

#### Telegraph（预警组件）

**位置**: `src/infrastructure/components/boss.rs`

**用途**: 技能预警区域标记

```rust
use crate::domain::boss::TelegraphArea;

/// 技能预警组件
#[derive(Component, Debug, Clone)]
pub struct Telegraph {
    pub skill_id: String,                        // 关联的技能 ID
    pub area: TelegraphArea,                     // 预警区域定义
    pub warning_duration: f32,                   // 预警持续时间（秒，至少 0.5s）
    pub elapsed_time: f32,                       // 已过去时间
    pub damage_area: TelegraphArea,              // 实际伤害区域（用于验证）
}
```

---

#### BossSkill（Boss 技能状态）

**位置**: `src/infrastructure/components/boss.rs`

**用途**: Boss 技能冷却和状态

```rust
/// Boss 技能状态组件（附加到 Boss 实体）
#[derive(Component, Debug, Clone)]
pub struct BossSkill {
    pub skill_id: String,
    pub cooldown_remaining: f32,                 // 剩余冷却时间
    pub priority: u8,                            // 优先级（1-4）
    pub is_on_cooldown: bool,                    // 是否在冷却中
}
```

---

### 2.2 资源（Resources）

#### BossConfig（Boss 配置资源）

**位置**: `src/infrastructure/resources/boss_config.rs`

**用途**: 存储从 RON 文件加载的 Boss 定义

```rust
use bevy::prelude::*;
use crate::domain::boss::BossPhase;
use serde::{Deserialize, Serialize};

/// Boss 定义（从 RON 文件加载）
#[derive(Debug, Clone, Deserialize, Serialize, Resource)]
pub struct BossDefinition {
    pub id: String,
    pub name: String,
    pub max_health: f32,
    pub phases: Vec<BossPhase>,
}

/// Boss 配置资源
#[derive(Resource, Debug)]
pub struct BossConfig {
    pub bosses: Vec<BossDefinition>,
}

impl BossConfig {
    /// 根据 ID 查找 Boss 定义
    pub fn get_boss(&self, boss_id: &str) -> Option<&BossDefinition> {
        self.bosses.iter().find(|b| b.id == boss_id)
    }
}
```

---

### 2.3 事件（Events）

#### BossEncounterStarted（Boss 遭遇开始）

**位置**: `src/infrastructure/events/boss.rs`

**用途**: Boss 遭遇战开始事件

```rust
use bevy::prelude::*;

/// Boss 遭遇战开始事件
#[derive(Event, Debug, Clone)]
pub struct BossEncounterStarted {
    pub boss_entity: Entity,
    pub boss_id: String,
}
```

---

#### BossPhaseTransition（Boss 阶段转换）

**位置**: `src/infrastructure/events/boss.rs`

**用途**: Boss 阶段转换事件

```rust
/// Boss 阶段转换事件
#[derive(Event, Debug, Clone)]
pub struct BossPhaseTransition {
    pub boss_entity: Entity,
    pub from_phase: usize,
    pub to_phase: usize,
}
```

---

#### BossDefeated（Boss 被击败）

**位置**: `src/infrastructure/events/boss.rs`

**用途**: Boss 被击败事件（复用 Dungeon Context 的事件）

```rust
/// Boss 被击败事件
#[derive(Event, Debug, Clone)]
pub struct BossDefeated {
    pub boss_entity: Entity,
    pub boss_id: String,
}
```

---

## 3. 配置层数据模型（Configuration Layer）

### 3.1 bosses.ron（Boss 定义配置文件）

**位置**: `assets/data/bosses.ron`

**用途**: Boss 定义的 RON 配置文件

```ron
// Boss 定义配置文件
(
    bosses: [
        (
            id: "orc_warlord",
            name: "兽人战将",
            max_health: 1000.0,
            phases: [
                (
                    phase_index: 0,
                    health_threshold: 1.0,  // 100% 起始
                    invulnerability_duration: 1.5,  // 1.5 秒无敌
                    skill_ids: ["basic_attack", "ground_slam"],
                    attack_frequency: 2.0,  // 每秒 2 次攻击
                    move_speed: 50.0,
                ),
                (
                    phase_index: 1,
                    health_threshold: 0.5,  // 50% 触发
                    invulnerability_duration: 2.0,
                    skill_ids: ["frenzy_attack", "ground_slam", "charge"],
                    attack_frequency: 3.0,
                    move_speed: 70.0,
                ),
                (
                    phase_index: 2,
                    health_threshold: 0.25,  // 25% 触发
                    invulnerability_duration: 1.5,
                    skill_ids: ["frenzy_attack", "charge", "area_explosion"],
                    attack_frequency: 4.0,
                    move_speed: 90.0,
                ),
            ],
        ),
    ],
)
```

---

## 4. 数据验证规则

| 字段 | 验证规则 | 错误处理 |
|------|---------|---------|
| `phases[].health_threshold` | 0.0-1.0，必须递减 | 配置加载时验证，无效配置导致 panic |
| `phases[].invulnerability_duration` | 1.0-2.0 秒 | 限制在范围内，超出则截断 |
| `phases[].skill_ids` | 非空，技能 ID 必须存在 | 配置加载时验证 |
| `phases[].attack_frequency` | > 0.0 | 默认值为 1.0 |
| `max_health` | > 0.0 | panic!（配置错误） |
| `phase_index` | 从 0 开始连续递增 | 配置加载时验证 |

---

## 5. 数据流程图

```
Boss 配置加载 (RON) 
    ↓
BossConfig Resource
    ↓
Boss 实体生成（Boss + BossController 组件）
    ↓
战斗开始 → BossEncounterStarted 事件
    ↓
伤害系统 → 检测血量阈值
    ↓
阶段转换逻辑（Domain Layer）
    ↓
BossController 更新（health_lock = true）
    ↓
BossPhaseTransition 事件
    ↓
无敌状态（1-2秒）
    ↓
解锁血量（health_lock = false）
    ↓
新阶段技能选择
    ↓
技能预警（Telegraph 组件）
    ↓
技能释放 → 伤害判定
    ↓
Boss 死亡 → BossDefeated 事件
```

---

**Document Status**: ✅ 数据模型设计已完成
**Constitution Compliance**: ✅ 所有组件为纯数据结构，系统为纯行为函数，遵循领域驱动设计






