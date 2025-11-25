# 实施计划：玩家移动系统

**Branch**: `001-player-movement` | **Date**: 2025-11-24 | **Spec**: [spec.md](./spec.md)
**Input**: 功能规范来自 `/specs/001-player-movement/spec.md`
**Architecture**: 严格遵守 DDD（领域驱动设计）架构，使用 Rust + Bevy

**Note**: 本文档由 `/speckit.plan` 命令生成。工作流程详见 `.specify/templates/commands/plan.md`。

---

## 摘要

实现玩家角色移动系统，支持键盘（WASD）和手柄输入，提供地面移动、跳跃和空中控制功能。使用 bevy_tnua 字符控制器和 bevy_rapier2d 物理引擎，确保像素完美渲染（16×16 网格锁定）和 60 FPS 性能目标。

**技术方案**:
- **领域层**（Domain Layer）：纯 Rust 函数，零 Bevy 依赖，实现移动逻辑、状态转换、输入验证
- **基础设施层**（Infrastructure Layer）：Bevy 系统桥接，连接领域逻辑与 ECS 组件
- **插件化架构**：PlayerPlugin 封装所有玩家移动相关组件、系统、事件

**主要技术挑战**:
1. bevy_tnua 集成（字符控制器适配 2D 侧视图）
2. 像素完美渲染（16×16 网格锁定、相机对齐）
3. 输入缓冲系统（解决掉帧输入丢失）
4. 跨平台输入统一（键盘 + 手柄）

---

## 技术上下文

**Language/Version**: Rust 1.91.1 (stable)

**Primary Dependencies**:
- Bevy 0.17.0（游戏引擎，ECS 架构）
- **Avian2d 0.4**（2D 物理引擎，XPBD 算法，碰撞检测）
  - ⚠️ 技术变更：原计划 bevy_rapier2d 因版本冲突改用 Avian2d
- bevy-tnua 0.26（字符控制器，地面检测）
- bevy-tnua-avian2d 0.8（Avian2d 集成桥梁）
- leafwing-input-manager 0.17（输入抽象层，键盘+手柄统一）
- bevy_ecs_ldtk latest stable（关卡加载，可选）

**Storage**: 
- RON 文件用于配置数据（移动参数、输入映射）
- 无数据库需求（移动系统无持久化需求）

**Testing**: 
- cargo test（单元测试、集成测试）
- criterion（性能基准测试，验证 <1ms 帧时间）
- 测试覆盖率目标：≥85%（移动系统核心逻辑）

**Target Platform**: 
- Windows/Linux/macOS desktop（Tier 1）
- Web WASM（Tier 1）
- Android（Tier 1）
- iOS（Tier 2，v1.2+）

**Project Type**: 单一游戏项目，基于插件的模块化架构

**Performance Goals**: 
- 60 FPS（16.67ms 帧预算）
- 移动系统占用 <1ms（6% 帧预算）
- 输入延迟 <1ms（一帧内响应）
- 内存占用 <10MB（移动系统运行时）

**Constraints**: 
- 像素完美渲染（16×16 网格锁定，无亚像素模糊）
- 确定性移动（相同输入产生相同结果，为回放系统准备）
- DDD 架构强制分层（领域层零 Bevy 依赖）

**Scale/Scope**: 
- 单个玩家角色移动系统
- 4 种移动状态（Idle、Walking、Jumping、Falling）
- 2 种输入设备（键盘、手柄）
- 3 种动画（待机、行走、跳跃/落地）

---

## Constitution Check

*GATE: 必须在 Phase 0 研究前通过。Phase 1 设计后重新检查。*

### Core Principles Compliance

- [x] **Rust Memory Safety**: 本功能不需要 `unsafe` 代码。所有物理计算和移动逻辑使用安全的 Rust API（bevy_tnua、bevy_rapier2d）
- [x] **Bevy ECS Architecture**: 严格遵循 ECS 模式：
  - 组件：Player, InputState, MovementState, PhysicsBody（纯数据结构）
  - 系统：input_system, movement_system, animation_system（纯行为函数）
  - 事件：PlayerMoved, StateChanged（系统间通信）
- [x] **60 FPS Performance**: 移动系统预算 <1ms（见下方性能预算详情）
- [x] **Pixel Art Consistency**: 所有精灵使用 16×16 基础网格，相机锁定到像素网格
- [ ] **Combat Mechanics Testing**: N/A（移动系统不涉及战斗机制）
- [x] **Open Source MIT**: 所有依赖项（Bevy、bevy_rapier2d、bevy_tnua、leafwing-input-manager）均为 MIT 或 Apache-2.0 许可，兼容项目 MIT 许可证
- [x] **Modular Design**: 移动系统封装为独立的 PlayerPlugin，无循环依赖
- [x] **Language Separation**: 代码使用英文标识符（Player, InputState），文档使用中文

### Performance Budget

移动系统属于性能关键功能，帧时间预算分配如下（总计 <1ms）：

- **Input processing**: 0.15ms（输入收集、缓冲、映射）
- **Physics/collision**: 0.40ms（bevy_rapier2d 碰撞检测、地面检测）
- **Movement logic**: 0.25ms（移动计算、状态转换）
- **Animation update**: 0.10ms（动画状态更新）
- **Camera follow**: 0.05ms（相机位置更新、像素对齐）
- **Buffer**: 0.05ms（余量）
- **Total**: 1.00ms（占 60 FPS 帧预算的 6%）

**验证方法**: 使用 `criterion` 基准测试，在典型游戏场景下测量移动系统帧时间。

### Testing Requirements

- [x] **Unit tests for movement logic**: 
  - 移动速度计算（地面、空中）
  - 跳跃高度计算
  - 状态转换逻辑（Idle → Walking → Jumping → Falling → Idle）
  - 输入缓冲处理
- [x] **Integration tests for system interactions**:
  - 输入系统 → 移动系统 → 动画系统管道测试
  - 物理系统（碰撞）与移动系统交互
  - 相机跟随系统与玩家移动交互
- [x] **Performance benchmarks for critical paths**:
  - 单帧移动系统执行时间（目标 <1ms）
  - 连续跳跃性能（1000 次跳跃，平均帧时间）
  - 多种输入场景（键盘、手柄、混合）性能对比
- [x] **Tests written FIRST**: 遵循 TDD（测试驱动开发），先编写失败的测试，再实现功能

---

## Project Structure

### Documentation (this feature)

```text
specs/001-player-movement/
├── plan.md              # 本文件（/speckit.plan 输出）
├── research.md          # Phase 0 输出（技术研究）
├── data-model.md        # Phase 1 输出（ECS 数据模型）
├── quickstart.md        # Phase 1 输出（开发者快速入门）
├── contracts/           # Phase 1 输出（API 契约，本功能无）
├── checklists/          # 质量检查清单
│   └── requirements.md  # 规范质量检查（已完成）
└── tasks.md             # Phase 2 输出（/speckit.tasks - 尚未创建）
```

### Source Code (repository root)

**DDD 架构分层**（严格遵守领域驱动设计原则）：

```text
src/
├── main.rs              # 游戏入口点，注册所有插件
├── lib.rs               # 库根，导出公共 API（用于测试）
│
├── domain/              # 领域层（Domain Layer） - 零 Bevy 依赖
│   ├── mod.rs
│   └── movement/        # 移动领域逻辑
│       ├── mod.rs
│       ├── velocity.rs  # 速度计算（纯函数）
│       ├── jump.rs      # 跳跃逻辑（纯函数）
│       ├── state.rs     # 状态转换（纯函数）
│       └── input.rs     # 输入验证（纯函数）
│
├── infrastructure/      # 基础设施层（Infrastructure Layer） - Bevy 桥接
│   ├── mod.rs
│   ├── plugins/         # Bevy 插件（系统注册、资源初始化）
│   │   ├── mod.rs
│   │   └── player.rs    # PlayerPlugin（注册移动相关系统）
│   ├── components/      # ECS 组件（纯数据结构）
│   │   ├── mod.rs
│   │   ├── player.rs    # Player, InputState, MovementState
│   │   └── physics.rs   # PhysicsBody（bevy_rapier2d 包装）
│   ├── systems/         # ECS 系统（行为函数，调用领域层）
│   │   ├── mod.rs
│   │   ├── input.rs     # input_system（收集输入）
│   │   ├── movement.rs  # movement_system（应用移动逻辑）
│   │   ├── physics.rs   # physics_sync_system（同步物理状态）
│   │   ├── animation.rs # animation_system（更新动画）
│   │   └── camera.rs    # camera_follow_system（相机跟随）
│   ├── resources/       # 全局资源
│   │   ├── mod.rs
│   │   ├── input_config.rs # InputConfig（键位映射、手柄配置）
│   │   └── movement_config.rs # MovementConfig（速度、跳跃高度等）
│   └── events/          # 事件定义
│       ├── mod.rs
│       └── movement.rs  # PlayerMoved, StateChanged 事件
│
tests/
├── integration/         # 集成测试（完整系统测试）
│   ├── mod.rs
│   ├── movement_pipeline_test.rs  # 输入→移动→动画管道测试
│   └── physics_integration_test.rs # 物理交互测试
└── unit/                # 单元测试（领域层纯函数测试）
    ├── mod.rs
    ├── velocity_test.rs # 速度计算测试
    ├── jump_test.rs     # 跳跃逻辑测试
    └── state_test.rs    # 状态转换测试

assets/
├── sprites/             # 像素艺术资产（16x16 grid）
│   ├── player_idle.png  # 待机动画（32×32）
│   ├── player_walk.png  # 行走动画（32×32）
│   └── player_jump.png  # 跳跃动画（32×32）
├── data/                # RON 配置文件
│   ├── movement_config.ron  # 移动参数配置
│   └── input_config.ron     # 输入映射配置
└── levels/              # 测试关卡（LDtk 格式，可选）
    └── test_movement.ldtk   # 移动测试关卡

benches/                 # 性能基准测试
└── movement_bench.rs    # 移动系统性能测试

Cargo.toml               # 依赖和元数据
```

**Structure Decision**: 

采用 **DDD 分层架构**，将领域逻辑（domain/）与基础设施（infrastructure/）严格分离：

1. **domain/** 目录包含纯 Rust 函数，无任何 Bevy 依赖。所有移动逻辑、状态转换、输入验证都在此实现。这些函数易于单元测试，且可移植到其他引擎。

2. **infrastructure/** 目录包含 Bevy 特定代码：
   - **plugins/**: 注册系统、资源、事件
   - **components/**: ECS 组件（纯数据结构）
   - **systems/**: ECS 系统（从组件提取数据，调用领域函数，更新组件）
   - **resources/**: 全局配置和状态
   - **events/**: 系统间通信

3. **测试分层**：
   - **unit/**: 测试领域层纯函数（无 Bevy 上下文）
   - **integration/**: 测试完整 Bevy 系统管道（使用 `App::new()` 创建测试环境）

这种架构符合 Constitution 原则 II（Bevy ECS）和原则 VII（模块化设计），同时遵守用户要求的"严格符合 DDD 架构"。

---

## Complexity Tracking

> **仅在 Constitution Check 有违规需要辩护时填写**

**无违规**: 本实施计划完全遵守 Constitution v1.0.1 的所有原则，无需额外辩护。

---

## Phase 0: 技术研究与决策

### 研究任务

1. **bevy_tnua 集成可行性**
   - 调研 bevy_tnua 是否支持 2D 侧视图游戏
   - 验证地面检测、斜坡行走功能
   - 评估与 bevy_rapier2d 的集成难度
   - 确定回退方案（手写字符控制器）

2. **像素完美渲染方案**
   - 研究 Bevy 像素完美渲染最佳实践
   - 验证 16×16 网格锁定方案
   - 确定相机配置（整数缩放、像素对齐）
   - 测试不同分辨率下的渲染效果

3. **输入系统设计**
   - 评估 leafwing-input-manager 功能
   - 设计输入缓冲机制（150ms 缓冲窗口）
   - 确定键盘和手柄输入映射
   - 验证跨平台输入一致性

4. **性能基准测试方案**
   - 设计移动系统性能测试场景
   - 确定 criterion 基准测试编写方式
   - 定义性能回归检测阈值（>10% 回归）

### 输出

生成 `research.md`，包含：
- **Decision**: 每项技术选择的最终决定
- **Rationale**: 选择理由和权衡分析
- **Alternatives Considered**: 考虑的备选方案及拒绝原因
- **Implementation Notes**: 实施时的注意事项

---

## Phase 1: 设计与契约

**前提条件**: `research.md` 完成

### 设计任务

1. **数据模型设计** → `data-model.md`
   - 定义 ECS 组件：Player, InputState, MovementState, PhysicsBody
   - 定义资源：InputConfig, MovementConfig
   - 定义事件：PlayerMoved, StateChanged
   - 定义领域模型：Velocity, JumpParams, StateTransition

2. **系统架构设计** → `data-model.md`（系统交互图）
   - 输入系统管道：input_system → movement_system → animation_system
   - 物理系统交互：physics_sync_system ↔ movement_system
   - 相机系统：camera_follow_system（监听 PlayerMoved 事件）

3. **API 契约**（本功能无外部 API）
   - 无需生成 REST/GraphQL 契约（纯单机游戏功能）
   - 内部事件契约在 data-model.md 中定义

4. **快速入门指南** → `quickstart.md`
   - 项目设置（cargo new、依赖安装）
   - 运行测试（cargo test）
   - 运行基准测试（cargo bench）
   - 调试技巧（Bevy inspector、日志配置）

### 输出

生成文件：
- `data-model.md`：ECS 组件、资源、事件、领域模型定义
- `quickstart.md`：开发者快速入门指南
- `contracts/`：无（本功能不需要）

### Agent Context Update

运行脚本更新 AI Agent 上下文文件：

```bash
.specify/scripts/bash/update-agent-context.sh cursor-agent
```

这将在 `.specify/agent-context.md` 中添加：
- Rust 1.91.1 + Bevy 0.17.0 技术栈
- bevy_tnua、bevy_rapier2d、leafwing-input-manager 依赖
- 玩家移动系统项目结构

---

## 下一步

Phase 1 完成后，运行以下命令生成任务清单：

```bash
/speckit.tasks
```

这将生成 `tasks.md`，包含：
- Phase 1: Setup（项目初始化、依赖配置）
- Phase 2: Foundational（基础系统、测试框架）
- Phase 3-6: User Stories（按优先级实施 P1-P3 用户故事）
- Phase 7: Polish（优化、文档、验收）

每个任务将包含：
- 任务 ID（T001、T002...）
- 并行标记（[P]，如果可并行）
- 用户故事标记（[US1]、[US2]...）
- 精确的文件路径和操作描述

---

**Document Status**: ✅ 实施计划已完成
**Constitution Compliance**: ✅ 所有原则已验证
**Ready for**: Phase 0 研究 → Phase 1 设计 → 任务生成
