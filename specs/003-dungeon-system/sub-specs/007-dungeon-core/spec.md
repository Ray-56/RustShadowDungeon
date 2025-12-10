# Feature Specification: Dungeon Core System

**Feature Branch**: `007-dungeon-core`
**Created**: 2025-11-27
**Status**: Draft
**Constitution**: v1.0.1 (Language Separation Rule enforced)
**Input**: User description: "实现地下城系统，包括房间管理、房间过渡、进度追踪。玩家可以进入地下城，清理多个房间，房间之间有门连接，清理完所有敌人后可以进入下一个房间。遵守 Constitution v1.0.1。"

---

**语言规范说明（Language Guidelines）**:
- 本文档使用中文（This document uses Chinese for Chinese projects）
- 代码示例使用英文标识符（Code examples use English identifiers）
- 技术术语保持英文（Technical terms remain in English）

## User Scenarios & Testing *(mandatory)*

### User Story 1 - 进入地下城与房间初始化 (Priority: P1)

玩家开始地下城探险，系统加载第一个房间并生成敌人。

**Why this priority**: 这是地下城体验的基础入口，没有它无法开始游戏。

**Independent Test**: 可以独立测试场景加载和实体生成逻辑，确保玩家进入后能看到正确的环境和敌人。

**Acceptance Scenarios**:

1. **Given** 玩家位于地下城入口, **When** 玩家选择进入地下城, **Then** 系统加载第一个房间 (Room 1) 并将玩家放置在出生点。
2. **Given** 房间初始化, **When** 房间加载完成, **Then** 该房间配置的敌人生成在指定位置。
3. **Given** 玩家进入房间, **When** 房间未被清理, **Then** 房间的出口门处于锁定或不可交互状态。

---

### User Story 2 - 清理房间与解锁 (Priority: P1)

玩家击败房间内所有敌人后，房间状态更新为已清理，通往下一个房间的门解锁。

**Why this priority**: 这是核心玩法循环：战斗 -> 胜利 -> 前进。

**Independent Test**: 可以通过作弊指令或实际战斗杀死敌人，验证门的状态变化逻辑。

**Acceptance Scenarios**:

1. **Given** 玩家在有敌人的房间中, **When** 玩家击败一个敌人但仍有剩余敌人, **Then** 门保持锁定状态。
2. **Given** 玩家在有敌人的房间中, **When** 玩家击败最后一个敌人, **Then** 系统提示房间清理完成 (Room Cleared)。
3. **Given** 房间清理完成, **When** 状态更新, **Then** 所有连接其他房间的门解锁/变为可交互。

---

### User Story 3 - 房间过渡 (Priority: P1)

玩家通过解锁的门进入下一个房间，系统处理场景切换和新房间加载。

**Why this priority**: 实现地下城的连续性探索。

**Independent Test**: 在已解锁的门处触发交互，验证玩家坐标变更和新环境加载。

**Acceptance Scenarios**:

1. **Given** 门已解锁, **When** 玩家与门交互, **Then** 玩家被传送至连接的下一个房间 (Room 2) 的对应入口点。
2. **Given** 房间过渡, **When** 进入新房间, **Then** 如果新房间未被探索，生成新的敌人；如果已清理，不生成敌人。
3. **Given** 玩家进入新房间, **When** 过渡完成, **Then** 摄像机和相关UI更新为新房间的上下文。

---

### User Story 4 - 进度追踪 (Priority: P2)

系统记录地下城的清理进度，确保已清理的房间不会重复刷新敌人。

**Why this priority**: 保证探索的持久性和连贯性，支持非线性探索（如果有）。

**Independent Test**: 清理房间 -> 离开 -> 返回，验证房间状态保持。

**Acceptance Scenarios**:

1. **Given** 玩家已清理 Room A, **When** 玩家离开 Room A 并进入 Room B, **Then** 系统记录 Room A 为 "Cleared"。
2. **Given** 玩家返回已清理的 Room A, **When** 房间加载, **Then** 不会再次生成敌人。

---

### Edge Cases

- **玩家死亡**: 如果在房间清理过程中玩家死亡，地下城进度重置还是保留？（根据Roguelike惯例通常重置，或回档。这里假设本次地下城挑战失败，重置当前房间或整个地下城。暂定为重置当前房间状态或结束run）。
- **无敌人的房间**: 如果房间配置为无敌人（如奖励房），进入时应直接判定为 Cleared 并解锁门。
- **并发状态**: 如果敌人死亡的同时玩家尝试开门（极低概率），系统应确保状态一致性。

### Assumptions

- **Dungeon Generation**: 本功能假设地下城布局（房间连接图）是预先定义好或由独立模块生成的，本系统仅负责加载和运行。
- **Combat & Movement**: 假设战斗（伤害计算、死亡判定）和玩家移动（`01-movement`, `002-combat-core`）已实现并可用。
- **Progression**: 假设地下城进度是单次运行（Run-based），玩家离开地下城或死亡通常意味着进度重置（除非另有持久化设计，本阶段暂不涉及跨Run持久化）。

### Combat Mechanics Testing (if applicable)

虽然本功能主要关注系统架构，但涉及"清理敌人"，需要验证：

- **Hit Detection**: 确保所有敌人死亡被正确计数。
- **Death State**: 敌人死亡动画播放完毕后，从存活列表移除，触发房间检查。

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST 能够加载定义好的 Dungeon 结构，包含多个 Room 和连接它们的 Door。
- **FR-002**: System MUST 追踪当前激活的 Room 和玩家在其中的位置。
- **FR-003**: System MUST 在玩家进入未清理房间时生成配置的 Enemy 列表。
- **FR-004**: System MUST 实时监控房间内 Enemy 的存活状态。
- **FR-005**: System MUST 在房间内所有 Enemy 死亡时，触发 Room Cleared 事件。
- **FR-006**: System MUST 在 Room Cleared 事件发生时，解锁当前房间的所有出口 Door。
- **FR-007**: System MUST 处理玩家与 Door 的交互，将玩家转移至目标 Room。
- **FR-008**: System MUST 维护一个 Session 级的数据结构，记录所有已清理的 Room ID。

### Key Entities *(include if feature involves data)*

- **DungeonManager**: 管理整个地下城生命周期，持有房间图谱。
- **Room**: 包含敌人生成点、门位置、房间状态（Active, Cleared, Uncleared）。
- **Door/Gateway**: 连接两个 Room 的实体，具有状态（Locked, Unlocked）。
- **EnemySpawner**: 负责在房间激活时实例化敌人。

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 玩家可以连续通过至少 3 个房间的完整流程（进入 -> 战斗 -> 解锁 -> 离开）。
- **SC-002**: 房间清理判定准确率 100%，无"敌人全死门不开"或"门提前开"的错误。
- **SC-003**: 房间切换过程流畅，玩家坐标准确重置在门口。
- **SC-004**: 已清理房间在返回时保持无敌人状态。
