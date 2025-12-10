# Implementation Plan: Enemy AI System

**Branch**: `004-enemy-ai` | **Date**: 2025-01-27 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/003-dungeon-system/sub-specs/004-enemy-ai/spec.md`

## Summary

实现敌人 AI 系统，包括有限状态机（Patrol、Chase、Attack）、感知系统（距离和视线检测）、仇恨系统、攻击行为。敌人可以检测玩家，追逐玩家，在攻击范围内发动攻击。系统采用 DDD 架构，领域层（纯函数）处理状态转换逻辑和感知计算，基础设施层（Bevy ECS）处理 AI 组件、状态管理和行为执行。与地下城系统集成，支持敌人在房间内的 AI 行为。

## Technical Context

**Language/Version**: Rust 1.82.0 (stable)
**Primary Dependencies**: 
- Bevy 0.17.0 (ECS game engine)
- avian2d 0.4 (physics engine, for collision detection)
- bevy-tnua 0.26 (character controller, for enemy movement)
- rand 0.8 (random number generation for patrol behavior)
- serde 1.0 (serialization for AI configuration)

**Storage**: RON files for enemy AI configuration (`assets/data/enemies.ron`)
**Testing**: cargo test, criterion for benchmarks
**Target Platform**: Windows/Linux/macOS desktop, potential WASM
**Project Type**: Single game project with plugin-based DDD architecture
**Performance Goals**: 60 FPS (16.67ms frame budget), AI updates <2ms per frame (within AI & Dungeon Logic budget)
**Constraints**: 
- Pixel-perfect rendering (16×16 grid)
- Domain layer must have ZERO Bevy dependencies
- Infrastructure layer bridges domain logic to ECS
- Language separation: English code, Chinese documentation
- Must integrate with existing dungeon system and combat system

**Scale/Scope**: 
- Multiple enemy types (melee, ranged)
- State machine with 3+ states (Patrol, Chase, Attack)
- Perception system (detection range, line of sight)
- Aggro system (target selection, aggro drop)
- Attack cooldown management
- Pathfinding (simplified, direct movement toward target)

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### Core Principles Compliance

- [x] **Rust Memory Safety**: No `unsafe` code required. All AI logic uses safe Rust APIs.
- [x] **Bevy ECS Architecture**: All systems follow ECS patterns:
  - Components: `EnemyAI`, `AIState`, `Perception`, `AggroTarget` (pure data)
  - Systems: `patrol_system`, `perception_system`, `chase_system`, `attack_system` (behavior)
  - Resources: `AIConfig` (optional, for global AI settings)
  - Events: `EnemyDetectedPlayer`, `EnemyLostTarget`, `EnemyAttackTriggered` (inter-system communication)
- [x] **60 FPS Performance**: AI updates allocated <2ms per frame budget (from project frame budget: AI & Dungeon Logic: 2.0ms). Perception checks can be throttled (every N frames) to reduce cost.
- [x] **Pixel Art Consistency**: No new visual assets required for MVP. Existing enemy sprites and animations used.
- [x] **Combat Mechanics Testing**: AI attack behavior requires unit tests (domain layer functions). Integration tests for full AI state machine workflows.
- [x] **Open Source MIT**: All dependencies (Bevy, rand, serde) are MIT or Apache-2.0 compatible.
- [x] **Modular Design**: Implemented as `EnemyAIPlugin` in `src/infrastructure/plugins/enemy.rs`, independent and testable.
- [x] **Language Separation**: Code uses English identifiers (`EnemyAI`, `AIState`, `Patrol`), documentation uses Chinese (spec.md, plan.md, doc comments).

### Performance Budget

Enemy AI system frame time allocation:
- Perception checks (distance, line of sight): <0.5ms (throttled to every 3-5 frames)
- State machine updates: <0.3ms
- Patrol behavior (random movement): <0.2ms
- Chase behavior (pathfinding/movement): <0.5ms
- Attack behavior (cooldown, animation triggers): <0.3ms
- Aggro system updates: <0.2ms
- **Total per frame**: <2.0ms (within AI & Dungeon Logic budget)

### Testing Requirements

- [x] Unit tests for domain layer (`src/domain/enemy/ai.rs`):
  - `should_transition_to_chase` function (perception check)
  - `should_transition_to_attack` function (attack range check)
  - `should_drop_aggro` function (aggro drop logic)
  - `calculate_patrol_target` function (random patrol point)
- [x] Integration tests for full workflows:
  - Patrol → Chase → Attack → Chase cycle
  - Aggro drop and return to patrol
  - Multiple enemies with different targets
- [x] Performance benchmarks for AI updates (`benches/enemy_ai_bench.rs`)
- [x] Tests written FIRST and verified to fail before implementation (TDD approach)

## Project Structure

### Documentation (this feature)

```text
specs/003-dungeon-system/sub-specs/004-enemy-ai/
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
├── main.rs              # 游戏入口点，注册 EnemyAIPlugin
├── lib.rs               # 库根，导出公共 API（用于测试）
│
├── domain/              # 领域层（Domain Layer） - 零 Bevy 依赖
│   ├── mod.rs
│   └── enemy/           # 敌人领域逻辑
│       ├── mod.rs
│       └── ai.rs        # AI 状态转换逻辑（纯函数）
│           # Functions:
│           # - should_transition_to_chase(distance, detection_range, has_los) -> bool
│           # - should_transition_to_attack(distance, attack_range, cooldown_ready) -> bool
│           # - should_drop_aggro(distance, aggro_drop_range, time_since_last_seen) -> bool
│           # - calculate_patrol_target(current_pos, patrol_radius) -> Position
│           # - calculate_chase_direction(enemy_pos, target_pos) -> Direction
│           # - check_line_of_sight(enemy_pos, target_pos, obstacles) -> bool
│
├── infrastructure/      # 基础设施层（Infrastructure Layer） - Bevy 桥接
│   ├── mod.rs
│   ├── plugins/         # Bevy 插件（系统注册、资源初始化）
│   │   ├── mod.rs
│   │   └── enemy.rs     # EnemyAIPlugin（注册所有 AI 系统）
│   ├── components/      # ECS 组件（纯数据结构）
│   │   ├── mod.rs
│   │   └── enemy.rs     # EnemyAI, AIState, Perception, AggroTarget, PatrolConfig
│   ├── systems/         # ECS 系统（行为函数，调用领域层）
│   │   ├── mod.rs
│   │   └── enemy.rs     # 所有敌人 AI 相关系统
│   │       # Systems:
│   │       # - perception_system (detect players, update perception state)
│   │       # - ai_state_machine_system (handle state transitions)
│   │       # - patrol_system (handle patrol behavior)
│   │       # - chase_system (handle chase behavior, update movement target)
│   │       # - attack_system (handle attack behavior, trigger attacks)
│   │       # - aggro_system (manage aggro list, target selection)
│   │       # - attack_cooldown_system (manage attack cooldowns)
│   │       # - return_to_patrol_system (handle aggro drop, return to spawn)
│   ├── resources/       # 全局游戏状态
│   │   ├── mod.rs
│   │   └── enemy.rs     # AIConfig (optional, for global AI settings)
│   └── events/          # 事件定义
│       ├── mod.rs
│       └── enemy.rs     # EnemyDetectedPlayer, EnemyLostTarget, EnemyAttackTriggered
│
tests/
├── integration/         # 完整系统测试
│   └── enemy/
│       ├── ai_state_machine_test.rs      # 完整 AI 状态机流程
│       └── multi_enemy_aggro_test.rs      # 多敌人仇恨系统测试
└── unit/                # 组件/系统单元测试
    └── enemy/
        └── ai_test.rs                    # AI 逻辑（领域层）

assets/
├── data/
│   └── enemies.ron      # 敌人 AI 配置（检测范围、攻击范围、速度等）
│
benches/                 # 性能基准测试
└── enemy_ai_bench.rs   # AI 更新性能测试
```

**Structure Decision**: 采用现有的 DDD 架构模式，与 `001-player-movement`、`002-combat-core` 和 `003-dungeon-system` 保持一致。领域层 (`src/domain/enemy/`) 包含纯函数，零 Bevy 依赖，便于测试和复用。基础设施层 (`src/infrastructure/`) 作为 Bevy ECS 桥接，处理组件、系统、资源和事件。注意：现有的 `src/infrastructure/systems/enemy.rs` 和 `src/infrastructure/components/enemy.rs` 已存在，需要扩展以支持完整的 AI 系统。

## Complexity Tracking

> **No violations - all principles upheld**

No complexity violations. The implementation follows established DDD patterns and Bevy ECS best practices. The AI system integrates with existing enemy components and systems, extending rather than replacing current functionality.


