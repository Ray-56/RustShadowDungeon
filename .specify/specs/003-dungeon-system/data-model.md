# Data Model: Dungeon System

**Feature**: 003-dungeon-system  
**Date**: 2025-01-27  
**Status**: Complete

## Overview

本文档定义地下城系统的所有数据结构和实体。遵循 DDD 架构原则：领域层使用纯 Rust 类型（零 Bevy 依赖），基础设施层使用 Bevy ECS 组件和资源。

## Domain Layer (Pure Rust)

### RoomState

房间状态枚举，用于领域逻辑判断。

```rust
// src/domain/dungeon/progression.rs

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoomState {
    /// 当前激活的房间
    Active,
    /// 已清理（所有敌人死亡）
    Cleared,
    /// 未清理（有敌人或未探索）
    Uncleared,
}
```

**用途**: 领域层函数使用此枚举判断房间状态，决定是否生成敌人、是否解锁门等。

### RoomId

房间 ID 类型别名，全局唯一标识符。

```rust
// src/domain/dungeon/progression.rs

pub type RoomId = u32;
```

**约束**: 
- 在整个地下城内全局唯一
- 从 1 开始（0 保留为无效值）

### DoorId

门 ID 类型别名，房间内唯一标识符。

```rust
// src/domain/dungeon/progression.rs

pub type DoorId = u32;
```

**约束**:
- 在房间内唯一
- 不同房间可以有相同的门 ID
- 全局标识使用复合键 `(RoomId, DoorId)`

### Position

位置坐标，用于领域层计算。

```rust
// src/domain/dungeon/progression.rs

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}
```

**用途**: 领域层函数计算目标房间入口位置等。

## Infrastructure Layer (Bevy ECS)

### Components (Pure Data Structures)

#### Room

房间组件，存储房间的基本信息和状态。

```rust
// src/infrastructure/components/dungeon.rs

use bevy::prelude::*;
use crate::domain::dungeon::progression::RoomState;

/// 房间组件 - 存储房间的基本信息和状态
#[derive(Component, Debug, Clone)]
pub struct Room {
    /// 房间 ID（全局唯一）
    pub room_id: u32,
    /// 房间状态
    pub state: RoomState,
    /// 房间宽度（像素）
    pub width: f32,
    /// 房间高度（像素）
    pub height: f32,
}
```

**字段说明**:
- `room_id`: 房间 ID，全局唯一
- `state`: 房间状态（Active, Cleared, Uncleared）
- `width`: 房间宽度（像素）
- `height`: 房间高度（像素）

**生命周期**: 房间加载时创建，卸载时删除。

#### Door

门组件，存储门的连接信息和状态。

```rust
// src/infrastructure/components/dungeon.rs

use bevy::prelude::*;
use bevy::math::Vec2;

/// 门状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DoorState {
    /// 锁定（房间未清理）
    Locked,
    /// 解锁（房间已清理）
    Unlocked,
}

/// 门组件 - 连接两个房间的实体
#[derive(Component, Debug, Clone)]
pub struct Door {
    /// 门 ID（房间内唯一）
    pub door_id: u32,
    /// 所属房间 ID
    pub room_id: u32,
    /// 连接的房间 ID
    pub connected_room_id: u32,
    /// 门状态
    pub state: DoorState,
    /// 目标房间的入口位置（玩家通过此门时传送到的位置）
    pub entrance_position: Vec2,
}
```

**字段说明**:
- `door_id`: 门 ID，在房间内唯一
- `room_id`: 所属房间 ID
- `connected_room_id`: 连接的房间 ID（目标房间）
- `state`: 门状态（Locked, Unlocked）
- `entrance_position`: 目标房间的入口位置（Vec2）

**约束**:
- 门是双向的，支持玩家在房间之间自由往返
- 每个房间可以有多个门
- 门 ID 在房间内唯一，全局标识使用 `(room_id, door_id)`

#### DungeonManager

地下城管理器组件，存储当前激活的房间和房间图谱。

```rust
// src/infrastructure/components/dungeon.rs

use bevy::prelude::*;
use std::collections::HashMap;

/// 房间连接信息
#[derive(Debug, Clone)]
pub struct RoomConnection {
    pub room_id: u32,
    pub doors: Vec<u32>,  // Door IDs in this room
}

/// 地下城管理器组件 - 管理整个地下城生命周期
#[derive(Component, Debug, Clone)]
pub struct DungeonManager {
    /// 当前激活的房间 ID
    pub current_room_id: u32,
    /// 房间图谱（房间 ID -> 连接信息）
    pub room_graph: HashMap<u32, RoomConnection>,
}
```

**字段说明**:
- `current_room_id`: 当前激活的房间 ID
- `room_graph`: 房间图谱，存储每个房间的连接信息

**用途**: 用于房间过渡时查找目标房间和门连接关系。

#### EnemySpawnPoint

敌人生成点组件，存储生成点的位置和敌人类型。

```rust
// src/infrastructure/components/dungeon.rs

use bevy::prelude::*;

/// 敌人生成点组件 - 定义敌人的生成位置和类型
#[derive(Component, Debug, Clone)]
pub struct EnemySpawnPoint {
    /// 生成位置（世界坐标）
    pub position: Vec2,
    /// 敌人类型 ID（对应 assets/data/enemies.ron 中的敌人类型）
    pub enemy_type: String,
    /// 是否在房间激活时生成（true: 立即生成, false: 延迟生成）
    pub spawn_on_activate: bool,
}
```

**字段说明**:
- `position`: 生成位置（Vec2，世界坐标）
- `enemy_type`: 敌人类型 ID（字符串，对应配置文件中的敌人类型）
- `spawn_on_activate`: 是否在房间激活时立即生成

**约束**:
- 每个生成点独立配置位置和敌人类型
- 敌人类型必须存在于 `assets/data/enemies.ron` 中

### Resources (Global State)

#### DungeonSession

地下城会话资源，记录已清理的房间。

```rust
// src/infrastructure/resources/dungeon.rs

use bevy::prelude::*;
use std::collections::HashSet;

/// 地下城会话资源 - 记录已清理的房间（Session 级状态）
#[derive(Resource, Default, Debug)]
pub struct DungeonSession {
    /// 已清理的房间 ID 集合
    pub cleared_rooms: HashSet<u32>,
}
```

**字段说明**:
- `cleared_rooms`: 已清理的房间 ID 集合（HashSet<u32>）

**生命周期**:
- 游戏开始时初始化（空集合）
- 房间清理时添加房间 ID
- 玩家离开地下城或死亡时重置（可选，根据需求）

**用途**: 
- 判断房间是否已清理（`is_room_cleared`）
- 已清理的房间不生成敌人
- 支持非线性探索（返回已清理的房间）

#### DungeonConfig

地下城配置资源，存储从 RON 文件加载的配置。

```rust
// src/infrastructure/resources/dungeon.rs

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// 敌人生成点配置
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EnemySpawnPointConfig {
    /// 生成位置 [x, y]
    pub position: [f32; 2],
    /// 敌人类型 ID
    pub enemy_type: String,
}

/// 门配置
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DoorConfig {
    /// 门 ID（房间内唯一）
    pub door_id: u32,
    /// 连接的房间 ID
    pub connected_room_id: u32,
    /// 目标房间的入口位置 [x, y]
    pub entrance_position: [f32; 2],
}

/// 房间配置
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RoomConfig {
    /// 房间 ID（全局唯一）
    pub room_id: u32,
    /// 房间宽度（像素）
    pub width: f32,
    /// 房间高度（像素）
    pub height: f32,
    /// 敌人生成点列表
    pub spawn_points: Vec<EnemySpawnPointConfig>,
    /// 门列表
    pub doors: Vec<DoorConfig>,
}

/// 地下城配置
#[derive(Debug, Clone, Deserialize, Serialize, Resource)]
pub struct DungeonConfig {
    /// 地下城 ID
    pub dungeon_id: String,
    /// 起始房间 ID
    pub start_room_id: u32,
    /// 房间列表
    pub rooms: Vec<RoomConfig>,
}
```

**字段说明**:
- `dungeon_id`: 地下城 ID（字符串）
- `start_room_id`: 起始房间 ID（玩家进入地下城时的第一个房间）
- `rooms`: 房间列表（每个房间包含大小、生成点、门等信息）

**加载**: 从 `assets/data/dungeons/*.ron` 文件加载，使用 Bevy 的 `AssetServer`。

**验证**: 启动时验证配置（格式、必需字段、数据有效性），错误时回退到默认测试地下城。

### Events (Inter-System Communication)

#### RoomCleared

房间清理事件，当房间内所有敌人死亡时触发。

```rust
// src/infrastructure/events/dungeon.rs

use bevy::prelude::*;

/// 房间清理事件 - 当房间内所有敌人死亡时触发
#[derive(Event, Debug, Clone)]
pub struct RoomCleared {
    /// 清理的房间 ID
    pub room_id: u32,
    /// 清理时间戳（可选，用于调试）
    pub cleared_at: f64,
}
```

**触发时机**: 敌人死亡动画播放完成后，从存活列表移除时，检查发现房间内无敌人。

**监听系统**: `handle_room_cleared_system` 监听此事件，更新房间状态并解锁门。

#### RoomEntered

房间进入事件，当玩家进入房间时触发。

```rust
// src/infrastructure/events/dungeon.rs

use bevy::prelude::*;
use bevy::math::Vec2;

/// 房间进入事件 - 当玩家进入房间时触发
#[derive(Event, Debug, Clone)]
pub struct RoomEntered {
    /// 进入的房间 ID
    pub room_id: u32,
    /// 玩家位置
    pub player_position: Vec2,
}
```

**触发时机**: 玩家进入新房间时（包括初始进入和房间过渡）。

**监听系统**: `handle_room_entered_system` 监听此事件，触发敌人生成。

#### DoorUnlocked

门解锁事件，当门从锁定状态变为解锁状态时触发。

```rust
// src/infrastructure/events/dungeon.rs

use bevy::prelude::*;

/// 门解锁事件 - 当门从锁定状态变为解锁状态时触发
#[derive(Event, Debug, Clone)]
pub struct DoorUnlocked {
    /// 解锁的门 ID
    pub door_id: u32,
    /// 所属房间 ID
    pub room_id: u32,
}
```

**触发时机**: 房间清理后，所有门解锁时。

**监听系统**: `update_door_visual_system` 监听此事件，更新门的视觉表现。

#### RoomTransitioned

房间过渡事件，当玩家通过门进入新房间时触发。

```rust
// src/infrastructure/events/dungeon.rs

use bevy::prelude::*;
use bevy::math::Vec2;

/// 房间过渡事件 - 当玩家通过门进入新房间时触发
#[derive(Event, Debug, Clone)]
pub struct RoomTransitioned {
    /// 源房间 ID
    pub from_room_id: u32,
    /// 目标房间 ID
    pub to_room_id: u32,
    /// 玩家新位置
    pub player_position: Vec2,
}
```

**触发时机**: 玩家通过解锁的门进入新房间时。

**监听系统**: `update_camera_on_room_transition_system` 监听此事件，更新摄像机位置。

#### PlayerDeathInRoom

玩家在房间内死亡事件。

```rust
// src/infrastructure/events/dungeon.rs

use bevy::prelude::*;
use bevy::math::Vec2;

/// 玩家在房间内死亡事件
#[derive(Event, Debug, Clone)]
pub struct PlayerDeathInRoom {
    /// 死亡时的房间 ID
    pub room_id: u32,
    /// 死亡位置
    pub death_position: Vec2,
}
```

**触发时机**: 玩家在房间内死亡时（来自战斗系统）。

**监听系统**: `handle_player_death_in_dungeon_system` 监听此事件，在房间入口重生玩家。

## Configuration Files (RON Format)

### Dungeon Configuration Schema

```ron
// assets/data/dungeons/test_dungeon.ron

(
    dungeon_id: "test_dungeon",
    start_room_id: 1,
    rooms: [
        (
            room_id: 1,
            width: 640.0,
            height: 360.0,
            spawn_points: [
                (
                    position: [100.0, 0.0],
                    enemy_type: "goblin",
                ),
                (
                    position: [200.0, 0.0],
                    enemy_type: "goblin",
                ),
            ],
            doors: [
                (
                    door_id: 1,
                    connected_room_id: 2,
                    entrance_position: [100.0, 0.0],
                ),
            ],
        ),
        // ... more rooms
    ],
)
```

**字段说明**:
- `dungeon_id`: 地下城 ID（字符串）
- `start_room_id`: 起始房间 ID（u32）
- `rooms`: 房间列表
  - `room_id`: 房间 ID（u32，全局唯一）
  - `width`: 房间宽度（f32，像素）
  - `height`: 房间高度（f32，像素）
  - `spawn_points`: 敌人生成点列表
    - `position`: 生成位置 [x, y]（[f32; 2]）
    - `enemy_type`: 敌人类型 ID（字符串，对应 `assets/data/enemies.ron`）
  - `doors`: 门列表
    - `door_id`: 门 ID（u32，房间内唯一）
    - `connected_room_id`: 连接的房间 ID（u32）
    - `entrance_position`: 目标房间的入口位置 [x, y]（[f32; 2]）

**验证规则**:
- `start_room_id` 必须存在于 `rooms` 中
- 所有 `connected_room_id` 必须存在于 `rooms` 中
- `enemy_type` 必须存在于 `assets/data/enemies.ron` 中
- 房间 ID 必须全局唯一
- 门 ID 在房间内必须唯一

## State Transitions

### Room State Machine

```
Uncleared (初始状态)
    ↓ (所有敌人死亡)
Cleared
    ↓ (玩家进入)
Active (当前激活)
    ↓ (玩家离开)
Cleared (保持已清理状态)
```

**状态转换规则**:
- 房间加载时，如果 `DungeonSession.cleared_rooms` 包含该房间 ID，初始状态为 `Cleared`，否则为 `Uncleared`
- 房间内所有敌人死亡时，状态从 `Uncleared` 变为 `Cleared`
- 玩家进入房间时，状态变为 `Active`
- 玩家离开房间时，如果已清理，状态保持 `Cleared`，否则保持 `Uncleared`

### Door State Machine

```
Locked (初始状态)
    ↓ (房间清理完成)
Unlocked
    ↓ (玩家通过)
[保持 Unlocked，支持返回]
```

**状态转换规则**:
- 房间加载时，如果房间未清理，门状态为 `Locked`
- 房间清理完成时，所有门状态变为 `Unlocked`
- 门解锁后保持 `Unlocked`，支持玩家往返

## Relationships

### Entity Relationships

```
DungeonManager (1) ──→ (N) Room
Room (1) ──→ (N) Door
Room (1) ──→ (N) EnemySpawnPoint
Door (N) ──→ (1) Room (connected_room_id)
```

### Resource Relationships

```
DungeonSession.cleared_rooms: HashSet<RoomId>
DungeonConfig.rooms: Vec<RoomConfig>
```

## Validation Rules

### Room Validation

- 房间 ID 必须全局唯一
- 房间大小（width, height）必须 > 0
- 房间必须至少有一个门（除非是终点房间）

### Door Validation

- 门 ID 在房间内必须唯一
- `connected_room_id` 必须存在于配置中
- `entrance_position` 必须在目标房间范围内

### EnemySpawnPoint Validation

- `enemy_type` 必须存在于敌人配置中
- `position` 必须在房间范围内

## Data Flow

### Room Loading Flow

```
1. Load DungeonConfig from RON file
2. Validate configuration
3. Create DungeonManager component
4. For each room in config:
   - Create Room component
   - Create Door components
   - Create EnemySpawnPoint components
5. Initialize first room (start_room_id)
6. Emit RoomEntered event
```

### Room Clearing Flow

```
1. EnemyDefeated event (from combat system)
2. monitor_enemy_deaths_system checks enemy count
3. If enemy count == 0:
   - Emit RoomCleared event
4. handle_room_cleared_system:
   - Update Room.state to Cleared
   - Add room_id to DungeonSession.cleared_rooms
   - Update all Door.state to Unlocked
   - Emit DoorUnlocked events
```

### Room Transition Flow

```
1. Player approaches unlocked door
2. door_interaction_detection_system shows interaction prompt
3. Player presses interaction key
4. handle_door_interaction_system:
   - Emit RoomTransitioned event
5. transition_to_room_system:
   - Teleport player to target room entrance
6. unload_current_room_system:
   - Despawn current room entities
7. load_target_room_system:
   - Load target room
   - Check room state (cleared or not)
   - Spawn enemies if not cleared
8. update_camera_on_room_transition_system:
   - Update camera to new room context
```

