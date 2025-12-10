# Quickstart: Enemy AI System

**Feature**: 004-enemy-ai  
**Date**: 2025-01-27  
**Status**: Complete

## Overview

本文档提供敌人 AI 系统的快速入门指南，包括测试场景、验证步骤和常见问题。

## Prerequisites

- Rust 1.82.0+ installed
- Bevy 0.17.0 project setup
- 已完成 `001-player-movement`、`002-combat-core` 和 `003-dungeon-system` 功能
- 了解 Bevy ECS 基础概念
- 了解状态机基础概念

## Quick Test Scenarios

### Scenario 1: 敌人巡逻行为

**目标**: 验证敌人在无玩家时能够进行巡逻。

**步骤**:
1. 启动游戏，进入有敌人的房间
2. 确保玩家不在敌人检测范围内
3. 观察敌人行为
4. 验证敌人是否在巡逻范围内移动

**预期结果**:
- ✅ 敌人在巡逻范围内移动
- ✅ 敌人到达目标点后停留短暂时间
- ✅ 敌人选择下一个巡逻目标点
- ✅ 敌人不会离开巡逻范围

**验证命令**:
```bash
cargo run
# 在游戏中，观察控制台日志：
# - "Enemy entering Patrol state"
# - "Enemy moving to patrol target: (x, y)"
```

**调试技巧**:
- 如果敌人不移动，检查 `PatrolConfig` 是否正确设置
- 如果敌人移动超出范围，检查 `patrol_radius` 配置
- 启用调试可视化查看巡逻范围

### Scenario 2: 敌人发现玩家

**目标**: 验证敌人能够检测到玩家并切换到追逐状态。

**步骤**:
1. 在敌人附近，确保玩家在检测范围内
2. 确保玩家和敌人之间有视线（无障碍物）
3. 观察敌人行为
4. 验证敌人是否切换到 Chase 状态

**预期结果**:
- ✅ 玩家进入检测范围后，敌人检测到玩家
- ✅ 触发 `EnemyDetectedPlayer` 事件
- ✅ 敌人状态从 Patrol 切换到 Chase
- ✅ 敌人开始向玩家移动

**验证命令**:
```bash
# 在游戏中，观察控制台日志：
# - "Enemy detected player at distance: X"
# - "EnemyDetectedPlayer event emitted"
# - "Enemy transitioning to Chase state"
```

**调试技巧**:
- 如果敌人不检测玩家，检查 `detection_range` 配置
- 如果敌人检测但不追逐，检查视线检测逻辑
- 如果检测延迟，检查感知系统的节流设置

### Scenario 3: 敌人追逐玩家

**目标**: 验证敌人能够追逐玩家并保持跟随。

**步骤**:
1. 确保敌人处于 Chase 状态
2. 玩家移动，改变位置
3. 观察敌人行为
4. 验证敌人是否跟随玩家移动

**预期结果**:
- ✅ 敌人向玩家当前位置移动
- ✅ 敌人每秒更新路径至少 2 次
- ✅ 敌人能够绕过简单障碍物（如果实现）
- ✅ 敌人保持在合理距离内

**验证命令**:
```bash
# 在游戏中，观察控制台日志：
# - "Enemy chasing target at: (x, y)"
# - "Enemy updating chase direction"
```

**调试技巧**:
- 如果敌人不移动，检查移动系统集成
- 如果敌人移动方向错误，检查 `calculate_chase_direction` 函数
- 如果敌人卡住，检查障碍物处理逻辑

### Scenario 4: 敌人攻击行为

**目标**: 验证敌人能够在攻击范围内发动攻击。

**步骤**:
1. 确保敌人处于 Chase 状态
2. 玩家进入敌人攻击范围
3. 观察敌人行为
4. 验证敌人是否切换到 Attack 状态并执行攻击

**预期结果**:
- ✅ 玩家进入攻击范围后，敌人切换到 Attack 状态
- ✅ 敌人停止移动（近战）或保持距离（远程）
- ✅ 触发 `EnemyAttackTriggered` 事件
- ✅ 攻击动画播放
- ✅ 攻击冷却时间开始计时

**验证命令**:
```bash
# 在游戏中，观察控制台日志：
# - "Enemy transitioning to Attack state"
# - "EnemyAttackTriggered event emitted"
# - "Enemy attack cooldown started: X seconds"
```

**调试技巧**:
- 如果敌人不攻击，检查 `attack_range` 配置
- 如果攻击频率过高，检查 `attack_cooldown` 配置
- 如果攻击不造成伤害，检查战斗系统集成

### Scenario 5: 敌人脱战

**目标**: 验证敌人能够在玩家离开后放弃追逐并返回。

**步骤**:
1. 确保敌人处于 Chase 状态
2. 玩家快速离开，超出脱战范围
3. 观察敌人行为
4. 验证敌人是否切换到 Return 状态

**预期结果**:
- ✅ 玩家超出脱战范围后，触发 `EnemyLostTarget` 事件
- ✅ 敌人状态从 Chase 切换到 Return
- ✅ 敌人返回生成位置
- ✅ 敌人返回后切换到 Patrol 状态

**验证命令**:
```bash
# 在游戏中，观察控制台日志：
# - "Enemy lost target: OutOfRange"
# - "EnemyLostTarget event emitted"
# - "Enemy transitioning to Return state"
# - "Enemy returning to spawn position"
```

**调试技巧**:
- 如果敌人不脱战，检查 `aggro_drop_range` 配置
- 如果敌人不返回，检查 Return 状态逻辑
- 如果返回后不巡逻，检查状态转换逻辑

## Unit Test Examples

### Domain Layer Tests

```rust
// tests/unit/enemy/ai_test.rs

use rust_shadow_dungeon::domain::enemy::ai::*;

#[test]
fn test_should_transition_to_chase_in_range() {
    assert!(should_transition_to_chase(100.0, 200.0, true));
}

#[test]
fn test_should_transition_to_chase_out_of_range() {
    assert!(!should_transition_to_chase(300.0, 200.0, true));
}

#[test]
fn test_should_transition_to_chase_no_los() {
    assert!(!should_transition_to_chase(100.0, 200.0, false));
}

#[test]
fn test_should_transition_to_attack_in_range() {
    assert!(should_transition_to_attack(30.0, 32.0, true));
}

#[test]
fn test_should_transition_to_attack_cooldown_not_ready() {
    assert!(!should_transition_to_attack(30.0, 32.0, false));
}

#[test]
fn test_should_drop_aggro_out_of_range() {
    assert!(should_drop_aggro(500.0, 400.0, 0.0, 5.0));
}

#[test]
fn test_should_drop_aggro_lost_sight() {
    assert!(should_drop_aggro(200.0, 400.0, 6.0, 5.0));
}
```

### Integration Test Example

```rust
// tests/integration/enemy/ai_state_machine_test.rs

use bevy::prelude::*;
use rust_shadow_dungeon::infrastructure::*;

#[test]
fn test_ai_state_machine_flow() {
    let mut app = App::new();
    // Setup test app with AI systems
    
    // 1. Spawn enemy in Patrol state
    app.update();
    // Verify enemy in Patrol state
    
    // 2. Spawn player within detection range
    app.update();
    // Verify EnemyDetectedPlayer event
    // Verify enemy transitions to Chase
    
    // 3. Move player into attack range
    app.update();
    // Verify enemy transitions to Attack
    // Verify EnemyAttackTriggered event
    
    // 4. Move player out of aggro range
    app.update();
    // Verify EnemyLostTarget event
    // Verify enemy transitions to Return
    
    // 5. Enemy returns to spawn
    app.update();
    // Verify enemy transitions to Patrol
}
```

## Performance Testing

### AI Update Benchmark

```rust
// benches/enemy_ai_bench.rs

use criterion::{criterion_group, criterion_main, Criterion};
use rust_shadow_dungeon::infrastructure::*;

fn bench_ai_perception_check(c: &mut Criterion) {
    c.bench_function("perception_check", |b| {
        b.iter(|| {
            // Check perception for single enemy
            // Target: <0.5ms
        });
    });
}

fn bench_ai_state_machine_update(c: &mut Criterion) {
    c.bench_function("state_machine_update", |b| {
        b.iter(|| {
            // Update state machine for single enemy
            // Target: <0.3ms
        });
    });
}

fn bench_ai_batch_update(c: &mut Criterion) {
    c.bench_function("batch_update_10_enemies", |b| {
        b.iter(|| {
            // Update 10 enemies
            // Target: <2ms total
        });
    });
}

criterion_group!(benches, bench_ai_perception_check, bench_ai_state_machine_update, bench_ai_batch_update);
criterion_main!(benches);
```

**目标性能**:
- 单个敌人感知检查: <0.5ms
- 单个敌人状态机更新: <0.3ms
- 10 个敌人批量更新: <2ms total
- 所有 AI 系统总开销: <2ms per frame

## Configuration File Example

### Minimal Enemy AI Configuration

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
            ),
            attack_config: (
                attack_range: 32.0,
                attack_cooldown: 1.5,
                attack_damage: 10.0,
                attack_type: "melee",
            ),
        ),
    ],
)
```

## Common Issues & Solutions

### Issue 1: 敌人不检测玩家

**症状**: 玩家靠近敌人，但敌人不切换到 Chase 状态。

**可能原因**:
- `detection_range` 配置太小
- 视线检测失败（障碍物阻挡）
- 感知系统节流导致延迟
- 玩家实体查询失败

**解决方案**:
1. 检查 `Perception.detection_range` 配置
2. 验证视线检测逻辑（检查障碍物）
3. 检查感知系统的节流设置（`check_interval`）
4. 验证玩家实体查询（确保 `Player` 组件存在）

### Issue 2: 敌人不攻击

**症状**: 敌人追逐玩家但不攻击。

**可能原因**:
- `attack_range` 配置太小
- 攻击冷却未完成
- 状态转换条件不满足
- 攻击系统未正确注册

**解决方案**:
1. 检查 `AttackConfig.attack_range` 配置
2. 验证攻击冷却逻辑（`current_cooldown`）
3. 检查状态转换条件（`should_transition_to_attack`）
4. 验证攻击系统是否正确注册到插件

### Issue 3: 敌人不脱战

**症状**: 玩家离开后，敌人继续追逐。

**可能原因**:
- `aggro_drop_range` 配置太大
- 脱战条件检查失败
- 状态转换逻辑错误
- 目标位置未更新

**解决方案**:
1. 检查 `Perception.aggro_drop_range` 配置
2. 验证脱战条件检查（`should_drop_aggro`）
3. 检查状态转换逻辑（Chase → Return）
4. 验证目标位置更新（`last_seen_position`）

### Issue 4: 性能问题

**症状**: AI 系统占用过多帧时间（>2ms）。

**可能原因**:
- 感知检查未节流
- 状态机更新过于频繁
- 批量处理未优化
- 距离计算未优化

**解决方案**:
1. 增加感知检查间隔（`check_interval`）
2. 减少状态机更新频率（如果可能）
3. 使用 Bevy 并行查询批量处理
4. 优化距离计算（使用距离平方避免开方）

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

### Visual Debugging

```rust
// In a system, draw debug visualization
fn debug_ai_visualization(
    query: Query<(&Transform, &Perception, &EnemyAI)>,
    mut gizmos: Gizmos,
) {
    for (transform, perception, ai) in query.iter() {
        // Draw detection range circle
        gizmos.circle_2d(
            transform.translation.truncate(),
            perception.detection_range,
            Color::YELLOW,
        );
        
        // Draw attack range circle
        gizmos.circle_2d(
            transform.translation.truncate(),
            perception.attack_range,
            Color::RED,
        );
        
        // Draw state text
        // (requires text rendering system)
    }
}
```

### Inspect ECS State

```rust
// In a system, query components and resources
fn debug_ai_state(
    query: Query<(Entity, &EnemyAI, &Perception, &AggroTarget)>,
) {
    for (entity, ai, perception, aggro) in query.iter() {
        println!("Enemy {:?}: state={:?}, target={:?}, distance={:?}",
            entity, ai.state, aggro.current_target, perception.target_position);
    }
}
```

## Next Steps

1. **实现基础功能**: 按照 `tasks.md` 中的任务顺序实现
2. **编写测试**: 先写测试，确保测试失败，然后实现功能
3. **性能优化**: 使用基准测试验证性能目标
4. **集成测试**: 测试完整的 AI 状态机流程
5. **扩展功能**: 添加远程敌人、不同攻击类型等

## References

- [Specification](./spec.md) - 完整功能规范
- [Data Model](./data-model.md) - 数据结构定义
- [Research](./research.md) - 技术决策和研究
- [Tasks](./tasks.md) - 实现任务列表（待生成）
- [Bevy ECS Book](https://bevyengine.org/learn/book/getting-started/ecs/) - Bevy ECS 文档
- [State Machine Patterns](https://gameprogrammingpatterns.com/state.html) - 状态机模式


