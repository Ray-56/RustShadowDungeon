# Phase 1 数据模型：玩家移动系统

**Created**: 2025-11-24
**Feature**: 001-player-movement
**Purpose**: 定义 ECS 组件、资源、事件和领域模型

---

## 数据模型概述

本文档定义玩家移动系统的所有数据结构，严格遵循 DDD（领域驱动设计）架构和 Bevy ECS 模式。数据分为三层：

1. **领域层（Domain Layer）**: 纯 Rust 数据结构，无 Bevy 依赖
2. **基础设施层（Infrastructure Layer）**: Bevy ECS 组件、资源、事件
3. **配置层（Configuration Layer）**: RON 文件配置

---

## 1. 领域层数据模型（Domain Layer）

### 1.1 Velocity（速度）

**位置**: `src/domain/movement/velocity.rs`

**用途**: 表示玩家的移动速度（地面和空中）

```rust
/// 移动速度（像素/秒）
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Velocity {
    pub x: f32,  // 水平速度
    pub y: f32,  // 垂直速度
}

impl Velocity {
    /// 创建新速度
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// 零速度
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    /// 计算速度大小
    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}
```

---

### 1.2 JumpParams（跳跃参数）

**位置**: `src/domain/movement/jump.rs`

**用途**: 跳跃计算的参数

```rust
/// 跳跃参数
#[derive(Debug, Clone, Copy)]
pub struct JumpParams {
    pub initial_velocity: f32,  // 初始跳跃速度（像素/秒）
    pub gravity: f32,            // 重力加速度（像素/秒²）
    pub terminal_velocity: f32,  // 终端速度（像素/秒）
}

impl JumpParams {
    /// 从目标跳跃高度计算初始速度
    pub fn from_height(height: f32, duration: f32) -> Self {
        let gravity = (2.0 * height) / (duration * duration);
        let initial_velocity = gravity * duration;
        Self {
            initial_velocity,
            gravity,
            terminal_velocity: 128.0, // 固定终端速度
        }
    }
}
```

---

### 1.3 MovementState（移动状态）

**位置**: `src/domain/movement/state.rs`

**用途**: 玩家当前的移动状态

```rust
/// 移动状态枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MovementState {
    Idle,      // 待机（站立，无输入）
    Walking,   // 行走（地面移动）
    Jumping,   // 跳跃（上升阶段）
    Falling,   // 坠落（下降阶段）
}

impl MovementState {
    /// 是否在地面
    pub fn is_grounded(&self) -> bool {
        matches!(self, MovementState::Idle | MovementState::Walking)
    }

    /// 是否在空中
    pub fn is_airborne(&self) -> bool {
        matches!(self, MovementState::Jumping | MovementState::Falling)
    }
}
```

---

### 1.4 StateTransition（状态转换）

**位置**: `src/domain/movement/state.rs`

**用途**: 状态转换逻辑（纯函数）

```rust
/// 状态转换函数
pub fn transition_state(
    current_state: MovementState,
    is_grounded: bool,
    has_move_input: bool,
    has_jump_input: bool,
    velocity_y: f32,
) -> MovementState {
    match current_state {
        MovementState::Idle => {
            if has_jump_input && is_grounded {
                MovementState::Jumping
            } else if has_move_input && is_grounded {
                MovementState::Walking
            } else if !is_grounded {
                MovementState::Falling
            } else {
                MovementState::Idle
            }
        }
        MovementState::Walking => {
            if has_jump_input && is_grounded {
                MovementState::Jumping
            } else if !has_move_input && is_grounded {
                MovementState::Idle
            } else if !is_grounded {
                MovementState::Falling
            } else {
                MovementState::Walking
            }
        }
        MovementState::Jumping => {
            if velocity_y <= 0.0 {
                MovementState::Falling
            } else {
                MovementState::Jumping
            }
        }
        MovementState::Falling => {
            if is_grounded {
                if has_move_input {
                    MovementState::Walking
                } else {
                    MovementState::Idle
                }
            } else {
                MovementState::Falling
            }
        }
    }
}
```

---

### 1.5 InputDirection（输入方向）

**位置**: `src/domain/movement/input.rs`

**用途**: 标准化的输入方向

```rust
/// 输入方向
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputDirection {
    Left,
    Right,
    None,
}

impl InputDirection {
    /// 转换为速度系数（-1.0, 0.0, 1.0）
    pub fn to_velocity_factor(&self) -> f32 {
        match self {
            InputDirection::Left => -1.0,
            InputDirection::Right => 1.0,
            InputDirection::None => 0.0,
        }
    }
}
```

---

## 2. 基础设施层数据模型（Infrastructure Layer）

### 2.1 ECS 组件（Components）

#### Player（玩家）

**位置**: `src/infrastructure/components/player.rs`

**用途**: 标记玩家实体

```rust
use bevy::prelude::*;

/// 玩家组件（标记组件）
#[derive(Component, Debug)]
pub struct Player;
```

---

#### InputState（输入状态）

**位置**: `src/infrastructure/components/player.rs`

**用途**: 存储当前帧的输入状态

```rust
use crate::domain::movement::InputDirection;

/// 输入状态组件
#[derive(Component, Debug, Clone)]
pub struct InputState {
    pub move_direction: InputDirection,  // 移动方向
    pub jump_pressed: bool,               // 跳跃键是否按下
    pub jump_just_pressed: bool,          // 跳跃键是否刚按下（本帧）
}

impl Default for InputState {
    fn default() -> Self {
        Self {
            move_direction: InputDirection::None,
            jump_pressed: false,
            jump_just_pressed: false,
        }
    }
}
```

---

#### MovementStateComponent（移动状态组件）

**位置**: `src/infrastructure/components/player.rs`

**用途**: 存储玩家当前的移动状态

```rust
use crate::domain::movement::MovementState;

/// 移动状态组件
#[derive(Component, Debug, Clone)]
pub struct MovementStateComponent {
    pub current: MovementState,      // 当前状态
    pub previous: MovementState,     // 前一状态（用于检测状态变化）
    pub state_duration: f32,         // 当前状态持续时间（秒）
}

impl Default for MovementStateComponent {
    fn default() -> Self {
        Self {
            current: MovementState::Idle,
            previous: MovementState::Idle,
            state_duration: 0.0,
        }
    }
}
```

---

#### VelocityComponent（速度组件）

**位置**: `src/infrastructure/components/player.rs`

**用途**: 存储玩家当前的速度

```rust
use crate::domain::movement::Velocity;

/// 速度组件
#[derive(Component, Debug, Clone)]
pub struct VelocityComponent {
    pub velocity: Velocity,
}

impl Default for VelocityComponent {
    fn default() -> Self {
        Self {
            velocity: Velocity::zero(),
        }
    }
}
```

---

#### GroundedState（地面状态）

**位置**: `src/infrastructure/components/physics.rs`

**用途**: 存储玩家是否在地面上（由物理系统更新）

```rust
/// 地面状态组件
#[derive(Component, Debug, Clone)]
pub struct GroundedState {
    pub is_grounded: bool,           // 是否在地面
    pub ground_normal: Vec2,         // 地面法线（用于斜坡）
}

impl Default for GroundedState {
    fn default() -> Self {
        Self {
            is_grounded: false,
            ground_normal: Vec2::Y, // 默认垂直向上
        }
    }
}
```

---

#### PixelSnap（像素对齐标记）

**位置**: `src/infrastructure/components/player.rs`

**用途**: 标记需要像素对齐的实体

```rust
/// 像素对齐标记组件
#[derive(Component, Debug)]
pub struct PixelSnap;
```

---

### 2.2 资源（Resources）

#### MovementConfig（移动配置）

**位置**: `src/infrastructure/resources/movement_config.rs`

**用途**: 存储移动系统的全局配置

```rust
use bevy::prelude::*;
use crate::domain::movement::JumpParams;

/// 移动配置资源
#[derive(Resource, Debug, Clone)]
pub struct MovementConfig {
    pub ground_speed: f32,       // 地面移动速度（像素/秒）
    pub air_speed_factor: f32,   // 空中速度系数（相对地面速度）
    pub jump_params: JumpParams, // 跳跃参数
    pub grid_size: f32,          // 像素网格大小（16.0）
}

impl Default for MovementConfig {
    fn default() -> Self {
        Self {
            ground_speed: 48.0,  // 3 个格子/秒
            air_speed_factor: 0.6, // 空中速度为地面的 60%
            jump_params: JumpParams::from_height(32.0, 0.4), // 跳跃高度 32 像素，持续 0.4 秒
            grid_size: 16.0,
        }
    }
}
```

---

#### InputBuffer（输入缓冲）

**位置**: `src/infrastructure/resources/input_config.rs`

**用途**: 存储最近的输入，实现输入缓冲

```rust
use bevy::prelude::*;
use std::collections::VecDeque;

/// 输入缓冲资源
#[derive(Resource, Debug)]
pub struct InputBuffer {
    pub jump_buffer: VecDeque<f32>,  // 跳跃输入时间戳（秒）
    pub buffer_duration: f32,         // 缓冲窗口（秒）
}

impl Default for InputBuffer {
    fn default() -> Self {
        Self {
            jump_buffer: VecDeque::new(),
            buffer_duration: 0.15, // 150ms
        }
    }
}

impl InputBuffer {
    /// 添加跳跃输入
    pub fn add_jump(&mut self, time: f32) {
        self.jump_buffer.push_back(time);
    }

    /// 检查是否有有效的跳跃输入
    pub fn has_valid_jump(&self, current_time: f32) -> bool {
        self.jump_buffer.iter().any(|&time| {
            current_time - time < self.buffer_duration
        })
    }

    /// 清理过期输入
    pub fn clean_expired(&mut self, current_time: f32) {
        self.jump_buffer.retain(|&time| {
            current_time - time < self.buffer_duration
        });
    }

    /// 消耗跳跃输入
    pub fn consume_jump(&mut self) {
        self.jump_buffer.clear();
    }
}
```

---

### 2.3 事件（Events）

#### PlayerMoved（玩家移动事件）

**位置**: `src/infrastructure/events/movement.rs`

**用途**: 通知其他系统玩家已移动（如相机系统）

```rust
use bevy::prelude::*;
use crate::domain::movement::Velocity;

/// 玩家移动事件
#[derive(Event, Debug, Clone)]
pub struct PlayerMoved {
    pub entity: Entity,       // 玩家实体
    pub new_position: Vec2,   // 新位置
    pub velocity: Velocity,   // 当前速度
}
```

---

#### StateChanged（状态改变事件）

**位置**: `src/infrastructure/events/movement.rs`

**用途**: 通知其他系统玩家状态已改变（如动画系统）

```rust
use crate::domain::movement::MovementState;

/// 状态改变事件
#[derive(Event, Debug, Clone)]
pub struct StateChanged {
    pub entity: Entity,          // 玩家实体
    pub from_state: MovementState, // 前一状态
    pub to_state: MovementState,   // 新状态
}
```

---

## 3. 配置层数据模型（Configuration Layer）

### 3.1 movement_config.ron

**位置**: `assets/data/movement_config.ron`

**用途**: 移动参数配置文件

```ron
(
    ground_speed: 48.0,         // 地面速度（像素/秒）
    air_speed_factor: 0.6,      // 空中速度系数
    jump_params: (
        initial_velocity: 80.0,  // 初始跳跃速度
        gravity: 200.0,          // 重力加速度
        terminal_velocity: 128.0, // 终端速度
    ),
    grid_size: 16.0,             // 像素网格大小
)
```

---

### 3.2 input_config.ron

**位置**: `assets/data/input_config.ron`

**用途**: 输入映射配置文件

```ron
(
    keyboard: (
        move_left: "A",
        move_right: "D",
        jump: "Space",
    ),
    gamepad: (
        move_left: "DPadLeft",
        move_right: "DPadRight",
        jump: "South", // A/Cross
    ),
    buffer_duration: 0.15, // 输入缓冲窗口（秒）
)
```

---

## 4. 系统交互图

```
┌─────────────────────────────────────────────────────────────┐
│                     Input System                            │
│  - 收集键盘/手柄输入                                         │
│  - 更新 InputState 组件                                     │
│  - 管理 InputBuffer 资源                                    │
└─────────────────┬───────────────────────────────────────────┘
                  │ InputState
                  ↓
┌─────────────────────────────────────────────────────────────┐
│                  Movement System                            │
│  - 读取 InputState, MovementStateComponent, GroundedState  │
│  - 调用领域层函数（transition_state, calculate_velocity）  │
│  - 更新 VelocityComponent, MovementStateComponent          │
│  - 发出 PlayerMoved, StateChanged 事件                     │
└─────────────────┬───────────────────────────────────────────┘
                  │ VelocityComponent
                  ↓
┌─────────────────────────────────────────────────────────────┐
│                  Physics Sync System                        │
│  - 应用 VelocityComponent 到 bevy_rapier2d 刚体           │
│  - 更新 GroundedState（根据 bevy_tnua 地面检测）          │
└─────────────────┬───────────────────────────────────────────┘
                  │ Transform (by physics engine)
                  ↓
┌─────────────────────────────────────────────────────────────┐
│                  Pixel Snap System                          │
│  - 读取 Transform, 带 PixelSnap 标记的实体                │
│  - 将位置取整到像素网格                                    │
└─────────────────┬───────────────────────────────────────────┘
                  │ Transform (snapped)
                  ↓
┌─────────────────────────────────────────────────────────────┐
│                  Animation System                           │
│  - 监听 StateChanged 事件                                  │
│  - 根据 MovementState 切换动画                             │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│                  Camera Follow System                       │
│  - 监听 PlayerMoved 事件                                   │
│  - 更新相机 Transform，保持玩家在屏幕中心                  │
│  - 应用像素对齐到相机位置                                  │
└─────────────────────────────────────────────────────────────┘
```

---

## 5. 实体蓝图（Entity Blueprint）

### Player Entity

玩家实体包含以下组件：

```rust
commands.spawn((
    Player,                          // 标记组件
    InputState::default(),           // 输入状态
    MovementStateComponent::default(), // 移动状态
    VelocityComponent::default(),    // 速度
    GroundedState::default(),        // 地面状态
    PixelSnap,                       // 像素对齐标记
    SpriteBundle {                   // Bevy 精灵
        texture: asset_server.load("sprites/player_idle.png"),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    },
    // bevy_tnua 组件（字符控制器）
    TnuaController::default(),
    // bevy_rapier2d 组件（物理刚体）
    RigidBody::Dynamic,
    Collider::cuboid(8.0, 16.0),    // 16×32 碰撞箱
));
```

---

## 6. 数据验证规则

| 字段 | 验证规则 | 错误处理 |
|------|---------|---------|
| `ground_speed` | > 0.0 | panic!（配置错误） |
| `air_speed_factor` | 0.0..=1.0 | 限制为 0.6 |
| `jump_params.initial_velocity` | > 0.0 | panic!（配置错误） |
| `jump_params.gravity` | > 0.0 | panic!（配置错误） |
| `buffer_duration` | 0.0..=1.0 | 限制为 0.15 |
| `grid_size` | > 0.0 | 固定为 16.0 |

---

## 7. 数据流程图

```
用户输入 → leafwing-input-manager → InputState 组件
                                          ↓
                              Movement System（领域层函数）
                                          ↓
                              VelocityComponent 更新
                                          ↓
                              bevy_rapier2d 刚体速度
                                          ↓
                              Transform 更新（物理引擎）
                                          ↓
                              Pixel Snap（取整到网格）
                                          ↓
                              渲染（像素完美）
```

---

**Document Status**: ✅ 数据模型设计已完成
**Constitution Compliance**: ✅ 所有组件为纯数据结构，系统为纯行为函数
**Ready for**: quickstart.md 生成 → 任务清单生成









