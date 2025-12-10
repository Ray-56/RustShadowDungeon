# Implementation Plan: Boss Encounter System

**Branch**: `006-boss-encounter` | **Date**: 2025-12-03 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `specs/003-dungeon-system/sub-specs/006-boss-encounter/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

实现 Boss 遭遇战系统，包括多阶段 Boss AI、阶段转换机制、特殊技能系统和技能预警（Telegraph）系统。Boss 支持 3-5 个可配置阶段，每个阶段有不同的攻击模式和技能组合。系统基于 RON 配置文件进行数据驱动设计，建立在 Enemy AI 基础架构之上，复用 Combat Core 和 Loot System。

## Technical Context

**Language/Version**: Rust 1.75+ (stable)
**Primary Dependencies**: Bevy 0.12+, bevy_rapier2d (physics), bevy_ecs_ldtk (level loading), ron (configuration)
**Storage**: RON files for boss definitions (`assets/data/bosses.ron`), runtime-loaded configuration
**Testing**: cargo test, criterion for benchmarks
**Target Platform**: Windows/Linux/macOS desktop, potential WASM
**Project Type**: Single game project with plugin-based architecture
**Performance Goals**: 60 FPS (16.67ms frame budget), Boss AI + telegraph rendering <3ms per frame
**Constraints**: Pixel-perfect rendering, deterministic combat for replays, phase transitions must complete within 1 second
**Scale/Scope**: 
- Boss count: 1-5 bosses per dungeon (initial: 1 boss)
- Phase count: 3-5 phases per boss (configurable per boss)
- Skills per phase: 2-5 skills (configurable)
- Telegraph rendering: Real-time semi-transparent red gradient overlays with flashing animation

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### Core Principles Compliance

- [ ] **Rust Memory Safety**: No `unsafe` code required. All logic uses safe Rust patterns.
- [ ] **Bevy ECS Architecture**: All systems follow ECS patterns - BossController as component, systems for AI logic, phase transitions, telegraph rendering
- [ ] **60 FPS Performance**: Boss AI + phase transitions + telegraph rendering budget <3ms per frame (within combat logic budget)
- [ ] **Pixel Art Consistency**: Boss sprites follow 16x16 grid, phase transition animations pixel-aligned
- [ ] **Combat Mechanics Testing**: Unit tests planned for phase transitions, telegraph accuracy, lock-health mechanism, invulnerability frames
- [ ] **Open Source MIT**: All dependencies (Bevy, ron) are MIT-compatible
- [ ] **Modular Design**: Implemented as independent Bevy plugin (`BossPlugin`) with clear interfaces
- [ ] **Language Separation**: Code uses English identifiers, documentation uses Chinese

### Performance Budget

Boss encounter system frame time allocation:
- Boss AI processing: 1.5ms (state machine updates, skill selection)
- Phase transition logic: 0.5ms (threshold checks, state changes)
- Telegraph rendering: 1.0ms (semi-transparent overlay rendering)
- Skill cooldown/priority management: 0.2ms
- Total: <3.2ms (within combat logic budget of 3ms, slight overage acceptable for Boss-specific features)

### Testing Requirements

- [ ] Unit tests for phase transition logic (threshold detection, lock-health mechanism)
- [ ] Unit tests for telegraph accuracy (warning area matches damage area)
- [ ] Unit tests for invulnerability frames during phase transitions
- [ ] Integration tests for full Boss encounter (activation → phase transitions → defeat)
- [ ] Performance benchmarks for Boss AI loop
- [ ] Tests written FIRST and verified to fail before implementation

## Project Structure

### Documentation (this feature)

```text
specs/003-dungeon-system/sub-specs/006-boss-encounter/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
src/
├── domain/
│   └── boss/
│       ├── mod.rs
│       ├── phase_transition.rs    # Pure phase transition logic (thresholds, state changes)
│       ├── skill_priority.rs      # Skill cooldown and priority queue logic
│       └── telegraph.rs           # Telegraph area calculations (pure math)
├── plugins/
│   ├── mod.rs
│   └── boss.rs                    # BossPlugin - registers all Boss systems and resources
├── components/
│   ├── mod.rs
│   └── boss.rs                    # BossController, BossPhase, BossSkill, Telegraph
├── systems/
│   ├── mod.rs
│   └── boss_systems.rs            # Boss AI systems, phase transitions, telegraph rendering
├── resources/
│   ├── mod.rs
│   └── boss_config.rs             # Loaded Boss definitions from RON files
└── events/
    ├── mod.rs
    └── boss.rs                    # BossEncounterStarted, BossPhaseTransition, BossDefeated

tests/
├── integration/
│   └── boss_encounter_test.rs     # Full Boss encounter flow test
└── unit/
    ├── phase_transition_test.rs   # Phase transition logic tests
    ├── telegraph_test.rs          # Telegraph accuracy tests
    └── lock_health_test.rs        # Lock-health mechanism tests

assets/
└── data/
    └── bosses.ron                 # Boss definitions (phases, skills, thresholds)

benches/
└── boss_ai_bench.rs              # Boss AI performance benchmarks
```

**Structure Decision**: 

采用 DDD 分层架构，遵循项目已有的架构模式：
- **Domain Layer** (`src/domain/boss/`): 纯 Rust 函数，无 Bevy 依赖。包含阶段转换逻辑、技能优先级队列、预警区域计算等纯逻辑。
- **Infrastructure Layer**: Bevy ECS 组件、系统、插件，作为领域逻辑与引擎之间的桥梁。
- **Configuration Layer**: RON 配置文件，在运行时加载。

Boss 系统作为独立的 Bevy 插件 (`BossPlugin`)，与 Enemy AI 系统解耦，但可以复用其基础架构。

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| N/A | No violations | All principles followed |

---

## Phase 0 & Phase 1 Completion Status

✅ **Phase 0: Research Complete**
- `research.md` generated with all technical decisions
- All unknowns resolved (Boss AI architecture, Telegraph rendering, phase transitions, RON config)

✅ **Phase 1: Design Complete**
- `data-model.md` generated with complete ECS component definitions
- `contracts/system-interfaces.md` generated with system integration contracts
- `quickstart.md` generated with developer quickstart guide

**Generated Artifacts**:
- ✅ `plan.md` (this file)
- ✅ `research.md`
- ✅ `data-model.md`
- ✅ `contracts/system-interfaces.md`
- ✅ `quickstart.md`

**Next Steps**: Run `/speckit.tasks` to generate task breakdown for implementation.

