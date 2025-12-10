# Feature Specification: Boss Encounter System

**Feature Branch**: `006-boss-encounter`
**Created**: 2025-11-27
**Status**: Draft
**Constitution**: v1.0.1 (Language Separation Rule enforced)
**Input**: User description: "实现Boss遭遇战系统，包括Boss AI、阶段转换、特殊技能。Boss有多个阶段，每个阶段有不同的攻击模式。遵守 Constitution v1.0.1。"

---

**语言规范说明（Language Guidelines）**:
- 本文档使用中文（This document uses Chinese for Chinese projects）
- 代码示例使用英文标识符（Code examples use English identifiers）
- 技术术语保持英文（Technical terms remain in English）

## Clarifications

### Session 2025-12-03

- Q: Boss 配置数据（阶段、技能、阈值）应如何存储/定义？ → A: RON 配置文件，放在 `assets/data/`（符合项目标准）
- Q: 每个 Boss 最多可有多少个阶段？ → A: 3-5 个阶段，每个 Boss 可配置上限（灵活但受控）
- Q: 阶段转换时 Boss 的无敌时间应持续多久？ → A: 1-2 秒，每个 Boss 可配置（平衡动画与战斗节奏）
- Q: 技能预警（Telegraph）的视觉呈现方式是什么？ → A: 半透明红色渐变区域，带闪烁动画（清晰直观的危险提示）
- Q: 快速击杀时 Boss 的锁血机制应如何工作？ → A: 血量降至阈值时立即锁定，强制触发阶段转换，完成后解锁（防止跳过阶段）

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Boss 激活与遭遇 (Priority: P1)

玩家进入 Boss 房间，触发 Boss 出现或唤醒，遭遇战开始。

**Why this priority**: 定义了战斗的开始，没有它就无法进行后续体验。

**Independent Test**: 进入指定区域，验证 Boss 是否生成/激活，UI 是否显示 Boss 血条。

**Acceptance Scenarios**:

1. **Given** 玩家进入 Boss 房间区域, **When** 触发过场动画或事件, **Then** Boss 实体激活，入口封锁，Boss 血条显示。
2. **Given** 战斗开始, **When** Boss 初始化完成, **Then** Boss 立即进入第一阶段的战斗循环。

---

### User Story 2 - 战斗阶段转换 (Priority: P1)

Boss 血量降低到一定阈值时，进入下一个阶段，改变外观或攻击模式。

**Why this priority**: 增加战斗的深度和挑战性，是 Boss 战的核心体验。

**Independent Test**: 使用调试工具直接修改 Boss 血量到阈值，观察阶段切换逻辑。

**Acceptance Scenarios**:

1. **Given** Boss 处于阶段 1, **When** 血量降至 50% (可配置), **Then** Boss 播放转阶段动画，进入无敌状态（1-2 秒，可配置），随后切换到阶段 2。
2. **Given** Boss 进入阶段 2, **When** 恢复战斗, **Then** Boss 使用新的技能组合或攻击频率增加。

---

### User Story 3 - 特殊技能释放 (Priority: P1)

Boss 定期或在特定条件下释放强力特殊技能（如 AoE、冲锋）。

**Why this priority**: 区分 Boss 与普通敌人的关键特征，迫使玩家进行走位和策略应对。

**Independent Test**: 观察 Boss 行为循环，验证技能预警和释放效果。

**Acceptance Scenarios**:

1. **Given** Boss 能量满或冷却结束, **When** 判定释放特殊技能, **Then** 显示预警范围提示 (Telegraph)：半透明红色渐变区域，带闪烁动画。
2. **Given** 预警结束, **When** 技能释放, **Then** 在预警区域内的玩家受到伤害或异常状态。

---

### Edge Cases

- **玩家死亡**: Boss 战中玩家死亡，Boss 状态应完全重置（血量、阶段、位置）。
- **快速击杀**: 如果玩家伤害极高（秒杀），Boss 血量降至阈值时立即锁定，强制触发阶段转换。阶段转换完成后（包括无敌时间结束）解锁血量，继续正常战斗。这确保所有阶段转换逻辑都能完整执行，避免因跳过阶段导致的崩溃。
- **脱离战斗**: 如果玩家通过某种方式（Bug）离开 Boss 房，Boss 应重置并传回出生点。

### Combat Mechanics Testing (if applicable)

- **Telegraph Accuracy**: 验证预警范围与实际伤害范围是否一致。
- **Phase Transition Invulnerability**: 验证转阶段期间 Boss 是否正确免疫伤害。
- **Hitbox Changes**: 如果 Boss 模型在不同阶段发生变化，验证 Hitbox 是否同步更新。

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST 提供 Boss 专用的 AI 控制器，支持多阶段 (Multi-Phase) 逻辑管理。每个 Boss 最多支持 3-5 个阶段（可在配置中设置）。
- **FR-002**: System MUST 支持基于血量百分比或时间的阶段切换触发器。当血量降至阈值时，必须立即锁定血量并强制触发阶段转换，防止快速击杀跳过阶段。阶段转换完成后（包括无敌时间）解锁血量。
- **FR-003**: System MUST 实现技能预警系统 (Telegraphing)，在场景中渲染技能范围提示。预警显示为半透明红色渐变区域，带有闪烁动画效果。
- **FR-004**: System MUST 管理 Boss 的技能冷却和优先级队列，确保技能释放有规律但不完全可预测。
- **FR-005**: System MUST 提供 Boss 专用的 UI 组件（大血条、阶段指示器）。
- **FR-006**: System MUST 处理 Boss 死亡事件，触发掉落（调用 Loot System）和关卡完成逻辑。

### Key Entities *(include if feature involves data)*

- **BossController**: 核心 AI 控制器。
- **BossPhase**: 定义每个阶段的属性（可用技能、攻击频率、移动速度）。
- **SkillDefinition**: 定义特殊技能的参数（范围、伤害、冷却、预警时间）。
- **PhaseTransition**: 阶段转换配置（血量阈值、无敌时间 1-2 秒可配置）。

**配置存储**:
- Boss 定义（阶段、技能、阈值）存储在 RON 配置文件中，位于 `assets/data/bosses.ron`（或类似路径）。
- 配置文件在运行时加载，支持不重新编译即可调整 Boss 参数。

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Boss 阶段转换在血量达到阈值的 1 秒内触发，且状态切换无卡顿。
- **SC-002**: 技能预警时间足够（至少 0.5s），允许熟练玩家进行反应和躲避。
- **SC-003**: 战斗重置逻辑（玩家死亡后）在 3 秒内完成，确保下次尝试无状态残留。

### Assumptions

- **Enemy AI Base**: Boss AI 建立在 `004-enemy-ai` 的基础架构之上。
- **Combat Core**: 伤害判定和血量管理复用 `002-combat-core`。
- **Loot System**: Boss 死亡后的掉落逻辑复用 `005-loot-inventory`。
