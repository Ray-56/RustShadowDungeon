# Feature Specification: Enemy AI System

**Feature Branch**: `004-enemy-ai`
**Created**: 2025-11-27
**Status**: Draft
**Constitution**: v1.0.1 (Language Separation Rule enforced)
**Input**: User description: "实现敌人AI系统，包括状态机（巡逻、追逐、攻击）、仇恨系统、攻击行为。敌人可以检测玩家，追逐玩家，在范围内攻击。遵守 Constitution v1.0.1。"

---

**语言规范说明（Language Guidelines）**:
- 本文档使用中文（This document uses Chinese for Chinese projects）
- 代码示例使用英文标识符（Code examples use English identifiers）
- 技术术语保持英文（Technical terms remain in English）

## Clarifications

### Session 2025-01-27

- Q: 视线检测（Line of Sight）的实现方式是什么？ → A: 使用简化的射线检测（检查敌人到玩家路径上是否有障碍物碰撞），在 2D 侧视图中足够，性能开销低，符合性能预算
- Q: 攻击冷却时间的配置方式是什么？ → A: 每个敌人类型在配置文件中独立配置攻击冷却时间（如 `assets/data/enemies.ron`），支持灵活平衡不同敌人类型
- Q: 仇恨系统的实现方式是什么？ → A: 使用简化的当前目标锁定机制（单个目标实体 + 仇恨值），当前需求足够，未来可扩展为完整仇恨列表
- Q: 感知检测的频率和节流机制是什么？ → A: 每 3-5 帧检测一次（约 50-100ms 间隔），平衡响应性和性能，符合 <2ms 性能预算
- Q: 巡逻行为的具体实现方式是什么？ → A: 支持两种模式：默认随机点巡逻（在巡逻半径内随机选择目标点），可选预设路径点巡逻（配置中指定路径点列表）
- Q: 攻击判定帧的确定方式是什么？ → A: 在配置文件中为每个敌人类型指定攻击判定帧时间（相对于攻击动画开始的时间，如 0.2 秒），支持灵活调整不同攻击动画

## User Scenarios & Testing *(mandatory)*

### User Story 1 - 敌人巡逻 (Priority: P2)

当周围没有玩家时，敌人应按照预设或随机逻辑进行移动，展示出活跃的生态。

**Why this priority**: 增加游戏世界的生动感，避免敌人像雕像一样站立。

**Independent Test**: 在无玩家干预的情况下观察敌人行为。

**Acceptance Scenarios**:

1. **Given** 敌人初始化在空房间, **When** 没有检测到目标, **Then** 敌人根据配置进行巡逻：默认在巡逻半径内随机选择目标点移动，或按预设路径点顺序移动（如果配置了路径点）。
2. **Given** 敌人到达巡逻点, **When** 到达目标, **Then** 敌人停留短暂时间后选择下一个目标点。

---

### User Story 2 - 发现与追逐 (Priority: P1)

敌人感知到玩家接近后，切换状态并开始追逐玩家。

**Why this priority**: 核心战斗体验的前提，建立威胁感。

**Independent Test**: 玩家进入和离开敌人视野/范围，验证状态切换。

**Acceptance Scenarios**:

1. **Given** 敌人在巡逻, **When** 玩家进入感知范围 (Detection Range), **Then** 敌人切换到 Chase 状态并向玩家移动。
2. **Given** 敌人在追逐, **When** 玩家离开最大追击范围 (Aggro Drop Range), **Then** 敌人放弃追逐，返回初始位置或恢复巡逻。
3. **Given** 障碍物阻挡, **When** 玩家在墙后（无视线）, **Then** 敌人不会立即发现玩家（基于简化的射线检测，检查路径上的障碍物碰撞）。

---

### User Story 3 - 攻击行为 (Priority: P1)

当追逐的玩家进入攻击范围，敌人发动攻击。

**Why this priority**: 构成战斗的实质性威胁。

**Independent Test**: 允许敌人接近玩家，观察攻击触发频率和距离。

**Acceptance Scenarios**:

1. **Given** 敌人在 Chase 状态, **When** 玩家进入攻击范围 (Attack Range), **Then** 敌人停止移动（或根据攻击类型保持移动）并播放攻击动画/产生伤害判定。
2. **Given** 敌人刚完成一次攻击, **When** 玩家仍在范围内, **Then** 敌人等待冷却时间 (Cooldown) 结束后再次攻击。

---

### Edge Cases

- **丢失目标**: 玩家瞬移或快速离开导致寻路目标失效，AI 应平滑处理（如移动到最后已知位置）。
- **无法到达**: 玩家处于不可达位置（如悬崖对面），AI 应保持在最近可达点或进入远程攻击（如果有）/待机。
- **多目标**: 如果存在多个可攻击目标（如召唤物），AI 应基于简化的仇恨系统选择目标（当前选择仇恨值最高的目标，未来可扩展为完整仇恨列表）。

### Combat Mechanics Testing (if applicable)

- **Hit Detection**: 验证敌人攻击的 Hitbox 是否准确匹配视觉效果。
- **Attack Range**: 验证攻击判定是否严格遵守配置的距离，避免"空气刀"。
- **Attack Hit Frame**: 验证攻击判定是否在配置的判定帧时间触发，确保与攻击动画同步。
- **Cooldown**: 验证攻击频率是否符合设计，防止连续高频判定。

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST 实现一个有限状态机 (FSM)，包含至少 `Patrol`, `Chase`, `Attack` 三种状态。
- **FR-002**: System MUST 每帧或定期检测玩家与敌人的距离和视线 (Line of Sight)。感知检测每 3-5 帧执行一次（约 50-100ms 间隔），视线检测使用简化的射线检测（检查敌人到玩家路径上是否有障碍物碰撞），性能开销低，符合 <2ms 性能预算。
- **FR-003**: System MUST 在 `Patrol` 状态下控制敌人移动。支持两种模式：默认随机点巡逻（在巡逻半径内随机选择目标点），可选预设路径点巡逻（配置中指定路径点列表，按顺序或循环访问）。
- **FR-004**: System MUST 在发现玩家时将状态切换为 `Chase`，并更新寻路目标为玩家当前位置。
- **FR-005**: System MUST 在玩家进入 `Attack Range` 且攻击未冷却时，切换为 `Attack` 状态执行攻击逻辑。
- **FR-006**: System MUST 维护简化的当前目标锁定机制（单个目标实体 + 仇恨值），支持当前需求。未来可扩展为完整的仇恨列表（支持多个目标、仇恨转移等）。
- **FR-007**: System MUST 处理攻击冷却 (Cooldown) 和状态恢复逻辑（攻击结束后回到 Chase 或 Patrol）。攻击冷却时间在配置文件中为每个敌人类型独立配置（如 `assets/data/enemies.ron`）。

### Key Entities *(include if feature involves data)*

- **EnemyAIComponent**: 核心组件，存储当前状态、配置参数（视野距离、速度、攻击冷却时间等）。攻击冷却时间从配置文件（`assets/data/enemies.ron`）中加载，每个敌人类型独立配置。
- **State Machine**: 管理状态转换逻辑。
- **Senses/Perception**: 负责环境检测（视觉、听觉/距离）。

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 敌人在发现玩家 0.5 秒内做出反应（切换状态）。
- **SC-002**: 追逐过程中，敌人每秒更新路径至少 2 次（或根据位置变化动态更新），确保紧跟玩家。
- **SC-003**: 攻击判定准确，只有在攻击范围内且攻击动作生效帧期间才造成伤害。攻击判定帧时间在配置文件中为每个敌人类型指定（相对于攻击动画开始的时间）。
- **SC-004**: 玩家脱战后，敌人能正确返回原位或进入巡逻，不卡在 Chase 状态。

### Assumptions

- **Movement System**: 假设基础移动和寻路 (Pathfinding/Navigation) 功能由 `01-movement` 或引擎提供。
- **Combat Core**: 假设伤害计算和生命值扣除接口由 `002-combat-core` 提供。
- **Animations**: 假设已有基础的 Idle, Run, Attack 动画资源或占位符供状态机调用。
