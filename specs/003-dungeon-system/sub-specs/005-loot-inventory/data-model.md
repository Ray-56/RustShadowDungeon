# Data Model: Loot and Inventory System

**Feature**: 005-loot-inventory
**Date**: 2025-01-27

## Overview

本文档定义战利品与库存系统的数据模型，包括领域层数据结构（零 Bevy 依赖）和基础设施层 ECS 组件（Bevy 桥接）。

## Domain Layer Entities (Pure Rust, Zero Bevy Dependencies)

### LootDrop

掉落物结果（领域层计算输出）

```rust
// src/domain/loot/drop_table.rs

/// 掉落物结果
pub struct LootDrop {
    /// 物品 ID
    pub item_id: ItemId,
    /// 数量
    pub quantity: u32,
}

/// 物品 ID（类型别名，实际类型待定）
pub type ItemId = u32; // 或 String，取决于项目约定
```

### LootTableEntry

掉落表条目（配置数据）

```rust
// src/domain/loot/drop_table.rs

/// 掉落表条目
pub struct LootTableEntry {
    /// 物品 ID
    pub item_id: ItemId,
    /// 掉落概率（0.0 - 1.0）
    pub chance: f32,
    /// 最小数量
    pub quantity_min: u32,
    /// 最大数量
    pub quantity_max: u32,
}

/// 掉落表
pub struct LootTable {
    /// 表 ID（用于配置查找）
    pub id: String,
    /// 掉落条目列表
    pub entries: Vec<LootTableEntry>,
}
```

### InventorySlot

库存槽位（领域层数据结构）

```rust
// src/domain/loot/inventory.rs

/// 库存槽位
#[derive(Clone, Debug)]
pub struct InventorySlot {
    /// 物品 ID
    pub item_id: ItemId,
    /// 数量
    pub quantity: u32,
}

/// 库存（领域层数据结构）
pub struct Inventory {
    /// 固定 30 个槽位
    pub slots: [Option<InventorySlot>; 30],
}

/// 库存错误
#[derive(Debug)]
pub enum InventoryError {
    /// 库存已满
    Full,
    /// 槽位索引无效
    InvalidSlot(usize),
    /// 堆叠失败（物品不匹配或不可堆叠）
    StackingFailed,
}
```

### StackResult

堆叠计算结果

```rust
// src/domain/loot/stacking.rs

/// 堆叠结果
#[derive(Debug)]
pub enum StackResult {
    /// 完全合并
    Merge(u32), // 最终数量
    /// 部分合并，有溢出
    Split {
        /// 当前槽位剩余数量（达到 max_stack）
        remaining: u32,
        /// 溢出数量（需要新槽位）
        overflow: u32,
    },
}
```

### ItemDefinition

物品定义（领域层数据结构）

```rust
// src/domain/loot/inventory.rs

/// 物品定义
pub struct ItemDefinition {
    /// 物品 ID
    pub id: ItemId,
    /// 物品名称（用于显示）
    pub name: String,
    /// 图标路径（用于 UI）
    pub icon_path: String,
    /// 是否可堆叠
    pub stackable: bool,
    /// 最大堆叠数量（默认 99）
    pub max_stack: u32,
}
```

## Infrastructure Layer Components (Bevy ECS)

### LootTable (Component/Resource)

掉落表配置（从 RON 文件加载）

```rust
// src/infrastructure/components/loot.rs

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// 掉落表配置（从 RON 文件加载）
#[derive(Asset, TypePath, Deserialize, Serialize, Clone, Debug)]
pub struct LootTableAsset {
    /// 表 ID
    pub id: String,
    /// 掉落条目
    pub entries: Vec<LootTableEntryAsset>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct LootTableEntryAsset {
    pub item_id: u32,
    pub chance: f32,
    pub quantity_min: u32,
    pub quantity_max: u32,
}
```

### InventoryComponent

玩家库存组件

```rust
// src/infrastructure/components/loot.rs

/// 库存组件（附着于玩家实体）
#[derive(Component, Clone, Debug)]
pub struct InventoryComponent {
    /// 固定 30 个槽位
    pub slots: [Option<InventorySlot>; 30],
}

/// 库存槽位（基础设施层）
#[derive(Clone, Debug)]
pub struct InventorySlot {
    pub item_id: ItemId,
    pub quantity: u32,
}
```

### ItemDefinition (Resource/Asset)

物品定义资源（从配置加载）

```rust
// src/infrastructure/components/loot.rs

/// 物品定义资源（从 RON 文件加载）
#[derive(Asset, TypePath, Deserialize, Serialize, Clone, Debug)]
pub struct ItemDefinitionAsset {
    pub id: u32,
    pub name: String,
    pub icon_path: String,
    pub stackable: bool,
    pub max_stack: u32,
}
```

### WorldItem

场景中的物品实体组件

```rust
// src/infrastructure/components/loot.rs

/// 场景中的物品实体
#[derive(Component, Clone, Debug)]
pub struct WorldItem {
    /// 物品 ID
    pub item_id: ItemId,
    /// 数量
    pub quantity: u32,
    /// 是否已被拾取（用于清理）
    pub picked_up: bool,
}
```

## Resources

### LootConfig

全局掉落配置

```rust
// src/infrastructure/resources/loot.rs

/// 全局掉落配置
#[derive(Resource, Default)]
pub struct LootConfig {
    /// 默认掉落表（如果敌人没有指定）
    pub default_loot_table_id: Option<String>,
}
```

### PickupConfig

拾取配置

```rust
// src/infrastructure/resources/loot.rs

/// 拾取模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PickupMode {
    /// 手动拾取（按键触发）
    Manual,
    /// 自动拾取（进入范围即拾取）
    Automatic,
}

/// 拾取配置
#[derive(Resource)]
pub struct PickupConfig {
    /// 拾取模式
    pub mode: PickupMode,
    /// 拾取范围（米）
    pub range: f32, // 2.0
    /// 拾取按键（手动模式）
    pub pickup_key: KeyCode, // KeyCode::E
}
```

## Events

### ItemDropped

物品掉落事件

```rust
// src/infrastructure/events/loot.rs

/// 物品掉落事件
#[derive(Event)]
pub struct ItemDropped {
    /// 掉落物实体
    pub world_item_entity: Entity,
    /// 物品 ID
    pub item_id: ItemId,
    /// 数量
    pub quantity: u32,
    /// 掉落位置
    pub position: Vec2,
}
```

### ItemPickedUp

物品拾取事件

```rust
// src/infrastructure/events/loot.rs

/// 物品拾取事件
#[derive(Event)]
pub struct ItemPickedUp {
    /// 玩家实体
    pub player_entity: Entity,
    /// 物品 ID
    pub item_id: ItemId,
    /// 数量
    pub quantity: u32,
}
```

### InventoryFull

库存已满事件

```rust
// src/infrastructure/events/loot.rs

/// 库存已满事件
#[derive(Event)]
pub struct InventoryFull {
    /// 玩家实体
    pub player_entity: Entity,
    /// 尝试拾取的物品 ID
    pub item_id: ItemId,
}
```

### ItemStacked

物品堆叠事件

```rust
// src/infrastructure/events/loot.rs

/// 物品堆叠事件
#[derive(Event)]
pub struct ItemStacked {
    /// 玩家实体
    pub player_entity: Entity,
    /// 物品 ID
    pub item_id: ItemId,
    /// 堆叠后的总数量
    pub total_quantity: u32,
    /// 槽位索引
    pub slot_index: usize,
}
```

## Relationships

```
EnemyDefeated (Event)
    ↓
LootDropSystem
    ↓
calculate_loot_drops (Domain)
    ↓
ItemDropped (Event)
    ↓
Spawn WorldItem (Entity with WorldItem component)
    ↓
PickupRangeCheckSystem / ManualPickupSystem
    ↓
is_within_pickup_range (Domain)
    ↓
ItemPickedUp (Event)
    ↓
InventoryManagementSystem
    ↓
can_add_item, find_stackable_slot (Domain)
    ↓
InventoryComponent (Updated)
    ↓
ItemStacked (Event) / InventoryFull (Event)
```

## Validation Rules

1. **库存容量**: 固定 30 个槽位，不能超过
2. **堆叠规则**: 
   - 只有相同 `item_id` 且 `stackable = true` 的物品可以堆叠
   - 堆叠后数量不能超过 `max_stack`（默认 99）
3. **拾取范围**: 玩家必须在物品 2 米范围内才能拾取
4. **掉落概率**: 每个物品独立计算，范围 0.0 - 1.0
5. **数量范围**: `quantity_min <= quantity_max`，且都 >= 1

## State Transitions

### WorldItem 生命周期

```
Spawned (with WorldItem component)
    ↓
Player enters pickup range
    ↓
Pickup triggered (manual or automatic)
    ↓
ItemPickedUp event
    ↓
InventoryManagementSystem processes
    ↓
WorldItem.picked_up = true
    ↓
CleanupSystem removes entity
```

### InventorySlot 状态

```
Empty (None)
    ↓
Item added → Some(InventorySlot { item_id, quantity })
    ↓
Stacking → quantity increases (up to max_stack)
    ↓
Item removed → Empty (None)
```

## Data Flow

1. **掉落流程**:
   - `EnemyDefeated` event → `loot_drop_system`
   - 调用 `domain::loot::calculate_loot_drops()`
   - 生成 `Vec<LootDrop>`
   - 为每个 `LootDrop` 创建 `WorldItem` 实体
   - 发送 `ItemDropped` 事件

2. **拾取流程**:
   - `pickup_range_check_system` 或 `manual_pickup_system` 检测拾取条件
   - 调用 `domain::loot::is_within_pickup_range()`
   - 发送 `ItemPickedUp` 事件
   - `inventory_management_system` 处理事件
   - 调用 `domain::loot::can_add_item()`, `find_stackable_slot()`
   - 更新 `InventoryComponent`
   - 发送 `ItemStacked` 或 `InventoryFull` 事件

3. **UI 更新流程**:
   - `inventory_ui_system` 查询 `InventoryComponent`
   - 转换为 UI 数据格式
   - 更新 UI 资源（由 UI 系统消费）

