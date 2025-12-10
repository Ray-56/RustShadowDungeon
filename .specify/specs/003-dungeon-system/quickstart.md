# Quickstart: Dungeon System

**Feature**: 003-dungeon-system  
**Date**: 2025-01-27  
**Status**: Complete

## Overview

本文档提供地下城系统的快速入门指南，包括测试场景、验证步骤和常见问题。

## Prerequisites

- Rust 1.82.0+ installed
- Bevy 0.17.0 project setup
- 已完成 `001-player-movement` 和 `002-combat-core` 功能
- 了解 Bevy ECS 基础概念

## Quick Test Scenarios

### Scenario 1: 进入地下城并加载第一个房间

**目标**: 验证地下城系统能够加载配置并初始化第一个房间。

**步骤**:
1. 启动游戏
2. 选择进入地下城（或自动进入测试地下城）
3. 观察第一个房间是否加载
4. 验证玩家位置是否正确（房间出生点）
5. 验证敌人是否在配置的位置生成

**预期结果**:
- ✅ 第一个房间（Room 1）成功加载
- ✅ 玩家出现在正确的出生点
- ✅ 敌人按照配置在指定位置生成
- ✅ 所有门处于锁定状态（不显示交互提示）

**验证命令**:
```bash
cargo run
# 进入游戏后，观察控制台日志：
# - "Dungeon loaded: test_dungeon"
# - "Room initialized: 1"
# - "Enemies spawned: 3"
```

### Scenario 2: 清理房间并解锁门

**目标**: 验证房间清理逻辑和门解锁功能。

**步骤**:
1. 在房间内战斗，击败所有敌人
2. 等待敌人死亡动画播放完成
3. 观察房间状态变化
4. 验证所有门是否解锁
5. 验证门是否显示交互提示

**预期结果**:
- ✅ 最后一个敌人死亡后，触发 `RoomCleared` 事件
- ✅ 房间状态更新为 `Cleared`
- ✅ 所有门状态更新为 `Unlocked`
- ✅ 玩家靠近门时显示"按 E 交互"提示

**验证命令**:
```bash
# 在游戏中击败所有敌人后，观察控制台日志：
# - "Room cleared: 1"
# - "Doors unlocked: 2"
```

**调试技巧**:
- 如果门未解锁，检查 `DungeonSession.cleared_rooms` 是否包含房间 ID
- 如果敌人死亡但房间未清理，检查 `monitor_enemy_deaths_system` 是否正确监听 `EnemyDefeated` 事件

### Scenario 3: 房间过渡

**目标**: 验证玩家通过门进入下一个房间的功能。

**步骤**:
1. 确保当前房间已清理，门已解锁
2. 玩家靠近解锁的门
3. 观察是否显示交互提示
4. 按交互键（默认 E）
5. 观察玩家是否传送到目标房间
6. 验证新房间是否正确加载
7. 验证敌人是否根据房间状态生成

**预期结果**:
- ✅ 玩家靠近门时显示交互提示
- ✅ 按交互键后，玩家传送到目标房间对应门的入口点
- ✅ 当前房间的敌人和临时实体被卸载
- ✅ 目标房间正确加载
- ✅ 如果目标房间未清理，生成敌人；如果已清理，不生成敌人
- ✅ 摄像机更新到新房间上下文

**验证命令**:
```bash
# 在游戏中通过门后，观察控制台日志：
# - "Room transition: 1 -> 2"
# - "Player teleported to: (100.0, 0.0)"
# - "Room loaded: 2"
```

**调试技巧**:
- 如果玩家位置不正确，检查 `entrance_position` 配置
- 如果新房间未加载，检查 `load_target_room_system` 是否正确执行
- 如果敌人未生成，检查 `DungeonSession.cleared_rooms` 和房间状态

### Scenario 4: 进度追踪

**目标**: 验证已清理房间的状态持久化。

**步骤**:
1. 清理 Room A
2. 通过门进入 Room B
3. 通过门返回 Room A
4. 验证 Room A 的状态

**预期结果**:
- ✅ Room A 清理后，`DungeonSession.cleared_rooms` 包含 Room A 的 ID
- ✅ 返回 Room A 时，房间状态为 `Cleared`
- ✅ Room A 不重新生成敌人
- ✅ Room A 的门保持解锁状态

**验证命令**:
```bash
# 在游戏中返回已清理的房间后，观察控制台日志：
# - "Room state restored: 1 (Cleared)"
# - "Enemies not spawned (room already cleared)"
```

**调试技巧**:
- 如果房间状态未保持，检查 `DungeonSession` 是否正确更新
- 如果敌人重新生成，检查 `spawn_enemies_system` 是否正确检查 `DungeonSession`

### Scenario 5: 玩家死亡处理

**目标**: 验证玩家在房间内死亡时的重生逻辑。

**步骤**:
1. 在房间内战斗
2. 玩家生命值降至 0（触发死亡）
3. 观察玩家重生位置
4. 验证房间状态是否保持不变

**预期结果**:
- ✅ 玩家在房间入口重生
- ✅ 当前房间状态保持不变（已死亡的敌人不重新生成）
- ✅ 已解锁的门保持解锁状态
- ✅ 已清理的其他房间状态保持不变

**验证命令**:
```bash
# 在游戏中玩家死亡后，观察控制台日志：
# - "Player died in room: 1"
# - "Player respawned at: (64.0, 288.0)"
# - "Room state preserved: Cleared"
```

## Unit Test Examples

### Domain Layer Tests

```rust
// tests/unit/dungeon/progression_test.rs

use rust_shadow_dungeon::domain::dungeon::progression::*;

#[test]
fn test_should_spawn_enemies_uncleared() {
    assert!(should_spawn_enemies(RoomState::Uncleared, false));
}

#[test]
fn test_should_spawn_enemies_cleared() {
    assert!(!should_spawn_enemies(RoomState::Cleared, true));
}

#[test]
fn test_check_room_cleared_no_enemies() {
    assert!(check_room_cleared(0));
}

#[test]
fn test_check_room_cleared_with_enemies() {
    assert!(!check_room_cleared(3));
}

#[test]
fn test_should_unlock_doors_cleared() {
    assert!(should_unlock_doors(RoomState::Cleared));
}

#[test]
fn test_should_unlock_doors_uncleared() {
    assert!(!should_unlock_doors(RoomState::Uncleared));
}
```

### Integration Test Example

```rust
// tests/integration/dungeon/room_transition_test.rs

use bevy::prelude::*;
use rust_shadow_dungeon::infrastructure::*;

#[test]
fn test_room_transition_flow() {
    let mut app = App::new();
    // Setup test app with dungeon systems
    
    // 1. Load dungeon
    app.update();
    // Verify Room 1 loaded
    
    // 2. Clear room
    // Simulate enemy deaths
    app.update();
    // Verify RoomCleared event
    
    // 3. Transition to Room 2
    // Simulate door interaction
    app.update();
    // Verify player position updated
    // Verify Room 2 loaded
}
```

## Performance Testing

### Room Loading Benchmark

```rust
// benches/dungeon_bench.rs

use criterion::{criterion_group, criterion_main, Criterion};
use rust_shadow_dungeon::infrastructure::*;

fn bench_room_loading(c: &mut Criterion) {
    c.bench_function("load_room", |b| {
        b.iter(|| {
            // Load room configuration
            // Spawn entities
            // Target: <100ms
        });
    });
}

criterion_group!(benches, bench_room_loading);
criterion_main!(benches);
```

**目标性能**:
- 房间加载: <100ms（多帧，带过渡效果）
- 状态查询: <0.5ms per frame
- 门交互检测: <0.3ms per frame

## Configuration File Example

### Minimal Test Dungeon

```ron
// assets/data/dungeons/minimal_test.ron

(
    dungeon_id: "minimal_test",
    start_room_id: 1,
    rooms: [
        (
            room_id: 1,
            width: 640.0,
            height: 360.0,
            spawn_points: [
                (
                    position: [320.0, 0.0],
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
        (
            room_id: 2,
            width: 640.0,
            height: 360.0,
            spawn_points: [],
            doors: [
                (
                    door_id: 1,
                    connected_room_id: 1,
                    entrance_position: [540.0, 0.0],
                ),
            ],
        ),
    ],
)
```

## Common Issues & Solutions

### Issue 1: 门未解锁

**症状**: 房间清理后，门仍然锁定。

**可能原因**:
- `RoomCleared` 事件未正确触发
- `handle_room_cleared_system` 未正确监听事件
- `DungeonSession` 未正确更新

**解决方案**:
1. 检查 `monitor_enemy_deaths_system` 是否正确监听 `EnemyDefeated` 事件
2. 验证敌人死亡动画完成后才从存活列表移除
3. 检查 `check_room_clear_system` 是否正确检查敌人数量
4. 验证 `handle_room_cleared_system` 是否正确更新门状态

### Issue 2: 房间过渡后玩家位置不正确

**症状**: 玩家通过门后，位置不在目标房间的入口点。

**可能原因**:
- `entrance_position` 配置错误
- `transition_to_room_system` 未正确设置玩家位置
- 目标房间的门配置错误

**解决方案**:
1. 检查 `test_dungeon.ron` 中的 `entrance_position` 配置
2. 验证门是双向的，确保两个房间的门配置匹配
3. 检查 `transition_to_room_system` 是否正确使用 `entrance_position`

### Issue 3: 已清理房间重新生成敌人

**症状**: 返回已清理的房间时，敌人重新生成。

**可能原因**:
- `DungeonSession.cleared_rooms` 未正确记录
- `spawn_enemies_system` 未检查 `DungeonSession`
- 房间状态未正确恢复

**解决方案**:
1. 检查 `track_room_clear_system` 是否正确添加房间 ID 到 `DungeonSession`
2. 验证 `spawn_enemies_system` 在生成敌人前检查 `DungeonSession.cleared_rooms`
3. 检查 `restore_room_state_system` 是否正确恢复房间状态

### Issue 4: 配置文件加载失败

**症状**: 游戏启动时无法加载地下城配置。

**可能原因**:
- RON 文件格式错误
- 文件路径错误
- 必需字段缺失

**解决方案**:
1. 检查 `assets/data/dungeons/test_dungeon.ron` 文件是否存在
2. 验证 RON 文件格式（使用 `ron` crate 验证）
3. 检查配置验证逻辑，查看错误日志
4. 系统应回退到默认测试地下城配置

## Debugging Tips

### Enable Debug Logging

```rust
// In main.rs or plugin setup
use bevy::log::LogPlugin;

App::new()
    .add_plugins(DefaultPlugins.set(LogPlugin {
        level: bevy::log::Level::DEBUG,
        ..default()
    }))
```

### Inspect ECS State

```rust
// In a system, query components and resources
fn debug_dungeon_state(
    rooms: Query<&Room>,
    doors: Query<&Door>,
    session: Res<DungeonSession>,
) {
    for room in rooms.iter() {
        println!("Room {}: {:?}", room.room_id, room.state);
    }
    println!("Cleared rooms: {:?}", session.cleared_rooms);
}
```

### Visual Debugging

- 使用 Bevy 的 `bevy_inspector_egui` 插件实时查看组件和资源
- 在房间边界和门位置绘制调试线框
- 显示房间 ID 和状态文本标签

## Next Steps

1. **实现基础功能**: 按照 `tasks.md` 中的任务顺序实现
2. **编写测试**: 先写测试，确保测试失败，然后实现功能
3. **性能优化**: 使用基准测试验证性能目标
4. **集成测试**: 测试完整的房间过渡流程
5. **错误处理**: 添加配置文件验证和错误恢复逻辑

## References

- [Specification](./spec.md) - 完整功能规范
- [Data Model](./data-model.md) - 数据结构定义
- [Research](./research.md) - 技术决策和研究
- [Tasks](./tasks.md) - 实现任务列表
- [Bevy ECS Book](https://bevyengine.org/learn/book/getting-started/ecs/) - Bevy ECS 文档

