# Quickstart: Loot and Inventory System

**Feature**: 005-loot-inventory
**Date**: 2025-01-27

## 概述

本快速入门指南帮助开发者快速理解和使用战利品与库存系统。

## 架构概览

系统采用 DDD 架构，分为两层：

1. **领域层** (`src/domain/loot/`): 纯 Rust 逻辑，零 Bevy 依赖
2. **基础设施层** (`src/infrastructure/`): Bevy ECS 桥接

## 核心概念

### 1. 掉落表 (Loot Table)

掉落表定义敌人死亡时的掉落规则。每个条目包含：
- 物品 ID
- 掉落概率 (0.0 - 1.0)
- 数量范围 (min, max)

**配置示例** (`assets/data/loot_tables.ron`):

```ron
LootTableAsset(
    id: "goblin",
    entries: [
        (
            item_id: 1,        // 金币
            chance: 0.8,       // 80% 概率
            quantity_min: 5,
            quantity_max: 15,
        ),
        (
            item_id: 2,        // 治疗药水
            chance: 0.3,        // 30% 概率（独立计算）
            quantity_min: 1,
            quantity_max: 2,
        ),
    ],
)
```

### 2. 库存 (Inventory)

玩家库存包含 30 个固定槽位。每个槽位可以存储：
- 一个物品 ID
- 数量（可堆叠物品）

**堆叠规则**:
- 相同物品 ID 且 `stackable = true` 可以堆叠
- 默认最大堆叠数量: 99
- 可在 `ItemDefinition` 中配置每个物品的最大堆叠数

### 3. 拾取 (Pickup)

物品拾取支持两种模式：
- **手动模式**: 玩家按下拾取键（默认 `E`）
- **自动模式**: 玩家进入 2 米拾取范围自动拾取

## 使用示例

### 1. 注册插件

```rust
// src/main.rs
use rust_shadow_dungeon::infrastructure::plugins::loot::LootInventoryPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LootInventoryPlugin) // 注册战利品/库存插件
        .run();
}
```

### 2. 配置掉落表

```rust
// 在游戏初始化时加载掉落表
fn setup_loot_tables(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut loot_tables: ResMut<Assets<LootTableAsset>>,
) {
    // 从 RON 文件加载
    let handle: Handle<LootTableAsset> = asset_server.load("data/loot_tables.ron");
    // 系统会自动处理加载完成后的初始化
}
```

### 3. 监听掉落事件

```rust
// 其他系统可以监听物品掉落事件
fn on_item_dropped(
    mut events: EventReader<ItemDropped>,
) {
    for event in events.read() {
        println!("Item dropped: {:?} x{} at {:?}", 
            event.item_id, 
            event.quantity, 
            event.position
        );
    }
}
```

### 4. 查询玩家库存

```rust
// UI 系统或其他系统可以查询库存
fn display_inventory(
    inventory_query: Query<&InventoryComponent, With<Player>>,
) {
    if let Ok(inventory) = inventory_query.get_single() {
        for (slot_idx, slot) in inventory.slots.iter().enumerate() {
            if let Some(slot) = slot {
                println!("Slot {}: Item {} x{}", 
                    slot_idx, 
                    slot.item_id, 
                    slot.quantity
                );
            }
        }
    }
}
```

### 5. 使用领域层函数（测试）

```rust
// tests/unit/loot/drop_table_test.rs
use rust_shadow_dungeon::domain::loot::drop_table::*;
use rand::thread_rng;

#[test]
fn test_calculate_loot_drops() {
    let loot_table = LootTable {
        id: "test".to_string(),
        entries: vec![
            LootTableEntry {
                item_id: 1,
                chance: 0.5, // 50% 概率
                quantity_min: 1,
                quantity_max: 3,
            },
        ],
    };
    
    let mut rng = thread_rng();
    let drops = calculate_loot_drops(&loot_table, &mut rng);
    
    // 验证掉落结果
    assert!(drops.len() <= 1); // 最多掉落 1 个物品（只有一个条目）
}
```

## 系统执行顺序

1. **掉落阶段** (敌人死亡时):
   - `EnemyDefeated` 事件触发
   - `loot_drop_system` 处理事件
   - 调用 `domain::loot::calculate_loot_drops()` 计算掉落
   - 创建 `WorldItem` 实体
   - 发送 `ItemDropped` 事件

2. **拾取检测阶段** (每帧):
   - `pickup_range_check_system` 检查玩家是否在拾取范围内
   - 或 `manual_pickup_system` 检测拾取按键

3. **拾取处理阶段** (拾取触发时):
   - 发送 `ItemPickedUp` 事件
   - `inventory_management_system` 处理事件
   - 调用领域层函数验证和添加物品
   - 更新 `InventoryComponent`
   - 发送 `ItemStacked` 或 `InventoryFull` 事件

4. **UI 更新阶段** (每帧):
   - `inventory_ui_system` 查询 `InventoryComponent`
   - 更新 UI 数据

## 性能注意事项

- **拾取范围检测**: 每 3-5 帧检查一次（throttled），减少计算成本
- **掉落计算**: 仅在敌人死亡时执行（事件驱动），不影响每帧性能
- **库存更新**: 仅在拾取时执行，不每帧更新

## 测试

### 运行单元测试

```bash
# 测试领域层逻辑
cargo test --lib domain::loot

# 测试特定模块
cargo test drop_table_test
cargo test inventory_test
cargo test stacking_test
```

### 运行集成测试

```bash
# 测试完整工作流程
cargo test --test loot_drop_workflow_test
cargo test --test inventory_management_test
```

### 性能基准测试

```bash
cargo bench --bench loot_bench
```

## 常见问题

### Q: 如何修改拾取范围？

A: 修改 `PickupConfig` 资源中的 `range` 字段（默认 2.0 米）。

### Q: 如何切换拾取模式？

A: 修改 `PickupConfig` 资源中的 `mode` 字段（`PickupMode::Manual` 或 `PickupMode::Automatic`）。

### Q: 如何添加新的物品类型？

A: 在 `assets/data/items.ron` 中添加 `ItemDefinitionAsset` 条目。

### Q: 如何自定义堆叠上限？

A: 在 `ItemDefinitionAsset` 中设置 `max_stack` 字段（默认 99）。

## 下一步

- 查看 [data-model.md](./data-model.md) 了解详细数据模型
- 查看 [research.md](./research.md) 了解技术决策
- 查看 [plan.md](./plan.md) 了解完整实施计划
- 运行 `/speckit.tasks` 生成任务列表

