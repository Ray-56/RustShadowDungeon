# Contracts: Loot and Inventory System

**Feature**: 005-loot-inventory
**Date**: 2025-01-27

## Overview

本功能不涉及外部 API 或 REST/GraphQL 接口。所有交互通过 Bevy ECS 内部机制（组件、系统、事件、资源）实现。

## Internal Contracts

### Event Contracts

系统通过以下事件与其他系统通信：

- **消费事件**:
  - `EnemyDefeated` (from combat/enemy systems) - 触发掉落逻辑

- **发布事件**:
  - `ItemDropped` - 物品掉落时发布
  - `ItemPickedUp` - 物品被拾取时发布
  - `InventoryFull` - 库存已满时发布
  - `ItemStacked` - 物品堆叠时发布

### Component Contracts

其他系统可以查询以下组件：

- `InventoryComponent` - 查询玩家库存状态
- `WorldItem` - 查询场景中的物品

### Resource Contracts

其他系统可以访问以下资源：

- `LootConfig` - 全局掉落配置
- `PickupConfig` - 拾取配置（模式、范围、按键）

### Domain Layer API

领域层提供纯函数接口（零 Bevy 依赖）：

```rust
// src/domain/loot/drop_table.rs
pub fn calculate_loot_drops(
    loot_table: &LootTable,
    rng: &mut impl Rng,
) -> Vec<LootDrop>;

// src/domain/loot/inventory.rs
pub fn can_add_item(
    inventory: &Inventory,
    item_id: ItemId,
    quantity: u32,
) -> bool;

pub fn find_empty_slot(inventory: &Inventory) -> Option<usize>;

pub fn find_stackable_slot(
    inventory: &Inventory,
    item_id: ItemId,
) -> Option<usize>;

// src/domain/loot/stacking.rs
pub fn can_stack_items(
    item1: &ItemDefinition,
    item2: &ItemDefinition,
) -> bool;

pub fn calculate_stack_result(
    current_quantity: u32,
    add_quantity: u32,
    max_stack: u32,
) -> StackResult;

// src/domain/loot/pickup.rs
pub fn is_within_pickup_range(
    item_pos: Vec2,
    player_pos: Vec2,
    range: f32,
) -> bool;
```

## Integration Points

### With Combat System

- **输入**: `EnemyDefeated` 事件（包含 enemy_type, position）
- **输出**: `ItemDropped` 事件

### With UI System

- **输入**: UI 系统查询 `InventoryComponent` 获取库存数据
- **输出**: 无直接事件，UI 系统通过组件查询获取数据

### With Player System

- **输入**: 玩家实体位置（`Transform` 组件）
- **输出**: `ItemPickedUp` 事件（更新玩家库存）

