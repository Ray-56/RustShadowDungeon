# Tasks: Loot and Inventory System

**Input**: Design documents from `/specs/003-dungeon-system/sub-specs/005-loot-inventory/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/
**Constitution**: v1.0.1 (Language Separation Rule enforced)

---

**任务描述语言规范（Task Description Language Guidelines）**:
- 任务描述使用中文（Task descriptions in Chinese for Chinese projects）
- 文件名、类型名、函数名使用英文（File names, type names, function names in English）
- 示例：`创建 InventoryComponent 组件在 src/infrastructure/components/loot.rs` ✅
- 示例：`创建库存组件在 src/infrastructure/components/库存.rs` ❌

**Tests**: 根据规范要求，本功能需要测试（TDD 方法）。所有测试任务标记为 [P] 表示可以并行编写。

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Rust + Bevy Game**: `src/domain/`, `src/infrastructure/plugins/`, `src/infrastructure/components/`, `src/infrastructure/systems/`, `tests/`
- **Domain Layer**: Pure functions in `src/domain/loot/`
- **Infrastructure Layer**: Bevy ECS bridge in `src/infrastructure/`
- **Components**: Pure data structures in `src/infrastructure/components/`
- **Systems**: Behavior functions in `src/infrastructure/systems/`
- **Plugins**: Feature modules in `src/infrastructure/plugins/`
- **Tests**: Unit tests in `tests/unit/loot/`, integration in `tests/integration/loot/`
- **Benchmarks**: Performance tests in `benches/`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [x] T001 创建领域层模块结构 `src/domain/loot/mod.rs`
- [x] T002 [P] 创建基础设施层模块结构 `src/infrastructure/components/loot.rs` 和 `src/infrastructure/systems/loot.rs`
- [x] T003 [P] 创建事件定义模块 `src/infrastructure/events/loot.rs`
- [x] T004 [P] 创建资源定义模块 `src/infrastructure/resources/loot.rs`
- [x] T005 检查并更新 Cargo.toml 添加依赖 (rand 0.8, serde 1.0)
- [x] T006 [P] 创建测试目录结构 `tests/unit/loot/` 和 `tests/integration/loot/`
- [x] T007 [P] 创建性能基准测试目录 `benches/loot_bench.rs`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

### Domain Layer Foundation

- [x] T010 创建领域层类型定义 `src/domain/loot/drop_table.rs` (ItemId, LootDrop, LootTableEntry, LootTable)
- [x] T011 [P] 创建领域层库存类型定义 `src/domain/loot/inventory.rs` (InventorySlot, Inventory, InventoryError)
- [x] T012 [P] 创建领域层堆叠类型定义 `src/domain/loot/stacking.rs` (StackResult)
- [x] T013 [P] 创建领域层拾取类型定义 `src/domain/loot/pickup.rs` (pickup range constants)
- [x] T014 在 `src/domain/loot/mod.rs` 中导出所有模块

### Infrastructure Layer Foundation

- [x] T015 创建基础设施层组件定义 `src/infrastructure/components/loot.rs` (LootTableAsset, InventoryComponent, ItemDefinitionAsset, WorldItem, InventorySlot)
- [x] T016 [P] 创建基础设施层资源定义 `src/infrastructure/resources/loot.rs` (LootConfig, PickupConfig, PickupMode)
- [x] T017 [P] 创建基础设施层事件定义 `src/infrastructure/events/loot.rs` (ItemDropped, ItemPickedUp, InventoryFull, ItemStacked)
- [x] T018 在 `src/infrastructure/components/mod.rs` 中导出 loot 模块
- [x] T019 [P] 在 `src/infrastructure/resources/mod.rs` 中导出 loot 模块
- [x] T020 [P] 在 `src/infrastructure/events/mod.rs` 中导出 loot 模块

### Asset Configuration

- [x] T021 创建掉落表配置示例 `assets/data/loot_tables.ron`
- [x] T022 [P] 创建物品定义配置示例 `assets/data/items.ron`

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - 战利品掉落 (Priority: P1) 🎯 MVP

**Goal**: 当敌人被击败时，系统根据预设概率生成物品并掉落在场景中。

**Independent Test**: 在测试场景中生成并杀死敌人，验证是否有物品生成。

### Tests for User Story 1 ⚠️

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [x] T030 [P] [US1] 单元测试：掉落表概率计算 `tests/unit/loot/drop_table_test.rs` (calculate_loot_drops, roll_item_drop, calculate_item_quantity)
- [x] T031 [P] [US1] 集成测试：完整掉落流程 `tests/integration/loot/loot_drop_workflow_test.rs` (EnemyDefeated → ItemDropped → WorldItem spawned)
- [x] T032 [P] [US1] 性能基准测试：掉落计算性能 `benches/loot_bench.rs` (calculate_loot_drops benchmark)

### Implementation for User Story 1

#### Domain Layer (Pure Functions)

- [x] T033 [P] [US1] 实现掉落表计算函数 `src/domain/loot/drop_table.rs` (calculate_loot_drops, roll_item_drop, calculate_item_quantity)
- [x] T034 [US1] 在 `src/domain/loot/mod.rs` 中导出 drop_table 模块

#### Infrastructure Layer (Bevy ECS Bridge)

- [x] T035 [P] [US1] 实现掉落系统 `src/infrastructure/systems/loot.rs` (loot_drop_system - 监听 EnemyDefeated 事件)
- [x] T036 [US1] 实现 WorldItem 生成逻辑（在 loot_drop_system 中创建 WorldItem 实体）
- [x] T037 [US1] 创建 LootInventoryPlugin `src/infrastructure/plugins/loot.rs` (注册 loot_drop_system)
- [x] T038 [US1] 在 `src/infrastructure/plugins/mod.rs` 中导出 loot 模块
- [x] T039 [US1] 在 `src/main.rs` 中注册 LootInventoryPlugin

#### Asset Loading

- [x] T040 [US1] 在 LootInventoryPlugin 中实现掉落表资源加载逻辑
- [x] T041 [US1] 实现掉落表查找逻辑（根据 enemy_type 查找对应的 loot_table_id）

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently. Enemies should drop items when defeated.

---

## Phase 4: User Story 2 - 物品拾取 (Priority: P1) 🎯 MVP

**Goal**: 玩家接近掉落物并与之交互，物品从场景消失并进入玩家库存。

**Independent Test**: 控制玩家接近物品并触发拾取操作。

### Tests for User Story 2 ⚠️

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [x] T050 [P] [US2] 单元测试：拾取范围检测 `tests/unit/loot/pickup_test.rs` (is_within_pickup_range)
- [x] T051 [P] [US2] 单元测试：库存管理函数 `tests/unit/loot/inventory_test.rs` (can_add_item, find_empty_slot, find_stackable_slot)
- [x] T052 [P] [US2] 单元测试：堆叠逻辑 `tests/unit/loot/stacking_test.rs` (can_stack_items, calculate_stack_result)
- [x] T053 [P] [US2] 集成测试：完整拾取流程 `tests/integration/loot/loot_drop_workflow_test.rs` (WorldItem → Pickup → InventoryComponent)
- [x] T054 [P] [US2] 集成测试：库存已满场景 `tests/integration/loot/inventory_management_test.rs` (InventoryFull event)

### Implementation for User Story 2

#### Domain Layer (Pure Functions)

- [x] T055 [P] [US2] 实现拾取范围检测函数 `src/domain/loot/pickup.rs` (is_within_pickup_range, calculate_pickup_range)
- [x] T056 [P] [US2] 实现库存管理函数 `src/domain/loot/inventory.rs` (can_add_item, find_empty_slot, find_stackable_slot, add_item_to_slot)
- [x] T057 [P] [US2] 实现堆叠逻辑函数 `src/domain/loot/stacking.rs` (can_stack_items, calculate_stack_result, split_stack)

#### Infrastructure Layer (Bevy ECS Bridge)

- [x] T058 [US2] 实现拾取范围检测系统 `src/infrastructure/systems/loot.rs` (pickup_range_check_system - 节流检查每 3-5 帧)
- [x] T059 [US2] 实现手动拾取系统 `src/infrastructure/systems/loot.rs` (manual_pickup_system - 监听按键输入)
- [x] T060 [US2] 实现自动拾取系统 `src/infrastructure/systems/loot.rs` (automatic_pickup_system - 自动拾取范围内物品)
- [x] T061 [US2] 实现库存管理系统 `src/infrastructure/systems/loot.rs` (inventory_management_system - 处理 ItemPickedUp 事件)
- [x] T062 [US2] 在 LootInventoryPlugin 中注册所有拾取相关系统
- [x] T063 [US2] 实现 PickupConfig 资源初始化（默认模式、范围、按键）
- [x] T064 [US2] 实现 InventoryComponent 初始化（为玩家实体添加组件）

#### Integration

- [x] T065 [US2] 确保 loot_drop_system 和 pickup 系统正确集成（WorldItem 可以被拾取）
- [x] T066 [US2] 实现物品拾取后 WorldItem 实体清理逻辑

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently. Players can pick up dropped items and they are stored in inventory.

---

## Phase 5: User Story 3 - 库存管理 (Priority: P2)

**Goal**: 玩家查看背包，可以看到已获得的物品列表。

**Independent Test**: 打开库存界面，检查物品显示是否正确。

### Tests for User Story 3 ⚠️

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [x] T070 [P] [US3] 集成测试：库存 UI 数据更新 `tests/integration/loot/inventory_management_test.rs` (InventoryComponent → UI data format)
- [x] T071 [P] [US3] 集成测试：堆叠物品显示 `tests/integration/loot/inventory_management_test.rs` (multiple stackable items show as single slot)

### Implementation for User Story 3

#### Infrastructure Layer (Bevy ECS Bridge)

- [x] T072 [US3] 实现库存 UI 数据系统 `src/infrastructure/systems/loot.rs` (inventory_ui_system - 查询 InventoryComponent 并转换为 UI 数据格式)
- [x] T073 [US3] 创建 UI 数据资源类型（用于 UI 系统消费，如 `InventoryUIData`）
- [x] T074 [US3] 在 LootInventoryPlugin 中注册 inventory_ui_system
- [x] T075 [US3] 实现物品定义资源加载（从 `assets/data/items.ron` 加载 ItemDefinitionAsset）

#### UI Integration

- [ ] T076 [US3] 确保 UI 系统可以查询 InventoryUIData 资源（UI 系统不在本功能范围内，但需要提供数据接口）

**Checkpoint**: At this point, all user stories should now be independently functional. Players can see their inventory contents in the UI.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

### Edge Cases & Error Handling

- [x] T080 [P] 实现并发拾取处理（多人模式下，两个玩家同时拾取同一物品，只能一人成功）
- [x] T081 [P] 实现掉落位置验证（物品掉落在不可达区域时，弹射到最近有效位置）
- [x] T082 [P] 实现库存溢出处理（拾取数量超过堆叠上限时，多余部分开启新槽位或保留在地面）

### Performance Optimization

- [x] T083 优化拾取范围检测（确保节流机制正常工作，每 3-5 帧检查一次）
- [x] T084 [P] 运行性能基准测试并验证 <1ms 帧预算
- [x] T085 [P] 优化掉落计算性能（如果基准测试显示需要）

### Documentation & Testing

- [x] T086 [P] 添加领域层函数文档注释（中文文档，英文代码）
- [x] T087 [P] 添加基础设施层组件和系统文档注释
- [x] T088 [P] 验证所有测试通过（单元测试、集成测试、性能测试）
- [ ] T089 运行 quickstart.md 验证流程

### Code Quality

- [x] T090 [P] 运行 cargo fmt 格式化代码
- [x] T091 [P] 运行 cargo clippy 检查代码质量
- [x] T092 [P] 确保所有代码符合 Constitution v1.0.1 要求

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3-5)**: All depend on Foundational phase completion
  - User Story 1 (Phase 3): Can start immediately after Foundational
  - User Story 2 (Phase 4): Depends on User Story 1 (needs WorldItem from US1)
  - User Story 3 (Phase 5): Depends on User Story 2 (needs InventoryComponent from US2)
- **Polish (Phase 6)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P1)**: Depends on User Story 1 (needs WorldItem entities and loot drop system)
- **User Story 3 (P2)**: Depends on User Story 2 (needs InventoryComponent and pickup system)

### Within Each User Story

- Tests (if included) MUST be written and FAIL before implementation (TDD)
- Domain layer functions before infrastructure layer systems
- Components before systems
- Systems before plugin registration
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- All Foundational tasks marked [P] can run in parallel (within Phase 2)
- Domain layer functions for different modules can run in parallel (T033, T055, T056, T057)
- All tests for a user story marked [P] can run in parallel
- Different components within a story marked [P] can run in parallel
- Polish phase tasks marked [P] can run in parallel

---

## Parallel Example: User Story 1

```bash
# Launch all tests for User Story 1 together:
Task: "Unit test for drop table calculation in tests/unit/loot/drop_table_test.rs"
Task: "Integration test for loot drop workflow in tests/integration/loot/loot_drop_workflow_test.rs"
Task: "Performance benchmark for loot calculation in benches/loot_bench.rs"

# Launch domain layer implementation in parallel (different files):
Task: "Implement drop table calculation functions in src/domain/loot/drop_table.rs"
Task: "Export drop_table module in src/domain/loot/mod.rs"

# Launch infrastructure layer in parallel (different systems):
Task: "Implement loot drop system in src/infrastructure/systems/loot.rs"
Task: "Create LootInventoryPlugin in src/infrastructure/plugins/loot.rs"
```

---

## Parallel Example: User Story 2

```bash
# Launch all domain layer functions in parallel (different files):
Task: "Implement pickup range detection in src/domain/loot/pickup.rs"
Task: "Implement inventory management functions in src/domain/loot/inventory.rs"
Task: "Implement stacking logic in src/domain/loot/stacking.rs"

# Launch all tests in parallel:
Task: "Unit test for pickup range in tests/unit/loot/pickup_test.rs"
Task: "Unit test for inventory management in tests/unit/loot/inventory_test.rs"
Task: "Unit test for stacking logic in tests/unit/loot/stacking_test.rs"

# Launch infrastructure systems in parallel (after domain layer complete):
Task: "Implement pickup range check system in src/infrastructure/systems/loot.rs"
Task: "Implement manual pickup system in src/infrastructure/systems/loot.rs"
Task: "Implement automatic pickup system in src/infrastructure/systems/loot.rs"
```

---

## Implementation Strategy

### MVP First (User Stories 1 & 2 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1 (战利品掉落)
4. Complete Phase 4: User Story 2 (物品拾取)
5. **STOP and VALIDATE**: Test User Stories 1 & 2 independently
6. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy/Demo (Basic loot drops!)
3. Add User Story 2 → Test independently → Deploy/Demo (Pickup works!)
4. Add User Story 3 → Test independently → Deploy/Demo (Full inventory system!)
5. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1 (loot drops)
   - Developer B: Prepare User Story 2 domain layer (while waiting for US1)
3. Once User Story 1 is done:
   - Developer A: User Story 3 (inventory UI)
   - Developer B: User Story 2 (pickup system)
4. Stories complete and integrate independently

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Verify tests fail before implementing (TDD approach)
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Domain layer functions have ZERO Bevy dependencies (pure Rust)
- Infrastructure layer bridges domain logic to Bevy ECS
- All code uses English identifiers, documentation uses Chinese
- Performance budget: <1ms per frame for loot/inventory updates
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence

---

## Task Summary

- **Total Tasks**: 92
- **Phase 1 (Setup)**: 7 tasks
- **Phase 2 (Foundational)**: 13 tasks
- **Phase 3 (User Story 1)**: 12 tasks
- **Phase 4 (User Story 2)**: 17 tasks
- **Phase 5 (User Story 3)**: 6 tasks
- **Phase 6 (Polish)**: 13 tasks

### Task Count per User Story

- **User Story 1**: 12 tasks (5 tests + 7 implementation)
- **User Story 2**: 17 tasks (5 tests + 12 implementation)
- **User Story 3**: 6 tasks (2 tests + 4 implementation)

### Parallel Opportunities Identified

- **Setup Phase**: 5 parallel tasks
- **Foundational Phase**: 8 parallel tasks
- **User Story 1**: 8 parallel tasks (tests + domain layer)
- **User Story 2**: 12 parallel tasks (tests + domain layer + some systems)
- **User Story 3**: 4 parallel tasks
- **Polish Phase**: 8 parallel tasks

### Independent Test Criteria

- **User Story 1**: 在测试场景中生成并杀死敌人，验证是否有物品生成
- **User Story 2**: 控制玩家接近物品并触发拾取操作
- **User Story 3**: 打开库存界面，检查物品显示是否正确

### Suggested MVP Scope

**MVP = User Stories 1 & 2** (P1 priorities)
- User Story 1: 战利品掉落功能
- User Story 2: 物品拾取功能
- User Story 3 (P2) can be added in next iteration

### Format Validation

✅ All tasks follow the checklist format:
- Checkbox: `- [ ]`
- Task ID: T001, T002, etc.
- [P] marker: Present where applicable
- [Story] label: Present for user story tasks (US1, US2, US3)
- File paths: Included in all task descriptions

