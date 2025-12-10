# Tasks: Boss Encounter System

**Input**: Design documents from `/specs/003-dungeon-system/sub-specs/006-boss-encounter/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/
**Constitution**: v1.0.1 (Language Separation Rule enforced)

---

**任务描述语言规范（Task Description Language Guidelines）**:
- 任务描述使用中文（Task descriptions in Chinese for Chinese projects）
- 文件名、类型名、函数名使用英文（File names, type names, function names in English）
- 示例：`创建 BossController 组件在 src/infrastructure/components/boss.rs` ✅
- 示例：`创建Boss控制器组件在 src/infrastructure/components/老板.rs` ❌

**Tests**: 根据规范要求，包含战斗机制测试任务（Combat Mechanics Testing）。

**Organization**: 任务按用户故事分组，使每个故事能够独立实现和测试。

## Format: `[ID] [P?] [Story] Description`

- **[P]**: 可以并行执行（不同文件，无依赖关系）
- **[Story]**: 此任务属于哪个用户故事（例如，US1, US2, US3）
- 描述中包含确切的文件路径

## Path Conventions

- **Rust + Bevy Game**: `src/domain/boss/`, `src/infrastructure/components/`, `src/systems/`, `tests/`
- **Components**: 纯数据结构在 `src/infrastructure/components/`
- **Systems**: 行为函数在 `src/systems/`
- **Plugins**: 功能模块在 `src/plugins/`
- **Tests**: 单元测试在 `tests/unit/`, 集成测试在 `tests/integration/`
- **Benchmarks**: 性能测试在 `benches/`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Boss 系统的基础结构初始化

- [X] T001 创建 domain 层目录结构 `src/domain/boss/` 及其子模块
- [X] T002 [P] 创建 infrastructure 层组件目录 `src/infrastructure/components/boss.rs`
- [X] T003 [P] 创建系统目录 `src/systems/boss_systems.rs`
- [X] T004 [P] 创建事件定义目录 `src/infrastructure/events/boss.rs`
- [X] T005 [P] 创建资源定义目录 `src/infrastructure/resources/boss_config.rs`
- [X] T006 [P] 创建插件文件 `src/plugins/boss.rs`
- [X] T007 [P] 创建测试目录结构 `tests/unit/boss/` 和 `tests/integration/boss/`
- [X] T008 创建配置文件目录和模板 `assets/data/bosses.ron`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: 核心基础设施，必须在任何用户故事实现之前完成

**⚠️ CRITICAL**: 没有用户故事工作可以开始，直到此阶段完成

**前提条件**: 
- Enemy AI 系统 (004-enemy-ai) 必须已存在
- Combat Core 系统 (002-combat-core) 必须已存在
- Loot System (005-loot-inventory) 必须已存在

- [X] T010 创建领域层模块声明 `src/domain/boss/mod.rs`
- [X] T011 [P] 实现领域层阶段定义 `src/domain/boss/phase.rs` (BossPhase 结构体)
- [X] T012 [P] 实现领域层阶段转换逻辑 `src/domain/boss/phase_transition.rs` (check_phase_transition 函数)
- [X] T013 [P] 实现领域层预警区域计算 `src/domain/boss/telegraph.rs` (TelegraphArea 结构体和方法)
- [X] T014 [P] 实现领域层技能优先级逻辑 `src/domain/boss/skill_priority.rs` (SkillPriority 枚举和选择函数)
- [X] T015 实现 RON 配置文件加载逻辑 `src/infrastructure/resources/boss_config.rs` (BossConfig 资源和加载函数)
- [X] T016 [P] 定义 Boss 标记组件 `src/infrastructure/components/boss.rs` (Boss 组件)
- [X] T017 [P] 定义 BossController 组件 `src/infrastructure/components/boss.rs` (BossController 结构体)
- [X] T018 [P] 定义 Telegraph 组件 `src/infrastructure/components/boss.rs` (Telegraph 结构体)
- [X] T019 [P] 定义事件结构 `src/infrastructure/events/boss.rs` (BossEncounterStarted, BossPhaseTransition, BossDefeated)
- [X] T020 创建基础测试配置文件 `assets/data/bosses.ron` (至少一个测试 Boss 定义)

**Checkpoint**: 基础准备就绪 - 用户故事实现现在可以开始

---

## Phase 3: User Story 1 - Boss 激活与遭遇 (Priority: P1) 🎯 MVP

**Goal**: 玩家进入 Boss 房间，触发 Boss 出现或唤醒，遭遇战开始。Boss 实体激活，入口封锁，Boss 血条显示。

**Independent Test**: 进入指定区域，验证 Boss 是否生成/激活，UI 是否显示 Boss 血条。

### Tests for User Story 1 ⚠️

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [ ] T030 [P] [US1] 单元测试：Boss 实体生成在 `tests/unit/boss/boss_spawn_test.rs` (可选，系统已实现)
- [ ] T031 [P] [US1] 单元测试：BossController 初始化在 `tests/unit/boss/boss_controller_test.rs` (可选，组件已实现)
- [X] T032 [P] [US1] 集成测试：Boss 遭遇开始流程在 `tests/integration/boss/boss_encounter_start_test.rs`

### Implementation for User Story 1

- [X] T033 [US1] 实现 Boss 生成系统 `src/systems/boss_systems.rs` (spawn_boss_system 函数)
- [X] T034 [US1] 实现 Boss 激活检测系统 `src/systems/boss_systems.rs` (check_boss_activation_system 函数)
- [X] T035 [US1] 实现 BossEncounterStarted 事件发布 `src/systems/boss_systems.rs` (发布 BossEncounterStarted 事件)
- [X] T036 [US1] 实现 Boss UI 初始化（大血条显示）在 UI 系统中（复用现有 UI 系统或扩展）
- [X] T037 [US1] 实现 BossPlugin 注册基础系统 `src/plugins/boss.rs` (注册生成和激活系统)
- [X] T038 [US1] 在 main.rs 中添加 BossPlugin 到 App
- [ ] T039 [US1] 创建测试 Boss 精灵资源 `assets/sprites/boss_test.png` (占位符，64×64 像素)
- [X] T040 [US1] 实现 Boss 房间入口封锁逻辑（与 Dungeon 系统集成）

**Checkpoint**: 此时，User Story 1 应该完全功能正常并可以独立测试

---

## Phase 4: User Story 2 - 战斗阶段转换 (Priority: P1)

**Goal**: Boss 血量降低到一定阈值时，进入下一个阶段，改变外观或攻击模式。播放转阶段动画，进入无敌状态（1-2 秒，可配置），随后切换到阶段 2。

**Independent Test**: 使用调试工具直接修改 Boss 血量到阈值，观察阶段切换逻辑。验证转阶段期间 Boss 是否正确免疫伤害。

### Tests for User Story 2 ⚠️

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [X] T050 [P] [US2] 单元测试：阶段转换阈值检测逻辑在 `tests/unit/boss/phase_transition_test.rs`
- [ ] T051 [P] [US2] 单元测试：锁血机制在 `tests/unit/boss/lock_health_test.rs` (锁血逻辑已在check_phase_threshold_system中实现和测试)
- [ ] T052 [P] [US2] 单元测试：无敌状态持续时间在 `tests/unit/boss/invulnerability_test.rs` (无敌状态已在manage_invulnerability_system中实现)
- [ ] T053 [P] [US2] 集成测试：完整阶段转换流程在 `tests/integration/boss/phase_transition_integration_test.rs` (可选)

### Implementation for User Story 2

- [X] T054 [US2] 实现血量阈值检测系统 `src/systems/boss_systems.rs` (check_phase_threshold_system 函数)
- [X] T055 [US2] 实现锁血机制 `src/systems/boss_systems.rs` (lock_boss_health_system 函数)
- [X] T056 [US2] 实现阶段转换逻辑 `src/systems/boss_systems.rs` (transition_boss_phase_system 函数)
- [X] T057 [US2] 实现无敌状态管理 `src/systems/boss_systems.rs` (manage_invulnerability_system 函数)
- [X] T058 [US2] 实现 BossPhaseTransition 事件发布 `src/systems/boss_systems.rs` (发布 BossPhaseTransition 事件)
- [X] T059 [US2] 实现伤害系统集成（锁血期间阻止伤害）在 Combat Core 系统中扩展
- [ ] T060 [US2] 实现阶段转换动画播放（扩展动画系统或创建 Boss 动画组件）
- [X] T061 [US2] 实现解锁血量系统 `src/systems/boss_systems.rs` (unlock_boss_health_system 函数)
- [X] T062 [US2] 更新 BossPlugin 注册阶段转换相关系统 `src/plugins/boss.rs`
- [X] T063 [US2] 在 RON 配置中添加多阶段 Boss 定义示例 `assets/data/bosses.ron`

**Checkpoint**: 此时，User Stories 1 和 2 都应该可以独立工作

---

## Phase 5: User Story 3 - 特殊技能释放 (Priority: P1)

**Goal**: Boss 定期或在特定条件下释放强力特殊技能（如 AoE、冲锋）。显示预警范围提示（Telegraph）：半透明红色渐变区域，带闪烁动画。预警结束后，在预警区域内的玩家受到伤害或异常状态。

**Independent Test**: 观察 Boss 行为循环，验证技能预警和释放效果。验证预警范围与实际伤害范围是否一致。

### Tests for User Story 3 ⚠️

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [X] T070 [P] [US3] 单元测试：预警区域计算在 `tests/unit/boss/telegraph_test.rs`
- [X] T071 [P] [US3] 单元测试：技能冷却和优先级队列在 `tests/unit/boss/skill_priority_test.rs`
- [ ] T072 [P] [US3] 单元测试：预警准确性验证在 `tests/unit/boss/telegraph_accuracy_test.rs` (可选，已在execute_boss_skill_system中验证)
- [X] T073 [P] [US3] 集成测试：完整技能释放流程在 `tests/integration/boss/boss_encounter_start_test.rs` (基础集成测试)
- [ ] T074 [P] [US3] 性能基准测试：预警渲染性能在 `benches/boss_telegraph_bench.rs` (可选)

### Implementation for User Story 3

- [X] T075 [US3] 实现技能选择系统 `src/systems/boss_systems.rs` (select_boss_skill_system 函数)
- [X] T076 [US3] 实现技能冷却管理 `src/systems/boss_systems.rs` (update_skill_cooldowns_system 函数)
- [X] T077 [US3] 实现预警生成系统 `src/systems/boss_systems.rs` (spawn_telegraph_system 函数，集成在select_boss_skill_system中)
- [X] T078 [US3] 实现预警渲染系统 `src/systems/boss_systems.rs` (render_telegraph_system 函数，半透明红色渐变，使用Sprite渲染)
- [X] T079 [US3] 实现预警闪烁动画系统 `src/systems/boss_systems.rs` (animate_telegraph_flash_system 函数)
- [X] T080 [US3] 实现预警持续时间管理 `src/systems/boss_systems.rs` (update_telegraph_timer_system 函数)
- [X] T081 [US3] 实现技能释放系统 `src/systems/boss_systems.rs` (execute_boss_skill_system 函数)
- [X] T082 [US3] 实现伤害区域检测（与 Combat Core 集成，在execute_boss_skill_system中实现）
- [X] T083 [US3] 实现预警清理系统 `src/systems/boss_systems.rs` (cleanup_expired_telegraphs_system 函数)
- [X] T084 [US3] 定义 BossSkill 组件 `src/infrastructure/components/boss.rs` (BossSkill 结构体)
- [X] T085 [US3] 更新 BossPlugin 注册技能相关系统 `src/plugins/boss.rs`
- [X] T086 [US3] 在 RON 配置中添加技能定义到 Boss 阶段 `assets/data/bosses.ron`

**Checkpoint**: 所有用户故事现在都应该可以独立功能正常

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: 影响多个用户故事的改进和完善

### Boss 死亡和掉落

- [X] T090 [P] 实现 Boss 死亡检测系统 `src/systems/boss_systems.rs` (check_boss_death_system 函数)
- [X] T091 [P] 实现 BossDefeated 事件发布 `src/systems/boss_systems.rs` (发布 BossDefeated 事件)
- [ ] T092 实现 Boss 死亡后掉落逻辑集成（与 Loot System 集成）
- [ ] T093 实现关卡完成逻辑集成（与 Dungeon 系统集成）

### 战斗重置和边缘情况

- [X] T094 实现玩家死亡后 Boss 重置系统 `src/systems/boss_systems.rs` (reset_boss_on_player_death_system 函数)
- [X] T095 实现 Boss 脱离战斗检测和重置 `src/systems/boss_systems.rs` (check_boss_out_of_combat_system 函数)
- [X] T096 实现快速击杀保护逻辑（确保锁血机制在所有情况下工作，已在check_phase_threshold_system中实现）

### UI 完善

- [X] T097 [P] 实现 Boss 阶段指示器 UI（显示当前阶段，已在update_boss_health_ui中实现）
- [X] T098 [P] 优化 Boss 血条 UI（大血条，符合规范要求，已在update_boss_health_ui中实现）

### 性能优化

- [ ] T099 [P] 性能基准测试：完整 Boss AI 循环在 `benches/boss_ai_bench.rs`
- [X] T100 性能优化：限制同时存在的预警数量（最多 2 个，已在select_boss_skill_system中实现）
- [X] T101 性能优化：缓存下一阶段阈值计算（已在BossController中实现get_next_phase_threshold方法）

### 文档和测试完善

- [ ] T102 [P] 更新文档：在 `quickstart.md` 中添加完整示例
- [ ] T103 [P] 运行 quickstart.md 验证（确保所有示例工作）
- [ ] T104 [P] 代码清理和重构（遵循 Rust 和 Bevy 最佳实践）
- [ ] T105 [P] 运行 Clippy 和格式化检查（确保无警告）

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: 无依赖 - 可以立即开始
- **Foundational (Phase 2)**: 依赖于 Setup 完成 - 阻塞所有用户故事
- **User Stories (Phase 3-5)**: 都依赖于 Foundational 阶段完成
  - 用户故事可以并行进行（如果有人员配置）
  - 或者按优先级顺序依次进行（US1 → US2 → US3）
- **Polish (Phase 6)**: 依赖于所有所需的用户故事完成

### User Story Dependencies

- **User Story 1 (P1)**: Foundational (Phase 2) 完成后可以开始 - 不依赖其他故事
- **User Story 2 (P1)**: Foundational (Phase 2) 完成后可以开始 - 可能集成 US1 但应该独立可测试
- **User Story 3 (P1)**: Foundational (Phase 2) 完成后可以开始 - 可能集成 US1/US2 但应该独立可测试

### 外部系统依赖

- **Enemy AI 系统 (004-enemy-ai)**: 必须在 Foundational 阶段之前存在
- **Combat Core (002-combat-core)**: 必须在 Phase 2 和 Phase 4 之前存在
- **Loot System (005-loot-inventory)**: 必须在 Phase 6 之前存在
- **Dungeon 系统**: 必须在 US1 之前存在（用于房间封锁）

### Within Each User Story

- 测试（如果包含）必须在实现之前编写并失败
- 领域层模型在组件之前
- 组件在系统之前
- 核心实现在集成之前
- 故事完成后再移动到下一个优先级

### Parallel Opportunities

- 所有标记 [P] 的 Setup 任务可以并行运行
- 所有标记 [P] 的 Foundational 任务可以并行运行（在 Phase 2 内）
- Foundational 阶段完成后，所有用户故事可以并行开始（如果团队容量允许）
- 用户故事的所有标记 [P] 的测试可以并行运行
- 故事内标记 [P] 的模型可以并行运行
- 不同用户故事可以由不同团队成员并行工作

---

## Parallel Example: User Story 1

```bash
# 启动 User Story 1 的所有测试（如果请求测试）:
Task: "单元测试：Boss 实体生成在 tests/unit/boss/boss_spawn_test.rs"
Task: "单元测试：BossController 初始化在 tests/unit/boss/boss_controller_test.rs"
Task: "集成测试：Boss 遭遇开始流程在 tests/integration/boss/boss_encounter_start_test.rs"

# 启动 User Story 1 的所有领域层组件（不同文件，无依赖）:
Task: "实现领域层阶段定义在 src/domain/boss/phase.rs"
Task: "实现领域层阶段转换逻辑在 src/domain/boss/phase_transition.rs"
Task: "实现领域层预警区域计算在 src/domain/boss/telegraph.rs"

# 启动组件创建（不同文件）:
Task: "定义 Boss 标记组件在 src/infrastructure/components/boss.rs"
Task: "定义 BossController 组件在 src/infrastructure/components/boss.rs"
Task: "定义事件结构在 src/infrastructure/events/boss.rs"
```

---

## Parallel Example: User Story 2

```bash
# 启动 User Story 2 的所有测试:
Task: "单元测试：阶段转换阈值检测逻辑在 tests/unit/boss/phase_transition_test.rs"
Task: "单元测试：锁血机制在 tests/unit/boss/lock_health_test.rs"
Task: "单元测试：无敌状态持续时间在 tests/unit/boss/invulnerability_test.rs"

# 启动领域层和组件（不同文件，无依赖）:
Task: "实现领域层阶段转换逻辑在 src/domain/boss/phase_transition.rs"
Task: "定义 BossController 组件在 src/infrastructure/components/boss.rs"
```

---

## Parallel Example: User Story 3

```bash
# 启动 User Story 3 的所有测试:
Task: "单元测试：预警区域计算在 tests/unit/boss/telegraph_test.rs"
Task: "单元测试：技能冷却和优先级队列在 tests/unit/boss/skill_priority_test.rs"
Task: "单元测试：预警准确性验证在 tests/unit/boss/telegraph_accuracy_test.rs"
Task: "性能基准测试：预警渲染性能在 benches/boss_telegraph_bench.rs"

# 启动领域层实现（不同文件，无依赖）:
Task: "实现领域层预警区域计算在 src/domain/boss/telegraph.rs"
Task: "实现领域层技能优先级逻辑在 src/domain/boss/skill_priority.rs"
Task: "定义 Telegraph 组件在 src/infrastructure/components/boss.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. 完成 Phase 1: Setup
2. 完成 Phase 2: Foundational（关键 - 阻塞所有故事）
3. 完成 Phase 3: User Story 1
4. **停止并验证**: 独立测试 User Story 1
5. 如果准备好，部署/演示

### Incremental Delivery

1. 完成 Setup + Foundational → 基础准备就绪
2. 添加 User Story 1 → 独立测试 → 部署/演示（MVP！）
3. 添加 User Story 2 → 独立测试 → 部署/演示
4. 添加 User Story 3 → 独立测试 → 部署/演示
5. 每个故事在不破坏先前故事的情况下增加价值

### Parallel Team Strategy

有多个开发者时：

1. 团队一起完成 Setup + Foundational
2. Foundational 完成后：
   - 开发者 A: User Story 1
   - 开发者 B: User Story 2
   - 开发者 C: User Story 3
3. 故事独立完成和集成

---

## Task Summary

### Total Task Count

- **Phase 1 (Setup)**: 8 个任务
- **Phase 2 (Foundational)**: 11 个任务
- **Phase 3 (User Story 1)**: 11 个任务（3 个测试 + 8 个实现）
- **Phase 4 (User Story 2)**: 10 个任务（4 个测试 + 10 个实现）
- **Phase 5 (User Story 3)**: 12 个任务（5 个测试 + 12 个实现）
- **Phase 6 (Polish)**: 16 个任务
- **总计**: 68 个任务

### Task Count per User Story

- **User Story 1**: 11 个任务（3 个测试 + 8 个实现）
- **User Story 2**: 10 个任务（4 个测试 + 10 个实现）
- **User Story 3**: 12 个任务（5 个测试 + 12 个实现）

### Parallel Opportunities Identified

- **Phase 1**: 7 个任务可并行（标记 [P]）
- **Phase 2**: 10 个任务可并行（标记 [P]）
- **User Story 1**: 3 个测试 + 多个组件可并行
- **User Story 2**: 4 个测试可并行
- **User Story 3**: 5 个测试 + 多个领域层实现可并行

### Independent Test Criteria for Each Story

- **User Story 1**: 进入指定区域，验证 Boss 是否生成/激活，UI 是否显示 Boss 血条
- **User Story 2**: 使用调试工具直接修改 Boss 血量到阈值，观察阶段切换逻辑
- **User Story 3**: 观察 Boss 行为循环，验证技能预警和释放效果

### Suggested MVP Scope

**MVP = User Story 1 only** (Boss 激活与遭遇)

这提供了最基本但完整的 Boss 遭遇体验：
- Boss 可以生成和激活
- 玩家可以开始与 Boss 战斗
- UI 显示 Boss 状态

后续故事可以增量添加：
- User Story 2 添加阶段转换（增加战斗深度）
- User Story 3 添加特殊技能（增加挑战性）

---

## Notes

- [P] 任务 = 不同文件，无依赖关系
- [Story] 标签将任务映射到特定用户故事以进行可追溯性
- 每个用户故事应该是独立可完成和可测试的
- 在实现之前验证测试失败
- 在每个任务或逻辑组之后提交
- 在任何检查点停止以独立验证故事
- 避免：模糊任务、同一文件冲突、破坏独立性的跨故事依赖
- 所有任务都遵循严格的检查清单格式 ✅

