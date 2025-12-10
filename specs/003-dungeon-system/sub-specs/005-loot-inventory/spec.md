# Feature Specification: Loot and Inventory System

**Feature Branch**: `005-loot-inventory`
**Created**: 2025-11-27
**Status**: Draft
**Constitution**: v1.0.1 (Language Separation Rule enforced)
**Input**: User description: "实现战利品与库存系统，包括掉落表、物品拾取、库存管理。敌人死亡后掉落物品，玩家可以拾取并存储在库存中。遵守 Constitution v1.0.1。"

---

**语言规范说明（Language Guidelines）**:
- 本文档使用中文（This document uses Chinese for Chinese projects）
- 代码示例使用英文标识符（Code examples use English identifiers）
- 技术术语保持英文（Technical terms remain in English）

## Clarifications

### Session 2025-01-27

- Q: 库存容量（槽位数量）是多少？ → A: 30 个槽位
- Q: 拾取交互方式是手动、自动还是两者都支持？ → A: 两者都支持，可配置
- Q: 物品堆叠上限的默认值是多少？ → A: 默认 99，可配置
- Q: 拾取范围（距离）是多少？ → A: 2 米
- Q: Loot Table 概率计算方式是独立概率还是互斥概率？ → A: 独立概率，可多物品同时掉落

## User Scenarios & Testing *(mandatory)*

### User Story 1 - 战利品掉落 (Priority: P1)

当敌人被击败时，系统根据预设概率生成物品并掉落在场景中。

**Why this priority**: 奖励机制是游戏循环的关键部分。

**Independent Test**: 在测试场景中生成并杀死敌人，验证是否有物品生成。

**Acceptance Scenarios**:

1. **Given** 敌人死亡, **When** 掉落逻辑触发, **Then** 系统根据 Loot Table 使用独立概率计算并生成掉落物实体（可同时掉落多个物品）。
2. **Given** 掉落物生成, **When** 物品出现在场景中, **Then** 它具有视觉模型和可交互提示。

---

### User Story 2 - 物品拾取 (Priority: P1)

玩家接近掉落物并与之交互，物品从场景消失并进入玩家库存。

**Why this priority**: 获取资源的唯一途径。

**Independent Test**: 控制玩家接近物品并触发拾取操作。

**Acceptance Scenarios**:

1. **Given** 玩家站在掉落物 2 米拾取范围内, **When** 玩家按下拾取键（手动模式）或进入拾取范围（自动模式，如果启用）, **Then** 物品实体销毁，玩家收到"获得物品"的提示。
2. **Given** 玩家背包已满, **When** 尝试拾取, **Then** 拾取失败，系统提示"背包已满"。

---

### User Story 3 - 库存管理 (Priority: P2)

玩家查看背包，可以看到已获得的物品列表。

**Why this priority**: 玩家需要确认和管理自己的战利品。

**Independent Test**: 打开库存界面，检查物品显示是否正确。

**Acceptance Scenarios**:

1. **Given** 玩家已拾取物品 A, **When** 打开库存界面, **Then** 列表中显示物品 A 的图标和数量。
2. **Given** 玩家拥有多个同类可堆叠物品, **When** 查看库存, **Then** 物品显示为一个堆叠槽位，数量正确增加。

---

### Edge Cases

- **并发拾取**: 多人模式下（如果有），两个玩家同时尝试拾取同一个物品，只能有一人成功。
- **掉落位置**: 物品如果掉落在不可达区域（如墙内），应有防卡死机制（如弹射到最近有效位置）。
- **库存溢出**: 拾取数量超过堆叠上限时，多余部分应开启新槽位或保留在地面。

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST 支持 Loot Table 配置，允许为不同敌人定义不同的掉落池和概率。使用独立概率计算，多个物品可以同时掉落。
- **FR-002**: System MUST 在场景中实例化 World Item，具备物理碰撞（或触发器）和视觉表现。
- **FR-003**: System MUST 提供 Inventory 数据结构，支持固定大小的槽位管理（30 个槽位）。
- **FR-004**: System MUST 支持物品堆叠逻辑，对于标记为 `Stackable` 的物品合并数量。默认堆叠上限为 99，可在 ItemDefinition 中配置。
- **FR-005**: System MUST 提供拾取交互接口，处理从 World Item 到 Inventory Item 的转换。支持手动拾取（按键触发）和自动拾取（进入范围触发），拾取模式可配置。拾取范围为 2 米。
- **FR-006**: System MUST 提供基础的 UI 数据接口，用于显示当前库存内容。

### Key Entities *(include if feature involves data)*

- **LootTable**: 定义掉落规则（Item ID, Chance, Quantity Range）。使用独立概率计算，每个物品独立判断是否掉落，可同时掉落多个物品。
- **InventoryComponent**: 附着于玩家，存储 `Vec<Slot>` 或类似结构（固定 30 个槽位）。
- **ItemDefinition**: 定义物品静态属性（Name, Icon, MaxStack）。MaxStack 默认值为 99，可配置。
- **WorldItem**: 场景中的物品实体，具有 2 米拾取范围。

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 掉落概率准确，100次测试中的实际掉落分布符合配置的误差范围（±5%）。
- **SC-002**: 拾取响应无延迟，物品进入背包的数据更新在下一帧立即可见。
- **SC-003**: 背包满时拾取操作被正确拒绝，且无物品丢失或异常销毁。

### Assumptions

- **Combat System**: 敌人死亡事件由 `002-combat-core` 或 `004-enemy-ai` 触发。
- **UI System**: 假设已有基础 UI 框架用于绘制库存界面，本功能主要提供数据。
