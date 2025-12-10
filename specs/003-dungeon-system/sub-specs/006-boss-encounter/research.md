# Phase 0 研究：Boss 遭遇战系统技术决策

**Created**: 2025-12-03
**Feature**: 006-boss-encounter
**Purpose**: 解决技术实施中的未知问题，为 Phase 1 设计提供依据

---

## 研究概述

本文档记录 Boss 遭遇战系统的技术研究结果，涵盖多阶段 AI 状态机设计、技能预警渲染方案、阶段转换机制和 RON 配置文件格式。所有决策均基于对备选方案的充分评估，并符合 Constitution v1.0.1 原则。

---

## 研究 1: Boss AI 系统架构设计

### Decision（决策）

**采用基于状态机的多阶段 Boss AI 系统**，扩展 Enemy AI 基础架构（004-enemy-ai），使用独立的 `BossController` 组件和阶段状态机。

### Rationale（理由）

1. **复用 Enemy AI 基础**: Boss 是特殊类型的敌人，复用 Enemy AI 的基础架构（如状态机模式、目标检测）可减少重复代码。

2. **扩展性设计**: 使用独立的 `BossController` 组件，允许 Boss 有更复杂的行为（多阶段、特殊技能），同时保持与普通敌人的接口一致性。

3. **阶段状态机清晰**: 每个阶段作为独立状态，包含阶段特定的技能列表、攻击频率、移动速度等，状态转换逻辑明确。

4. **符合 ECS 模式**: Boss 作为实体，`BossController` 作为组件，阶段转换逻辑在系统中处理，完全符合 Bevy ECS 架构。

5. **性能可控**: 状态机模式计算开销低，Boss 数量少（通常 1 个），性能影响可忽略。

### Alternatives Considered（考虑的备选方案）

| 方案 | 优点 | 缺点 | 拒绝原因 |
|------|------|------|---------|
| **完全独立的 Boss AI 系统** | 完全解耦，Boss 逻辑独立 | 代码重复（状态机、目标检测等），维护成本高 | 违反 DRY 原则，增加维护负担 |
| **Boss 作为普通 Enemy，仅增加阶段标记** | 实现简单，代码复用率高 | 阶段逻辑混在 Enemy AI 中，难以管理复杂多阶段行为 | 无法满足多阶段、特殊技能等复杂需求 |
| **脚本化 Boss AI（Lua/Rhai）** | 灵活性高，可配置性强 | 性能开销（脚本引擎），Rust 类型安全丢失，调试困难 | 不符合性能目标，过度工程 |

### Implementation Notes（实施注意事项）

1. **组件设计**:
   ```rust
   #[derive(Component)]
   pub struct BossController {
       current_phase: usize,        // 当前阶段索引（0-based）
       phases: Vec<BossPhase>,      // 阶段配置（从 RON 加载）
       health_lock: bool,           // 血量锁定标志（阶段转换时）
       transition_start_time: Option<f32>, // 阶段转换开始时间
   }
   ```

2. **与 Enemy AI 的关系**:
   - Boss 继承 `Enemy` 组件，获得基础敌人功能（生命值、受击检测等）
   - `BossController` 组件扩展 Boss 特定行为
   - Boss AI 系统在 Enemy AI 系统之后运行，可覆盖部分行为（如移动、攻击）

3. **状态转换触发**:
   - 在伤害系统中检测血量阈值（每帧或每次受击时检查）
   - 达到阈值时立即触发阶段转换，锁定血量
   - 播放转换动画，进入无敌状态（1-2 秒可配置）
   - 解锁血量，切换到新阶段

4. **技能选择逻辑**:
   - 每个阶段维护可用技能列表
   - 技能选择基于冷却时间、优先级、随机性
   - 优先级队列确保高优先级技能优先释放（但不完全可预测）

---

## 研究 2: 技能预警（Telegraph）渲染方案

### Decision（决策）

**使用 Bevy 的 SpriteBundle + ColorMaterial 实现半透明红色渐变预警区域**，通过自定义着色器或透明度动画实现闪烁效果。

### Rationale（理由）

1. **Bevy 内置支持**: Bevy 的 SpriteBundle 支持透明度（Alpha 通道），ColorMaterial 支持自定义颜色，无需第三方库。

2. **性能优化**: 使用简单的矩形/圆形精灵作为预警区域，渲染开销低（远低于复杂粒子效果）。

3. **像素完美兼容**: 预警区域可配置为像素对齐（16×16 网格），符合像素完美渲染要求。

4. **灵活性**: 通过修改 Color 的 alpha 值和变换，可实现渐变、闪烁、缩放等效果。

5. **易于测试**: 预警区域作为独立实体，易于单元测试（位置、大小、颜色验证）。

### Alternatives Considered（考虑的备选方案）

| 方案 | 优点 | 缺点 | 拒绝原因 |
|------|------|------|---------|
| **Bevy UI 系统（Node + BackgroundColor）** | 层级控制方便，易于叠加 | UI 系统渲染在独立层级，与游戏世界坐标转换复杂 | 坐标转换增加复杂度，性能不如 SpriteBundle |
| **粒子系统（bevy_hanabi）** | 视觉效果丰富，可做复杂动画 | 性能开销大（粒子计算），过度工程（简单预警不需要） | 不符合性能目标，简单预警不需要复杂效果 |
| **自定义着色器（Shader）** | 完全控制，效果丰富 | 开发时间长，需了解 GLSL/SPIR-V，调试困难 | 过度工程，SpriteBundle 已满足需求 |
| **线框绘制（bevy_prototype_debug_lines）** | 简单直接，开发快速 | 仅适合调试，不符合"半透明红色渐变"的视觉要求 | 不符合规范要求 |

### Implementation Notes（实施注意事项）

1. **预警区域实体**:
   ```rust
   #[derive(Component)]
   pub struct Telegraph {
       skill_id: SkillId,
       warning_duration: f32,      // 预警持续时间（至少 0.5s）
       area_shape: TelegraphShape, // 圆形、矩形、扇形等
       damage_area: Area,          // 实际伤害区域（用于验证准确性）
   }
   
   // 预警实体渲染
   commands.spawn((
       SpriteBundle {
           sprite: Sprite {
               color: Color::srgba(1.0, 0.0, 0.0, 0.5), // 半透明红色
               custom_size: Some(Vec2::new(width, height)),
               ..default()
           },
           transform: Transform::from_translation(area_center),
           ..default()
       },
       Telegraph { /* ... */ },
   ));
   ```

2. **闪烁动画实现**:
   ```rust
   // 在系统中每帧更新透明度
   fn update_telegraph_flash(
       mut query: Query<&mut Sprite, With<Telegraph>>,
       time: Res<Time>,
   ) {
       for mut sprite in query.iter_mut() {
           // 使用 sin 函数实现周期性闪烁（0.5-0.8 alpha 范围）
           let flash_factor = (time.elapsed_seconds() * 4.0).sin() * 0.15 + 0.65;
           sprite.color.set_alpha(flash_factor);
       }
   }
   ```

3. **渐变效果**:
   - 使用多个重叠的精灵，从外到内逐渐增加 alpha 值
   - 或使用自定义着色器实现径向渐变（Phase 2 优化项）

4. **性能考虑**:
   - 限制同时存在的预警数量（通常 1-2 个）
   - 预警区域使用简单形状（矩形/圆形），避免复杂多边形
   - 渲染层级：预警在 Boss 和玩家之间（Z 坐标控制）

5. **测试验证**:
   - 单元测试：验证预警区域尺寸与伤害区域完全一致
   - 集成测试：验证预警显示时长（≥0.5s）和闪烁动画流畅度

---

## 研究 3: 阶段转换机制实现

### Decision（决策）

**采用状态标志 + 时间戳的锁血机制**，在血量降至阈值时立即锁定，强制触发阶段转换，转换完成后（包括无敌时间）解锁。

### Rationale（理由）

1. **防止快速击杀跳过阶段**: 锁血机制确保即使玩家伤害极高，Boss 也能完整执行所有阶段转换逻辑，避免状态不一致。

2. **清晰的转换流程**: 状态标志（`health_lock`）和时间戳（`transition_start_time`）明确定义转换状态，易于调试和测试。

3. **性能开销低**: 仅增加简单的布尔标志和可选时间戳，无额外系统开销。

4. **符合规范要求**: 规范明确要求"血量降至阈值时立即锁定，强制触发阶段转换，完成后解锁"。

5. **易于扩展**: 未来可支持更复杂的转换条件（如时间触发、技能触发等）。

### Alternatives Considered（考虑的备选方案）

| 方案 | 优点 | 缺点 | 拒绝原因 |
|------|------|------|---------|
| **允许跳过阶段，直接进入下一阶段** | 实现简单，无需锁血逻辑 | 快速击杀时 Boss 可能直接死亡，跳过所有阶段，体验差 | 不符合规范要求，破坏 Boss 战设计 |
| **延迟伤害处理（暂存伤害）** | 可处理极端伤害情况 | 实现复杂（需维护伤害队列），玩家感受延迟，可能造成死亡判定混乱 | 实现复杂，不符合实时战斗体验 |
| **仅阻止血量低于阈值（不锁定）** | 部分防止跳过阶段 | 极端伤害仍可能跳过阶段，不完整 | 不符合规范要求 |

### Implementation Notes（实施注意事项）

1. **锁血检测逻辑**:
   ```rust
   // 在伤害系统中检测血量阈值
   fn check_phase_transition(
       mut boss_query: Query<(&mut BossController, &mut Health), With<Boss>>,
       time: Res<Time>,
   ) {
       for (mut controller, mut health) in boss_query.iter_mut() {
           // 检查是否达到下一阶段阈值
           if let Some(next_threshold) = controller.get_next_phase_threshold() {
               let health_percentage = health.current / health.max;
               if health_percentage <= next_threshold && !controller.health_lock {
                   // 立即锁定血量，触发阶段转换
                   controller.health_lock = true;
                   controller.transition_start_time = Some(time.elapsed_seconds());
                   
                   // 触发阶段转换事件
                   events.send(BossPhaseTransition {
                       boss: boss_entity,
                       from_phase: controller.current_phase,
                       to_phase: controller.current_phase + 1,
                   });
               }
           }
       }
   }
   ```

2. **伤害处理中的锁血检查**:
   ```rust
   // 在应用伤害前检查锁血标志
   fn apply_damage_to_boss(
       mut boss_query: Query<(&mut Health, &BossController), With<Boss>>,
       damage_events: EventReader<DamageDealt>,
   ) {
       for (mut health, controller) in boss_query.iter_mut() {
           if controller.health_lock {
               // 锁血期间不接受伤害（通过无敌状态实现）
               continue;
           }
           // 正常处理伤害...
       }
   }
   ```

3. **解锁逻辑**:
   ```rust
   // 阶段转换完成后解锁血量
   fn unlock_boss_health(
       mut boss_query: Query<&mut BossController, With<Boss>>,
       time: Res<Time>,
       config: Res<BossConfig>,
   ) {
       for mut controller in boss_query.iter_mut() {
           if let Some(start_time) = controller.transition_start_time {
               let elapsed = time.elapsed_seconds() - start_time;
               let invulnerability_duration = config.phases[controller.current_phase]
                   .transition_invulnerability_duration;
               
               if elapsed >= invulnerability_duration {
                   // 解锁血量
                   controller.health_lock = false;
                   controller.transition_start_time = None;
               }
           }
       }
   }
   ```

4. **快速击杀边界情况处理**:
   - 单次伤害超过阶段阈值：立即锁定，触发阶段转换
   - 多次伤害在同一帧超过阈值：仅触发一次阶段转换
   - 所有阶段已触发但 Boss 未死亡：正常接受伤害，正常死亡

5. **测试场景**:
   - 正常阶段转换（血量逐渐降低）
   - 快速击杀（单次伤害超过阈值）
   - 极端伤害（伤害超过总血量，但锁血机制应保护最后阶段）

---

## 研究 4: RON 配置文件格式设计

### Decision（决策）

**使用 RON (Rusty Object Notation) 格式定义 Boss 配置**，存储在 `assets/data/bosses.ron`，使用 serde 和 ron crate 在运行时加载。

### Rationale（理由）

1. **项目标准**: 项目已使用 RON 文件配置（如 `movement_config.ron`, `enemies.ron`），保持一致性。

2. **可读性强**: RON 格式类似 Rust 语法，人类可读，易于编辑（无需 JSON 引号、逗号等繁琐语法）。

3. **类型安全**: RON 支持 Rust 类型（如枚举、元组），serde 自动序列化/反序列化，编译时类型检查。

4. **无编译依赖**: RON 文件在运行时加载，修改配置无需重新编译，支持快速迭代。

5. **MIT 兼容**: `ron` crate 使用 MIT/Apache-2.0 双重许可证，符合 Constitution 要求。

### Alternatives Considered（考虑的备选方案）

| 方案 | 优点 | 缺点 | 拒绝原因 |
|------|------|------|---------|
| **硬编码在 Rust 代码中（结构体/枚举）** | 编译时检查，类型安全 | 修改需重新编译，迭代慢，不符合数据驱动设计 | 不符合规范要求（已明确 RON 配置文件） |
| **JSON/YAML 配置** | 通用格式，工具支持多 | 类型信息丢失（需手动验证），语法繁琐（JSON）或缩进敏感（YAML） | RON 格式更适合 Rust 项目，类型信息保留 |
| **SQLite 数据库** | 查询方便，可动态修改 | 过度工程（简单配置不需要数据库），加载复杂 | 不符合项目已有配置模式 |

### Implementation Notes（实施注意事项）

1. **配置文件结构**:
   ```ron
   // assets/data/bosses.ron
   (
       bosses: [
           (
               id: "orc_warlord",
               name: "兽人战将",
               max_health: 1000.0,
               phases: [
                   (
                       phase_index: 0,
                       health_threshold: 1.0,  // 100%
                       invulnerability_duration: 1.5, // 秒
                       skills: ["basic_attack", "ground_slam"],
                       attack_frequency: 2.0,  // 每秒攻击次数
                       move_speed: 50.0,
                   ),
                   (
                       phase_index: 1,
                       health_threshold: 0.5,  // 50%
                       invulnerability_duration: 2.0,
                       skills: ["frenzy_attack", "ground_slam", "charge"],
                       attack_frequency: 3.0,
                       move_speed: 70.0,
                   ),
               ],
           ),
       ],
   )
   ```

2. **Rust 数据结构**:
   ```rust
   use serde::{Deserialize, Serialize};
   
   #[derive(Debug, Clone, Deserialize, Serialize)]
   pub struct BossDefinitions {
       pub bosses: Vec<BossDefinition>,
   }
   
   #[derive(Debug, Clone, Deserialize, Serialize)]
   pub struct BossDefinition {
       pub id: String,
       pub name: String,
       pub max_health: f32,
       pub phases: Vec<BossPhaseConfig>,
   }
   
   #[derive(Debug, Clone, Deserialize, Serialize)]
   pub struct BossPhaseConfig {
       pub phase_index: usize,
       pub health_threshold: f32,  // 0.0-1.0
       pub invulnerability_duration: f32,
       pub skills: Vec<String>,     // 技能 ID 列表
       pub attack_frequency: f32,
       pub move_speed: f32,
   }
   ```

3. **加载逻辑**:
   ```rust
   use bevy::asset::AssetServer;
   use ron::de::from_bytes;
   
   fn load_boss_config(
       asset_server: &AssetServer,
   ) -> Result<BossDefinitions, ron::Error> {
       let bytes = asset_server.load("data/bosses.ron").read().unwrap();
       from_bytes(&bytes)
   }
   ```

4. **配置验证**:
   - 阶段阈值必须递减（100% → 50% → 25%）
   - 技能 ID 必须在技能定义中存在
   - 无敌时间必须在 1-2 秒范围内（或放宽为 >0）

5. **热重载支持**:
   - 开发模式：监听文件变化，自动重新加载配置
   - 生产模式：仅在启动时加载一次

---

## 研究总结

所有技术决策已完成，主要结论：

| 技术领域 | 最终方案 | 风险级别 | 缓解措施 |
|----------|---------|---------|---------|
| **Boss AI 架构** | 基于状态机的多阶段系统，扩展 Enemy AI | 低 | 复用已验证的 Enemy AI 基础，独立组件解耦 |
| **预警渲染** | Bevy SpriteBundle + 透明度动画 | 低 | Bevy 内置支持，性能可控，易于测试 |
| **阶段转换机制** | 状态标志 + 时间戳锁血 | 低 | 简单实现，符合规范，易于调试 |
| **配置文件** | RON 格式，运行时加载 | 低 | 项目标准，类型安全，serde 支持 |

**准备状态**: ✅ 所有技术未知问题已解决，可进入 Phase 1 设计阶段。

---

**Next Phase**: 生成 `data-model.md`（ECS 组件设计）和 `quickstart.md`（开发者快速入门）






