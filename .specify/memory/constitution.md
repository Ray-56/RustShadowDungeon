<!--
Sync Impact Report:
Version: 1.0.0 → 1.0.1
Modified Principles: None
Added Sections:
  - Principle VIII: Language Separation Rule (Chinese/English strict separation)
Removed Sections: None
Templates Requiring Updates:
  ✅ plan-template.md - updated with language separation guidelines
  ✅ spec-template.md - updated with language separation guidelines
  ✅ tasks-template.md - updated with language separation guidelines
  ✅ .specify/specs/00-project-overview/spec.md - updated to reference v1.0.1
Follow-up TODOs: None
Note: User explicitly requested v1.0.1 (PATCH) rather than v1.1.0 (MINOR per semver).
-->

# RustShadowDungeon Constitution

## Core Principles

### I. Rust Memory Safety (NON-NEGOTIABLE)

All code MUST leverage Rust's ownership system and compile without `unsafe` blocks unless:
- Absolutely required for FFI or low-level optimization
- Documented with detailed safety rationale and invariants
- Reviewed and approved with justification in code comments
- Accompanied by comprehensive unit tests validating safety boundaries

**Rationale**: Memory safety is foundational to game stability. Crashes and undefined behavior destroy player experience. Rust's compile-time guarantees eliminate entire classes of bugs that plague C++ game engines.

### II. Bevy ECS Architecture (NON-NEGOTIABLE)

All game logic MUST follow Bevy's Entity Component System patterns:
- Systems operate on queries of components, never direct entity references
- Components are pure data structures (no behavior)
- Systems contain behavior and are composable
- Resources for global state, sparingly used
- Events for inter-system communication
- Plugins for feature organization

**Rationale**: ECS architecture enables parallelization, testability, and maintainability. Fighting the ECS pattern leads to unmaintainable code and performance bottlenecks.

### III. 60 FPS Performance Target (MANDATORY)

Game MUST maintain 60 FPS (16.67ms frame budget) on target hardware:
- Frame time monitoring in development builds
- Performance budgets allocated per system category:
  - Input processing: <1ms
  - Physics/collision: <3ms
  - Combat logic: <3ms
  - Rendering preparation: <4ms
  - Remaining: <5.67ms buffer
- Profiling required before optimizing
- Performance regression tests for critical paths

**Rationale**: DNF-style action combat demands responsive controls. Frame drops break combo timing and player muscle memory, making the game feel unresponsive.

### IV. Pixel Art Consistency (MANDATORY)

All visual assets MUST adhere to unified pixel art standards:
- Fixed pixel grid: 16x16 base unit
- Consistent color palette across all sprites
- No rotation/scaling of pixel art (causes blurring)
- Camera locked to pixel grid for crisp rendering
- Sprite sheet organization documented
- Animation frame timing documented

**Rationale**: Visual consistency defines game identity. Mismatched pixel densities or blurred sprites break immersion and appear unprofessional.

### V. Combat Mechanics Testing (NON-NEGOTIABLE)

Every combat system MUST have comprehensive unit tests:
- Damage calculations (base, modifiers, critical hits)
- Hit detection and collision
- Status effects (application, duration, stacking)
- Skill cooldowns and resource costs
- Combo systems and canceling mechanics
- Invincibility frames (i-frames)

Tests MUST be written FIRST, verified to fail, then implemented (TDD).

**Rationale**: Combat is the core gameplay loop. Untested combat leads to game-breaking exploits, inconsistent feel, and impossible balancing. Bugs in damage calculation or hit detection destroy competitive integrity.

### VI. Open Source MIT License (MANDATORY)

Project MUST maintain MIT license terms:
- All original code under MIT license
- Third-party dependencies compatible with MIT
- Asset licenses clearly documented (MIT or compatible)
- Copyright notices preserved
- License file at repository root

**Rationale**: MIT license maximizes community contribution potential and downstream use. Open source accelerates debugging, attracts contributors, and builds trust.

### VII. Modular Design (MANDATORY)

Game systems MUST be independently testable and loosely coupled:
- Each major system as a separate Bevy plugin
- Clear interfaces between systems (events/resources)
- No circular dependencies
- Systems testable without full game context
- Mock components for isolated testing

**Rationale**: Modularity enables parallel development, easier debugging, and cleaner tests. Tightly coupled systems become unmaintainable as the game grows.

### VIII. Language Separation Rule (NON-NEGOTIABLE)

All human-readable content MUST use Chinese, all code-layer content MUST use English:

**中文使用场景（Chinese Usage）**:
- 对话、计划文档（conversations, planning documents）
- 规范说明书（specifications, spec.md）
- README、文档（README, documentation）
- Commit messages、GitHub Issues/PR
- 文档注释（`///` 或 `/** */` doc comments）
- 里程碑名称（milestone names）
- 任务描述（task descriptions）
- 测试用例名称（test case names）

**英文使用场景（English Usage）**:
- 类型名、函数名、变量名（type names, function names, variable names）
- 模块名、文件名、crate 名称（module names, file names, crate names）
- 代码内注释（`//` 或 `/* */` inline comments）- 优先英文，关键逻辑可用中文补充
- 错误信息、日志输出（error messages, log output）
- 配置文件键名（configuration keys）
- Cargo.toml、mod.rs、pub API 中的所有标识符

**严格禁止（Strictly Prohibited）**:
- 中英混杂命名（Mixed naming）: `PlayerController_玩家控制器.rs` ❌
- 中文出现在 Cargo.toml、mod.rs、pub API ❌
- 英文出现在用户可见文档（中文项目）❌

**示例（Examples）**:

```rust
// ✅ GOOD: English code, Chinese doc comments
/// 玩家组件 - 存储玩家的基本属性
/// Player component - stores player's basic attributes
#[derive(Component)]
pub struct Player {
    pub health: f32,
    pub max_health: f32,
}

// ✅ GOOD: Chinese commit message
// git commit -m "feat: 添加玩家生命值系统"

// ❌ BAD: Mixed naming
pub struct 玩家Controller { } // WRONG
pub struct PlayerController玩家 { } // WRONG

// ❌ BAD: Chinese in error messages (breaks international community)
panic!("玩家生命值不能为负"); // WRONG - use English
panic!("Player health cannot be negative"); // CORRECT
```

**Rationale**: 保持中文沟通最高效率（团队协作、需求讨论、文档理解），同时保证代码在国际开源社区、AI 工具（GitHub Copilot、LLM）、搜索引擎、IDE 中最高可读性与复用性。中英严格分离避免命名混乱，降低维护成本。

## Performance Standards

### Frame Budget Enforcement

- CI pipeline runs performance benchmarks on critical systems
- PRs rejected if they cause >10% regression in any benchmark
- Frame time displayed in development builds
- Profiling data captured for slow frames (>20ms)

### Asset Loading

- Async loading for all non-critical assets
- Loading screens for level transitions
- Asset streaming for large dungeons
- Memory budget: <1GB for base game on target hardware

### Network Performance (if multiplayer)

- Client-side prediction for local player
- Server authoritative for combat results
- <100ms input latency target
- Rollback netcode for combat synchronization

## AI Development Rules

### Specification-Driven Development (MANDATORY)

AI-assisted development MUST follow this workflow:

1. **Specification First**: Feature specified in `/specs/[feature]/spec.md` before any code
2. **Test Generation**: AI generates tests that verify specification
3. **User Approval**: User reviews and approves tests before implementation
4. **Red-Green-Refactor**: Tests fail → Implementation → Tests pass → Refactor
5. **Constitution Compliance**: AI validates all changes against this constitution

### AI Code Generation Standards

AI-generated code MUST:
- Include comprehensive inline documentation
- Follow Rust idioms (no Java/C++ patterns)
- Respect Bevy ECS patterns (no OOP anti-patterns)
- Include error handling (no unwrap/expect without justification)
- Use descriptive variable names (no single-letter except iterators)
- Provide unit tests alongside implementation
- **Follow Language Separation Rule (Principle VIII)**: English code, Chinese documentation

### AI Prohibited Actions

AI MUST NOT:
- Generate `unsafe` code without explicit user request and justification
- Bypass type system with excessive `Any` trait usage
- Create tightly coupled systems that bypass ECS patterns
- Ignore performance budgets
- Skip test generation phase
- Commit code that doesn't compile or pass existing tests
- Violate Language Separation Rule (Chinese in code identifiers or English in Chinese docs)

### AI Review Checklist

Before submitting generated code, AI MUST verify:
- [ ] Compiles without warnings on stable Rust
- [ ] All tests pass (existing + new)
- [ ] Follows Bevy ECS patterns
- [ ] Meets performance budget (if performance-critical)
- [ ] Documented (module-level + public API)
- [ ] No unsafe code (or justified and documented)
- [ ] Constitution principles upheld
- [ ] Language Separation Rule enforced (English code, Chinese docs)

## Governance

### Amendment Procedure

1. Propose amendment with rationale in GitHub issue
2. Discuss impact on existing code and specifications
3. Update constitution with version bump
4. Update all dependent templates and documentation
5. Merge via PR with explicit constitution change label

### Versioning Policy

Constitution follows semantic versioning:
- **MAJOR**: Breaking changes to core principles (requires codebase audit)
- **MINOR**: New principles or substantial expansions
- **PATCH**: Clarifications, typo fixes, non-semantic changes

### Compliance Review

- All PRs MUST pass constitution compliance check
- Violations require explicit justification and mitigation plan
- Technical debt tracked and scheduled for resolution
- Quarterly review of compliance and principle effectiveness

### Non-Negotiable vs. Mandatory

- **NON-NEGOTIABLE**: Violations NEVER accepted, block all PRs
- **MANDATORY**: Required but may have approved exceptions with strong justification

**Version**: 1.0.1 | **Ratified**: 2025-11-24 | **Last Amended**: 2025-11-24
