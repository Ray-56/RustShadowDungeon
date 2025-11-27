# 战斗系统核心 (Combat System Core) - M2

**Milestone**: M2 - 战斗系统基础（3周）  
**Feature Branch**: `002-combat-core`  
**Status**: ✅ 规范已完成，等待技术计划  
**Created**: 2025-11-25

---

## 📋 规范概览

实现 DNF 风格的战斗系统核心，包括：

- ⚔️ **伤害计算系统**（纯函数，零 Bevy 依赖）
- 💥 **碰撞检测系统**（HitBox vs HurtBox）
- 🔥 **3连击系统**（轻-轻-重）
- ✨ **打击感反馈**（hitfreeze、屏幕震动、粒子特效）
- 🎯 **基础技能系统**（技能冷却、MP 消耗）
- 👾 **史莱姆敌人**（16×16，生命值显示）

---

## 📁 文档结构

```
.specify/specs/02-combat/
├── README.md                    # 本文件（快速开始指南）
├── spec.md                      # ✅ 完整规范（6 个 User Stories，50 个需求）
├── checklists/
│   └── requirements.md          # ✅ 规范质量检查清单（22/22 通过）
├── tasks/                       # 待创建（运行 /speckit.tasks）
└── contracts/                   # 可选（API 规范，如需要）
```

---

## 🚀 下一步行动

### 1️⃣ 创建技术实施计划（必需）

```bash
/speckit.plan
```

**输出**：
- `plan.md` - 技术实施计划（架构、技术选型、文件结构）
- `research.md` - 技术决策（碰撞检测算法、粒子系统选择）
- `data-model.md` - 战斗实体数据模型
- `quickstart.md` - 开发者快速开始指南

---

### 2️⃣ 生成任务分解（必需）

在 `/speckit.plan` 完成后运行：

```bash
/speckit.tasks
```

**输出**：
- `tasks.md` - 详细任务列表（按 User Story 组织，标记并行任务 [P]）

---

### 3️⃣ 开始实施（TDD）

按照 `tasks.md` 中的任务顺序：

1. **Week 1**: 伤害计算 + 碰撞检测（领域层纯函数）
2. **Week 2**: 连击系统 + 打击感反馈
3. **Week 3**: 技能系统 + 史莱姆敌人

**TDD 工作流**：
1. 编写失败测试（参考 `spec.md` 中的 Combat Mechanics Testing）
2. 运行 `cargo test`（验证测试失败）
3. 实现最小代码让测试通过
4. 重构代码（保持测试通过）
5. 重复

---

## 📊 关键指标

### 性能目标

- ✅ 60 FPS（帧时间 <16.67ms）
- ✅ 战斗逻辑 <3ms
- ✅ 碰撞检测 <3ms

### 代码质量目标

- ✅ 测试覆盖率 ≥85%
- ✅ 零 unsafe 代码
- ✅ 零 clippy 警告

### 游戏体验目标

- ✅ 打击感评分 ≥7/10
- ✅ 音效同步误差 <2 帧
- ✅ 玩家可在 30 秒内学会 3 连击

---

## 🏗️ 架构约束（重要）

根据 Constitution v1.0.1：

### 领域层（零 Bevy 依赖）

```rust
// ✅ 正确：纯函数，可独立测试
// src/domain/combat/damage.rs
pub fn calculate_damage(
    base: f32,
    attacker_stats: &Stats,
    defender_stats: &Stats,
    element: Element,
) -> DamageResult {
    // 纯计算逻辑，无 Bevy import
}
```

### 基础设施层（Bevy 桥接）

```rust
// ✅ 正确：系统桥接领域层与 ECS
// src/infrastructure/systems/combat_systems.rs
pub fn apply_damage_system(
    attackers: Query<(&Attack, &Stats)>,
    mut defenders: Query<(&mut Health, &Stats)>,
) {
    // 提取 ECS 数据
    // 调用领域层纯函数
    let result = domain::combat::calculate_damage(...);
    // 更新 ECS 组件
}
```

---

## 📦 依赖项

### 前置条件

- ✅ M1 (001-player-movement) 已完成

### 技术栈

- Rust 1.91.1 (stable)
- Bevy 0.17.0
- bevy_rapier2d 0.29+（碰撞检测）

### 资产需求

**精灵**：
- `assets/sprites/player_attack.png` - 玩家攻击动画（3 连击）
- `assets/sprites/enemy_slime.png` - 史莱姆敌人（16×16）
- `assets/sprites/hit_effect.png` - 打击特效（8×8）
- `assets/sprites/skill_fireball.png` - 火球技能

**音效**：
- `assets/audio/hit_light.ogg` - 轻击
- `assets/audio/hit_heavy.ogg` - 重击
- `assets/audio/hit_critical.ogg` - 暴击

**配置**：
- `assets/data/skills.ron` - 技能数据
- `assets/data/enemies.ron` - 敌人数据

---

## 📝 User Stories 优先级

### P1（必需，Week 1-2）

- **US1**: 基础攻击与伤害（MVP）
- **US2**: 3连击系统
- **US3**: 打击感反馈

### P2（重要，Week 3）

- **US4**: 基础技能系统
- **US6**: 无敌帧系统

### P3（可选，v1.1+）

- **US5**: 技能取消机制

---

## ❓ 常见问题

### Q: 为什么领域层必须零 Bevy 依赖？

**A**: Constitution Principle II 要求。纯函数领域逻辑可以：
- 独立测试（无需启动 Bevy App）
- 更快的测试执行速度
- 潜在的跨引擎复用
- 更清晰的关注点分离

### Q: 打击感反馈如何调优？

**A**: 参考 DNF 的参数作为基准：
- Hitfreeze: 3 帧（轻击）、5 帧（重击）
- 屏幕震动: 2-4 像素（重击）
- 然后根据测试玩家反馈微调

### Q: 如何保证 60 FPS？

**A**: 
1. 性能预算分配（战斗 <3ms，碰撞 <3ms）
2. 使用 `cargo bench` 定期基准测试
3. 限制同时存在的 HitBox 数量 (<50)
4. 使用 bevy_rapier2d 的空间分区优化碰撞检测

### Q: 测试覆盖率 ≥85% 如何实现？

**A**:
- 使用 `cargo-tarpaulin` 或 `cargo-llvm-cov` 生成覆盖率报告
- 优先测试领域层（伤害计算、连击逻辑）
- 参考 `spec.md` 中的 Combat Mechanics Testing 章节
- CI 管道自动检查覆盖率

---

## 🎮 战斗系统使用说明

### 基本使用

#### 1. 注册战斗插件

在 `main.rs` 中添加战斗系统插件：

```rust
use rust_shadow_dungeon::infrastructure::plugins::CombatPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CombatPlugin)  // 注册战斗系统
        .run();
}
```

#### 2. 创建玩家实体

```rust
commands.spawn((
    Player,
    Health { current: 100.0, max: 100.0 },
    Stats {
        attack: 10.0,
        defense: 0.0,
        crit_rate: 0.1,
        crit_multiplier: 2.0,
        ..Default::default()
    },
    Combo::default(),
    Skill {
        skill_id: "fireball".to_string(),
        cooldown: 5.0,
        remaining_cooldown: 0.0,
        mp_cost: 20.0,
        damage: 30.0,
        element: Element::Fire,
    },
    MP { current: 100.0, max: 100.0 },
));
```

#### 3. 创建敌人实体

```rust
commands.spawn((
    Enemy { id: "slime".to_string(), enemy_type: EnemyType::Slime },
    Health { current: 30.0, max: 30.0 },
    HurtBox {
        rect: Rect::new(0.0, 0.0, 16.0, 16.0),
        is_invincible: false,
    },
    Stats {
        attack: 5.0,
        defense: 0.0,
        ..Default::default()
    },
));
```

### 输入控制

战斗系统监听以下按键：

- **J 键**: 基础攻击（触发连击系统）
- **K 键**: 技能施放（火球术）

### 战斗机制

#### 连击系统

- **第 1 击**: 10 点伤害（轻击）
- **第 2 击**: 10 点伤害（轻击），显示 "2 HIT COMBO"
- **第 3 击**: 20 点伤害（重击，2倍），显示 "3 HIT COMBO"，敌人被击退
- **连击窗口**: 1 秒，超时后重置

#### 技能系统

- **火球术**: 
  - 冷却时间: 5 秒
  - MP 消耗: 20
  - 伤害: 30 点火元素伤害
  - 弹道速度: 300 像素/秒

#### 打击感反馈

- **Hitfreeze**: 轻击 3 帧，重击 5 帧，暴击 7 帧
- **屏幕震动**: 重击/暴击时触发
- **粒子特效**: 根据攻击类型生成不同数量的粒子
- **伤害数字**: 显示伤害值并向上飘动

### 配置调整

所有战斗参数可在配置文件中调整：

**`assets/data/combat_config.ron`**:
```ron
(
    combo_window: 1.0,              // 连击窗口时间
    hitfreeze_light: 0.05,         // 轻击定格时间
    hitfreeze_heavy: 0.083,        // 重击定格时间
    invincibility_duration: 0.5,   // 无敌帧时长
    // ... 更多参数
)
```

**`assets/data/skills.ron`**:
```ron
[
    (
        id: "fireball",
        cooldown: 5.0,
        mp_cost: 20.0,
        damage: 30.0,
        element: Fire,
    ),
]
```

### 事件监听

战斗系统发布以下事件，可在其他系统中监听：

```rust
use rust_shadow_dungeon::infrastructure::events::combat::{
    DamageDealt, ComboExtended, EnemyDefeated, SkillActivated
};

fn my_system(mut damage_events: MessageReader<DamageDealt>) {
    for event in damage_events.read() {
        println!("Damage dealt: {} to {:?}", 
                 event.result.final_damage, 
                 event.target);
    }
}
```

### 性能监控

运行性能基准测试：

```bash
# 战斗逻辑性能
cargo bench --bench combat_bench

# 碰撞检测性能
cargo bench --bench collision_bench
```

### 测试

运行测试套件：

```bash
# 所有测试
cargo test

# 仅单元测试
cargo test --lib

# 仅集成测试
cargo test --test integration

# 生成覆盖率报告
cargo tarpaulin --lib
```

---

## 📞 联系与支持

- **规范问题**: 阅读 `spec.md`（6 个 User Stories，50 个需求）
- **质量检查**: 阅读 `checklists/requirements.md`（22/22 通过）
- **技术决策**: 阅读 `plan.md`（技术实施计划）
- **任务分解**: 阅读 `tasks.md`（详细任务列表）
- **实施经验**: 阅读 `quickstart.md`（实际实施经验）

---

**Status**: ✅ Implementation Complete  
**Next Steps**: 修复集成测试 API，补充测试覆盖率  
**Completed**: M2 战斗系统核心功能已实现


