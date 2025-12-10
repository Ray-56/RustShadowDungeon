# 敌人 AI 系统 - 当前状态与下一步行动

**最后更新**: 2025-01-27  
**当前状态**: 核心逻辑和移动系统已实施 ✅

## 当前状态总结

✅ **已完成**:
- AI 状态机（Patrol, Chase, Attack, Return）✅
- 感知系统（每 3-5 帧检测玩家）✅
- 事件系统（EnemyDetectedPlayer, EnemyLostTarget, EnemyAttackTriggered）✅
- 攻击系统（攻击判定帧、冷却管理）✅
- **移动系统（patrol_system, chase_system, return_to_patrol_system）✅**
  - `patrol_system`: 巡逻移动，速度 50 像素/秒，支持路径点和随机目标
  - `chase_system`: 追逐玩家，速度 80 像素/秒
  - `return_to_patrol_system`: 返回生成点，速度 50 像素/秒
- 单元测试（8 个测试通过）✅
- 系统注册：所有移动系统已在 `EnemyPlugin` 中注册 ✅

✅ **已完成**:
- **视线检测（完整实现）** ✅
  - 位置：`perception_system` 中的 `check_line_of_sight` 调用
  - 实现：完整的步进式射线检测，查询所有带 `Obstacle` 组件的实体
  - 障碍物支持：`Obstacle` 标记组件 + `ObstacleCollider`（可选，用于自定义碰撞矩形）
  - 平台已标记：所有测试平台已添加 `Obstacle` 组件
- **精确距离计算（已优化）** ✅
  - 位置：`ai_state_machine_system` 和 `perception_system`
  - 实现：使用实际 `Transform.translation` 位置计算精确距离
  - 所有状态转换判断都使用真实距离值

## 下一步优先级

### ✅ 已完成：完善视线检测和优化距离计算

**状态**: 视线检测和距离计算已完全实现 ✅

**实现详情**:
```rust
// 在 src/domain/enemy/ai.rs 中
pub fn check_line_of_sight(
    enemy_pos: Vec2,
    target_pos: Vec2,
    obstacles: &[Rect], // 障碍物列表（从碰撞系统获取）
) -> bool {
    // 简化的射线检测：检查从敌人到目标的路径上是否有障碍物
    let direction = target_pos - enemy_pos;
    let distance = direction.length();
    let step_size = 8.0; // 每 8 像素检查一次
    let steps = (distance / step_size).ceil() as usize;
    
    for i in 0..steps {
        let t = (i as f32) / (steps as f32);
        let check_pos = enemy_pos + direction * t;
        
        // 检查是否有障碍物
        for obstacle in obstacles {
            if obstacle.contains(check_pos) {
                return false;
            }
        }
    }
    
    true
}
```

**实现位置**:
- `src/infrastructure/components/obstacle.rs`: 定义了 `Obstacle` 和 `ObstacleCollider` 组件
- `src/infrastructure/systems/enemy.rs`: `perception_system` 查询障碍物并传递给 `check_line_of_sight`
- `src/infrastructure/plugins/player.rs`: 所有测试平台已添加 `Obstacle` 组件

**距离计算优化**:
- `ai_state_machine_system`: 使用 `enemy_pos.distance(target_pos)` 计算精确距离
- `perception_system`: 使用 `enemy_transform.translation.truncate().distance(...)` 计算精确距离
- 所有状态转换判断都基于真实距离值

### 🟢 优先级 1：性能测试（验证）

**目标**: 验证 AI 系统是否符合 <2ms 性能预算。

**当前状态**: 性能基准测试文件已存在 `benches/enemy_ai_bench.rs`

**实现**:
```rust
// 在 benches/enemy_ai_bench.rs 中
#[bench]
fn bench_ai_update_100_enemies(b: &mut Bencher) {
    // 创建 100 个敌人
    // 运行 AI 系统
    // 测量时间
    // 目标: <2ms
}
```

**预计时间**: 1 小时

### 🟢 优先级 2：集成测试（完整）

**目标**: 使用 Bevy TestApp 实现完整的集成测试。

**当前状态**: 已有部分集成测试 `tests/integration/enemy/ai_state_machine_test.rs`

**实现**:
```rust
// 在 tests/integration/enemy/ai_state_machine_test.rs 中
#[test]
fn test_patrol_to_chase_transition() {
    let mut app = TestApp::new();
    
    // 设置测试场景
    // 生成敌人和玩家
    // 移动玩家进入检测范围
    // 验证状态转换
    // 验证敌人实际移动
}
```

**预计时间**: 1-2 小时

## 已实现的移动系统详情

### patrol_system（巡逻系统）

**位置**: `src/infrastructure/systems/enemy.rs:624`

**功能**:
- 支持路径点模式（waypoints）和随机目标模式
- 到达目标后等待，然后选择下一个目标
- 移动速度：50 像素/秒
- 直接更新 `Transform.translation`

**实现方式**: 直接更新 Transform（方案 A）

### chase_system（追逐系统）

**位置**: `src/infrastructure/systems/enemy.rs:470`

**功能**:
- 追逐 `Perception.target_position`（玩家位置）
- 移动速度：80 像素/秒（比巡逻快）
- 直接更新 `Transform.translation`

**实现方式**: 直接更新 Transform（方案 A）

### return_to_patrol_system（返回巡逻系统）

**位置**: `src/infrastructure/systems/enemy.rs:502`

**功能**:
- 返回 `PatrolConfig.spawn_position`（生成位置）
- 移动速度：50 像素/秒（与巡逻相同）
- 到达生成点后（距离 < 5.0）停止移动
- 直接更新 `Transform.translation`

**实现方式**: 直接更新 Transform（方案 A）

### 系统执行顺序

在 `EnemyPlugin` 中，系统按以下顺序执行：
1. `perception_system` - 感知检测（节流，每 3-5 帧）
2. `ai_state_machine_system` - 状态机更新
3. `patrol_system` - 巡逻移动
4. `chase_system` - 追逐移动
5. `attack_system` - 攻击行为
6. `return_to_patrol_system` - 返回移动
7. `attack_cooldown_system` - 冷却管理

## 实施建议

### ✅ 已完成：视线检测和距离计算优化

**已完成的工作**:

1. **创建障碍物组件系统**:
   - 定义了 `Obstacle` 标记组件
   - 定义了 `ObstacleCollider` 组件（可选，用于自定义碰撞矩形）
   - 组件已导出到 `src/infrastructure/components/mod.rs`

2. **完善视线检测**:
   - `perception_system` 现在查询所有带 `Obstacle` 组件的实体
   - 收集障碍物的碰撞矩形（优先使用 `ObstacleCollider`，否则使用 `Sprite` 大小）
   - 将障碍物列表传递给 `check_line_of_sight` 函数
   - 视线检测现在会正确检测障碍物阻挡

3. **优化距离计算**:
   - `ai_state_machine_system` 使用 `enemy_pos.distance(target_pos)` 计算精确距离
   - `perception_system` 使用实际 Transform 位置计算距离
   - 所有状态转换判断都基于真实距离值

4. **标记现有平台**:
   - 所有测试平台（4 个）已添加 `Obstacle` 组件
   - 敌人 AI 现在可以正确检测平台阻挡视线

### 下一步：性能测试（1 小时）

1. **创建性能基准测试**:
   - 测试 10、50、100 个敌人的性能
   - 验证是否符合 <2ms 预算

2. **优化**（如果需要）:
   - 进一步节流感知检查
   - 使用空间分区优化

### 下一步：集成测试（1-2 小时）

1. **实现完整集成测试**:
   - 使用 Bevy TestApp
   - 测试完整 AI 流程
   - 验证敌人实际移动

2. **验证**:
   - 所有测试通过

## 预计时间

- **✅ 视线检测和距离计算优化**: 已完成
- **性能测试**: 1 小时
- **集成测试**: 1-2 小时

**剩余时间**: 2-3 小时

## 快速验证

如果你想验证敌人移动系统是否工作：

1. **运行游戏**:
   ```bash
   cargo run
   ```

2. **观察敌人行为**:
   - 敌人应该在生成位置附近巡逻
   - 当玩家接近时，敌人应该追逐玩家
   - 当玩家远离时，敌人应该返回生成位置

3. **检查系统日志**:
   - 查看是否有 AI 状态转换的日志
   - 确认 `patrol_system`、`chase_system`、`return_to_patrol_system` 正在运行

## 总结

**当前状态**: 敌人 AI 移动系统、视线检测和距离计算已完全实现并集成 ✅

**最关键的下一步**: 性能测试和集成测试，验证系统性能和完整性。

**推荐顺序**:
1. ✅ 视线检测完善（已完成）
2. ✅ 距离计算优化（已完成）
3. 性能测试（验证）
4. 集成测试（完整）

**注意**: 如果游戏中敌人不移动，请检查：
- 敌人实体是否包含必要的组件（`EnemyAI`、`PatrolConfig`、`Perception`）
- 状态机是否正确初始化
- 系统是否正确注册在 `EnemyPlugin` 中
