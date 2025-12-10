# Tasks: Enemy AI System

**Input**: Design documents from `/specs/003-dungeon-system/sub-specs/004-enemy-ai/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md
**Constitution**: v1.0.1 (Language Separation Rule enforced)

---

**任务描述语言规范（Task Description Language Guidelines）**:
- 任务描述使用中文（Task descriptions in Chinese for Chinese projects）
- 文件名、类型名、函数名使用英文（File names, type names, function names in English）
- 示例：`创建 EnemyAI 组件在 src/infrastructure/components/enemy.rs` ✅
- 示例：`创建敌人AI组件在 src/infrastructure/components/敌人.rs` ❌

**Tests**: Tests are REQUIRED per Constitution Principle V (Combat Mechanics Testing). All domain layer functions must have unit tests, and full AI workflows must have integration tests.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Rust + Bevy Game**: `src/domain/`, `src/infrastructure/plugins/`, `src/infrastructure/components/`, `src/infrastructure/systems/`, `tests/`
- **Domain Layer**: Pure Rust functions in `src/domain/enemy/ai.rs` (ZERO Bevy dependencies)
- **Components**: Pure data structures in `src/infrastructure/components/enemy.rs`
- **Systems**: Behavior functions in `src/infrastructure/systems/enemy.rs`
- **Plugins**: Feature modules in `src/infrastructure/plugins/enemy.rs`
- **Tests**: Unit tests in `tests/unit/enemy/`, integration in `tests/integration/enemy/`
- **Benchmarks**: Performance tests in `benches/enemy_ai_bench.rs`

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure for Enemy AI system

- [X] T001 创建 src/domain/enemy/ 目录结构
- [X] T002 [P] 创建 src/domain/enemy/mod.rs 导出 ai 模块
- [X] T003 [P] 创建 src/infrastructure/events/enemy.rs 定义事件类型（EnemyDetectedPlayer, EnemyLostTarget, EnemyAttackTriggered）
- [X] T004 [P] 创建 src/infrastructure/resources/enemy.rs 定义 AIConfig 资源（可选）
- [X] T005 [P] 更新 src/infrastructure/events/mod.rs 导出 enemy 模块
- [X] T006 [P] 更新 src/infrastructure/resources/mod.rs 导出 enemy 模块（如果创建了资源）

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [X] T010 创建 src/domain/enemy/ai.rs 定义 AIState 枚举（Patrol, Chase, Attack, Return）
- [X] T011 [P] 在 src/domain/enemy/ai.rs 实现基础类型别名（Direction, Position）
- [X] T012 [P] 扩展 src/infrastructure/components/enemy.rs 添加 EnemyAI 组件（state, previous_state, state_timer, state_entered_at）
- [X] T013 [P] 扩展 src/infrastructure/components/enemy.rs 添加 Perception 组件（detection_range, attack_range, aggro_drop_range, target_position, has_line_of_sight, last_check_time, check_interval）
- [X] T014 [P] 扩展 src/infrastructure/components/enemy.rs 添加 AggroTarget 组件（current_target, aggro_value, last_seen_position, time_since_last_seen, max_time_without_sight）
- [X] T015 [P] 扩展 src/infrastructure/components/enemy.rs 添加 PatrolConfig 组件（spawn_position, patrol_radius, wait_time, current_target, wait_timer, use_waypoints, waypoints, current_waypoint_index）
- [X] T016 [P] 扩展 src/infrastructure/components/enemy.rs 添加 AttackConfig 组件（attack_range, attack_cooldown, current_cooldown, attack_damage, attack_type, attack_animation_duration, attack_hit_frame）
- [X] T017 [P] 更新 src/infrastructure/components/mod.rs 确保 enemy 模块正确导出
- [X] T018 更新 assets/data/enemies.ron 添加 AI 配置字段（ai_config, attack_config）

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 2 - 发现与追逐 (Priority: P1) 🎯 MVP

**Goal**: 敌人感知到玩家接近后，切换状态并开始追逐玩家。这是核心战斗体验的前提，建立威胁感。

**Independent Test**: 玩家进入和离开敌人视野/范围，验证状态切换。敌人在发现玩家 0.5 秒内做出反应（切换状态）。

### Tests for User Story 2

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [X] T020 [P] [US2] 单元测试 should_transition_to_chase 函数在 tests/unit/enemy/ai_test.rs
- [X] T021 [P] [US2] 单元测试 should_drop_aggro 函数在 tests/unit/enemy/ai_test.rs
- [X] T022 [P] [US2] 单元测试 check_line_of_sight 函数在 tests/unit/enemy/ai_test.rs
- [X] T023 [P] [US2] 集成测试 AI 状态机流程（Patrol → Chase → Return）在 tests/integration/enemy/ai_state_machine_test.rs

### Implementation for User Story 2

- [X] T024 [US2] 在 src/domain/enemy/ai.rs 实现 should_transition_to_chase 函数（distance, detection_range, has_line_of_sight）-> bool
- [X] T025 [US2] 在 src/domain/enemy/ai.rs 实现 should_drop_aggro 函数（distance, aggro_drop_range, time_since_last_seen, max_time_without_sight）-> bool
- [X] T026 [US2] 在 src/domain/enemy/ai.rs 实现 check_line_of_sight 函数（enemy_pos, target_pos, obstacles）-> bool（简化的射线检测）
- [X] T027 [US2] 在 src/domain/enemy/ai.rs 实现 calculate_chase_direction 函数（enemy_pos, target_pos）-> Direction
- [X] T028 [US2] 在 src/infrastructure/systems/enemy.rs 实现 perception_system（每 3-5 帧检测一次，更新 Perception 组件）
- [X] T029 [US2] 在 src/infrastructure/systems/enemy.rs 实现 ai_state_machine_system（处理状态转换，调用领域层函数）
- [X] T030 [US2] 在 src/infrastructure/systems/enemy.rs 实现 chase_system（处理追逐行为，更新移动目标）
- [X] T031 [US2] 在 src/infrastructure/systems/enemy.rs 实现 return_to_patrol_system（处理脱战，返回生成位置）
- [X] T032 [US2] 在 src/infrastructure/plugins/enemy.rs 注册 perception_system, ai_state_machine_system, chase_system, return_to_patrol_system
- [X] T033 [US2] 在 src/infrastructure/events/enemy.rs 实现 EnemyDetectedPlayer 事件发送逻辑
- [X] T034 [US2] 在 src/infrastructure/events/enemy.rs 实现 EnemyLostTarget 事件发送逻辑

**Checkpoint**: At this point, User Story 2 should be fully functional and testable independently. Enemy can detect player, chase player, and return to patrol when player leaves.

---

## Phase 4: User Story 3 - 攻击行为 (Priority: P1) 🎯 MVP

**Goal**: 当追逐的玩家进入攻击范围，敌人发动攻击。构成战斗的实质性威胁。

**Independent Test**: 允许敌人接近玩家，观察攻击触发频率和距离。攻击判定准确，只有在攻击范围内且攻击动作生效帧期间才造成伤害。

### Tests for User Story 3

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [X] T040 [P] [US3] 单元测试 should_transition_to_attack 函数在 tests/unit/enemy/ai_test.rs
- [X] T041 [P] [US3] 集成测试完整 AI 状态机流程（Patrol → Chase → Attack → Chase）在 tests/integration/enemy/ai_state_machine_test.rs
- [X] T042 [P] [US3] 集成测试攻击冷却时间在 tests/integration/enemy/ai_state_machine_test.rs

### Implementation for User Story 3

- [X] T043 [US3] 在 src/domain/enemy/ai.rs 实现 should_transition_to_attack 函数（distance, attack_range, cooldown_ready）-> bool
- [X] T044 [US3] 在 src/infrastructure/systems/enemy.rs 实现 attack_system（处理攻击行为，触发攻击动画和伤害判定）
- [X] T045 [US3] 在 src/infrastructure/systems/enemy.rs 实现 attack_cooldown_system（管理攻击冷却时间）
- [X] T046 [US3] 在 src/infrastructure/plugins/enemy.rs 注册 attack_system, attack_cooldown_system
- [X] T047 [US3] 在 src/infrastructure/events/enemy.rs 实现 EnemyAttackTriggered 事件发送逻辑（包含 enemy, target, damage, attack_type, attack_position）
- [X] T048 [US3] 更新 ai_state_machine_system 支持 Attack 状态转换
- [X] T049 [US3] 更新 assets/data/enemies.ron 添加攻击配置（attack_cooldown, attack_hit_frame）

**Checkpoint**: At this point, User Stories 2 AND 3 should both work independently. Enemy can detect, chase, and attack player.

---

## Phase 5: User Story 1 - 敌人巡逻 (Priority: P2)

**Goal**: 当周围没有玩家时，敌人应按照预设或随机逻辑进行移动，展示出活跃的生态。增加游戏世界的生动感，避免敌人像雕像一样站立。

**Independent Test**: 在无玩家干预的情况下观察敌人行为。敌人在巡逻范围内移动，到达目标点后停留短暂时间，然后选择下一个目标点。

### Tests for User Story 1

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [X] T050 [P] [US1] 单元测试 calculate_patrol_target 函数在 tests/unit/enemy/ai_test.rs（随机点模式）
- [X] T051 [P] [US1] 单元测试预设路径点巡逻逻辑在 tests/unit/enemy/ai_test.rs
- [X] T052 [P] [US1] 集成测试巡逻行为在 tests/integration/enemy/ai_state_machine_test.rs

### Implementation for User Story 1

- [X] T053 [US1] 在 src/domain/enemy/ai.rs 实现 calculate_patrol_target 函数（current_pos, spawn_pos, patrol_radius）-> Position（随机点模式）
- [X] T054 [US1] 在 src/infrastructure/systems/enemy.rs 实现 patrol_system（处理巡逻行为，支持随机点和预设路径点两种模式）
- [X] T055 [US1] 在 src/infrastructure/plugins/enemy.rs 注册 patrol_system
- [X] T056 [US1] 更新 ai_state_machine_system 支持 Patrol 状态（初始状态）
- [X] T057 [US1] 更新 assets/data/enemies.ron 添加巡逻配置（patrol_radius, wait_time, use_waypoints, waypoints）

**Checkpoint**: At this point, all user stories should now be independently functional. Enemy can patrol, detect, chase, and attack.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [X] T060 [P] 性能基准测试 AI 更新性能在 benches/enemy_ai_bench.rs（目标 <2ms per frame）
- [X] T061 [P] 性能基准测试感知检查性能在 benches/enemy_ai_bench.rs（目标 <0.5ms）
- [X] T062 [P] 性能基准测试状态机更新性能在 benches/enemy_ai_bench.rs（目标 <0.3ms）
- [X] T063 [P] 集成测试多敌人仇恨系统在 tests/integration/enemy/multi_enemy_aggro_test.rs
- [X] T064 [P] 更新 README.md 添加 Enemy AI System 部分
- [X] T065 [P] 代码清理和重构（确保所有系统遵循 DDD 架构原则）
- [X] T066 [P] 运行 quickstart.md 验证所有测试场景
- [X] T067 [P] 验证 Constitution 合规性（零 unsafe 代码，ECS 模式，性能预算）

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3+)**: All depend on Foundational phase completion
  - User Story 2 (P1) and User Story 3 (P1) are MVP - should be implemented first
  - User Story 1 (P2) can be implemented after MVP
  - User stories can proceed in parallel (if staffed) after Foundational phase
- **Polish (Final Phase)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 2 (P1) - 发现与追逐**: Can start after Foundational (Phase 2) - No dependencies on other stories. This is MVP core functionality.
- **User Story 3 (P1) - 攻击行为**: Can start after Foundational (Phase 2) - Depends on User Story 2 for Chase state. Should be implemented immediately after US2.
- **User Story 1 (P2) - 敌人巡逻**: Can start after Foundational (Phase 2) - Can be implemented independently, but lower priority. Can run in parallel with US2/US3 if team capacity allows.

### Within Each User Story

- Tests (REQUIRED per Constitution) MUST be written and FAIL before implementation
- Domain layer functions before infrastructure systems
- Components before systems
- Systems before plugin registration
- Core implementation before integration
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- All Foundational tasks marked [P] can run in parallel (within Phase 2)
- Once Foundational phase completes, User Story 2 and User Story 3 can start (US3 depends on US2 for Chase state)
- User Story 1 can run in parallel with US2/US3 if team capacity allows
- All tests for a user story marked [P] can run in parallel
- Domain layer functions within a story marked [P] can run in parallel
- Different components within a story marked [P] can run in parallel

---

## Parallel Example: User Story 2

```bash
# Launch all tests for User Story 2 together:
Task: "Unit test should_transition_to_chase function in tests/unit/enemy/ai_test.rs"
Task: "Unit test should_drop_aggro function in tests/unit/enemy/ai_test.rs"
Task: "Unit test check_line_of_sight function in tests/unit/enemy/ai_test.rs"
Task: "Integration test AI state machine flow in tests/integration/enemy/ai_state_machine_test.rs"

# Launch all domain layer functions together (different functions, no dependencies):
Task: "Implement should_transition_to_chase function in src/domain/enemy/ai.rs"
Task: "Implement should_drop_aggro function in src/domain/enemy/ai.rs"
Task: "Implement check_line_of_sight function in src/domain/enemy/ai.rs"
Task: "Implement calculate_chase_direction function in src/domain/enemy/ai.rs"

# Launch all components together (different components, no dependencies):
Task: "Add EnemyAI component in src/infrastructure/components/enemy.rs"
Task: "Add Perception component in src/infrastructure/components/enemy.rs"
Task: "Add AggroTarget component in src/infrastructure/components/enemy.rs"
```

---

## Implementation Strategy

### MVP First (User Story 2 + User Story 3)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 2 (发现与追逐) - Core functionality
4. Complete Phase 4: User Story 3 (攻击行为) - Core functionality
5. **STOP and VALIDATE**: Test User Stories 2 and 3 independently
6. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 2 → Test independently → Deploy/Demo (MVP Part 1: Detection & Chase)
3. Add User Story 3 → Test independently → Deploy/Demo (MVP Part 2: Attack)
4. Add User Story 1 → Test independently → Deploy/Demo (Polish: Patrol)
5. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 2 (发现与追逐)
   - Developer B: Prepare User Story 3 (攻击行为) - waits for US2 Chase state
3. After US2 completes:
   - Developer A: User Story 3 (攻击行为)
   - Developer B: User Story 1 (敌人巡逻) - can run in parallel
4. Stories complete and integrate independently

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Tests are REQUIRED per Constitution Principle V - verify tests fail before implementing
- Domain layer functions have ZERO Bevy dependencies - pure Rust functions
- Infrastructure layer bridges domain logic to Bevy ECS
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence
- Performance budget: AI updates <2ms per frame (perception <0.5ms, state machine <0.3ms, etc.)
- Perception checks throttled to every 3-5 frames (~50-100ms interval)

---

## Summary

**Total Task Count**: 56 tasks

**Task Count per User Story**:
- Phase 1 (Setup): 6 tasks
- Phase 2 (Foundational): 9 tasks
- Phase 3 (User Story 2 - 发现与追逐): 15 tasks (4 tests + 11 implementation)
- Phase 4 (User Story 3 - 攻击行为): 10 tasks (3 tests + 7 implementation)
- Phase 5 (User Story 1 - 敌人巡逻): 8 tasks (3 tests + 5 implementation)
- Phase 6 (Polish): 8 tasks

**Parallel Opportunities**:
- Setup phase: 5 tasks can run in parallel
- Foundational phase: 8 tasks can run in parallel
- User Story 2: 4 tests + 4 domain functions + 3 components can run in parallel
- User Story 3: 3 tests can run in parallel
- User Story 1: 3 tests can run in parallel
- Polish phase: 8 tasks can run in parallel

**Independent Test Criteria**:
- **User Story 2**: 玩家进入和离开敌人视野/范围，验证状态切换。敌人在发现玩家 0.5 秒内做出反应。
- **User Story 3**: 允许敌人接近玩家，观察攻击触发频率和距离。攻击判定准确，只有在攻击范围内且攻击动作生效帧期间才造成伤害。
- **User Story 1**: 在无玩家干预的情况下观察敌人行为。敌人在巡逻范围内移动，到达目标点后停留短暂时间。

**Suggested MVP Scope**: User Story 2 (发现与追逐) + User Story 3 (攻击行为) - both P1 priority, core combat functionality

**Format Validation**: ✅ ALL tasks follow the checklist format (checkbox, ID, [P] marker where applicable, [Story] label for user story tasks, file paths included)

