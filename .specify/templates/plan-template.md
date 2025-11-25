# Implementation Plan: [FEATURE]

**Branch**: `[###-feature-name]` | **Date**: [DATE] | **Spec**: [link]
**Input**: Feature specification from `/specs/[###-feature-name]/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

[Extract from feature spec: primary requirement + technical approach from research]

## Technical Context

<!--
  ACTION REQUIRED: Replace the content in this section with the technical details
  for the project. The structure here is presented in advisory capacity to guide
  the iteration process.
-->

**Language/Version**: Rust 1.75+ (stable)
**Primary Dependencies**: Bevy 0.12+, bevy_rapier2d (physics), bevy_ecs_ldtk (level loading)
**Storage**: RON files for game data, SQLite for save games (optional)
**Testing**: cargo test, criterion for benchmarks
**Target Platform**: Windows/Linux/macOS desktop, potential WASM
**Project Type**: Single game project with plugin-based architecture
**Performance Goals**: 60 FPS (16.67ms frame budget), <1GB memory
**Constraints**: Pixel-perfect rendering, deterministic combat for replays
**Scale/Scope**: [Specify dungeon count, skill count, enemy types, etc. or NEEDS CLARIFICATION]

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### Core Principles Compliance

- [ ] **Rust Memory Safety**: Does this feature require `unsafe` code? If yes, document justification
- [ ] **Bevy ECS Architecture**: Are all systems following ECS patterns (components = data, systems = behavior)?
- [ ] **60 FPS Performance**: Does this feature fit within performance budget? Which systems affected?
- [ ] **Pixel Art Consistency**: Are new assets following 16x16 grid and color palette?
- [ ] **Combat Mechanics Testing**: If combat-related, are unit tests planned for all mechanics?
- [ ] **Open Source MIT**: Are all dependencies MIT-compatible?
- [ ] **Modular Design**: Is this feature implemented as independent Bevy plugin(s)?
- [ ] **Language Separation**: Code uses English identifiers, documentation uses Chinese (for Chinese projects)

### Performance Budget

If performance-critical feature, allocate frame time budget:
- Input processing: ___ms
- Physics/collision: ___ms
- Combat logic: ___ms
- Rendering: ___ms
- Total: <16.67ms

### Testing Requirements

- [ ] Unit tests for all combat mechanics (damage, hit detection, status effects)
- [ ] Integration tests for system interactions
- [ ] Performance benchmarks for critical paths
- [ ] Tests written FIRST and verified to fail before implementation

## Project Structure

### Documentation (this feature)

```text
specs/[###-feature]/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)
<!--
  ACTION REQUIRED: Replace the placeholder tree below with the concrete layout
  for this feature. Delete unused options and expand the chosen structure with
  real paths (e.g., apps/admin, packages/something). The delivered plan must
  not include Option labels.
-->

```text
# Rust + Bevy Game Project Structure
src/
├── main.rs              # Game entry point
├── lib.rs               # Library root (for testing)
├── plugins/             # Bevy plugins (one per major system)
│   ├── mod.rs
│   ├── player.rs        # Player movement, input, state
│   ├── combat.rs        # Combat system, damage, hit detection
│   ├── enemy.rs         # Enemy AI and behavior
│   ├── skills.rs        # Skill system, cooldowns, effects
│   └── ui.rs            # UI systems
├── components/          # ECS components (pure data)
│   ├── mod.rs
│   ├── player.rs
│   ├── combat.rs
│   └── physics.rs
├── systems/             # ECS systems (behavior)
│   ├── mod.rs
│   ├── movement.rs
│   ├── collision.rs
│   └── animation.rs
├── resources/           # Global game state
│   ├── mod.rs
│   ├── game_state.rs
│   └── asset_handles.rs
└── events/              # Event definitions
    ├── mod.rs
    ├── combat.rs
    └── input.rs

tests/
├── integration/         # Full system tests
│   ├── combat_test.rs
│   └── player_test.rs
└── unit/                # Component/system unit tests
    ├── damage_test.rs
    └── collision_test.rs

assets/
├── sprites/             # Pixel art assets (16x16 grid)
├── audio/
├── levels/              # LDtk or Tiled level files
└── data/                # RON config files

benches/                 # Performance benchmarks
└── combat_bench.rs

Cargo.toml               # Dependencies and metadata
```

**Structure Decision**: [Document the selected structure and reference the real
directories captured above]

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| [e.g., 4th project] | [current need] | [why 3 projects insufficient] |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient] |
