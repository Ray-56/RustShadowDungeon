# Implementation Plan: Loot and Inventory System

**Branch**: `005-loot-inventory` | **Date**: 2025-01-27 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/003-dungeon-system/sub-specs/005-loot-inventory/spec.md`

## Summary

实现战利品与库存系统，包括掉落表（Loot Table）配置、物品拾取、库存管理。敌人死亡后根据独立概率计算掉落物品，玩家可以手动或自动拾取物品并存储在 30 槽位的库存中。物品支持堆叠（默认上限 99），拾取范围为 2 米。系统采用 DDD 架构，领域层（纯函数）处理掉落概率计算、堆叠逻辑和库存管理规则，基础设施层（Bevy ECS）处理物品实体、拾取交互和 UI 数据接口。

## Technical Context

**Language/Version**: Rust 1.82.0 (stable)
**Primary Dependencies**: 
- Bevy 0.17.0 (ECS game engine)
- rand 0.8 (random number generation for loot drops)
- serde 1.0 (serialization for loot table configuration)
- avian2d 0.4 (physics engine, for pickup range detection)

**Storage**: RON files for loot table configuration (`assets/data/loot_tables.ron`)
**Testing**: cargo test, criterion for benchmarks
**Target Platform**: Windows/Linux/macOS desktop, potential WASM
**Project Type**: Single game project with plugin-based DDD architecture
**Performance Goals**: 60 FPS (16.67ms frame budget), loot/inventory updates <1ms per frame
**Constraints**: 
- Pixel-perfect rendering (16×16 grid)
- Domain layer must have ZERO Bevy dependencies
- Infrastructure layer bridges domain logic to ECS
- Language separation: English code, Chinese documentation
- Must integrate with existing combat system (enemy death events) and UI system

**Scale/Scope**: 
- Loot tables per enemy type (independent probability calculation)
- 30 inventory slots per player
- Item stacking (default max 99, configurable per item)
- 2-meter pickup range
- Manual and automatic pickup modes (configurable)
- World items with visual representation and pickup triggers

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

**Status**: ✅ Passed (Post-Design Review Complete)

### Core Principles Compliance

- [x] **Rust Memory Safety**: No `unsafe` code required. All loot/inventory logic uses safe Rust APIs.
- [x] **Bevy ECS Architecture**: All systems follow ECS patterns:
  - Components: `LootTable`, `InventoryComponent`, `ItemDefinition`, `WorldItem`, `InventorySlot` (pure data)
  - Systems: `loot_drop_system`, `pickup_system`, `inventory_management_system`, `stacking_system` (behavior)
  - Resources: `LootConfig` (optional, for global loot settings), `PickupConfig` (pickup mode settings)
  - Events: `ItemDropped`, `ItemPickedUp`, `InventoryFull`, `ItemStacked` (inter-system communication)
- [x] **60 FPS Performance**: Loot/inventory updates allocated <1ms per frame budget. Pickup range checks can be throttled (every N frames) to reduce cost.
- [x] **Pixel Art Consistency**: World items use existing item sprites (16×16 grid). No new visual assets required for MVP.
- [x] **Combat Mechanics Testing**: Loot drop probability calculation requires unit tests (domain layer functions). Integration tests for full loot-to-inventory workflow.
- [x] **Open Source MIT**: All dependencies (Bevy, rand, serde) are MIT or Apache-2.0 compatible.
- [x] **Modular Design**: Implemented as `LootInventoryPlugin` in `src/infrastructure/plugins/loot.rs`, independent and testable.
- [x] **Language Separation**: Code uses English identifiers (`LootTable`, `InventoryComponent`, `ItemDefinition`), documentation uses Chinese (spec.md, plan.md, doc comments).

### Performance Budget

Loot and inventory system frame time allocation:
- Loot drop calculation (on enemy death): <0.1ms (one-time event, not per-frame)
- Pickup range checks: <0.3ms (throttled to every 3-5 frames)
- Inventory stacking logic: <0.2ms
- Inventory slot management: <0.2ms
- UI data updates: <0.2ms
- **Total per frame**: <1.0ms (within budget)

### Testing Requirements

- [x] Unit tests for domain layer (`src/domain/loot/`):
  - `calculate_loot_drops` function (independent probability calculation)
  - `can_stack_items` function (stacking validation)
  - `find_empty_slot` function (inventory slot management)
  - `calculate_pickup_range` function (2-meter range check)
- [x] Integration tests for full workflows:
  - Enemy death → Loot drop → Pickup → Inventory storage
  - Stacking multiple items of same type
  - Inventory full scenario
  - Manual vs automatic pickup modes
- [x] Performance benchmarks for loot calculations (`benches/loot_bench.rs`)
- [x] Tests written FIRST and verified to fail before implementation (TDD approach)

## Project Structure

### Documentation (this feature)

```text
specs/003-dungeon-system/sub-specs/005-loot-inventory/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (N/A - no external API)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

**DDD 架构分层**（严格遵守领域驱动设计原则）：

```text
src/
├── main.rs              # 游戏入口点，注册 LootInventoryPlugin
├── lib.rs               # 库根，导出公共 API（用于测试）
│
├── domain/              # 领域层（Domain Layer） - 零 Bevy 依赖
│   ├── mod.rs
│   └── loot/            # 战利品领域逻辑
│       ├── mod.rs
│       ├── drop_table.rs    # 掉落表逻辑（纯函数）
│       │   # Functions:
│       │   # - calculate_loot_drops(loot_table, rng) -> Vec<LootDrop>
│       │   # - roll_item_drop(item_chance) -> bool (independent probability)
│       │   # - calculate_item_quantity(min, max, rng) -> u32
│       ├── inventory.rs     # 库存管理逻辑（纯函数）
│       │   # Functions:
│       │   # - can_add_item(inventory, item, quantity) -> bool
│       │   # - find_empty_slot(inventory) -> Option<usize>
│       │   # - find_stackable_slot(inventory, item_id) -> Option<usize>
│       │   # - add_item_to_slot(slot, item, quantity) -> Result<(), InventoryError>
│       ├── stacking.rs      # 堆叠逻辑（纯函数）
│       │   # Functions:
│       │   # - can_stack_items(item1, item2) -> bool
│       │   # - calculate_stack_result(current_qty, add_qty, max_stack) -> StackResult
│       │   # - split_stack(current_qty, max_stack) -> (u32, u32)
│       └── pickup.rs        # 拾取逻辑（纯函数）
│           # Functions:
│           # - is_within_pickup_range(item_pos, player_pos, range) -> bool
│           # - calculate_pickup_range() -> f32 (returns 2.0)
│
├── infrastructure/      # 基础设施层（Infrastructure Layer） - Bevy 桥接
│   ├── mod.rs
│   ├── plugins/         # Bevy 插件（系统注册、资源初始化）
│   │   ├── mod.rs
│   │   └── loot.rs      # LootInventoryPlugin（注册所有战利品/库存系统）
│   ├── components/      # ECS 组件（纯数据结构）
│   │   ├── mod.rs
│   │   └── loot.rs      # LootTable, InventoryComponent, ItemDefinition, WorldItem, InventorySlot
│   ├── systems/         # ECS 系统（行为函数，调用领域层）
│   │   ├── mod.rs
│   │   └── loot.rs      # 所有战利品/库存相关系统
│   │       # Systems:
│   │       # - loot_drop_system (handle enemy death, calculate drops, spawn world items)
│   │       # - pickup_range_check_system (check if player is within pickup range)
│   │       # - manual_pickup_system (handle manual pickup input)
│   │       # - automatic_pickup_system (handle automatic pickup)
│   │       # - inventory_management_system (add items to inventory, handle stacking)
│   │       # - inventory_ui_system (update UI data for inventory display)
│   ├── resources/       # 全局游戏状态
│   │   ├── mod.rs
│   │   └── loot.rs      # LootConfig, PickupConfig
│   └── events/          # 事件定义
│       ├── mod.rs
│       └── loot.rs      # ItemDropped, ItemPickedUp, InventoryFull, ItemStacked
│
tests/
├── integration/         # 完整系统测试
│   └── loot/
│       ├── loot_drop_workflow_test.rs      # 完整掉落→拾取→库存流程
│       └── inventory_management_test.rs    # 库存管理测试
└── unit/                # 组件/系统单元测试
    └── loot/
        ├── drop_table_test.rs              # 掉落表逻辑（领域层）
        ├── inventory_test.rs               # 库存逻辑（领域层）
        └── stacking_test.rs                # 堆叠逻辑（领域层）

assets/
├── data/
│   └── loot_tables.ron  # 掉落表配置（敌人类型 → 掉落池）
│
benches/                 # 性能基准测试
└── loot_bench.rs       # 掉落计算性能测试
```

**Structure Decision**: 采用现有的 DDD 架构模式，与 `001-player-movement`、`002-combat-core`、`003-dungeon-system` 和 `004-enemy-ai` 保持一致。领域层 (`src/domain/loot/`) 包含纯函数，零 Bevy 依赖，便于测试和复用。基础设施层 (`src/infrastructure/`) 作为 Bevy ECS 桥接，处理组件、系统、资源和事件。注意：需要监听 `EnemyDefeated` 事件（来自战斗系统或敌人 AI 系统）来触发掉落逻辑。

## Complexity Tracking

> **No violations - all principles upheld**

No complexity violations. The implementation follows established DDD patterns and Bevy ECS best practices. The loot/inventory system integrates with existing combat and enemy systems via events, maintaining loose coupling.

