# Implementation Plan: 战斗系统核心 (Combat System Core)

**Branch**: `002-combat-core` | **Date**: 2025-11-25 | **Spec**: [spec.md](./spec.md)  
**Milestone**: M2 - 战斗系统基础（3周）  
**Constitution**: v1.0.1

---

## Summary

实现 DNF 风格的战斗系统核心，包括纯函数伤害计算、HitBox/HurtBox 碰撞检测、3连击系统、打击感反馈（hitfreeze、屏幕震动、粒子）、基础技能系统和史莱姆敌人。核心架构约束：领域层零 Bevy 依赖，测试覆盖率 ≥85%，60 FPS 性能目标。

**技术方案**（from research.md）:
- **碰撞检测**: AABB 算法 + bevy_rapier2d 空间分区优化
- **粒子系统**: bevy_hanabi（GPU 粒子）或自建 CPU 粒子系统（性能权衡）
- **打击定格**: Time dilation 系统（全局时间缩放）
- **伤害计算**: 纯函数（零 Bevy 依赖），支持元素、暴击、防御减伤

---

## Technical Context

**Language/Version**: Rust 1.91.1 (stable)

**Primary Dependencies**:
- **Bevy**: 0.17.0（游戏引擎，ECS 架构）
- **bevy_rapier2d**: 0.29+（2D 物理引擎，用于碰撞检测和空间分区）
- **bevy_kira_audio**: latest（音频系统，支持多音效同时播放）
- **bevy_hanabi**: latest（可选，GPU 粒子系统，性能优化）
- **serde + ron**: latest（数据序列化，技能/敌人配置）
- **rand**: latest（随机数生成，暴击计算）

**Storage**: 
- RON 文件用于游戏数据（`assets/data/skills.ron`, `enemies.ron`）
- 无持久化存储（单机战斗系统）

**Testing**: 
- `cargo test`（单元测试 + 集成测试）
- `criterion`（性能基准测试，验证 <3ms 预算）
- `cargo-tarpaulin` 或 `cargo-llvm-cov`（测试覆盖率报告）

**Target Platform**: 
- Windows/Linux/macOS desktop（Tier 1，60 FPS）
- WASM（Tier 1，55 FPS acceptable）
- Android（Tier 1，60 FPS on Snapdragon 750G+）

**Project Type**: 单一游戏项目，基于 Bevy 插件架构

**Performance Goals**:
- **60 FPS**（16.67ms 帧预算）
- **战斗逻辑 <3ms**（帧预算 18%）
- **碰撞检测 <3ms**（帧预算 18%）
- **内存占用 <50MB**（仅战斗系统增量）

**Constraints**:
- **零 unsafe 代码**（领域层和基础设施层，除非有充分理由）
- **领域层零 Bevy 依赖**（`src/domain/combat/` 纯函数）
- **像素完美渲染**（16×16 网格，无旋转缩放）
- **确定性战斗**（相同输入 → 相同输出，便于 replay 系统）

**Scale/Scope**:
- **敌人类型**: 1 种（史莱姆）
- **技能数量**: 1-2 个（火球术为主）
- **连击种类**: 1 种（3 连击：轻-轻-重）
- **状态效果**: 无敌帧（简化版状态效果）
- **元素类型**: 4 种（Physical, Fire, Ice, Lightning）

---

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### Core Principles Compliance

- [x] **Rust Memory Safety**: 不需要 `unsafe` 代码。伤害计算为纯函数，碰撞检测使用 bevy_rapier2d（内部已处理 unsafe）
- [x] **Bevy ECS Architecture**: 所有系统遵循 ECS 模式：
  - 组件纯数据（`HitBox`, `HurtBox`, `Health`, `Combo`, `Skill`, `Invincibility`）
  - 系统纯行为（`collision_detection_system`, `apply_damage_system`, `combo_system`）
  - 事件通信（`DamageDealt`, `ComboExtended`, `EnemyDefeated`, `SkillActivated`）
- [x] **60 FPS Performance**: 性能预算已分配（见下方）
- [x] **Pixel Art Consistency**: 新资产遵循 16×16 网格（史莱姆 16×16，粒子 8×8）
- [x] **Combat Mechanics Testing**: 50+ 测试用例已在 spec.md 定义，≥85% 覆盖率目标
- [x] **Open Source MIT**: 所有依赖均为 MIT 或 Apache-2.0 兼容许可证
- [x] **Modular Design**: 战斗系统作为独立 Bevy 插件（`CombatPlugin`, `SkillPlugin`, `EnemyPlugin`）
- [x] **Language Separation**: 代码使用英文标识符，文档使用中文

### Performance Budget

战斗系统占用帧预算分配（基于项目规范 Section 5）：

| 系统分类 | 帧预算 (ms) | 帧预算 (%) | 监控系统 |
|---------|------------|-----------|---------|
| **碰撞检测** | 3.0 | 18% | `collision_detection_system`, `hitbox_cleanup_system` |
| **战斗逻辑** | 2.5 | 15% | `apply_damage_system`, `combo_system`, `status_effect_system` |
| **动画与状态** | 1.5 | 9% | `attack_animation_system`, `hit_reaction_system` |
| **粒子与反馈** | 1.0 | 6% | `particle_system`, `hitfreeze_system`, `screen_shake_system` |
| **音频触发** | 0.5 | 3% | `combat_audio_system` |
| **战斗总计** | **8.5** | **51%** | 所有战斗相关系统 |

**剩余预算**: 8.17ms（49%）用于渲染、输入、其他系统

**性能优化策略**:
1. 使用 bevy_rapier2d 的 BroadPhase（宽相位）进行空间分区，减少碰撞检测对数
2. HitBox 生命周期限制（最多存活 10 帧），自动清理
3. 粒子池化（对象池模式），避免频繁 spawn/despawn
4. 伤害计算为纯函数，可内联优化（`#[inline]`）
5. 限制同时存在的 HitBox 数量 (<50)

### Testing Requirements

- [x] **单元测试**（领域层）:
  - `tests/unit/combat/damage_test.rs` - 伤害计算（≥90% 覆盖率）
  - `tests/unit/combat/combo_test.rs` - 连击逻辑（≥85% 覆盖率）
  - `tests/unit/combat/collision_test.rs` - 碰撞检测辅助函数（≥80% 覆盖率）

- [x] **集成测试**（系统交互）:
  - `tests/integration/combat_flow_test.rs` - 完整战斗流程（攻击 → 伤害 → 死亡）
  - `tests/integration/combo_flow_test.rs` - 3 连击流程
  - `tests/integration/skill_flow_test.rs` - 技能施放流程

- [x] **性能基准测试**:
  - `benches/combat_bench.rs` - 战斗逻辑性能（目标 <2.5ms）
  - `benches/collision_bench.rs` - 碰撞检测性能（目标 <3ms，15 敌人场景）

- [x] **TDD 工作流**: 测试先行，验证失败 → 实现 → 测试通过 → 重构

---

## Project Structure

### Documentation (this feature)

```text
.specify/specs/02-combat/
├── plan.md              # 本文件（技术实施计划）
├── spec.md              # 功能规范（6 个 User Stories，50 个需求）
├── research.md          # Phase 0 输出（技术研究与决策）
├── data-model.md        # Phase 1 输出（战斗实体数据模型）
├── quickstart.md        # Phase 1 输出（开发者快速开始指南）
├── contracts/           # Phase 1 输出（可选，事件契约定义）
├── tasks.md             # Phase 2 输出（任务分解，运行 /speckit.tasks）
├── checklists/
│   └── requirements.md  # 规范质量检查清单（22/22 通过）
└── README.md            # 快速开始指南
```

### Source Code (repository root)

基于现有项目结构扩展：

```text
src/
├── main.rs                      # 游戏入口（已存在）
├── lib.rs                       # 库根（已存在）
│
├── domain/                      # ✅ 领域层（纯函数，零 Bevy 依赖）
│   ├── mod.rs
│   ├── movement/                # 已存在（M1）
│   └── combat/                  # ✅ 新增（M2）
│       ├── mod.rs
│       ├── damage.rs            # 伤害计算纯函数
│       ├── combo.rs             # 连击逻辑纯函数
│       ├── collision.rs         # 碰撞检测辅助函数（AABB）
│       └── element.rs           # 元素类型与计算
│
├── infrastructure/              # ✅ 基础设施层（Bevy 集成）
│   ├── mod.rs
│   │
│   ├── components/              # ECS 组件（纯数据）
│   │   ├── mod.rs
│   │   ├── player.rs            # 已存在（M1）
│   │   ├── health.rs            # 已存在，需扩展
│   │   ├── enemy.rs             # 已存在，需扩展
│   │   └── combat.rs            # ✅ 新增（M2）
│   │       ├── HitBox           # 攻击判定框
│   │       ├── HurtBox          # 受击判定框
│   │       ├── Combo            # 连击状态
│   │       ├── Skill            # 技能数据
│   │       ├── Invincibility    # 无敌帧
│   │       └── AttackAnimation  # 攻击动画状态
│   │
│   ├── events/                  # Bevy 事件
│   │   ├── mod.rs
│   │   ├── movement.rs          # 已存在（M1）
│   │   └── combat.rs            # ✅ 新增（M2）
│   │       ├── DamageDealt      # 伤害造成事件
│   │       ├── ComboExtended    # 连击延续事件
│   │       ├── EnemyDefeated    # 敌人死亡事件
│   │       └── SkillActivated   # 技能激活事件
│   │
│   ├── systems/                 # ECS 系统（行为）
│   │   ├── mod.rs
│   │   ├── movement.rs          # 已存在（M1）
│   │   ├── combat.rs            # 已存在，需扩展
│   │   ├── collision.rs         # ✅ 新增（M2）
│   │   │   ├── collision_detection_system
│   │   │   └── hitbox_cleanup_system
│   │   ├── damage.rs            # ✅ 新增（M2）
│   │   │   ├── apply_damage_system
│   │   │   └── death_system
│   │   ├── combo.rs             # ✅ 新增（M2）
│   │   │   ├── combo_system
│   │   │   └── combo_ui_system
│   │   ├── feedback.rs          # ✅ 新增（M2）
│   │   │   ├── hitfreeze_system
│   │   │   ├── screen_shake_system
│   │   │   └── damage_number_system
│   │   ├── skill.rs             # ✅ 新增（M2）
│   │   │   ├── skill_input_system
│   │   │   ├── skill_cooldown_system
│   │   │   └── projectile_system
│   │   ├── invincibility.rs     # ✅ 新增（M2）
│   │   │   ├── invincibility_timer_system
│   │   │   └── invincibility_flash_system
│   │   └── particles.rs         # ✅ 新增（M2）
│   │       └── hit_particle_system
│   │
│   ├── plugins/                 # Bevy 插件（系统注册）
│   │   ├── mod.rs
│   │   ├── player.rs            # 已存在（M1）
│   │   ├── physics.rs           # 已存在（M1）
│   │   ├── combat.rs            # ✅ 新增（M2）
│   │   │   └── CombatPlugin     # 注册战斗系统
│   │   ├── skill.rs             # ✅ 新增（M2）
│   │   │   └── SkillPlugin      # 注册技能系统
│   │   └── enemy.rs             # ✅ 新增（M2）
│   │       └── EnemyPlugin      # 注册敌人系统
│   │
│   └── resources/               # 全局资源
│       ├── mod.rs
│       ├── movement_config.rs   # 已存在（M1）
│       ├── combat_config.rs     # ✅ 新增（M2）
│       │   └── CombatConfig     # 战斗配置（伤害倍率、连击窗口）
│       ├── skill_database.rs    # ✅ 新增（M2）
│       │   └── SkillDatabase    # 技能数据库（从 RON 加载）
│       ├── enemy_database.rs    # ✅ 新增（M2）
│       │   └── EnemyDatabase    # 敌人数据库（从 RON 加载）
│       └── hitfreeze.rs         # ✅ 新增（M2）
│           └── HitfreezeTimer   # 全局打击定格计时器
│
tests/
├── unit/                        # 单元测试（领域层）
│   ├── combat/                  # ✅ 新增（M2）
│   │   ├── damage_test.rs       # 伤害计算测试
│   │   ├── combo_test.rs        # 连击逻辑测试
│   │   └── collision_test.rs    # 碰撞检测测试
│   └── ...
│
├── integration/                 # 集成测试（系统交互）
│   ├── combat_flow_test.rs     # ✅ 新增（M2）
│   ├── combo_flow_test.rs      # ✅ 新增（M2）
│   ├── skill_flow_test.rs      # ✅ 新增（M2）
│   └── ...
│
benches/                         # 性能基准测试
├── combat_bench.rs              # ✅ 新增（M2）
└── collision_bench.rs           # ✅ 新增（M2）
│
assets/
├── sprites/                     # 像素艺术资产
│   ├── player_attack.png        # ✅ 新增（M2）- 玩家攻击动画
│   ├── enemy_slime.png          # ✅ 新增（M2）- 史莱姆敌人
│   ├── hit_effect.png           # ✅ 新增（M2）- 打击特效粒子
│   └── skill_fireball.png       # ✅ 新增（M2）- 火球技能
│
├── audio/                       # 音频资产
│   ├── hit_light.ogg            # ✅ 新增（M2）- 轻击音效
│   ├── hit_heavy.ogg            # ✅ 新增（M2）- 重击音效
│   ├── hit_critical.ogg         # ✅ 新增（M2）- 暴击音效
│   └── skill_fireball.ogg       # ✅ 新增（M2）- 火球施法音效
│
└── data/                        # 数据配置（RON 格式）
    ├── skills.ron               # ✅ 新增（M2）- 技能配置
    ├── enemies.ron              # ✅ 新增（M2）- 敌人配置
    └── combat_config.ron        # ✅ 新增（M2）- 战斗配置
│
Cargo.toml                       # 依赖配置（需更新）
```

**Structure Decision**: 
- **领域层与基础设施层分离**：`src/domain/combat/`（纯函数）与 `src/infrastructure/`（Bevy 集成）严格分离，符合 Constitution Principle II 和 VII
- **插件化组织**：每个主要系统（战斗、技能、敌人）作为独立 Bevy 插件，便于测试和维护
- **测试按类型分离**：单元测试（领域层）、集成测试（系统交互）、基准测试（性能验证）
- **资产按类型组织**：精灵、音频、数据配置分别放在 `assets/` 子目录

---

## Complexity Tracking

无宪法违规需要辩解。

---

## Phase 0: Research & Technical Decisions

详见 [research.md](./research.md)

**关键技术决策**:
1. **碰撞检测算法**: AABB（轴对齐包围盒）+ bevy_rapier2d 空间分区
2. **粒子系统**: 自建 CPU 粒子系统（简单、跨平台兼容）
3. **打击定格**: Time dilation 系统（全局时间缩放，`Time<Virtual>`）
4. **伤害计算**: 纯函数，支持元素、暴击、防御减伤
5. **技能弹道**: 物理弹道（velocity + bevy_rapier2d），非插值

---

## Phase 1: Design & Contracts

### Data Model

详见 [data-model.md](./data-model.md)

**核心实体**:
- **DamageResult**: 伤害计算结果（pure data）
- **HitBox**: 攻击判定框（Component）
- **HurtBox**: 受击判定框（Component）
- **Combo**: 连击状态（Component）
- **Skill**: 技能数据（Component）
- **Invincibility**: 无敌帧（Component）

### Event Contracts

详见 [contracts/](./contracts/)

**战斗事件**:
- `DamageDealt`: 伤害造成（attacker, target, damage, is_critical）
- `ComboExtended`: 连击延续（player, combo_count, new_state）
- `EnemyDefeated`: 敌人死亡（enemy, position, loot_table）
- `SkillActivated`: 技能激活（player, skill_id, target_position）

### Quickstart Guide

详见 [quickstart.md](./quickstart.md)

**开发者快速开始**:
1. 克隆项目，安装 Rust 1.91.1
2. 运行 `cargo test` 验证环境
3. 阅读 `data-model.md` 理解实体关系
4. 按照 `tasks.md` 顺序实施（Week 1: 伤害+碰撞，Week 2: 连击+打击感，Week 3: 技能+敌人）

---

## Phase 2: Task Breakdown

运行 `/speckit.tasks` 生成详细任务列表。

**预期任务结构**:
- **Setup Phase**: 项目结构创建、依赖配置
- **Phase 1 (User Story 1)**: 基础攻击与伤害
- **Phase 2 (User Story 2)**: 3 连击系统
- **Phase 3 (User Story 3)**: 打击感反馈
- **Phase 4 (User Story 4)**: 基础技能系统
- **Phase 5 (User Story 6)**: 无敌帧系统
- **Polish Phase**: 性能优化、测试补全

---

## Implementation Timeline

根据项目计划（M2 - 3周）：

### Week 1: 伤害计算与碰撞检测（领域层基础）

**目标**: 建立战斗系统的数学和物理基础

- [ ] 实现领域层伤害计算（`src/domain/combat/damage.rs`）
- [ ] 实现 AABB 碰撞检测辅助函数（`src/domain/combat/collision.rs`）
- [ ] 实现 HitBox / HurtBox 组件（`src/infrastructure/components/combat.rs`）
- [ ] 实现碰撞检测系统（`src/infrastructure/systems/collision.rs`）
- [ ] 实现伤害应用系统（`src/infrastructure/systems/damage.rs`）
- [ ] 编写单元测试（`tests/unit/combat/damage_test.rs`，≥90% 覆盖率）
- [ ] 编写集成测试（`tests/integration/combat_flow_test.rs`）
- [ ] **验收**: 玩家可以攻击并造成伤害，史莱姆死亡

**交付物**: User Story 1（基础攻击与伤害）完成

---

### Week 2: 连击系统与打击感反馈（核心战斗体验）

**目标**: 实现 DNF 风格的战斗手感

- [ ] 实现连击逻辑（`src/domain/combat/combo.rs`）
- [ ] 实现 Combo 组件（`src/infrastructure/components/combat.rs`）
- [ ] 实现连击系统（`src/infrastructure/systems/combo.rs`）
- [ ] 实现连击 UI（连击计数器）
- [ ] 实现 Hitfreeze 系统（`src/infrastructure/systems/feedback.rs`）
- [ ] 实现屏幕震动系统
- [ ] 实现粒子系统（CPU 粒子）
- [ ] 实现伤害数字系统
- [ ] 实现音效触发系统
- [ ] 编写单元测试（`tests/unit/combat/combo_test.rs`）
- [ ] 编写集成测试（`tests/integration/combo_flow_test.rs`）
- [ ] **验收**: 打击感通过测试玩家评审（≥7/10）

**交付物**: User Story 2（3连击系统）+ User Story 3（打击感反馈）完成

---

### Week 3: 技能系统与敌人（完整战斗循环）

**目标**: 丰富战斗内容，完成可玩演示

- [ ] 实现技能数据结构（`src/infrastructure/components/combat.rs`）
- [ ] 实现技能系统（冷却、MP 消耗）（`src/infrastructure/systems/skill.rs`）
- [ ] 实现火球术技能（弹道、爆炸）
- [ ] 实现技能 UI（冷却显示）
- [ ] 实现史莱姆敌人组件和系统（`src/infrastructure/plugins/enemy.rs`）
- [ ] 实现生命值条 UI
- [ ] 实现受击动画和死亡动画
- [ ] 实现无敌帧系统（`src/infrastructure/systems/invincibility.rs`）
- [ ] 编写单元测试（`tests/unit/combat/skill_test.rs`, `invincibility_test.rs`）
- [ ] 编写集成测试（`tests/integration/skill_flow_test.rs`）
- [ ] 性能优化（60 FPS 验证，15 敌人场景）
- [ ] 性能基准测试（`benches/combat_bench.rs`, `collision_bench.rs`）
- [ ] **验收**: 玩家可以用攻击和技能击败史莱姆，60 FPS 稳定

**交付物**: User Story 4（技能系统）+ User Story 6（无敌帧）完成

---

## Risk Management

根据项目计划（M2 风险点）：

| 风险 | 可能性 | 影响 | 缓解措施 |
|------|--------|------|---------|
| **打击感调优耗时** | 高 | 中 | 使用 DNF 的打击感参数作为基准（hitfreeze 3帧，震动强度 0.2），快速迭代测试 |
| **碰撞检测性能** | 中 | 高 | 使用 bevy_rapier2d 的空间分区，限制 HitBox 数量（<50 同时存在），性能基准测试验证 |
| **连击系统复杂** | 中 | 中 | 先实现固定 3 连击，复杂连击链后续版本再做（US5 技能取消为 P3） |
| **数值平衡困难** | 低 | 中 | 使用 RON 配置文件，方便快速调整伤害、冷却等数值 |

---

## Next Steps

1. ✅ **Phase 0 完成**: 阅读 [research.md](./research.md) - 技术决策与方案选择
2. ✅ **Phase 1 完成**: 阅读 [data-model.md](./data-model.md), [contracts/](./contracts/), [quickstart.md](./quickstart.md)
3. ⏳ **Phase 2 待执行**: 运行 `/speckit.tasks` 生成详细任务列表（`tasks.md`）
4. ⏳ **开始实施**: 按照 `tasks.md` 顺序，TDD 工作流（测试先行 → 实现 → 重构）

---

**Status**: ✅ Phase 0 & Phase 1 Complete  
**Next Command**: `/speckit.tasks`  
**Estimated Time**: 3 weeks (M2)


