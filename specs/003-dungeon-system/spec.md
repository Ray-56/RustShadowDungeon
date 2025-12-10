# Feature Specification: Dungeon System

**Feature Branch**: `003-dungeon-system`
**Created**: 2025-11-27
**Status**: Draft
**Constitution**: v1.0.1 (Language Separation Rule enforced)
**Input**: User description: "实现地下城系统，包括房间管理、房间过渡、进度追踪。玩家可以进入地下城，清理多个房间，房间之间有门连接，清理完所有敌人后可以进入下一个房间。遵守 Constitution v1.0.1。"

---

**语言规范说明（Language Guidelines）**:
- 本文档使用中文（This document uses Chinese for Chinese projects）
- 代码示例使用英文标识符（Code examples use English identifiers）
- 技术术语保持英文（Technical terms remain in English）

## Clarifications

### Session 2025-01-27

- Q: 房间与门的连接关系是什么？（每个房间的门数量、门的单向/双向性） → A: 每个房间可以有多个门，门是双向的（支持分支、环形、返回路径）
- Q: 玩家在房间内死亡时，地下城状态应如何处理？ → A: 玩家在房间入口重生，当前房间状态保持不变（敌人仍死亡，门仍解锁）
- Q: 房间清理判定的触发时机是什么？（敌人死亡后何时检查房间是否清理完成） → A: 敌人死亡动画播放完成后判定（从存活列表移除时）
- Q: 玩家通过门进入新房间时，应出现在哪里？ → A: 目标房间对应门的入口点（从 Room A 的门1进入，出现在 Room B 连接门1的入口位置）
- Q: 玩家如何与门交互以进入下一个房间？ → A: 按键交互（玩家靠近门时显示提示，按交互键传送）
- Q: 房间和门的 ID 唯一性规则是什么？ → A: 房间 ID 全局唯一（整个地下城内不重复），门 ID 在房间内唯一（不同房间可以有相同的门 ID，使用复合键 (room_id, door_id) 来全局标识门）
- Q: 敌人生成点如何配置敌人类型？ → A: 每个生成点配置包含坐标和敌人类型 ID（如 `(position: [100, 0], enemy_type: "goblin")`），每个生成点独立配置敌人类型
- Q: 房间大小和边界如何定义？ → A: 房间配置中包含房间大小（width, height）和边界信息，每个房间可以有不同的尺寸
- Q: 房间加载的性能要求是什么？ → A: 允许多帧加载（<100ms），但需要加载动画/过渡效果，确保玩家体验流畅
- Q: 配置文件错误时如何处理？ → A: 启动时验证配置，错误时记录日志并回退到默认配置（测试地下城），确保系统可用性

## User Scenarios & Testing *(mandatory)*

### User Story 1 - 进入地下城与房间初始化 (Priority: P1)

玩家开始地下城探险，系统加载第一个房间并生成敌人。

**Why this priority**: 这是地下城体验的基础入口，没有它无法开始游戏。

**Independent Test**: 可以独立测试场景加载和实体生成逻辑，确保玩家进入后能看到正确的环境和敌人。

**Acceptance Scenarios**:

1. **Given** 玩家位于地下城入口, **When** 玩家选择进入地下城, **Then** 系统加载第一个房间 (Room 1) 并将玩家放置在出生点。
2. **Given** 房间初始化, **When** 房间加载完成, **Then** 该房间配置的敌人生成在指定位置。
3. **Given** 玩家进入房间, **When** 房间未被清理, **Then** 房间的出口门处于锁定状态（不显示交互提示，按键无效）。

---

### User Story 2 - 清理房间与解锁 (Priority: P1)

玩家击败房间内所有敌人后，房间状态更新为已清理，通往下一个房间的门解锁。

**Why this priority**: 这是核心玩法循环：战斗 -> 胜利 -> 前进。

**Independent Test**: 可以通过作弊指令或实际战斗杀死敌人，验证门的状态变化逻辑。

**Acceptance Scenarios**:

1. **Given** 玩家在有敌人的房间中, **When** 玩家击败一个敌人但仍有剩余敌人, **Then** 门保持锁定状态。
2. **Given** 玩家在有敌人的房间中, **When** 玩家击败最后一个敌人且其死亡动画播放完成, **Then** 系统提示房间清理完成 (Room Cleared)。
3. **Given** 房间清理完成, **When** 状态更新, **Then** 所有连接其他房间的门解锁/变为可交互。

---

### User Story 3 - 房间过渡 (Priority: P1)

玩家通过解锁的门进入下一个房间，系统处理场景切换和新房间加载。

**Why this priority**: 实现地下城的连续性探索。

**Independent Test**: 在已解锁的门处触发交互，验证玩家坐标变更和新环境加载。

**Acceptance Scenarios**:

1. **Given** 门已解锁, **When** 玩家靠近门并按交互键, **Then** 玩家被传送至目标房间对应门的入口点（从 Room A 的门1进入，出现在 Room B 连接门1的入口位置；门是双向的，可以从任意方向通过）。
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

- **玩家死亡**: 玩家在房间内死亡时，在房间入口重生，当前房间状态保持不变（已死亡的敌人不会重新生成，已解锁的门保持解锁状态）。已清理的其他房间状态也保持不变。
- **无敌人的房间**: 如果房间配置为无敌人（如奖励房），进入时应直接判定为 Cleared 并解锁门。
- **并发状态**: 如果敌人死亡的同时玩家尝试开门（极低概率），系统应确保状态一致性。
- **配置文件错误**: 如果 RON 配置文件格式错误、缺失或数据无效，系统应在启动时验证配置，错误时记录日志并回退到默认配置（测试地下城），确保系统可用性。

### Assumptions

- **Dungeon Generation**: 本功能假设地下城布局（房间连接图）是预先定义好或由独立模块生成的，本系统仅负责加载和运行。
- **Combat & Movement**: 假设战斗（伤害计算、死亡判定）和玩家移动（`01-movement`, `002-combat-core`）已实现并可用。
- **Progression**: 假设地下城进度是单次运行（Run-based），玩家离开地下城或死亡通常意味着进度重置（除非另有持久化设计，本阶段暂不涉及跨Run持久化）。

### Combat Mechanics Testing (if applicable)

虽然本功能主要关注系统架构，但涉及"清理敌人"，需要验证：

- **Hit Detection**: 确保所有敌人死亡被正确计数。
- **Death State**: 敌人死亡动画播放完毕后，从存活列表移除，此时触发房间清理检查（这是房间清理判定的唯一触发时机）。

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST 能够加载定义好的 Dungeon 结构，包含多个 Room 和连接它们的 Door。每个房间配置包含房间大小（width, height）和边界信息。
- **FR-002**: System MUST 追踪当前激活的 Room 和玩家在其中的位置。
- **FR-003**: System MUST 在玩家进入未清理房间时生成配置的 Enemy 列表。每个生成点配置包含位置坐标和敌人类型 ID，系统根据配置在指定位置生成对应类型的敌人。
- **FR-004**: System MUST 实时监控房间内 Enemy 的存活状态（Enemy 从存活列表移除时触发检查，即死亡动画完成后）。
- **FR-005**: System MUST 在房间内所有 Enemy 死亡动画播放完成并从存活列表移除后，触发 Room Cleared 事件。
- **FR-006**: System MUST 在 Room Cleared 事件发生时，解锁当前房间的所有 Door（门是双向的，解锁后玩家可以从任意方向通过）。
- **FR-007**: System MUST 处理玩家与 Door 的交互（玩家靠近解锁的门时显示交互提示，按交互键触发传送），将玩家转移至目标 Room 对应门的入口点（每个门在目标房间都有对应的入口位置）。
- **FR-008**: System MUST 维护一个 Session 级的数据结构，记录所有已清理的 Room ID。
- **FR-009**: System MUST 在玩家死亡时，将玩家重生在当前房间的入口点，保持房间状态不变（敌人状态、门状态均保持不变）。
- **FR-010**: System MUST 在启动时验证地下城配置文件（格式、必需字段、数据有效性），如果配置错误，记录日志并回退到默认测试地下城配置，确保系统可用性。

### Key Entities *(include if feature involves data)*

- **DungeonManager**: 管理整个地下城生命周期，持有房间图谱。
- **Room**: 包含房间大小（width, height）、边界信息、敌人生成点、门位置、房间状态（Active, Cleared, Uncleared）。房间 ID 在整个地下城内全局唯一。每个房间可以有不同的尺寸。
- **Door/Gateway**: 连接两个 Room 的实体，具有状态（Locked, Unlocked）。每个房间可以有多个门，门是双向的，支持玩家在房间之间自由往返。门 ID 在房间内唯一，全局标识使用复合键 (room_id, door_id)。
- **EnemySpawner**: 负责在房间激活时实例化敌人。每个生成点配置包含位置坐标和敌人类型 ID，系统根据配置在指定位置生成对应类型的敌人。

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 玩家可以连续通过至少 3 个房间的完整流程（进入 -> 战斗 -> 解锁 -> 离开）。
- **SC-002**: 房间清理判定准确率 100%，无"敌人全死门不开"或"门提前开"的错误。
- **SC-003**: 房间切换过程流畅，玩家坐标准确重置在目标房间对应门的入口点。
- **SC-004**: 已清理房间在返回时保持无敌人状态。
- **SC-005**: 房间加载和过渡时间 <100ms，需要加载动画/过渡效果确保玩家体验流畅。
