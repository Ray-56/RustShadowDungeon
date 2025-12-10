# Enemy AI System - 游戏中的表现

**功能**: 004-enemy-ai  
**创建日期**: 2025-01-27  
**状态**: 已实施

## 概述

本文档描述敌人 AI 系统在游戏中的实际表现和行为。系统已实施并注册，但部分功能需要与移动系统集成才能完全生效。

## 当前游戏中的表现

### 1. 敌人生成

**位置**: `src/infrastructure/systems/enemy.rs::spawn_slime_system`

当游戏启动时，系统会在位置 (200, 0) 生成一个史莱姆敌人，包含：

- ✅ **基础组件**: Enemy, EnemyType, Health, Stats, HurtBox
- ✅ **AI 组件**: EnemyAI (初始状态: Patrol), Perception, AggroTarget, PatrolConfig, AttackConfig
- ✅ **视觉**: 绿色 16×16 像素精灵

**当前状态**: ✅ 敌人生成时会自动添加所有 AI 组件

### 2. AI 系统运行流程

系统按以下顺序每帧运行（在 `EnemyPlugin` 中注册）：

```
Update 阶段:
1. perception_system (每 3-5 帧执行一次，约 50-100ms 间隔)
   ↓
2. ai_state_machine_system (每帧执行)
   ↓
3. patrol_system → chase_system → attack_system → return_to_patrol_system (链式执行)
   ↓
4. attack_cooldown_system (每帧执行)
```

### 3. 感知系统 (Perception System)

**行为**:
- ✅ 每 3-5 帧检测一次玩家位置（节流优化）
- ✅ 计算敌人到玩家的距离
- ✅ 检查视线（当前为简化实现，始终返回 true）
- ✅ 如果玩家在检测范围内（200 像素）且有视线，触发 `EnemyDetectedPlayer` 事件
- ✅ 如果玩家超出脱战范围（400 像素）或失去视线超过 5 秒，触发 `EnemyLostTarget` 事件

**当前表现**:
- ✅ 系统正常运行
- ⚠️ 视线检测为简化实现（需要后续完善）

### 4. 状态机系统 (AI State Machine)

**状态转换逻辑**:

```
Patrol (初始状态)
  ↓ (检测到玩家在检测范围内)
Chase (追逐状态)
  ↓ (玩家进入攻击范围且冷却完成)
Attack (攻击状态)
  ↓ (攻击动画完成)
Chase (如果目标仍在) 或 Return (如果目标丢失)
  ↓ (返回生成位置)
Patrol
```

**当前表现**:
- ✅ 状态转换逻辑已实现
- ⚠️ 距离计算在状态机中为简化实现（使用检测范围作为代理）

### 5. 巡逻系统 (Patrol System)

**行为**:
- ✅ 敌人在 Patrol 状态时选择巡逻目标
- ✅ 支持两种模式：
  - **随机点模式**: 在巡逻半径（100 像素）内随机选择目标点
  - **路径点模式**: 按预设路径点顺序移动（如果配置了 waypoints）
- ✅ 到达目标点后等待 2 秒，然后选择下一个目标

**当前表现**:
- ✅ 系统正常运行，会计算巡逻目标
- ⚠️ **敌人不会实际移动** - 因为 `patrol_system` 只计算目标，没有与移动系统集成

### 6. 追逐系统 (Chase System)

**行为**:
- ✅ 敌人在 Chase 状态时应该追逐玩家
- ✅ 系统会检查敌人是否在 Chase 状态

**当前表现**:
- ⚠️ **敌人不会实际移动** - 因为 `chase_system` 是占位符，没有与移动系统集成
- ⚠️ 需要与移动系统（bevy-tnua 或 VelocityComponent）集成才能实现实际移动

### 7. 攻击系统 (Attack System)

**行为**:
- ✅ 敌人在 Attack 状态时，在攻击动画的判定帧（0.2 秒）触发 `EnemyAttackTriggered` 事件
- ✅ 攻击动画完成后（0.5 秒）返回 Chase 或 Return 状态
- ✅ 触发的事件包含：enemy, target, damage, attack_type, attack_position

**当前表现**:
- ✅ 系统正常运行，会触发攻击事件
- ✅ 战斗系统可以监听 `EnemyAttackTriggered` 事件来处理伤害
- ⚠️ 攻击动画需要与动画系统集成

### 8. 攻击冷却系统 (Attack Cooldown System)

**行为**:
- ✅ 每帧递减攻击冷却时间
- ✅ 攻击动画完成后启动冷却（1.5 秒）

**当前表现**:
- ✅ 系统正常运行
- ⚠️ 冷却启动时机需要优化（当前在 attack_cooldown_system 中处理）

### 9. 返回巡逻系统 (Return to Patrol System)

**行为**:
- ✅ 敌人在 Return 状态时应该返回生成位置
- ✅ 返回完成后（1 秒后）切换到 Patrol 状态

**当前表现**:
- ⚠️ **敌人不会实际移动** - 因为 `return_to_patrol_system` 是占位符，没有与移动系统集成

## 当前限制

### 1. 移动系统集成缺失

**问题**: 敌人 AI 系统计算了移动目标，但没有实际移动敌人。

**原因**:
- `patrol_system` 只计算巡逻目标，不更新 Transform
- `chase_system` 是占位符，没有实现移动逻辑
- `return_to_patrol_system` 是占位符，没有实现移动逻辑

**解决方案**:
- 需要与移动系统集成（使用 VelocityComponent 或 bevy-tnua）
- 或者直接更新 Transform（如果敌人使用 Kinematic 刚体）

### 2. 视线检测简化

**问题**: `check_line_of_sight` 函数当前始终返回 true。

**原因**: 简化实现，避免复杂的射线检测。

**解决方案**:
- 实现完整的射线检测，查询碰撞体
- 检查从敌人到玩家的路径上是否有障碍物

### 3. 距离计算简化

**问题**: `ai_state_machine_system` 中的距离计算使用简化逻辑。

**原因**: 需要从 Transform 获取实际位置，当前使用检测范围作为代理。

**解决方案**:
- 在状态机系统中查询 Transform 组件
- 计算实际距离

## 游戏中的实际表现

### 当前状态

1. **敌人生成**: ✅ 正常，敌人会在 (200, 0) 生成，包含所有 AI 组件
2. **感知检测**: ✅ 正常，每 3-5 帧检测一次玩家
3. **状态转换**: ✅ 正常，状态机会根据感知结果转换状态
4. **事件触发**: ✅ 正常，会触发 EnemyDetectedPlayer, EnemyLostTarget, EnemyAttackTriggered 事件
5. **敌人移动**: ❌ **不工作** - 敌人不会移动，因为移动系统未集成

### 预期行为（移动系统集成后）

1. **巡逻**: 敌人在 (200, 0) 周围 100 像素范围内随机移动
2. **追逐**: 玩家进入 200 像素范围后，敌人开始追逐玩家
3. **攻击**: 玩家进入 32 像素攻击范围后，敌人停止移动并攻击
4. **返回**: 玩家离开 400 像素范围后，敌人返回 (200, 0) 并恢复巡逻

## 调试建议

### 查看 AI 状态

在游戏中，可以通过以下方式查看敌人 AI 状态：

1. **日志输出**: 系统会在控制台输出状态转换信息
2. **事件监听**: 监听 `EnemyDetectedPlayer`, `EnemyLostTarget`, `EnemyAttackTriggered` 事件
3. **组件查询**: 在调试系统中查询 `EnemyAI` 组件查看当前状态

### 验证系统运行

```rust
// 在调试系统中添加
fn debug_ai_state(
    query: Query<(Entity, &EnemyAI, &Perception, &AggroTarget), With<Enemy>>,
) {
    for (entity, ai, perception, aggro) in query.iter() {
        println!("Enemy {:?}: state={:?}, target={:?}, distance={:?}",
            entity, ai.state, aggro.current_target, perception.target_position);
    }
}
```

## 下一步

1. **移动系统集成**: 将 AI 系统与移动系统集成，实现实际移动
2. **完善视线检测**: 实现完整的射线检测
3. **优化距离计算**: 在状态机中使用实际 Transform 位置
4. **性能测试**: 运行性能基准测试，验证是否符合 <2ms 预算

## 总结

敌人 AI 系统的**核心逻辑已完全实施**，包括：
- ✅ 状态机
- ✅ 感知系统
- ✅ 事件系统
- ✅ 攻击系统

但**移动功能需要与移动系统集成**才能实现实际移动。当前敌人会检测玩家、转换状态、触发事件，但不会移动。


