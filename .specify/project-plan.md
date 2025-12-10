# 锈影地下城 - 全项目开发计划与里程碑路线图

**项目名称**: 锈影地下城 (RustShadowDungeon)
**创建日期**: 2025-11-24
**状态**: 主计划文档 - 项目调度唯一真相
**Constitution**: v1.0.1（严格遵守所有 NON-NEGOTIABLE 与 MANDATORY 规则）
**目标版本**: v0.1 → v1.0 公开 Demo
**总工时预估**: 15 周（约 3.5 个月全职开发）

---

## 执行摘要

本计划将《锈影地下城》从零开始构建至 v1.0 公开 Demo 版本，严格遵守项目宪法 v1.0.1 的所有原则。开发分为 6 个里程碑，每个里程碑都有明确的交付物、验收标准和风险管控措施。

**核心约束**:
- ✅ 60 FPS 性能（16.67ms 帧预算）
- ✅ 零 unsafe 代码（除非有充分理由）
- ✅ 测试覆盖率 ≥85%（战斗与移动系统）
- ✅ 像素完美渲染（16×16 网格锁定）
- ✅ 中文文档 + 英文代码（语言分离规则）

**技术栈锁定**:
- Rust: 1.91.1 (stable)
- Bevy: 0.17.0
- bevy_rapier2d: 0.29+
- bevy_tnua: latest stable
- leafwing-input-manager: latest stable

---

## 里程碑概览

| 里程碑 | 名称 | 工时 | 核心交付物 | 关键风险 |
|--------|------|------|-----------|---------|
| **M1** | 玩家移动核心 | 2周 | 玩家移动、物理、输入 | bevy_tnua 集成复杂度 |
| **M2** | 战斗系统基础 | 3周 | 伤害计算、碰撞检测、连击 | 打击感调优耗时 |
| **M3** | 第一个可玩地下城 | 3周 | 多房间、敌人AI、战利品 | 房间过渡性能 |
| **M4** | 装备与技能树 | 2周 | 装备系统、技能解锁 | 数值平衡迭代 |
| **M5** | 多人联机大厅 | 3周 | 联机、同步、延迟补偿 | 网络同步 bug |
| **M6** | 公开 Demo 打磨 | 2周 | 教程、优化、跨平台 | 移动端性能 |
| **总计** | - | **15周** | v1.0 公开 Demo | - |

---

## M1: 玩家移动核心（2周）

### 目标

建立技术可行性基础 — 玩家角色可在像素完美世界中流畅移动，所有输入响应 <1ms，60 FPS 稳定运行。

### 预计工时

- **开发时间**: 10 个工作日（2 周）
- **开发人员**: 1-2 人
- **测试时间**: 已包含在工时内（TDD 开发）

### 必须生成的 Spec 数量

1. **01-player-movement** - 玩家移动系统规范
2. **01-input-handling** - 输入处理系统规范  
3. **01-physics-integration** - 物理引擎集成规范

**Spec 位置**: `specs/001-player-movement/`

### 核心交付物

#### 代码交付物

- [ ] `src/plugins/player.rs` - 玩家插件（注册所有玩家相关系统）
- [ ] `src/components/player.rs` - Player, PlayerInput, PlayerState 组件
- [ ] `src/systems/movement.rs` - 移动系统（WASD + 手柄支持）
- [ ] `src/systems/physics.rs` - 物理集成（bevy_rapier2d + bevy_tnua）
- [ ] `src/resources/input_config.rs` - 输入配置资源
- [ ] `tests/integration/movement_test.rs` - 移动系统集成测试（≥85% 覆盖率）
- [ ] `tests/unit/player_test.rs` - 玩家组件单元测试

#### 资产交付物

- [ ] `assets/sprites/player_idle.png` - 玩家待机动画（32×32, 16×16 网格）
- [ ] `assets/sprites/player_walk.png` - 玩家行走动画（4-8 帧）
- [ ] `assets/sprites/player_jump.png` - 玩家跳跃动画（2-4 帧）
- [ ] `assets/data/player.ron` - 玩家属性配置（移动速度、跳跃力度）

#### 文档交付物

- [ ] `specs/001-player-movement/spec.md` - 移动系统规范（中文）
- [ ] `specs/001-player-movement/tests.md` - 测试用例定义
- [ ] `specs/001-player-movement/quickstart.md` - 开发者快速开始指南

### 核心功能点

1. **WASD 移动**
   - 地面移动（匀速）
   - 空中移动（减速控制）
   - 转身不卡顿

2. **跳跃系统**
   - 固定跳跃高度
   - 不允许二段跳（除非技能）
   - 落地检测准确

3. **物理碰撞**
   - 墙壁阻挡
   - 平台落地
   - 碰撞层配置（玩家 vs 地形）

4. **输入系统**
   - 键盘支持（WASD + 空格）
   - 手柄支持（左摇杆 + A 键）
   - 输入缓冲（解决掉帧输入丢失）

5. **像素完美渲染**
   - 相机锁定到 16×16 网格
   - 玩家精灵无模糊
   - 整数缩放（1×, 2×, 3×）

### 核心风险点

| 风险 | 可能性 | 影响 | 缓解措施 |
|------|--------|------|---------|
| **bevy_tnua 集成复杂** | 中 | 高 | 提前 1 周研究 bevy_tnua 示例，准备回退方案（手写字符控制器） |
| **像素完美卡顿** | 中 | 中 | 使用 Bevy 官方像素完美示例作为基础，避免浮点数位置 |
| **输入延迟 >1ms** | 低 | 高 | 使用 leafwing-input-manager，避免手写输入系统 |
| **手柄适配问题** | 低 | 中 | 优先支持 Xbox 手柄，其他手柄后续适配 |

### 验收标准（Constitution 合规）

#### 性能标准

- [ ] **60 FPS 稳定**: 单玩家移动时帧时间 <16.67ms（≥95% 帧）
- [ ] **输入延迟 <1ms**: 从按键到移动响应 <1ms（使用 leafwing-input-manager）
- [ ] **内存占用 <50MB**: 仅移动系统运行时内存占用

#### 代码质量标准

- [ ] **零 unsafe 代码**: 无 unsafe 块（或有充分文档说明）
- [ ] **测试全绿**: 所有测试通过（`cargo test` 零失败）
- [ ] **测试覆盖率 ≥85%**: 移动系统核心代码覆盖率（使用 cargo-tarpaulin）
- [ ] **Clippy 零警告**: `cargo clippy -- -D warnings` 通过
- [ ] **格式化通过**: `cargo fmt --check` 通过

#### 架构标准

- [ ] **Bevy ECS 合规**: 组件纯数据、系统纯行为、无直接实体引用
- [ ] **模块化设计**: PlayerPlugin 独立可测试，无循环依赖
- [ ] **语言分离**: 代码用英文、文档用中文、无混杂命名

#### 用户体验标准

- [ ] **像素完美**: 玩家精灵无模糊、相机锁定网格
- [ ] **流畅移动**: 无卡顿、无抖动、转身自然
- [ ] **输入响应**: 按键立即反应，无延迟感

### 依赖项

- **前置条件**: 无（首个里程碑）
- **技术依赖**: Rust 1.91.1, Bevy 0.17.0, bevy_rapier2d 0.29+, bevy_tnua, leafwing-input-manager
- **资产依赖**: 像素艺术家需提供玩家精灵（32×32）

### 下一步行动

1. **立即执行**: 运行 `/speckit.specify` 创建 `01-player-movement` 规范
2. **资产准备**: 设计师开始绘制玩家精灵（待机、行走、跳跃）
3. **技术预研**: 研究 bevy_tnua 文档和示例（1 天）

---

## M2: 战斗系统基础（3周）

### 目标

实现 DNF 风格的战斗系统核心 — 伤害计算、碰撞检测、连击系统、打击感反馈。玩家可以用普通攻击和 2-3 个技能击败敌人。

### 预计工时

- **开发时间**: 15 个工作日（3 周）
- **开发人员**: 1-2 人
- **测试时间**: 已包含在工时内（TDD 开发）

### 必须生成的 Spec 数量

1. **02-combat-core** - 战斗系统核心规范（伤害、碰撞、连击）
2. **02-combat-feedback** - 战斗反馈规范（打击感、音效、特效）
3. **02-skills-basic** - 基础技能系统规范

**Spec 位置**: `specs/002-combat-core/`

### 核心交付物

#### 代码交付物（领域层 - 纯函数）

- [ ] `src/domain/combat/damage.rs` - 伤害计算（纯函数，零 Bevy 依赖）
- [ ] `src/domain/combat/combo.rs` - 连击逻辑（纯函数）
- [ ] `src/domain/combat/status.rs` - 状态效果逻辑（纯函数）
- [ ] `tests/unit/damage_test.rs` - 伤害计算单元测试（≥85% 覆盖率）
- [ ] `tests/unit/combo_test.rs` - 连击系统单元测试

#### 代码交付物（基础设施层 - Bevy 集成）

- [ ] `src/plugins/combat.rs` - 战斗插件（注册战斗系统）
- [ ] `src/components/combat.rs` - Attack, HitBox, Health, StatusEffect 组件
- [ ] `src/systems/combat_systems.rs` - 战斗系统（碰撞检测、伤害应用）
- [ ] `src/systems/combo_systems.rs` - 连击系统
- [ ] `src/systems/feedback_systems.rs` - 反馈系统（hitfreeze、屏幕震动、粒子）
- [ ] `src/events/combat.rs` - DamageDealt, ComboExtended, EnemyDefeated 事件
- [ ] `tests/integration/combat_test.rs` - 战斗系统集成测试

#### 资产交付物

- [ ] `assets/sprites/player_attack.png` - 玩家攻击动画（3 连击）
- [ ] `assets/sprites/player_skill_fireball.png` - 火球技能动画
- [ ] `assets/sprites/enemy_slime.png` - 史莱姆敌人（16×16）
- [ ] `assets/sprites/hit_effect.png` - 打击特效（8×8 粒子）
- [ ] `assets/audio/hit_light.ogg` - 轻击音效
- [ ] `assets/audio/hit_heavy.ogg` - 重击音效
- [ ] `assets/data/skills.ron` - 技能数据配置

#### 文档交付物

- [ ] `specs/002-combat-core/spec.md` - 战斗系统规范（中文）
- [ ] `specs/002-combat-core/tests.md` - 战斗系统测试用例
- [ ] `specs/002-combat-core/data-model.md` - 战斗数据模型

### 核心功能点

1. **伤害计算系统**
   - 基础伤害 + 属性加成
   - 暴击系统（暴击率、暴击倍率）
   - 元素伤害（火、冰、雷等）
   - 防御减伤

2. **碰撞检测系统**
   - HitBox vs HurtBox 检测
   - 多段攻击处理
   - 穿透攻击支持
   - 无敌帧（i-frames）处理

3. **连击系统**
   - 3 连击（轻-轻-重）
   - 连击取消（攻击可取消进技能）
   - 连击计数器（UI 显示）
   - 连击超时（1 秒）

4. **打击感反馈**
   - Hitfreeze（打击时定格 2-5 帧）
   - 屏幕震动（重击时轻微震动）
   - 粒子特效（火花、灰尘）
   - 音效触发（轻击、重击不同音效）

5. **基础技能系统**
   - 技能冷却管理
   - 资源消耗（MP）
   - 技能动画锁定
   - 技能取消窗口

6. **敌人基础**
   - 简单敌人（史莱姆）
   - 生命值显示
   - 受击反馈（闪白、击退）
   - 死亡动画

### 核心风险点

| 风险 | 可能性 | 影响 | 缓解措施 |
|------|--------|------|---------|
| **打击感调优耗时** | 高 | 中 | 使用 DNF 的打击感参数作为基准（hitfreeze 3帧，震动强度 0.2），快速迭代 |
| **碰撞检测性能** | 中 | 高 | 使用 bevy_rapier2d 的空间分区，限制 HitBox 数量（<50 同时存在） |
| **连击系统复杂** | 中 | 中 | 先实现固定 3 连击，复杂连击链后续版本再做 |
| **数值平衡困难** | 低 | 中 | 使用 RON 配置文件，方便快速调整伤害、冷却等数值 |

### 验收标准（Constitution 合规）

#### 性能标准

- [ ] **60 FPS 稳定**: 15 个敌人同时战斗，帧时间 <16.67ms
- [ ] **战斗逻辑 <3ms**: 战斗系统执行时间 <3ms（帧预算 18%）
- [ ] **碰撞检测 <3ms**: 物理与碰撞检测 <3ms（帧预算 18%）

#### 代码质量标准

- [ ] **领域层零 Bevy 依赖**: `src/domain/combat/` 无 Bevy import
- [ ] **测试覆盖率 ≥85%**: 战斗核心逻辑（伤害、连击、状态）
- [ ] **测试全绿**: 所有单元测试和集成测试通过
- [ ] **零 unsafe 代码**: 无 unsafe（或有充分文档）
- [ ] **Clippy + fmt 通过**: 无警告、格式正确

#### 架构标准

- [ ] **纯函数领域逻辑**: 伤害计算、连击判定为纯函数
- [ ] **事件驱动**: 使用 Bevy Events（DamageDealt, ComboExtended）
- [ ] **组件纯数据**: Combat 组件无业务逻辑方法
- [ ] **语言分离**: 代码英文、文档中文、配置文件键英文

#### 游戏体验标准

- [ ] **打击感强烈**: 5 名测试玩家评分 ≥7/10（"打击感好"）
- [ ] **连击流畅**: 3 连击无卡顿、取消窗口准确
- [ ] **音效同步**: 打击音效与动画完美同步（误差 <2 帧）
- [ ] **特效不影响性能**: 100 个粒子同时存在，帧率不低于 55 FPS

### 依赖项

- **前置条件**: M1（玩家移动核心）完成
- **技术依赖**: M1 的所有依赖 + RON 配置文件
- **资产依赖**: 攻击动画、敌人精灵、音效、粒子特效

### 下一步行动

1. **M1 完成后**: 运行 `/speckit.specify` 创建 `02-combat-core` 规范
2. **资产准备**: 设计师绘制攻击动画（3 连击）、敌人精灵
3. **音效准备**: 音效师制作打击音效（轻击、重击、技能）

---

## M3: 第一个可玩地下城（3周）

### 目标

实现完整的游戏循环 — 玩家进入地下城，清理 3 个房间，击败 Boss，获得战利品，返回大厅。地下城系统、敌人 AI、房间过渡、战利品系统全部可用。

### 预计工时

- **开发时间**: 15 个工作日（3 周）
- **开发人员**: 2-3 人（系统较多，可并行开发）
- **测试时间**: 已包含在工时内

### 必须生成的 Spec 数量

1. **03-dungeon-system** - 地下城系统规范（房间、过渡、进度）
2. **03-enemy-ai** - 敌人 AI 规范（追逐、攻击、状态机）
3. **03-loot-inventory** - 战利品与库存系统规范
4. **03-boss-encounter** - Boss 战规范

**Spec 位置**: `specs/003-dungeon-system/`

### 核心交付物

#### 代码交付物（领域层）

- [ ] `src/domain/dungeon/progression.rs` - 房间清理逻辑（纯函数）
- [ ] `src/domain/loot/drop_table.rs` - 掉落表逻辑（纯函数）
- [ ] `tests/unit/dungeon_test.rs` - 地下城逻辑单元测试

#### 代码交付物（基础设施层）

- [ ] `src/plugins/dungeon.rs` - 地下城插件
- [ ] `src/plugins/enemy.rs` - 敌人插件
- [ ] `src/plugins/loot.rs` - 战利品插件
- [ ] `src/components/dungeon.rs` - Room, Door, SpawnPoint 组件
- [ ] `src/components/enemy.rs` - Enemy, AIState, AggroRange 组件
- [ ] `src/components/loot.rs` - LootDrop, InventoryItem 组件
- [ ] `src/systems/dungeon_systems.rs` - 房间管理、过渡系统
- [ ] `src/systems/enemy_ai_systems.rs` - 敌人 AI 系统
- [ ] `src/systems/loot_systems.rs` - 战利品掉落、拾取系统
- [ ] `src/systems/boss_systems.rs` - Boss AI 系统
- [ ] `src/events/dungeon.rs` - RoomCleared, BossDefeated, LootDropped 事件
- [ ] `tests/integration/dungeon_run_test.rs` - 完整地下城流程集成测试

#### 资产交付物

- [ ] `assets/levels/dungeon_01_room_01.ldtk` - 地下城第 1 房间关卡文件
- [ ] `assets/levels/dungeon_01_room_02.ldtk` - 第 2 房间
- [ ] `assets/levels/dungeon_01_room_03.ldtk` - 第 3 房间
- [ ] `assets/levels/dungeon_01_boss.ldtk` - Boss 房间
- [ ] `assets/sprites/enemy_goblin.png` - 哥布林敌人（近战）
- [ ] `assets/sprites/enemy_archer.png` - 弓箭手敌人（远程）
- [ ] `assets/sprites/boss_orc.png` - 兽人 Boss（64×64）
- [ ] `assets/sprites/loot_coin.png` - 金币图标（16×16）
- [ ] `assets/sprites/loot_potion.png` - 药水图标
- [ ] `assets/sprites/door_closed.png` - 关闭的门
- [ ] `assets/sprites/door_open.png` - 打开的门
- [ ] `assets/data/enemies.ron` - 敌人数据配置
- [ ] `assets/data/loot_tables.ron` - 掉落表配置

#### 文档交付物

- [ ] `specs/003-dungeon-system/spec.md` - 地下城系统规范（中文）
- [ ] `specs/003-dungeon-system/tests.md` - 测试用例
- [ ] `specs/003-dungeon-system/data-model.md` - 地下城数据模型

### 核心功能点

1. **地下城系统**
   - 多房间结构（3 个普通房间 + 1 个 Boss 房间）
   - 房间过渡（门触发器、加载下一房间）
   - 房间清理检测（所有敌人死亡 → 门打开）
   - 地下城进度跟踪（当前房间、已清理房间数）

2. **敌人 AI 系统**
   - **近战敌人**（哥布林）:
     - 巡逻状态（随机移动）
     - 追逐状态（发现玩家，追赶）
     - 攻击状态（靠近玩家，攻击）
   - **远程敌人**（弓箭手）:
     - 保持距离（与玩家保持 100-200 像素）
     - 射击状态（瞄准、发射箭矢）
   - **AI 状态机**: Idle → Patrol → Aggro → Attack → Death

3. **Boss 战系统**
   - Boss 生命值（普通敌人 5 倍）
   - 阶段机制（生命值 <50% 进入狂暴模式）
   - 特殊攻击（范围攻击、冲锋）
   - Boss 房间锁定（战斗中门关闭）

4. **战利品系统**
   - 敌人死亡掉落（根据掉落表）
   - 战利品拾取（碰撞检测）
   - 库存系统（格子背包，20 格）
   - 战利品类型（金币、药水、装备）

5. **房间过渡**
   - 平滑过渡（<500ms 加载时间）
   - 资产流式加载（当前房间 + 下一房间预加载）
   - 过渡动画（淡入淡出）

6. **UI 系统（基础）**
   - 生命值条（玩家 + 敌人）
   - 技能冷却显示
   - 连击计数器
   - 房间清理提示（"ROOM CLEAR!"）

### 核心风险点

| 风险 | 可能性 | 影响 | 缓解措施 |
|------|--------|------|---------|
| **房间过渡性能** | 高 | 高 | 使用资产流式加载、预加载下一房间、限制单房间实体数 <100 |
| **AI 路径寻找复杂** | 中 | 中 | 简化 AI（直线追逐），避免复杂寻路算法，2D 侧视图不需要 A* |
| **Boss 平衡困难** | 中 | 低 | 使用配置文件快速调整 Boss 数值，多次测试迭代 |
| **战利品系统范围膨胀** | 低 | 中 | 限制 v1.0 战利品类型（仅金币、药水、1-2 种装备） |

### 验收标准（Constitution 合规）

#### 性能标准

- [ ] **60 FPS 稳定**: 15 个敌人 + 玩家 + Boss，帧时间 <16.67ms
- [ ] **房间过渡 <500ms**: 从触发门到进入新房间 <500ms
- [ ] **AI 计算 <2ms**: 所有敌人 AI 更新 <2ms（帧预算 12%）
- [ ] **资产加载异步**: 不阻塞主线程，无卡顿

#### 代码质量标准

- [ ] **测试覆盖率 ≥75%**: 地下城系统（房间逻辑、AI、战利品）
- [ ] **集成测试**: 完整地下城流程测试（进入 → 清理 3 房间 → 击败 Boss）
- [ ] **零 unsafe 代码**: 无 unsafe（或有充分文档）
- [ ] **Clippy + fmt 通过**: 无警告

#### 架构标准

- [ ] **领域逻辑分离**: 房间清理逻辑、掉落表逻辑为纯函数
- [ ] **事件驱动**: RoomCleared, BossDefeated 事件触发后续逻辑
- [ ] **模块化**: 地下城、敌人、战利品各自独立插件
- [ ] **语言分离**: 代码英文、文档中文、UI 文本中文

#### 游戏体验标准

- [ ] **完整游戏循环**: 玩家可完成一次完整地下城（5-10 分钟）
- [ ] **敌人 AI 有挑战性**: 5 名测试玩家中 ≥3 人认为"有挑战但可战胜"
- [ ] **Boss 战有趣**: 测试玩家评分 ≥6/10（"Boss 战有意思"）
- [ ] **战利品有奖励感**: 击败 Boss 后玩家获得明显奖励（金币 + 装备）

### 依赖项

- **前置条件**: M1（移动）+ M2（战斗）完成
- **技术依赖**: bevy_ecs_ldtk（关卡加载）
- **资产依赖**: 关卡文件、敌人精灵、Boss 精灵、UI 元素

### 下一步行动

1. **M2 完成后**: 运行 `/speckit.specify` 创建 `03-dungeon-system` 规范
2. **关卡设计**: 使用 LDtk 编辑器设计 4 个房间布局
3. **Boss 设计**: 设计 Boss 技能和阶段机制

---

## M4: 装备与技能树（2周）

### 目标

实现角色成长系统 — 玩家可以装备武器和防具、升级技能、分配技能点。装备提供属性加成，技能树提供技能解锁和强化。

### 预计工时

- **开发时间**: 10 个工作日（2 周）
- **开发人员**: 1-2 人
- **测试时间**: 已包含在工时内

### 必须生成的 Spec 数量

1. **04-equipment-system** - 装备系统规范
2. **04-skill-tree** - 技能树系统规范
3. **04-character-progression** - 角色成长规范

**Spec 位置**: `specs/004-progression/`

### 核心交付物

#### 代码交付物（领域层）

- [ ] `src/domain/equipment/stats.rs` - 属性计算（纯函数）
- [ ] `src/domain/skills/skill_tree.rs` - 技能树逻辑（纯函数）
- [ ] `tests/unit/stats_test.rs` - 属性计算单元测试

#### 代码交付物（基础设施层）

- [ ] `src/plugins/equipment.rs` - 装备插件
- [ ] `src/plugins/skill_tree.rs` - 技能树插件
- [ ] `src/components/equipment.rs` - Equipment, EquipmentSlot 组件
- [ ] `src/components/skills.rs` - SkillTree, SkillNode 组件
- [ ] `src/systems/equipment_systems.rs` - 装备更换、属性计算系统
- [ ] `src/systems/skill_systems.rs` - 技能解锁、技能点分配系统
- [ ] `src/resources/equipment_database.rs` - 装备数据库资源
- [ ] `src/resources/skill_database.rs` - 技能数据库资源
- [ ] `tests/integration/equipment_test.rs` - 装备系统集成测试
- [ ] `tests/integration/skill_tree_test.rs` - 技能树集成测试

#### 资产交付物

- [ ] `assets/sprites/ui_inventory.png` - 库存界面（背包格子）
- [ ] `assets/sprites/ui_equipment.png` - 装备界面（装备槽位）
- [ ] `assets/sprites/ui_skill_tree.png` - 技能树界面
- [ ] `assets/sprites/equipment_sword_01.png` - 剑图标（16×16）
- [ ] `assets/sprites/equipment_armor_01.png` - 护甲图标
- [ ] `assets/sprites/skill_icon_fireball.png` - 技能图标
- [ ] `assets/data/equipment.ron` - 装备数据配置
- [ ] `assets/data/skill_tree.ron` - 技能树数据配置

#### 文档交付物

- [ ] `specs/004-progression/spec.md` - 成长系统规范（中文）
- [ ] `specs/004-progression/tests.md` - 测试用例
- [ ] `specs/004-progression/data-model.md` - 数据模型

### 核心功能点

1. **装备系统**
   - **装备槽位**: 武器、头盔、护甲、饰品（4 个槽位）
   - **装备更换**: 从背包拖拽到槽位、卸下装备
   - **属性加成**: 装备提供攻击力、防御力、生命值、暴击率等加成
   - **装备品质**: 普通（白）、稀有（蓝）、史诗（紫）

2. **技能树系统**
   - **技能点获得**: 每升 1 级获得 1 技能点
   - **技能解锁**: 消耗技能点解锁技能节点
   - **技能前置**: 某些技能需要先解锁前置技能
   - **技能强化**: 已解锁技能可继续升级（增加伤害、减少冷却）

3. **角色成长**
   - **经验值系统**: 击败敌人获得经验值
   - **升级系统**: 经验值满后升级，提升基础属性
   - **属性面板**: 显示当前属性（攻击、防御、暴击等）
   - **属性计算**: 基础属性 + 装备加成 + 技能加成

4. **UI 界面**
   - **库存界面**: 格子背包（20 格），显示物品图标和数量
   - **装备界面**: 装备槽位 + 属性面板
   - **技能树界面**: 树状结构，显示可解锁和已解锁技能

### 核心风险点

| 风险 | 可能性 | 影响 | 缓解措施 |
|------|--------|------|---------|
| **数值平衡困难** | 高 | 中 | 使用 RON 配置文件，快速迭代数值；参考 DNF 成长曲线 |
| **技能树 UI 复杂** | 中 | 低 | 简化技能树（线性树，不分支），v1.0 仅 10 个技能节点 |
| **属性计算性能** | 低 | 低 | 仅在装备更换时重新计算，不每帧计算 |
| **UI 输入冲突** | 低 | 中 | 使用 Bevy UI 状态管理，明确区分游戏输入和 UI 输入 |

### 验收标准（Constitution 合规）

#### 性能标准

- [ ] **UI 响应 <100ms**: 打开装备界面、切换装备 <100ms
- [ ] **属性计算 <1ms**: 装备更换后重新计算属性 <1ms
- [ ] **不影响 60 FPS**: UI 显示时游戏仍保持 60 FPS

#### 代码质量标准

- [ ] **测试覆盖率 ≥75%**: 装备系统、技能树系统
- [ ] **属性计算测试**: 验证装备加成、技能加成正确累加
- [ ] **零 unsafe 代码**: 无 unsafe（或有充分文档）
- [ ] **Clippy + fmt 通过**: 无警告

#### 架构标准

- [ ] **领域逻辑纯函数**: 属性计算、技能树逻辑为纯函数
- [ ] **数据驱动**: 装备、技能数据全部来自 RON 配置文件
- [ ] **UI 与逻辑分离**: UI 组件仅负责显示，逻辑在系统中
- [ ] **语言分离**: 代码英文、UI 文本中文、配置键英文

#### 游戏体验标准

- [ ] **成长感明显**: 升级后玩家明显感受到变强（伤害提升 ≥20%）
- [ ] **装备影响战斗**: 更换装备后战斗体验有区别
- [ ] **技能树易懂**: 新玩家看懂技能树结构 <1 分钟
- [ ] **数值合理**: 测试玩家评分 ≥6/10（"数值平衡合理"）

### 依赖项

- **前置条件**: M3（地下城系统）完成
- **技术依赖**: Bevy UI 系统、RON 配置文件
- **资产依赖**: UI 界面、装备图标、技能图标

### 下一步行动

1. **M3 完成后**: 运行 `/speckit.specify` 创建 `04-equipment-system` 规范
2. **数值设计**: 设计装备数值表、技能树结构
3. **UI 设计**: 设计库存、装备、技能树界面原型

---

## M5: 多人联机大厅（3周）

### 目标

实现 2-4 人合作联机功能 — 玩家可以创建大厅、加入大厅、联机进入地下城。战斗同步、延迟补偿、服务器权威验证全部可用。

### 预计工时

- **开发时间**: 15 个工作日（3 周）
- **开发人员**: 2 人（网络系统复杂，需专人负责）
- **测试时间**: 已包含在工时内（需更多网络测试）

### 必须生成的 Spec 数量

1. **05-networking-core** - 网络核心规范（连接、同步、协议）
2. **05-lobby-system** - 大厅系统规范（创建、加入、准备）
3. **05-combat-sync** - 战斗同步规范（服务器权威、预测）
4. **05-latency-compensation** - 延迟补偿规范

**Spec 位置**: `specs/005-multiplayer/`

### 核心交付物

#### 代码交付物（网络层）

- [ ] `src/plugins/networking.rs` - 网络插件
- [ ] `src/plugins/lobby.rs` - 大厅插件
- [ ] `src/components/network.rs` - NetworkPlayer, NetworkId, SyncState 组件
- [ ] `src/systems/network_systems.rs` - 网络连接、心跳、断线重连
- [ ] `src/systems/sync_systems.rs` - 状态同步系统
- [ ] `src/systems/prediction_systems.rs` - 客户端预测系统
- [ ] `src/systems/rollback_systems.rs` - 回滚系统（可选）
- [ ] `src/resources/network_config.rs` - 网络配置（服务器地址、端口）
- [ ] `src/events/network.rs` - PlayerJoined, PlayerLeft, NetworkStateUpdated 事件
- [ ] `tests/integration/network_test.rs` - 网络系统集成测试

#### 代码交付物（服务器）

- [ ] `server/main.rs` - 游戏服务器入口
- [ ] `server/lobby_manager.rs` - 大厅管理器
- [ ] `server/game_session.rs` - 游戏会话管理
- [ ] `server/authoritative_combat.rs` - 服务器权威战斗验证

#### 资产交付物

- [ ] `assets/sprites/ui_lobby.png` - 大厅界面
- [ ] `assets/sprites/ui_player_slot.png` - 玩家槽位（显示玩家头像）
- [ ] `assets/data/network_config.ron` - 网络配置文件

#### 文档交付物

- [ ] `specs/005-multiplayer/spec.md` - 联机系统规范（中文）
- [ ] `specs/005-multiplayer/tests.md` - 网络测试用例
- [ ] `specs/005-multiplayer/network-protocol.md` - 网络协议文档

### 核心功能点

1. **大厅系统**
   - **创建大厅**: 主机创建房间，设置房间名称、最大人数（2-4 人）
   - **加入大厅**: 客户端输入房间 ID 或从列表选择加入
   - **玩家列表**: 显示所有玩家、准备状态
   - **准备系统**: 所有玩家准备后，主机可开始游戏
   - **踢人功能**: 主机可踢出玩家

2. **网络核心**
   - **连接管理**: 客户端与服务器建立连接、维持心跳
   - **断线重连**: 客户端断线后 30 秒内可重连
   - **状态同步**: 玩家位置、生命值、技能状态同步
   - **帧同步 vs 状态同步**: 使用状态同步（更适合动作游戏）

3. **客户端预测**
   - **本地玩家预测**: 本地玩家输入立即响应，不等服务器确认
   - **平滑插值**: 远程玩家位置平滑插值（避免瞬移）
   - **预测回滚**: 服务器纠正错误预测时，平滑回滚

4. **服务器权威战斗**
   - **伤害验证**: 客户端请求攻击，服务器验证碰撞、计算伤害
   - **防作弊**: 服务器检查攻击范围、冷却时间、资源消耗
   - **同步结果**: 服务器广播伤害结果给所有客户端

5. **延迟补偿**
   - **输入延迟 <100ms**: 本地预测使输入感觉无延迟
   - **插值缓冲**: 远程玩家位置使用 100ms 插值缓冲
   - **延迟显示**: UI 显示当前网络延迟（ping）

### 核心风险点

| 风险 | 可能性 | 影响 | 缓解措施 |
|------|--------|------|---------|
| **网络同步 bug** | 高 | 高 | 大量集成测试、模拟高延迟环境（100ms、200ms）、使用成熟网络库 |
| **服务器性能** | 中 | 高 | 限制单服务器房间数（最多 10 个房间）、优化同步频率（20 Hz） |
| **客户端预测错误** | 中 | 中 | 使用保守预测策略、服务器纠正时平滑回滚 |
| **作弊风险** | 低 | 中 | 服务器权威验证所有战斗行为、客户端不信任原则 |

### 验收标准（Constitution 合规）

#### 性能标准

- [ ] **延迟 <100ms 体验良好**: 4 玩家联机，ping <100ms 时战斗流畅
- [ ] **同步频率 20 Hz**: 服务器每秒同步 20 次状态（50ms 间隔）
- [ ] **客户端 60 FPS**: 联机时客户端仍保持 60 FPS
- [ ] **服务器 CPU <50%**: 单核心 CPU 占用 <50%（10 个房间）

#### 代码质量标准

- [ ] **网络测试覆盖率 ≥60%**: 网络系统、大厅系统
- [ ] **模拟延迟测试**: 测试 50ms、100ms、150ms 延迟场景
- [ ] **断线重连测试**: 验证客户端断线后可重连
- [ ] **零 unsafe 代码**: 无 unsafe（或有充分文档）
- [ ] **Clippy + fmt 通过**: 无警告

#### 架构标准

- [ ] **服务器权威**: 所有战斗逻辑服务器验证
- [ ] **客户端预测**: 本地玩家输入立即响应
- [ ] **事件驱动**: 使用 Bevy Events（PlayerJoined, PlayerLeft）
- [ ] **语言分离**: 代码英文、UI 文本中文、协议字段英文

#### 游戏体验标准

- [ ] **联机流程顺畅**: 5 名测试玩家中 ≥4 人可成功创建/加入大厅
- [ ] **延迟 <100ms 无感知**: 测试玩家评分 ≥7/10（"联机体验流畅"）
- [ ] **无明显 bug**: <5 个网络相关 bug 报告
- [ ] **同步准确**: 玩家位置、生命值、技能冷却同步误差 <5%

### 依赖项

- **前置条件**: M1-M4 全部完成（单机游戏完全可玩）
- **技术依赖**: 网络库（bevy_quinnet 或 bevy_renet）
- **基础设施依赖**: 游戏服务器部署（可本地测试）

### 下一步行动

1. **M4 完成后**: 运行 `/speckit.specify` 创建 `05-networking-core` 规范
2. **网络库选择**: 研究 bevy_quinnet vs bevy_renet（1 天）
3. **服务器部署**: 准备测试服务器环境（云服务器或本地）

---

## M6: 公开 Demo 打磨（2周）

### 目标

将所有功能整合、优化、打磨，准备公开 Demo 发布。跨平台构建、教程系统、性能优化、bug 修复、本地化全部完成。

### 预计工时

- **开发时间**: 10 个工作日（2 周）
- **开发人员**: 2-3 人（全员打磨）
- **测试时间**: 重度测试（包含在工时内）

### 必须生成的 Spec 数量

1. **06-tutorial-system** - 教程系统规范
2. **06-optimization** - 性能优化规范
3. **06-localization** - 本地化规范（中文 + 英文）
4. **06-cross-platform** - 跨平台构建规范

**Spec 位置**: `specs/006-public-demo/`

### 核心交付物

#### 代码交付物

- [ ] `src/plugins/tutorial.rs` - 教程插件
- [ ] `src/systems/tutorial_systems.rs` - 教程提示、新手引导系统
- [ ] `src/resources/localization.rs` - 本地化资源
- [ ] `src/systems/performance_monitoring.rs` - 性能监控系统（开发模式）
- [ ] `benches/full_game_bench.rs` - 完整游戏性能基准测试
- [ ] `tests/e2e/full_game_test.rs` - 端到端测试（完整游戏流程）

#### 资产交付物

- [ ] `assets/sprites/ui_tutorial.png` - 教程提示界面
- [ ] `assets/locales/zh-CN.ftl` - 中文本地化文件
- [ ] `assets/locales/en-US.ftl` - 英文本地化文件
- [ ] `assets/audio/music_main_menu.ogg` - 主菜单音乐
- [ ] `assets/audio/music_dungeon.ogg` - 地下城背景音乐

#### 构建交付物

- [ ] `builds/windows/RustShadowDungeon.exe` - Windows 构建
- [ ] `builds/linux/rustshadowdungeon` - Linux 构建
- [ ] `builds/macos/RustShadowDungeon.app` - macOS 构建
- [ ] `builds/web/index.html` - WASM 构建
- [ ] `builds/android/rustshadowdungeon.apk` - Android 构建

#### 文档交付物

- [ ] `README.md` - 项目说明（更新）
- [ ] `CHANGELOG.md` - v1.0 更新日志
- [ ] `specs/006-public-demo/spec.md` - 公开 Demo 规范（中文）
- [ ] `specs/006-public-demo/release-checklist.md` - 发布检查清单

### 核心功能点

1. **教程系统**
   - **移动教程**: 新玩家首次进入，提示 WASD 移动、空格跳跃
   - **战斗教程**: 提示攻击键、技能键、连击系统
   - **UI 教程**: 提示如何打开背包、装备界面、技能树
   - **分步提示**: 教程分步进行，玩家完成后自动进入下一步

2. **性能优化**
   - **实体池化**: 子弹、粒子特效使用对象池（避免频繁 spawn/despawn）
   - **视锥剔除**: 屏幕外实体不渲染、不更新
   - **资产预加载**: 关键资产启动时预加载
   - **内存优化**: 使用 `cargo-bloat` 分析二进制大小，移除未使用依赖

3. **跨平台构建**
   - **Windows**: x86_64-pc-windows-msvc
   - **Linux**: x86_64-unknown-linux-gnu
   - **macOS**: aarch64-apple-darwin + x86_64-apple-darwin（通用二进制）
   - **WASM**: wasm32-unknown-unknown
   - **Android**: aarch64-linux-android

4. **本地化**
   - **中文**: 所有 UI 文本、教程、错误提示
   - **英文**: 所有 UI 文本翻译为英文（面向国际社区）
   - **语言切换**: 设置界面可切换语言

5. **Bug 修复**
   - **收集 Beta 测试反馈**: 从 M5 后开始 Beta 测试，收集 bug 报告
   - **优先修复**: P0（崩溃）、P1（卡死）、P2（功能 bug）
   - **低优先级延后**: P3（小问题）可延后到 v1.1

6. **发布准备**
   - **Steam 页面**: 准备 Steam 商店页面（截图、视频、描述）
   - **Itch.io 页面**: 准备 Itch.io 页面
   - **GitHub Release**: 创建 v1.0 Release，上传构建包
   - **宣传素材**: 预告片、游戏截图、GIF

### 核心风险点

| 风险 | 可能性 | 影响 | 缓解措施 |
|------|--------|------|---------|
| **移动端性能不达标** | 中 | 高 | 提前在 Android 设备测试，必要时降低粒子效果、减少敌人数量 |
| **跨平台 bug** | 中 | 中 | 在所有平台测试，提前发现平台特定 bug |
| **发布时间延期** | 中 | 中 | 预留 1 周缓冲时间，优先级低的 bug 可延后修复 |
| **WASM 构建失败** | 低 | 中 | 使用 Bevy 官方 WASM 示例作为基础，避免不兼容依赖 |

### 验收标准（Constitution 合规）

#### 性能标准

- [ ] **所有平台 60 FPS**: Windows/Linux/macOS 稳定 60 FPS
- [ ] **WASM 55 FPS**: Web 版本 ≥55 FPS（允许略低）
- [ ] **Android 60 FPS**: 中端 Android 设备（Snapdragon 750G）≥60 FPS
- [ ] **内存占用 <512MB**: 桌面版内存 <512MB
- [ ] **加载时间 <5 秒**: 从启动到主菜单 <5 秒

#### 代码质量标准

- [ ] **零崩溃**: Beta 测试无崩溃报告
- [ ] **零 P0/P1 bug**: 无严重 bug（崩溃、卡死、功能不可用）
- [ ] **P2 bug <10 个**: 功能 bug <10 个
- [ ] **测试全绿**: 所有单元测试、集成测试、端到端测试通过
- [ ] **Clippy + fmt 通过**: 无警告

#### 用户体验标准

- [ ] **新玩家完成教程 <5 分钟**: 5 名新玩家中 ≥4 人 <5 分钟完成教程
- [ ] **完整游戏流程顺畅**: 从启动到完成一次地下城，无卡顿、无 bug
- [ ] **用户评分 ≥7/10**: Beta 测试玩家平均评分 ≥7/10
- [ ] **本地化完整**: 中文、英文 UI 文本全部翻译，无遗漏

#### 发布标准

- [ ] **所有平台构建成功**: Windows/Linux/macOS/WASM/Android 全部构建通过
- [ ] **发布材料齐全**: Steam 页面、Itch.io 页面、GitHub Release 准备完毕
- [ ] **宣传素材完成**: 预告片、截图、GIF 准备完毕
- [ ] **社区准备**: Discord 服务器创建、GitHub Discussions 开启

### 依赖项

- **前置条件**: M1-M5 全部完成
- **技术依赖**: 跨平台构建工具（cargo-build-deps）
- **资产依赖**: 教程界面、本地化文本、音乐

### 下一步行动

1. **M5 完成后**: 开始 Beta 测试，收集反馈
2. **构建测试**: 在所有目标平台测试构建
3. **发布准备**: 准备 Steam/Itch.io 页面、宣传素材

---

## 总体时间线与关键路径

### 甘特图（简化版）

```
周次 | 里程碑              | 关键交付物
-----|--------------------|-----------------------------------------
W1   | M1 玩家移动核心     | 玩家移动、输入、物理
W2   | M1 玩家移动核心     | 完成 M1 验收
W3   | M2 战斗系统基础     | 伤害计算、碰撞检测
W4   | M2 战斗系统基础     | 连击系统、打击感
W5   | M2 战斗系统基础     | 完成 M2 验收
W6   | M3 第一个可玩地下城 | 地下城系统、房间过渡
W7   | M3 第一个可玩地下城 | 敌人 AI、战利品
W8   | M3 第一个可玩地下城 | 完成 M3 验收（可玩游戏）
W9   | M4 装备与技能树     | 装备系统、属性计算
W10  | M4 装备与技能树     | 完成 M4 验收
W11  | M5 多人联机大厅     | 网络核心、大厅系统
W12  | M5 多人联机大厅     | 客户端预测、服务器权威
W13  | M5 多人联机大厅     | 完成 M5 验收（联机可玩）
W14  | M6 公开 Demo 打磨   | 教程、优化、跨平台构建
W15  | M6 公开 Demo 打磨   | Bug 修复、发布准备
W16  | 🎉 v1.0 公开发布    | 🚀 公开 Demo 上线
```

### 关键路径

**关键路径（Critical Path）**: M1 → M2 → M3（阻塞后续）

- **M1 延期 → 全局延期**: M1 是基础，延期会导致所有后续延期
- **M2 延期 → 部分延期**: M2 是战斗核心，延期影响 M3
- **M3 延期 → 可接受**: M3 可玩游戏，延期影响不大（M4/M5 可并行）
- **M4/M5 可并行**: 装备系统和联机系统可部分并行开发
- **M6 缓冲时间**: 预留 1 周缓冲，应对意外延期

### 并行开发机会

- **M3 + 资产制作**: 程序员开发地下城系统，美术同时制作敌人精灵
- **M4 + M5 部分并行**: 装备系统不依赖联机，可由不同开发者并行
- **M6 全员**: 最后 2 周全员打磨、测试、修复 bug

---

## 风险管理与应急预案

### 高风险项

| 风险项 | 可能性 | 影响 | 应急预案 |
|--------|--------|------|---------|
| **M1 bevy_tnua 集成失败** | 中 | 高 | 回退到手写字符控制器（增加 3 天工时） |
| **M2 打击感不理想** | 高 | 中 | 参考 DNF 参数快速迭代（预留 2 天调优时间） |
| **M5 网络同步 bug 多** | 高 | 高 | 增加 1 周测试和修复时间（总工时 16 周） |
| **M6 移动端性能不达标** | 中 | 高 | 降级处理（减少粒子、降低敌人数量） |

### 应急预案

1. **时间延期应对**:
   - **延期 1 周**: 削减 M4 装备种类（仅 2 种装备）
   - **延期 2 周**: 推迟 M5 联机功能至 v1.1
   - **延期 >2 周**: 重新评估里程碑优先级

2. **人力不足应对**:
   - **仅 1 人**: 推迟 M5 至 v1.1，专注单机体验
   - **无美术**: 使用开源像素素材包（Kenney、OpenGameArt）

3. **技术难题应对**:
   - **bevy_tnua 不可用**: 手写字符控制器
   - **网络库问题**: 切换备选方案（bevy_quinnet ↔ bevy_renet）
   - **WASM 构建失败**: 先发布桌面版，WASM 延后

---

## Constitution 合规性检查清单

每个里程碑完成后必须通过以下检查：

### 原则 I-VIII 合规检查

- [ ] **I. Rust Memory Safety**: 零 unsafe（或有文档说明）
- [ ] **II. Bevy ECS Architecture**: 组件纯数据、系统纯行为
- [ ] **III. 60 FPS Performance**: 帧时间 <16.67ms（≥95% 帧）
- [ ] **IV. Pixel Art Consistency**: 16×16 网格、无旋转缩放
- [ ] **V. Combat Mechanics Testing**: 战斗测试覆盖率 ≥85%
- [ ] **VI. Open Source MIT License**: 所有依赖 MIT 兼容
- [ ] **VII. Modular Design**: 系统独立可测试、无循环依赖
- [ ] **VIII. Language Separation Rule**: 代码英文、文档中文

### CI 自动化检查

```bash
# 每次提交自动运行
cargo fmt --check          # 格式检查
cargo clippy -- -D warnings # 零警告
cargo test                 # 测试全绿
cargo bench --no-run       # 基准测试编译通过

# 每周运行一次
cargo tarpaulin            # 测试覆盖率报告
cargo bloat                # 二进制大小分析
```

---

## 发布检查清单（M6 最终验收）

### 技术检查

- [ ] 所有平台构建成功（Windows/Linux/macOS/WASM/Android）
- [ ] 性能达标（60 FPS 所有平台）
- [ ] 测试全绿（零失败）
- [ ] 零 P0/P1 bug
- [ ] Constitution 全部原则合规

### 内容检查

- [ ] 教程完整（新玩家可独立完成）
- [ ] 本地化完整（中文 + 英文）
- [ ] 3 个地下城可玩
- [ ] 联机功能可用（2-4 人）
- [ ] 装备与技能树可用

### 发布材料

- [ ] README.md 更新
- [ ] CHANGELOG.md 完成
- [ ] Steam 页面准备完毕
- [ ] Itch.io 页面准备完毕
- [ ] GitHub Release 创建
- [ ] 预告片 + 截图 + GIF
- [ ] Discord 服务器创建

---

## 下一步行动（立即开始）

### 立即执行（Week 1, Day 1）

1. **创建 M1 规范**:
   ```bash
   /speckit.specify 实现玩家角色移动系统（WASD + 手柄），使用 bevy_tnua 字符控制器和 bevy_rapier2d 物理引擎。必须支持地面移动、跳跃、空中控制，像素完美渲染（16×16 网格锁定）。遵守 Constitution v1.0.1。
   ```

2. **创建里程碑 Spec 文件夹**:
   ```bash
   mkdir -p specs/{001-movement,002-combat,003-dungeon,004-progression,005-multiplayer,006-public-demo}
   ```

3. **技术预研**（1 天）:
   - 研究 bevy_tnua 文档和示例
   - 研究 Bevy 像素完美渲染示例
   - 搭建项目骨架（Cargo.toml、基础插件）

4. **资产准备**（并行）:
   - 设计师开始绘制玩家精灵（待机、行走、跳跃）
   - 准备测试用地形 Tilemap

### 里程碑启动顺序

1. **M1 启动**: 2025-11-25（立即）
2. **M2 启动**: M1 完成后（预计 2 周后）
3. **M3 启动**: M2 完成后（预计 5 周后）
4. **M4/M5 启动**: M3 完成后（预计 8 周后，可部分并行）
5. **M6 启动**: M4+M5 完成后（预计 13 周后）
6. **v1.0 发布**: 预计 15 周后（2025-03 月初）

---

## 附录

### A. 技术栈版本锁定

| 依赖项 | 版本 | 用途 |
|--------|------|------|
| Rust | 1.91.1 (stable) | 编程语言 |
| Bevy | 0.17.0 | 游戏引擎 |
| bevy_rapier2d | 0.29+ | 2D 物理引擎 |
| bevy_tnua | latest stable | 字符控制器 |
| leafwing-input-manager | latest stable | 输入抽象 |
| bevy_ecs_ldtk | latest stable | LDtk 关卡加载 |
| serde | latest | 序列化 |
| ron | latest | 数据格式 |
| criterion | latest | 性能基准测试 |

### B. 开发环境要求

- **操作系统**: Windows 10+, macOS 12+, Ubuntu 20.04+
- **Rust 工具链**: rustup, cargo, rustfmt, clippy
- **IDE**: VSCode + rust-analyzer（推荐）或 RustRover
- **版本控制**: Git 2.30+
- **资产工具**: LDtk（关卡编辑）、Aseprite（像素绘制）

### C. 参考资源

- **Bevy 官方文档**: https://bevyengine.org/learn/
- **Bevy 像素完美示例**: https://github.com/bevyengine/bevy/tree/main/examples/2d/pixel_perfect
- **bevy_tnua 示例**: https://github.com/idanarye/bevy-tnua
- **DNF 机制参考**: Namu Wiki (DNF), DFO Global Wiki

---

**文档状态**: ✅ 已完成
**最后更新**: 2025-11-24
**负责人**: 项目负责人
**审核状态**: 待审核

此计划为《锈影地下城》项目调度唯一真相。所有开发工作必须遵循此计划，如需调整必须更新此文档并通知全体成员。

