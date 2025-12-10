# Research: Enemy AI System

**Feature**: 004-enemy-ai  
**Date**: 2025-01-27  
**Status**: Complete

## Overview

本文档记录敌人 AI 系统实现过程中的技术决策和研究结果。所有 "NEEDS CLARIFICATION" 标记已在规范中明确。

## Technical Decisions

### 1. 状态机实现方式

**Decision**: 使用枚举 `AIState` 表示 AI 状态，状态转换逻辑在领域层实现为纯函数。

**Rationale**:
- 状态明确，易于理解和维护
- 领域层纯函数便于测试，不依赖 Bevy
- 支持状态查询和状态转换的清晰分离
- 符合 DDD 架构原则

**Alternatives Considered**:
- **状态模式（State Pattern）**: 过度设计，对于简单的状态机来说太复杂
- **行为树（Behavior Tree）**: 未来扩展可以考虑，但当前需求不需要
- **仅使用组件状态**: 状态转换逻辑分散，难以测试和维护

**Implementation**:
```rust
// Domain layer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AIState {
    Patrol,    // 巡逻状态
    Chase,     // 追逐状态
    Attack,    // 攻击状态
    Return,    // 返回状态（脱战后返回原位）
}

// Infrastructure layer - Component
#[derive(Component, Debug, Clone)]
pub struct EnemyAI {
    pub state: AIState,
    pub previous_state: AIState,
    pub state_timer: f32,  // 状态持续时间
}
```

### 2. 感知系统实现

**Decision**: 使用距离检测和简化的视线检测（Line of Sight），每 3-5 帧检查一次（约 50-100ms 间隔）以优化性能。视线检测使用简化的射线检测（检查敌人到玩家路径上是否有障碍物碰撞）。

**Rationale**:
- 距离检测简单高效，O(1) 复杂度
- 简化的射线检测在 2D 侧视图中足够，性能开销低，符合性能预算
- 节流检查（每 3-5 帧）减少性能开销，符合 <2ms 预算
- 玩家移动速度有限，3-5 帧的延迟可接受

**Alternatives Considered**:
- **每帧检查**: 性能开销大，不符合预算
- **复杂视线检测（物理引擎）**: 过度设计，2D 侧视图不需要复杂算法
- **仅距离检测**: 不够真实，玩家在墙后也能被检测到

**Implementation**:
```rust
// Domain layer
pub fn should_transition_to_chase(
    distance: f32,
    detection_range: f32,
    has_line_of_sight: bool,
) -> bool {
    distance <= detection_range && has_line_of_sight
}

// Infrastructure layer - throttled system
fn perception_system(
    mut query: Query<&mut Perception>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    enemy_query: Query<&Transform, (With<Enemy>, Without<Player>)>,
    time: Res<Time>,
) {
    // Only check every 3-5 frames (throttle, ~50-100ms interval)
    if time.elapsed_seconds() % 0.1 < 0.033 {  // ~3 frames at 60 FPS
        // Check distance and line of sight (simplified raycast)
    }
}
```

### 3. 巡逻行为实现

**Decision**: 支持两种模式：默认随机点巡逻（在巡逻半径内随机选择目标点），可选预设路径点巡逻（配置中指定路径点列表，按顺序或循环访问）。敌人移动到目标点后停留短暂时间，然后选择下一个点。

**Rationale**:
- 随机目标点提供自然的巡逻行为，实现简单
- 预设路径点支持更精确的关卡设计，灵活性高
- 停留时间增加真实感，避免敌人像机器一样移动
- 两种模式可选，适应不同场景需求

**Alternatives Considered**:
- **仅随机点**: 不够灵活，无法支持精确的关卡设计
- **仅预设路径点**: 需要额外的关卡数据，增加复杂度
- **完全随机移动**: 可能看起来不自然
- **原地待机**: 不够生动，不符合需求

**Implementation**:
```rust
// Domain layer
pub fn calculate_patrol_target(
    current_pos: Vec2,
    spawn_pos: Vec2,
    patrol_radius: f32,
) -> Vec2 {
    // Random point within patrol radius
    let angle = rand::random::<f32>() * std::f32::consts::TAU;
    let distance = rand::random::<f32>() * patrol_radius;
    spawn_pos + Vec2::new(
        angle.cos() * distance,
        angle.sin() * distance,
    )
}

// Infrastructure layer
#[derive(Component, Debug)]
pub struct PatrolConfig {
    pub spawn_position: Vec2,
    pub patrol_radius: f32,
    pub wait_time: f32,  // 到达目标点后等待时间
    pub current_target: Option<Vec2>,
    pub wait_timer: f32,
}
```

### 4. 追逐行为实现

**Decision**: 直接移动向玩家位置，使用简化的路径寻找（直线移动，遇到障碍物时尝试绕过）。

**Rationale**:
- 2D 侧视图通常不需要复杂的 A* 寻路
- 直接移动简单高效，符合性能预算
- 障碍物处理可以通过碰撞检测和简单的避障逻辑实现
- 未来可以扩展为更复杂的寻路算法

**Alternatives Considered**:
- **A* 寻路算法**: 过度设计，2D 侧视图通常不需要
- **NavMesh**: 需要额外的关卡数据，增加复杂度
- **完全直线移动**: 可能卡在障碍物上，需要基本的避障

**Implementation**:
```rust
// Domain layer
pub fn calculate_chase_direction(
    enemy_pos: Vec2,
    target_pos: Vec2,
) -> Vec2 {
    (target_pos - enemy_pos).normalize()
}

// Infrastructure layer
fn chase_system(
    mut query: Query<(&mut Transform, &EnemyAI, &Perception), (With<Enemy>, With<AIState::Chase>)>,
    time: Res<Time>,
) {
    for (mut transform, ai, perception) in query.iter_mut() {
        if let Some(target_pos) = perception.target_position {
            let direction = calculate_chase_direction(
                transform.translation.truncate(),
                target_pos,
            );
            // Update movement (integrate with movement system)
        }
    }
}
```

### 5. 攻击行为实现

**Decision**: 攻击范围检测 + 攻击冷却时间（每个敌人类型在配置文件中独立配置），攻击判定帧时间在配置文件中指定（相对于攻击动画开始的时间）。攻击时触发攻击动画和伤害判定事件。

**Rationale**:
- 攻击范围确保攻击判定准确，避免"空气刀"
- 冷却时间在配置文件中独立配置，支持灵活平衡不同敌人类型
- 攻击判定帧时间可配置，支持不同攻击动画的精确调整
- 事件驱动设计，与战斗系统解耦
- 支持不同类型的攻击（近战、远程）

**Alternatives Considered**:
- **固定攻击间隔**: 不够灵活，无法支持不同敌人类型
- **硬编码判定帧**: 不够灵活，无法适应不同攻击动画
- **无冷却时间**: 可能导致攻击频率过高，不平衡
- **攻击动画驱动**: 需要动画系统支持，当前可能不完整

**Implementation**:
```rust
// Domain layer
pub fn should_transition_to_attack(
    distance: f32,
    attack_range: f32,
    cooldown_ready: bool,
) -> bool {
    distance <= attack_range && cooldown_ready
}

// Infrastructure layer
#[derive(Component, Debug)]
pub struct AttackConfig {
    pub attack_range: f32,
    pub attack_cooldown: f32,
    pub current_cooldown: f32,
    pub attack_damage: f32,
}

// Event
#[derive(Event, Debug, Clone)]
pub struct EnemyAttackTriggered {
    pub enemy: Entity,
    pub target: Entity,
    pub damage: f32,
    pub attack_type: AttackType,
}
```

### 6. 仇恨系统实现

**Decision**: 使用简化的当前目标锁定机制（单个目标实体 + 仇恨值），当前需求足够。未来可扩展为完整的仇恨列表（支持多个目标、仇恨转移等）。

**Rationale**:
- 当前需求不需要复杂的仇恨列表
- 简化实现，性能开销低
- 单个目标 + 仇恨值的设计为未来扩展预留空间
- 符合 MVP 原则

**Alternatives Considered**:
- **完整仇恨列表**: 过度设计，当前需求不需要
- **仅玩家目标（无仇恨值）**: 不够灵活，未来扩展困难
- **基于距离的目标选择**: 可能不够智能，无法处理多目标场景

**Implementation**:
```rust
// Infrastructure layer
#[derive(Component, Debug)]
pub struct AggroTarget {
    pub current_target: Option<Entity>,
    pub aggro_value: f32,  // 仇恨值（未来扩展）
    pub last_seen_position: Option<Vec2>,
    pub time_since_last_seen: f32,
}

// Domain layer
pub fn should_drop_aggro(
    distance: f32,
    aggro_drop_range: f32,
    time_since_last_seen: f32,
    max_time_without_sight: f32,
) -> bool {
    distance > aggro_drop_range || time_since_last_seen > max_time_without_sight
}
```

### 7. 性能优化策略

**Decision**: 使用节流检查（每 3-5 帧）、空间分区（未来）、批量更新来优化 AI 性能。

**Rationale**:
- 节流检查是最简单有效的优化方法
- 空间分区可以进一步优化，但当前需求不需要
- 批量更新减少系统调用次数
- 符合 <2ms 性能预算

**Alternatives Considered**:
- **每帧检查**: 性能开销大，不符合预算
- **复杂空间分区**: 过度优化，当前敌人数量不需要
- **异步 AI 更新**: 增加复杂度，当前不需要

**Implementation Strategy**:
- 感知检查：每 3-5 帧（~50-100ms）
- 状态机更新：每帧（但逻辑简单）
- 路径更新：每 2-3 帧（追逐时）
- 攻击冷却：每帧检查（但计算简单）

## Integration Points

### 与移动系统集成

- **移动控制**: AI 系统通过设置移动目标或方向来控制敌人移动
- **寻路**: 使用简化的直线移动，未来可以集成更复杂的寻路系统
- **碰撞**: 依赖物理引擎的碰撞检测，AI 系统不直接处理碰撞

### 与战斗系统集成

- **攻击触发**: AI 系统触发 `EnemyAttackTriggered` 事件，战斗系统处理伤害计算
- **伤害接收**: 敌人接收伤害后，AI 系统可能需要响应（如进入受击状态）
- **死亡处理**: 敌人死亡时，AI 系统清理相关组件

### 与地下城系统集成

- **房间上下文**: AI 系统需要知道当前房间，以便在房间切换时重置状态
- **敌人生成**: 地下城系统生成敌人时，设置 AI 组件的初始状态
- **房间清理**: 房间清理时，AI 系统可能需要停止或重置

## Performance Considerations

### AI 更新优化

- **节流检查**: 感知检查每 3-5 帧，减少距离计算次数
- **批量处理**: 使用 Bevy 的并行查询，批量处理多个敌人
- **早期退出**: 不在范围内的敌人跳过检查
- **状态缓存**: 缓存计算结果，避免重复计算

### 内存优化

- **组件设计**: 使用紧凑的数据结构，减少内存占用
- **可选字段**: 使用 `Option` 避免不必要的内存分配
- **资源复用**: 复用配置资源，避免每个敌人都存储配置

## Testing Strategy

### 单元测试（Domain Layer）

- 测试所有 `ai.rs` 中的纯函数
- 使用 mock 数据，不依赖 Bevy
- 覆盖所有状态转换和边界情况

### 集成测试（Infrastructure Layer）

- 测试完整的 AI 状态机流程
- 测试多敌人场景
- 使用 Bevy 测试工具（`TestApp`）模拟游戏环境

### 性能测试

- 基准测试 AI 更新性能（目标 <2ms per frame）
- 测试不同敌人数量下的性能
- 使用 `criterion` 进行基准测试

## Open Questions / Future Enhancements

1. **复杂寻路**: 当前使用简化寻路，未来可能需要 A* 或其他算法
2. **行为树**: 未来可以考虑使用行为树实现更复杂的 AI
3. **多目标仇恨**: 当前是简化实现，未来可能需要完整的仇恨列表
4. **AI 配置**: 当前使用代码配置，未来可能需要 RON 配置文件
5. **不同敌人类型**: 当前主要关注近战敌人，未来需要支持远程敌人

## References

- [Bevy ECS Documentation](https://bevyengine.org/learn/book/getting-started/ecs/)
- [Game AI Pro - State Machines](http://www.gameaipro.com/)
- Project Constitution v1.0.1
- Previous feature implementations: `001-player-movement`, `002-combat-core`, `003-dungeon-system`

