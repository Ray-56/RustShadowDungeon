---
description: "Implementation task list for Dungeon System"
---

# Tasks: 地下城系统 (Dungeon System)

**Input**: Design documents from `/specs/003-dungeon-system/`
**Prerequisites**: spec.md ✓
**Constitution**: v1.0.1 (Language Separation Rule enforced)
**Branch**: `003-dungeon-system`

---

**任务描述语言规范（Task Description Language Guidelines）**:
- 任务描述使用中文（Task descriptions in Chinese）
- 文件名、类型名、函数名使用英文（File names, type names, function names in English）
- 示例：`创建 Room 组件在 src/infrastructure/components/dungeon.rs` ✅
- 示例：`创建房间组件在 src/infrastructure/components/房间.rs` ❌

**Tests**: TDD approach required - ≥85% coverage on core systems (per Constitution III)

**Organization**: Tasks grouped by user story for independent implementation and testing.

---

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (US1, US2, US3, US4)
- Include exact file paths in descriptions

---

## Phase 1: Setup (项目初始化)

**Purpose**: 项目初始化和基础结构创建

- [X] T001 创建 src/domain/dungeon/ 目录结构
- [X] T002 [P] 创建 src/infrastructure/components/dungeon.rs 文件
- [X] T003 [P] 创建 src/infrastructure/systems/dungeon.rs 文件
- [X] T004 [P] 创建 src/infrastructure/plugins/dungeon.rs 文件
- [X] T005 [P] 创建 src/infrastructure/events/dungeon.rs 文件
- [X] T006 [P] 创建 assets/data/dungeons/ 目录结构用于存储地下城配置
- [X] T007 [P] 创建 tests/unit/dungeon/ 目录用于单元测试
- [X] T008 [P] 创建 tests/integration/dungeon/ 目录用于集成测试

---

## Phase 2: Foundational (基础设施层 - 阻塞前置条件)

**Purpose**: 核心基础设施，必须在任何用户故事实现前完成

**⚠️ CRITICAL**: 所有用户故事工作必须等待此阶段完成

- [X] T009 创建 src/domain/dungeon/mod.rs 导出领域模块
- [X] T010 [P] 创建 src/domain/dungeon/progression.rs 定义房间清理逻辑（纯函数，无 Bevy 依赖）
- [X] T011 [P] 创建 src/infrastructure/components/dungeon.rs 定义 Room, Door, DungeonManager 组件
- [X] T012 [P] 创建 src/infrastructure/events/dungeon.rs 定义 RoomCleared, RoomEntered, DoorUnlocked 事件
- [X] T013 [P] 创建 src/infrastructure/resources/dungeon.rs 定义 DungeonSession 资源（记录已清理房间）
- [X] T014 创建 src/infrastructure/plugins/dungeon.rs 注册 DungeonPlugin 和所有系统
- [X] T015 [P] 在 src/infrastructure/events/mod.rs 中导出 dungeon 事件模块
- [X] T016 [P] 在 src/infrastructure/components/mod.rs 中导出 dungeon 组件模块
- [X] T017 [P] 在 src/infrastructure/systems/mod.rs 中导出 dungeon 系统模块
- [X] T018 [P] 在 src/infrastructure/resources/mod.rs 中导出 dungeon 资源模块
- [X] T019 在 src/main.rs 中注册 DungeonPlugin

**Checkpoint**: ✅ 基础设施就绪 - 用户故事实现现在可以并行开始

---

## Phase 3: User Story 1 - 进入地下城与房间初始化 (Priority: P1) 🎯 MVP

**Goal**: 玩家开始地下城探险，系统加载第一个房间并生成敌人。

**Independent Test**: 可以独立测试场景加载和实体生成逻辑，确保玩家进入后能看到正确的环境和敌人。

### Domain Layer Implementation for User Story 1

- [X] T020 [P] [US1] 创建 RoomState 枚举在 src/domain/dungeon/progression.rs（Active, Cleared, Uncleared）
- [X] T021 [P] [US1] 创建 should_spawn_enemies 函数在 src/domain/dungeon/progression.rs（根据房间状态决定是否生成敌人）

### Infrastructure Layer Implementation for User Story 1

- [X] T022 [US1] 创建 Room 组件在 src/infrastructure/components/dungeon.rs（room_id, state, spawn_points）
- [X] T023 [P] [US1] 创建 Door 组件在 src/infrastructure/components/dungeon.rs（door_id, connected_room_id, door_state, entrance_position）
- [X] T024 [P] [US1] 创建 DungeonManager 组件在 src/infrastructure/components/dungeon.rs（current_room_id, room_graph）
- [X] T025 [P] [US1] 创建 EnemySpawnPoint 组件在 src/infrastructure/components/dungeon.rs（position, enemy_type, spawn_on_activate）
- [X] T026 [US1] 创建 load_dungeon_system 在 src/infrastructure/systems/dungeon.rs（加载地下城配置，初始化第一个房间）
- [X] T027 [P] [US1] 创建 initialize_room_system 在 src/infrastructure/systems/dungeon.rs（初始化房间，设置门状态为锁定）
- [X] T028 [P] [US1] 创建 spawn_enemies_system 在 src/infrastructure/systems/dungeon.rs（根据房间状态和配置生成敌人）
- [X] T029 [US1] 创建 RoomEntered 事件在 src/infrastructure/events/dungeon.rs（room_id, player_position）
- [X] T030 [P] [US1] 创建 handle_room_entered_system 在 src/infrastructure/systems/dungeon.rs（监听 RoomEntered 事件，触发敌人生成）

### Configuration for User Story 1

- [X] T031 [P] [US1] 创建 assets/data/dungeons/test_dungeon.ron 定义测试地下城配置（至少 3 个房间）
- [X] T032 [P] [US1] 创建 DungeonConfig 结构体在 src/infrastructure/resources/dungeon.rs（用于加载 RON 配置）

### Integration for User Story 1

- [X] T033 [US1] 在 DungeonPlugin 中注册所有 US1 相关系统到正确的调度阶段

**Checkpoint**: ✅ US1 完成 - 玩家可以进入地下城，第一个房间加载并生成敌人

---

## Phase 4: User Story 2 - 清理房间与解锁 (Priority: P1)

**Goal**: 玩家击败房间内所有敌人后，房间状态更新为已清理，通往下一个房间的门解锁。

**Independent Test**: 可以通过作弊指令或实际战斗杀死敌人，验证门的状态变化逻辑。

### Domain Layer Implementation for User Story 2

- [X] T034 [P] [US2] 创建 check_room_cleared 函数在 src/domain/dungeon/progression.rs（检查房间是否应该被标记为已清理）
- [X] T035 [P] [US2] 创建 should_unlock_doors 函数在 src/domain/dungeon/progression.rs（根据房间状态决定是否解锁门）

### Infrastructure Layer Implementation for User Story 2

- [X] T036 [US2] 创建 monitor_enemy_deaths_system 在 src/infrastructure/systems/dungeon.rs（监听 EnemyDefeated 事件，检查房间内敌人数量）
- [X] T037 [P] [US2] 创建 check_room_clear_system 在 src/infrastructure/systems/dungeon.rs（当敌人从存活列表移除时，检查房间是否清理完成）
- [X] T038 [US2] 创建 RoomCleared 事件在 src/infrastructure/events/dungeon.rs（room_id, cleared_at）
- [X] T039 [P] [US2] 创建 handle_room_cleared_system 在 src/infrastructure/systems/dungeon.rs（监听 RoomCleared 事件，更新房间状态，解锁所有门）
- [X] T040 [P] [US2] 创建 unlock_doors_system 在 src/infrastructure/systems/dungeon.rs（更新 Door 组件状态为 Unlocked，启用交互）
- [X] T041 [P] [US2] 创建 DoorUnlocked 事件在 src/infrastructure/events/dungeon.rs（door_id, room_id）
- [X] T042 [US2] 创建 update_door_visual_system 在 src/infrastructure/systems/dungeon.rs（根据门状态更新视觉表现和交互提示）

### Integration for User Story 2

- [X] T043 [US2] 确保 check_room_clear_system 在敌人死亡动画完成后触发（与 CombatPlugin 的死亡系统协调）
- [X] T044 [US2] 在 DungeonPlugin 中注册所有 US2 相关系统，确保执行顺序正确

**Checkpoint**: ✅ US2 完成 - 房间清理后门自动解锁

---

## Phase 5: User Story 3 - 房间过渡 (Priority: P1)

**Goal**: 玩家通过解锁的门进入下一个房间，系统处理场景切换和新房间加载。

**Independent Test**: 在已解锁的门处触发交互，验证玩家坐标变更和新环境加载。

### Domain Layer Implementation for User Story 3

- [X] T045 [P] [US3] 创建 get_target_room_entrance 函数在 src/domain/dungeon/progression.rs（根据门连接关系计算目标房间入口位置）
- [X] T046 [P] [US3] 创建 can_transition_to_room 函数在 src/domain/dungeon/progression.rs（检查是否可以进入目标房间）

### Infrastructure Layer Implementation for User Story 3

- [X] T047 [US3] 创建 door_interaction_detection_system 在 src/infrastructure/systems/dungeon.rs（检测玩家靠近解锁的门，显示交互提示）
- [X] T048 [P] [US3] 创建 handle_door_interaction_system 在 src/infrastructure/systems/dungeon.rs（处理玩家按交互键，触发房间过渡）
- [X] T049 [US3] 创建 transition_to_room_system 在 src/infrastructure/systems/dungeon.rs（传送玩家到目标房间对应门的入口点）
- [X] T050 [P] [US3] 创建 unload_current_room_system 在 src/infrastructure/systems/dungeon.rs（卸载当前房间的敌人和临时实体）
- [X] T051 [P] [US3] 创建 load_target_room_system 在 src/infrastructure/systems/dungeon.rs（加载目标房间，根据房间状态决定是否生成敌人）
- [X] T052 [P] [US3] 创建 update_camera_on_room_transition_system 在 src/infrastructure/systems/dungeon.rs（更新摄像机到新房间上下文）
- [X] T053 [US3] 创建 RoomTransitioned 事件在 src/infrastructure/events/dungeon.rs（from_room_id, to_room_id, player_position）
- [X] T054 [P] [US3] 创建 show_interaction_prompt_system 在 src/infrastructure/systems/dungeon.rs（在玩家靠近解锁的门时显示交互提示 UI）

### UI Integration for User Story 3

- [X] T055 [P] [US3] 创建交互提示 UI 组件在 src/infrastructure/components/ui.rs（InteractPrompt marker）
- [X] T056 [P] [US3] 创建交互提示 UI 系统在 src/infrastructure/systems/ui.rs（显示/隐藏"按 E 交互"提示）

### Integration for User Story 3

- [X] T057 [US3] 确保房间过渡系统与 PlayerPlugin 的移动系统协调（避免玩家在传送时卡住）
- [X] T058 [US3] 在 DungeonPlugin 中注册所有 US3 相关系统，确保执行顺序正确

**Checkpoint**: ✅ US3 完成 - 玩家可以通过门在不同房间间传送

---

## Phase 6: User Story 4 - 进度追踪 (Priority: P2)

**Goal**: 系统记录地下城的清理进度，确保已清理的房间不会重复刷新敌人。

**Independent Test**: 清理房间 -> 离开 -> 返回，验证房间状态保持。

### Domain Layer Implementation for User Story 4

- [X] T059 [P] [US4] 创建 mark_room_cleared 函数在 src/domain/dungeon/progression.rs（标记房间为已清理）
- [X] T060 [P] [US4] 创建 is_room_cleared 函数在 src/domain/dungeon/progression.rs（检查房间是否已清理）

### Infrastructure Layer Implementation for User Story 4

- [X] T061 [US4] 创建 DungeonSession 资源在 src/infrastructure/resources/dungeon.rs（cleared_rooms: HashSet<RoomId>）
- [X] T062 [P] [US4] 创建 track_room_clear_system 在 src/infrastructure/systems/dungeon.rs（当房间清理时，记录到 DungeonSession）
- [X] T063 [P] [US4] 创建 persist_room_state_system 在 src/infrastructure/systems/dungeon.rs（当玩家离开房间时，保存房间状态）
- [X] T064 [US4] 创建 restore_room_state_system 在 src/infrastructure/systems/dungeon.rs（当玩家返回房间时，根据 DungeonSession 恢复房间状态）
- [X] T065 [P] [US4] 更新 spawn_enemies_system 在 src/infrastructure/systems/dungeon.rs（检查 DungeonSession，已清理的房间不生成敌人）
- [X] T066 [P] [US4] 更新 initialize_room_system 在 src/infrastructure/systems/dungeon.rs（根据 DungeonSession 恢复房间状态和门状态）

### Integration for User Story 4

- [X] T067 [US4] 确保 DungeonSession 在游戏开始时初始化，在玩家离开地下城时重置
- [X] T068 [US4] 在 DungeonPlugin 中注册所有 US4 相关系统

**Checkpoint**: ✅ US4 完成 - 房间状态持久化，已清理房间不会重新生成敌人

---

## Phase 7: Edge Cases & Polish (边界情况与打磨)

**Purpose**: 处理边界情况和跨功能优化

### Player Death Handling

- [X] T069 创建 handle_player_death_in_dungeon_system 在 src/infrastructure/systems/dungeon.rs（玩家死亡时，在房间入口重生，保持房间状态不变）
- [X] T070 [P] 创建 PlayerDeathInRoom 事件在 src/infrastructure/events/dungeon.rs（room_id, death_position）
- [X] T071 [P] 更新 DungeonSession 确保玩家死亡不影响已清理房间记录

### Empty Room Handling

- [X] T072 创建 handle_empty_room_system 在 src/infrastructure/systems/dungeon.rs（无敌人的房间进入时直接标记为 Cleared 并解锁门）
- [X] T073 [P] 更新 check_room_cleared_system 处理空房间情况

### State Consistency

- [X] T074 创建 ensure_state_consistency_system 在 src/infrastructure/systems/dungeon.rs（确保敌人死亡和门解锁的状态一致性，处理并发情况）
- [X] T075 [P] 添加状态锁机制防止并发修改房间状态（通过系统执行顺序和状态一致性检查实现）

### Performance Optimization

- [X] T076 优化房间加载性能（延迟加载、资源池复用）
- [X] T077 [P] 添加房间卸载优化（延迟销毁、批量处理）

### Testing

- [X] T078 [P] 创建单元测试 tests/unit/dungeon/progression_test.rs 测试房间清理逻辑
- [X] T079 [P] 创建集成测试 tests/integration/dungeon/room_transition_test.rs 测试完整房间过渡流程
- [X] T080 [P] 创建集成测试 tests/integration/dungeon/progress_tracking_test.rs 测试进度追踪和状态持久化
- [X] T081 [P] 创建性能基准测试 benches/dungeon_bench.rs 测试房间加载和过渡性能（可选，性能优化阶段）

### Documentation

- [X] T082 添加代码文档注释（所有公共函数和结构体）
- [X] T083 [P] 更新 README.md 说明地下城系统使用方法（可选，文档完善阶段）
- [X] T084 [P] 创建开发者文档 docs/dungeon-system.md 说明系统架构和扩展方法（可选，文档完善阶段）

---

## Dependencies & Execution Order

### User Story Dependencies

```
US1 (房间初始化)
  ↓
US2 (清理与解锁) - 依赖 US1（需要房间和敌人系统）
  ↓
US3 (房间过渡) - 依赖 US1, US2（需要门解锁）
  ↓
US4 (进度追踪) - 依赖 US1, US2, US3（需要完整的房间生命周期）
```

### Parallel Execution Opportunities

**Within US1**:
- T020-T021 (Domain layer) can run in parallel
- T022-T025 (Components) can run in parallel
- T027-T028 (Systems) can run in parallel

**Within US2**:
- T034-T035 (Domain layer) can run in parallel
- T037, T039-T041 (Systems) can run in parallel after T036 completes

**Within US3**:
- T045-T046 (Domain layer) can run in parallel
- T050-T051 (Load/unload systems) can run in parallel
- T052, T054 (UI systems) can run in parallel

**Within US4**:
- T059-T060 (Domain layer) can run in parallel
- T062-T066 (Systems) can run in parallel after T061 completes

**Cross-Phase**:
- Phase 1 setup tasks (T001-T008) can mostly run in parallel
- Phase 2 foundational tasks (T009-T019) can mostly run in parallel after Phase 1

---

## Implementation Strategy

### MVP Scope (Minimum Viable Product)

**MVP includes**: US1 only (进入地下城与房间初始化)

This provides the foundation for all other features:
- ✅ Players can enter dungeon
- ✅ First room loads with enemies
- ✅ Doors are locked initially

**MVP excludes**:
- Room clearing logic (US2)
- Room transitions (US3)
- Progress tracking (US4)

### Incremental Delivery Plan

1. **Week 1**: Phase 1-2 (Setup + Foundational) + US1 MVP
2. **Week 2**: US2 (Room clearing) - enables core gameplay loop
3. **Week 3**: US3 (Room transitions) - enables multi-room exploration
4. **Week 4**: US4 (Progress tracking) + Polish - completes feature

### Testing Strategy

- **Unit Tests**: Test domain logic in isolation (progression.rs functions)
- **Integration Tests**: Test full workflows (room loading → enemy spawn → clear → transition)
- **Performance Tests**: Ensure room loading < 16ms (60 FPS budget)

---

## Summary

- **Total Tasks**: 84
- **Tasks by Phase**:
  - Phase 1 (Setup): 8 tasks
  - Phase 2 (Foundational): 11 tasks
  - Phase 3 (US1): 14 tasks
  - Phase 4 (US2): 11 tasks
  - Phase 5 (US3): 15 tasks
  - Phase 6 (US4): 10 tasks
  - Phase 7 (Polish): 15 tasks

- **Tasks by User Story**:
  - US1: 14 tasks (Domain: 2, Infrastructure: 9, Config: 2, Integration: 1)
  - US2: 11 tasks (Domain: 2, Infrastructure: 7, Integration: 2)
  - US3: 15 tasks (Domain: 2, Infrastructure: 9, UI: 2, Integration: 2)
  - US4: 10 tasks (Domain: 2, Infrastructure: 6, Integration: 2)

- **MVP Scope**: US1 only (14 tasks in Phase 3)
- **Parallel Opportunities**: ~40% of tasks can run in parallel
- **Independent Test Criteria**: Each user story has clear acceptance scenarios

---

**Next Steps**: Start with Phase 1-2, then proceed to US1 MVP implementation.
