# Research: Dungeon System

**Feature**: 003-dungeon-system  
**Date**: 2025-01-27  
**Status**: Complete

## Overview

本文档记录地下城系统实现过程中的技术决策和研究结果。所有 "NEEDS CLARIFICATION" 标记已在规范澄清阶段解决。

## Technical Decisions

### 1. RON 配置文件格式

**Decision**: 使用 RON (Rusty Object Notation) 格式存储地下城配置。

**Rationale**:
- 项目已使用 RON 进行其他配置（如 `assets/data/enemies.ron`, `assets/data/skills.ron`）
- RON 支持注释，便于配置文件的维护和调试
- 类型安全：通过 serde 反序列化到 Rust 结构体，编译时验证
- 人类可读：比 JSON 更易读，比 YAML 更类型安全

**Alternatives Considered**:
- **JSON**: 不支持注释，配置维护困难
- **YAML**: 类型不够明确，容易出错
- **TOML**: 更适合配置，但 RON 更适合游戏数据
- **LDtk/Tiled**: 更适合关卡编辑器，但本功能假设地下城布局已预定义

**Implementation**:
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DungeonConfig {
    pub dungeon_id: String,
    pub start_room_id: u32,
    pub rooms: Vec<RoomConfig>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RoomConfig {
    pub room_id: u32,
    pub width: f32,
    pub height: f32,
    pub spawn_points: Vec<EnemySpawnPointConfig>,
    pub doors: Vec<DoorConfig>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EnemySpawnPointConfig {
    pub position: [f32; 2],
    pub enemy_type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DoorConfig {
    pub door_id: u32,
    pub connected_room_id: u32,
    pub entrance_position: [f32; 2],
}
```

### 2. 房间状态管理

**Decision**: 使用枚举 `RoomState` 表示房间状态（Active, Cleared, Uncleared），状态存储在 `Room` 组件和 `DungeonSession` 资源中。

**Rationale**:
- 状态明确，易于理解和维护
- 支持状态查询（`is_room_cleared`）和状态转换（`mark_room_cleared`）
- `DungeonSession` 作为 Session 级资源，确保已清理房间状态在房间切换时持久化

**Alternatives Considered**:
- **仅使用组件状态**: 房间卸载后状态丢失，无法支持返回已清理房间的场景
- **仅使用资源状态**: 需要额外的查询逻辑，性能开销更大
- **事件溯源**: 过度设计，当前需求不需要完整的事件历史

**Implementation**:
```rust
// Domain layer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoomState {
    Active,      // 当前激活的房间
    Cleared,     // 已清理（所有敌人死亡）
    Uncleared,   // 未清理（有敌人或未探索）
}

// Infrastructure layer - Component
#[derive(Component, Debug, Clone)]
pub struct Room {
    pub room_id: u32,
    pub state: RoomState,
    pub width: f32,
    pub height: f32,
}

// Infrastructure layer - Resource
#[derive(Resource, Default, Debug)]
pub struct DungeonSession {
    pub cleared_rooms: HashSet<u32>,  // Room IDs
}
```

### 3. 门的状态和交互

**Decision**: 门使用 `DoorState` 枚举（Locked, Unlocked），玩家靠近解锁的门时显示交互提示，按交互键触发传送。

**Rationale**:
- 状态简单明确，易于实现和测试
- 交互提示提供良好的用户体验
- 按键交互符合项目已有的输入系统模式

**Alternatives Considered**:
- **自动传送**: 玩家靠近门自动传送，但可能误触发
- **碰撞触发**: 使用物理碰撞，但需要额外的碰撞层配置
- **菜单选择**: 过度复杂，不符合快节奏动作游戏体验

**Implementation**:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DoorState {
    Locked,    // 锁定（房间未清理）
    Unlocked,  // 解锁（房间已清理）
}

#[derive(Component, Debug, Clone)]
pub struct Door {
    pub door_id: u32,
    pub room_id: u32,  // 所属房间
    pub connected_room_id: u32,
    pub state: DoorState,
    pub entrance_position: Vec2,  // 目标房间的入口位置
}
```

### 4. 敌人生成策略

**Decision**: 每个生成点独立配置位置和敌人类型，房间加载时根据房间状态决定是否生成敌人。

**Rationale**:
- 配置灵活，支持每个生成点生成不同类型的敌人
- 符合数据驱动设计原则
- 房间状态检查在领域层（纯函数），易于测试

**Alternatives Considered**:
- **房间级别配置敌人列表**: 不够灵活，无法精确控制每个生成点
- **随机生成**: 不符合确定性要求，测试困难
- **生成表系统**: 过度设计，当前需求不需要复杂的生成逻辑

**Implementation**:
```rust
// Domain layer
pub fn should_spawn_enemies(room_state: RoomState, is_cleared: bool) -> bool {
    matches!(room_state, RoomState::Uncleared) && !is_cleared
}

// Infrastructure layer
fn spawn_enemies_system(
    mut commands: Commands,
    rooms: Query<&Room>,
    session: Res<DungeonSession>,
    spawn_points: Query<&EnemySpawnPoint>,
    // ... enemy asset handles
) {
    // 查询房间状态，调用领域层函数判断是否生成
    // 根据生成点配置生成敌人
}
```

### 5. 房间过渡性能优化

**Decision**: 房间加载允许多帧完成（<100ms），但需要加载动画/过渡效果确保玩家体验流畅。

**Rationale**:
- <100ms 的加载时间对玩家来说几乎不可感知
- 多帧加载允许异步资源加载和实体生成，避免单帧卡顿
- 过渡效果（淡入淡出、传送动画）提供视觉反馈，提升体验

**Alternatives Considered**:
- **单帧加载（<16.67ms）**: 对于复杂房间可能无法实现，导致帧率下降
- **长时间加载（>500ms）**: 需要加载屏幕，破坏游戏流畅性
- **预加载相邻房间**: 过度优化，当前需求不需要

**Implementation Strategy**:
- 使用 Bevy 的 `AssetServer` 异步加载资源
- 房间过渡时显示过渡动画（淡入淡出或传送效果）
- 在过渡期间继续渲染当前房间，避免黑屏
- 使用 `Commands` 批量生成实体，提高性能

### 6. 配置文件错误处理

**Decision**: 启动时验证配置，错误时记录日志并回退到默认测试地下城配置。

**Rationale**:
- 确保系统可用性，即使配置文件损坏也能运行
- 日志记录便于调试和问题追踪
- 默认配置提供基本的测试场景，便于开发

**Alternatives Considered**:
- **游戏无法启动**: 用户体验差，开发效率低
- **运行时动态验证**: 错误发现延迟，可能导致运行时崩溃
- **跳过错误房间**: 可能导致地下城不完整，影响游戏流程

**Implementation**:
```rust
fn load_dungeon_system(
    asset_server: Res<AssetServer>,
    mut dungeon_config: ResMut<DungeonConfig>,
) {
    match asset_server.load::<DungeonConfig, _>("dungeons/test_dungeon.ron") {
        Ok(config) => {
            // 验证配置
            if validate_dungeon_config(&config) {
                *dungeon_config = config;
            } else {
                warn!("Dungeon config validation failed, using default");
                *dungeon_config = default_test_dungeon();
            }
        }
        Err(e) => {
            error!("Failed to load dungeon config: {:?}", e);
            warn!("Using default test dungeon");
            *dungeon_config = default_test_dungeon();
        }
    }
}
```

## Integration Points

### 与战斗系统集成

- **事件监听**: 监听 `EnemyDefeated` 事件（来自 `002-combat-core`）
- **死亡时机**: 敌人死亡动画播放完成后从存活列表移除，此时触发房间清理检查
- **状态同步**: 确保战斗系统的敌人死亡状态与地下城系统的房间状态一致

### 与玩家移动系统集成

- **传送逻辑**: 房间过渡时直接设置玩家位置，避免与移动系统冲突
- **重生位置**: 玩家死亡时在房间入口重生，使用房间配置的入口位置
- **交互检测**: 使用玩家位置和门位置的距离检测，触发交互提示

### 与 UI 系统集成

- **交互提示**: 在玩家靠近解锁的门时显示"按 E 交互"提示
- **房间状态显示**: 可选功能，显示当前房间状态和已清理房间数

## Performance Considerations

### 房间加载优化

- **延迟加载**: 只在玩家进入房间时加载，不预加载所有房间
- **资源池**: 复用敌人实体，减少分配开销
- **批量生成**: 使用 `Commands` 批量生成实体，减少系统调用次数

### 状态查询优化

- **HashSet 查找**: `DungeonSession.cleared_rooms` 使用 `HashSet<u32>`，O(1) 查找
- **组件缓存**: 房间状态缓存在 `Room` 组件中，避免重复查询资源
- **事件驱动**: 使用事件通知状态变化，减少轮询查询

## Testing Strategy

### 单元测试（Domain Layer）

- 测试所有 `progression.rs` 中的纯函数
- 使用 mock 数据，不依赖 Bevy
- 覆盖所有状态转换和边界情况

### 集成测试（Infrastructure Layer）

- 测试完整的房间过渡流程
- 测试进度追踪和状态持久化
- 使用 Bevy 测试工具（`TestApp`）模拟游戏环境

### 性能测试

- 基准测试房间加载时间（目标 <100ms）
- 基准测试状态查询性能（目标 <0.5ms per frame）
- 使用 `criterion` 进行基准测试

## Open Questions / Future Enhancements

1. **跨 Run 持久化**: 当前实现是 Session-based，未来可能需要跨 Run 持久化（保存到文件）
2. **动态房间生成**: 当前假设房间布局预定义，未来可能需要程序化生成
3. **房间内环境交互**: 当前只关注敌人和门，未来可能需要陷阱、宝箱等交互元素
4. **多人联机同步**: 当前是单玩家，未来多人联机时需要同步房间状态

## References

- [Bevy ECS Documentation](https://bevyengine.org/learn/book/getting-started/ecs/)
- [RON Format Specification](https://github.com/ron-rs/ron)
- [Serde Documentation](https://serde.rs/)
- Project Constitution v1.0.1
- Previous feature implementations: `001-player-movement`, `002-combat-core`

