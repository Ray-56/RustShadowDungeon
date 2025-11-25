---

description: "Task list template for feature implementation"
---

# Tasks: [FEATURE NAME]

**Input**: Design documents from `/specs/[###-feature-name]/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/
**Constitution**: v1.0.1 (Language Separation Rule enforced)

---

**任务描述语言规范（Task Description Language Guidelines）**:
- 任务描述使用中文（Task descriptions in Chinese for Chinese projects）
- 文件名、类型名、函数名使用英文（File names, type names, function names in English）
- 示例：`创建 Player 组件在 src/components/player.rs` ✅
- 示例：`创建玩家组件在 src/components/玩家.rs` ❌

**Tests**: The examples below include test tasks. Tests are OPTIONAL - only include them if explicitly requested in the feature specification.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Rust + Bevy Game**: `src/plugins/`, `src/components/`, `src/systems/`, `tests/`
- **Components**: Pure data structures in `src/components/`
- **Systems**: Behavior functions in `src/systems/`
- **Plugins**: Feature modules in `src/plugins/`
- **Tests**: Unit tests in `tests/unit/`, integration in `tests/integration/`
- **Benchmarks**: Performance tests in `benches/`

<!-- 
  ============================================================================
  IMPORTANT: The tasks below are SAMPLE TASKS for illustration purposes only.
  
  The /speckit.tasks command MUST replace these with actual tasks based on:
  - User stories from spec.md (with their priorities P1, P2, P3...)
  - Feature requirements from plan.md
  - Entities from data-model.md
  - Endpoints from contracts/
  
  Tasks MUST be organized by user story so each story can be:
  - Implemented independently
  - Tested independently
  - Delivered as an MVP increment
  
  DO NOT keep these sample tasks in the generated tasks.md file.
  ============================================================================
-->

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [ ] T001 Create project structure per implementation plan (src/plugins, components, systems, etc.)
- [ ] T002 Initialize Cargo.toml with Bevy dependencies (bevy, bevy_rapier2d, etc.)
- [ ] T003 [P] Configure rustfmt.toml and clippy.toml with project standards
- [ ] T004 [P] Setup CI with cargo fmt, cargo clippy, cargo test
- [ ] T005 [P] Configure Bevy default plugins and window settings in main.rs

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

Examples of foundational tasks (adjust based on your project):

- [ ] T010 Create base component definitions (Transform, Sprite, Collider, Health)
- [ ] T011 [P] Setup asset loading plugin with AssetServer
- [ ] T012 [P] Configure camera system with pixel-perfect rendering (16x16 grid lock)
- [ ] T013 [P] Implement input handling plugin (keyboard/gamepad mapping)
- [ ] T014 Setup physics plugin (bevy_rapier2d) with collision layers
- [ ] T015 [P] Create game state management (AppState enum and transitions)
- [ ] T016 [P] Setup debug overlay system (FPS counter, hitbox visualization)
- [ ] T017 Create resource definitions (GameConfig, AssetHandles)
- [ ] T018 Implement event bus for combat/damage events

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - [Title] (Priority: P1) 🎯 MVP

**Goal**: [Brief description of what this story delivers]

**Independent Test**: [How to verify this story works on its own]

### Tests for User Story 1 (OPTIONAL - only if tests requested) ⚠️

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [ ] T019 [P] [US1] Unit test for [component behavior] in tests/unit/test_[name].rs
- [ ] T020 [P] [US1] Integration test for [system interaction] in tests/integration/test_[name].rs
- [ ] T021 [P] [US1] Benchmark for [critical path] in benches/[name]_bench.rs (if performance-critical)

### Implementation for User Story 1

- [ ] T022 [P] [US1] Define components in src/components/[feature].rs (e.g., PlayerInput, PlayerState)
- [ ] T023 [P] [US1] Define events in src/events/[feature].rs (e.g., PlayerMoved, SkillUsed)
- [ ] T024 [US1] Implement systems in src/systems/[feature].rs (e.g., player_movement_system)
- [ ] T025 [US1] Create plugin in src/plugins/[feature].rs that registers components/systems/events
- [ ] T026 [US1] Add plugin to main.rs App builder
- [ ] T027 [US1] Create resources if needed in src/resources/[feature].rs
- [ ] T028 [US1] Add sprite assets to assets/sprites/ following 16x16 grid
- [ ] T029 [US1] Load assets in plugin setup

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently

---

## Phase 4: User Story 2 - [Title] (Priority: P2)

**Goal**: [Brief description of what this story delivers]

**Independent Test**: [How to verify this story works on its own]

### Tests for User Story 2 (OPTIONAL - only if tests requested) ⚠️

- [ ] T018 [P] [US2] Contract test for [endpoint] in tests/contract/test_[name].py
- [ ] T019 [P] [US2] Integration test for [user journey] in tests/integration/test_[name].py

### Implementation for User Story 2

- [ ] T020 [P] [US2] Create [Entity] model in src/models/[entity].py
- [ ] T021 [US2] Implement [Service] in src/services/[service].py
- [ ] T022 [US2] Implement [endpoint/feature] in src/[location]/[file].py
- [ ] T023 [US2] Integrate with User Story 1 components (if needed)

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently

---

## Phase 5: User Story 3 - [Title] (Priority: P3)

**Goal**: [Brief description of what this story delivers]

**Independent Test**: [How to verify this story works on its own]

### Tests for User Story 3 (OPTIONAL - only if tests requested) ⚠️

- [ ] T024 [P] [US3] Contract test for [endpoint] in tests/contract/test_[name].py
- [ ] T025 [P] [US3] Integration test for [user journey] in tests/integration/test_[name].py

### Implementation for User Story 3

- [ ] T026 [P] [US3] Create [Entity] model in src/models/[entity].py
- [ ] T027 [US3] Implement [Service] in src/services/[service].py
- [ ] T028 [US3] Implement [endpoint/feature] in src/[location]/[file].py

**Checkpoint**: All user stories should now be independently functional

---

[Add more user story phases as needed, following the same pattern]

---

## Phase N: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [ ] TXXX [P] Documentation updates in docs/
- [ ] TXXX Code cleanup and refactoring
- [ ] TXXX Performance optimization across all stories
- [ ] TXXX [P] Additional unit tests (if requested) in tests/unit/
- [ ] TXXX Security hardening
- [ ] TXXX Run quickstart.md validation

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3+)**: All depend on Foundational phase completion
  - User stories can then proceed in parallel (if staffed)
  - Or sequentially in priority order (P1 → P2 → P3)
- **Polish (Final Phase)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) - May integrate with US1 but should be independently testable
- **User Story 3 (P3)**: Can start after Foundational (Phase 2) - May integrate with US1/US2 but should be independently testable

### Within Each User Story

- Tests (if included) MUST be written and FAIL before implementation
- Models before services
- Services before endpoints
- Core implementation before integration
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- All Foundational tasks marked [P] can run in parallel (within Phase 2)
- Once Foundational phase completes, all user stories can start in parallel (if team capacity allows)
- All tests for a user story marked [P] can run in parallel
- Models within a story marked [P] can run in parallel
- Different user stories can be worked on in parallel by different team members

---

## Parallel Example: User Story 1

```bash
# Launch all tests for User Story 1 together (if tests requested):
Task: "Unit test for player movement in tests/unit/test_player.rs"
Task: "Integration test for input-to-movement pipeline in tests/integration/test_player.rs"

# Launch all components for User Story 1 together (different files, no dependencies):
Task: "Define PlayerInput component in src/components/player.rs"
Task: "Define PlayerState component in src/components/player.rs"
Task: "Define PlayerMoved event in src/events/player.rs"

# Launch sprite asset creation in parallel (different files):
Task: "Create player idle sprite in assets/sprites/player_idle.png"
Task: "Create player walk sprite in assets/sprites/player_walk.png"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1
4. **STOP and VALIDATE**: Test User Story 1 independently
5. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy/Demo (MVP!)
3. Add User Story 2 → Test independently → Deploy/Demo
4. Add User Story 3 → Test independently → Deploy/Demo
5. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1
   - Developer B: User Story 2
   - Developer C: User Story 3
3. Stories complete and integrate independently

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Verify tests fail before implementing
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence
