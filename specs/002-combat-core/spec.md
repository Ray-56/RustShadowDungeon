# Feature Specification: 战斗系统核心 (Combat System Core)

**Feature Branch**: `002-combat-core`  
**Created**: 2025-11-25  
**Status**: Draft  
**Constitution**: v1.0.1 (Language Separation Rule enforced)  
**Milestone**: M2 - 战斗系统基础（3周）  
**依赖**: M1 (001-player-movement) 已完成

---

**语言规范说明（Language Guidelines）**:
- 本文档使用中文（This document uses Chinese for Chinese projects）
- 代码示例使用英文标识符（Code examples use English identifiers）
- 技术术语保持英文（Technical terms remain in English）

---

## 执行摘要

实现 DNF 风格的战斗系统核心，包括：
- **伤害计算系统**：纯函数实现，支持基础伤害、暴击、元素伤害、防御减伤
- **碰撞检测系统**：HitBox vs HurtBox，支持多段攻击、穿透、无敌帧
- **连击系统**：3连击（轻-轻-重），连击取消，连击计数器
- **打击感反馈**：Hitfreeze（打击定格）、屏幕震动、粒子特效、音效同步
- **基础技能系统**：技能冷却管理、MP资源消耗、技能动画锁定、技能取消窗口
- **简单敌人**：史莱姆敌人（16×16），生命值显示，受击反馈，死亡动画

**核心架构约束**：
- 领域层（`src/domain/combat/`）**零 Bevy 依赖**（纯函数）
- 基础设施层（`src/infrastructure/systems/`）桥接 Bevy ECS
- 测试覆盖率 ≥85%（战斗核心逻辑）
- 60 FPS 性能目标（战斗逻辑 <3ms，碰撞检测 <3ms）

---

## User Scenarios & Testing *(mandatory)*

### User Story 1 - 基础攻击与伤害 (Priority: P1)

**场景描述**：玩家可以使用基础攻击击败敌人，看到明确的伤害反馈。

**为什么是 P1**：这是战斗系统的最小可行功能（MVP）。没有攻击和伤害，就没有战斗。这是其他所有战斗功能的基础。

**独立测试**：可以通过生成一个玩家和一个史莱姆敌人，玩家按攻击键，史莱姆受到伤害并死亡来完全测试。无需连击、技能或其他复杂系统。

**Acceptance Scenarios**:

1. **Given** 玩家在地面，史莱姆在攻击范围内，**When** 玩家按下攻击键（J键），**Then**：
   - 玩家播放攻击动画
   - HitBox 在玩家前方生成（32×32 像素）
   - HitBox 与史莱姆的 HurtBox 碰撞
   - 伤害值计算（基础伤害 = 10）
   - 史莱姆生命值减少 10
   - 史莱姆播放受击动画（闪白效果）
   - 屏幕出现伤害数字 "10"（浮动文字）

2. **Given** 史莱姆生命值 = 10，**When** 玩家攻击造成 10 伤害，**Then**：
   - 史莱姆生命值归零
   - 史莱姆播放死亡动画
   - 史莱姆实体在动画结束后 despawn
   - 可选：掉落战利品（金币/物品）

3. **Given** 玩家攻击 HitBox 未与敌人重叠，**When** 玩家挥空攻击，**Then**：
   - 攻击动画正常播放
   - 无伤害事件触发
   - 无受击反馈
   - 攻击冷却正常开始

---

### User Story 2 - 3连击系统 (Priority: P1)

**场景描述**：玩家可以通过连续按攻击键执行 3 连击（轻-轻-重），最后一击造成更高伤害并有击退效果。

**为什么是 P1**：连击系统是 DNF 战斗的核心机制，定义了战斗的节奏感和流畅度。这是玩家最常用的攻击方式。

**独立测试**：可以通过测试玩家在 1 秒内连续按 3 次攻击键，验证是否触发完整连击，以及最后一击是否造成额外伤害来完全测试。

**Acceptance Scenarios**:

1. **Given** 玩家未在攻击状态，**When** 玩家按攻击键 1 次，**Then**：
   - 播放轻击动画（Light Attack 1）
   - 造成 10 点基础伤害
   - 进入连击窗口（1 秒内可接第 2 击）

2. **Given** 玩家在连击窗口内（第 1 击后 0.5 秒），**When** 玩家按攻击键第 2 次，**Then**：
   - 第 1 击动画取消
   - 播放第 2 轻击动画（Light Attack 2）
   - 造成 10 点基础伤害
   - 连击计数器显示 "2 HIT COMBO"
   - 连击窗口刷新（再次 1 秒内可接第 3 击）

3. **Given** 玩家在连击窗口内（第 2 击后 0.5 秒），**When** 玩家按攻击键第 3 次，**Then**：
   - 第 2 击动画取消
   - 播放重击动画（Heavy Attack）
   - 造成 20 点伤害（2 倍）
   - 敌人被击退 50 像素
   - 连击计数器显示 "3 HIT COMBO"
   - 连击结束，重置到初始状态

4. **Given** 玩家完成第 1 击，**When** 超过 1 秒未按第 2 击，**Then**：
   - 连击窗口关闭
   - 下次攻击重新从第 1 击开始
   - 连击计数器隐藏

---

### User Story 3 - 打击感反馈 (Priority: P1)

**场景描述**：玩家攻击命中敌人时，感受到强烈的打击感反馈（视觉、听觉、触觉）。

**为什么是 P1**：打击感是 DNF 战斗系统的灵魂。没有打击感，战斗会感觉"软弱无力"，即使数值正确也无法提供满意的体验。

**独立测试**：可以通过攻击史莱姆，观察是否出现 hitfreeze（定格 3 帧）、屏幕震动、粒子特效、音效播放来完全测试。

**Acceptance Scenarios**:

1. **Given** 玩家攻击命中史莱姆，**When** 伤害确认时，**Then**：
   - **Hitfreeze**：游戏定格 3 帧（50ms，3/60秒）
   - **屏幕震动**：相机轻微震动（振幅 2 像素，持续 100ms）
   - **粒子特效**：命中点生成火花粒子（8×8，白色/橙色）
   - **音效**：播放轻击音效（`hit_light.ogg`）
   - **伤害数字**：命中点上方浮现伤害数字（向上飘动，1 秒后淡出）

2. **Given** 玩家重击（连击第 3 击）命中，**When** 伤害确认时，**Then**：
   - **Hitfreeze**：游戏定格 5 帧（83ms）
   - **屏幕震动**：振幅更大（4 像素，持续 150ms）
   - **粒子特效**：更多粒子（15-20 个）
   - **音效**：播放重击音效（`hit_heavy.ogg`）
   - **伤害数字**：更大的字体（1.5 倍大小），红色

3. **Given** 玩家暴击命中，**When** 暴击触发时，**Then**：
   - 伤害数字显示为黄色
   - 粒子特效为金色
   - 播放特殊音效（`hit_critical.ogg`）
   - 伤害数字字体更大（2 倍）

---

### User Story 4 - 基础技能系统 (Priority: P2)

**场景描述**：玩家可以使用 2-3 个技能（如火球术），技能有冷却时间和 MP 消耗。

**为什么是 P2**：技能系统丰富了战斗策略，但基础攻击已经可以构成完整战斗循环。技能是锦上添花，不是必需品（MVP）。

**独立测试**：可以通过按技能键（K键），观察火球发射、MP 消耗、冷却计时器来完全测试。无需依赖其他复杂系统。

**Acceptance Scenarios**:

1. **Given** 玩家 MP = 100，技能"火球术"冷却完毕，**When** 玩家按技能键（K键），**Then**：
   - 玩家播放施法动画（0.3 秒）
   - 玩家进入动画锁定状态（无法移动/攻击）
   - 消耗 20 MP（MP 减至 80）
   - 火球弹道生成（从玩家位置向前飞行）
   - 技能进入冷却（5 秒）
   - UI 显示冷却计时器（圆形进度条，5 → 0 秒）

2. **Given** 火球飞行中，**When** 火球碰撞到敌人，**Then**：
   - 造成 30 点火元素伤害
   - 播放火球爆炸动画
   - 火球弹道 despawn
   - 触发打击感反馈（hitfreeze、音效、粒子）

3. **Given** 玩家 MP = 10（不足 20），**When** 玩家按技能键，**Then**：
   - 技能不触发
   - 播放"MP不足"音效
   - UI 闪烁 MP 条（红色闪烁）

4. **Given** 技能冷却中（剩余 2 秒），**When** 玩家按技能键，**Then**：
   - 技能不触发
   - UI 闪烁冷却计时器

---

### User Story 5 - 技能取消机制 (Priority: P3)

**场景描述**：玩家可以在连击中取消进入技能，增加战斗流畅度和连招可能性。

**为什么是 P3**：这是高级战斗机制，提升战斗深度，但不影响基础战斗体验。可以在 v1.1 或更晚实现。

**独立测试**：可以通过在连击第 2 击的取消窗口内按技能键，观察攻击动画是否被技能动画取消来测试。

**Acceptance Scenarios**:

1. **Given** 玩家在连击第 2 击的取消窗口（动画 30%-70%），**When** 玩家按技能键，**Then**：
   - 攻击动画立即取消
   - 技能动画开始播放
   - 连击计数器保持（不重置）
   - 技能命中后，连击计数增加

2. **Given** 玩家在连击第 2 击的非取消窗口（动画 0%-30% 或 70%-100%），**When** 玩家按技能键，**Then**：
   - 技能输入被缓冲
   - 攻击动画播放完毕后，技能触发

---

### User Story 6 - 无敌帧系统 (Priority: P2)

**场景描述**：玩家在受击后有短暂的无敌帧（i-frames），避免连续被击中无法反应。

**为什么是 P2**：无敌帧保证战斗公平性，防止玩家被"锁死"（stun-lock），是必要的保护机制。

**独立测试**：可以通过让玩家受到伤害后，在无敌帧期间再次受到攻击，验证第二次攻击是否被忽略来测试。

**Acceptance Scenarios**:

1. **Given** 玩家被敌人攻击命中，**When** 伤害确认时，**Then**：
   - 玩家进入无敌帧状态（0.5 秒）
   - 玩家精灵闪烁（白色闪光，每 0.1 秒切换一次）
   - 玩家标记为 invincible = true

2. **Given** 玩家在无敌帧状态，**When** 敌人再次攻击玩家，**Then**：
   - 碰撞检测忽略此次攻击
   - 无伤害应用
   - 无打击感反馈

3. **Given** 无敌帧持续 0.5 秒，**When** 时间到期，**Then**：
   - 玩家退出无敌帧状态
   - 精灵停止闪烁
   - 玩家可以正常受击

---

### Edge Cases

#### 战斗边界情况

1. **零伤害边界**：当伤害计算结果为 0 或负数时，如何处理？
   - 系统应用最小伤害 1（避免无限战斗）
   - 或者完全忽略此次攻击（根据游戏设计）

2. **伤害溢出**：当伤害值超过 f32 最大值时，如何防止溢出？
   - 伤害值上限设为 9999
   - 使用饱和算术（saturating arithmetic）

3. **同时死亡**：玩家和敌人同时死亡（相互攻击命中）时，谁先判定？
   - 服务器/主机权威判定先后顺序
   - 或允许双方同时死亡（"同归于尽"）

4. **技能施法中受击**：玩家施法动画中被打断，技能和 MP 如何处理？
   - 技能取消，MP 退还 50%
   - 或技能继续施法（Super Armor，霸体状态）

5. **负生命值处理**：生命值降至负数时，如何保证死亡逻辑正确？
   - 生命值 clamp 到 0..=max_health 范围
   - 死亡判定在 health <= 0 时立即触发

6. **连击超时边界**：连击窗口刚好到期（1.000 秒）时按攻击键，是否算第 2 击？
   - 使用严格不等式（< 1.0 秒有效）
   - 或宽容模式（<= 1.05 秒，给 50ms 缓冲）

7. **多个 HitBox 同时命中**：玩家多个攻击 HitBox 同时命中敌人，如何避免重复伤害？
   - 使用 HitBox ID 和 frame counter 去重
   - 同一帧内同一 HitBox ID 只能命中一次

8. **无敌帧重叠**：玩家同时触发多个无敌帧来源（受击 + 技能），如何计算持续时间？
   - 取最长的无敌帧持续时间
   - 或无敌帧时间累加（可能导致滥用）

---

### Combat Mechanics Testing *(MANDATORY per Constitution Principle V)*

#### 伤害计算测试

测试文件：`tests/unit/damage_test.rs`

**必须测试的场景**：

1. **基础伤害应用**：
   ```rust
   // Test: 基础伤害准确计算
   let result = calculate_damage(10.0, &attacker_stats, &defender_stats, Element::Physical);
   assert_eq!(result.final_damage, 10.0);
   ```

2. **元素伤害修正**：
   ```rust
   // Test: 火元素对冰弱点敌人造成 1.5 倍伤害
   let defender_stats = Stats { weakness: Element::Ice, .. };
   let result = calculate_damage(10.0, &attacker_stats, &defender_stats, Element::Fire);
   assert_eq!(result.final_damage, 15.0);
   ```

3. **暴击计算**：
   ```rust
   // Test: 暴击率 100%，暴击倍率 2.0
   let attacker_stats = Stats { crit_rate: 1.0, crit_multiplier: 2.0, .. };
   let result = calculate_damage(10.0, &attacker_stats, &defender_stats, Element::Physical);
   assert!(result.is_critical);
   assert_eq!(result.final_damage, 20.0);
   ```

4. **防御减伤**：
   ```rust
   // Test: 防御值 50，减伤 50%
   let defender_stats = Stats { defense: 50.0, .. };
   let result = calculate_damage(10.0, &attacker_stats, &defender_stats, Element::Physical);
   assert_eq!(result.final_damage, 5.0); // 10 * (1 - 0.5)
   ```

5. **伤害下限**：
   ```rust
   // Test: 防御极高时，伤害不低于 1
   let defender_stats = Stats { defense: 9999.0, .. };
   let result = calculate_damage(10.0, &attacker_stats, &defender_stats, Element::Physical);
   assert!(result.final_damage >= 1.0);
   ```

6. **伤害上限**：
   ```rust
   // Test: 伤害溢出保护
   let result = calculate_damage(99999.0, &attacker_stats, &defender_stats, Element::Physical);
   assert!(result.final_damage <= 9999.0);
   ```

#### 碰撞检测测试

测试文件：`tests/integration/collision_test.rs`

**必须测试的场景**：

1. **HitBox vs HurtBox 精确检测**：
   ```rust
   // Test: 1 像素重叠应该命中
   let hitbox = HitBox::new(Rect { x: 100, y: 100, width: 32, height: 32 });
   let hurtbox = HurtBox::new(Rect { x: 131, y: 100, width: 16, height: 16 }); // 1px overlap
   assert!(hitbox.intersects(&hurtbox));
   ```

2. **边界情况：0 像素不命中**：
   ```rust
   // Test: 0 像素重叠不应该命中
   let hitbox = HitBox::new(Rect { x: 100, y: 100, width: 32, height: 32 });
   let hurtbox = HurtBox::new(Rect { x: 132, y: 100, width: 16, height: 16 }); // 0px overlap
   assert!(!hitbox.intersects(&hurtbox));
   ```

3. **多段攻击处理**：
   ```rust
   // Test: 3 连击每击独立判定
   let combo_attacks = vec![attack1, attack2, attack3];
   let mut hit_count = 0;
   for attack in combo_attacks {
       if attack.hitbox.intersects(&enemy.hurtbox) {
           hit_count += 1;
       }
   }
   assert_eq!(hit_count, 3);
   ```

4. **穿透攻击**：
   ```rust
   // Test: 穿透攻击可以命中多个敌人
   let piercing_attack = Attack { can_pierce: true, .. };
   let enemies = vec![enemy1, enemy2, enemy3];
   let hits = check_piercing_collision(&piercing_attack, &enemies);
   assert_eq!(hits.len(), 3); // 所有敌人都被命中
   ```

5. **无敌帧忽略碰撞**：
   ```rust
   // Test: 无敌帧期间，碰撞检测忽略
   let player = Player { invincible: true, invincible_timer: 0.3, .. };
   let attack = enemy_attack;
   assert!(!should_apply_damage(&player, &attack));
   ```

#### 状态效果测试

测试文件：`tests/unit/status_effect_test.rs`

**必须测试的场景**：

1. **状态应用与持续时间**：
   ```rust
   // Test: Burn 状态每秒造成 5 点伤害，持续 3 秒
   let burn = StatusEffect::Burn { damage_per_sec: 5.0, duration: 3.0 };
   let mut enemy = Enemy { health: 100.0, .. };
   apply_status(&mut enemy, burn);
   
   // 模拟 3 秒
   for _ in 0..3 {
       update_status_effects(&mut enemy, 1.0); // 1 秒 delta
   }
   assert_eq!(enemy.health, 85.0); // 100 - 15
   ```

2. **状态叠加规则**：
   ```rust
   // Test: Burn 可叠加，最多 3 层
   let mut enemy = Enemy::default();
   apply_status(&mut enemy, StatusEffect::Burn { stacks: 1, .. });
   apply_status(&mut enemy, StatusEffect::Burn { stacks: 1, .. });
   apply_status(&mut enemy, StatusEffect::Burn { stacks: 1, .. });
   apply_status(&mut enemy, StatusEffect::Burn { stacks: 1, .. }); // 第 4 次
   
   assert_eq!(enemy.status_effects.get_burn_stacks(), 3); // 上限 3
   ```

3. **状态刷新持续时间**：
   ```rust
   // Test: 重新应用状态时，持续时间刷新
   let mut enemy = Enemy::default();
   apply_status(&mut enemy, StatusEffect::Burn { duration: 3.0, .. });
   
   // 过了 2 秒
   update_status_effects(&mut enemy, 2.0);
   
   // 重新应用 Burn
   apply_status(&mut enemy, StatusEffect::Burn { duration: 3.0, .. });
   
   // 检查持续时间是否刷新为 3 秒（而不是剩余 1 秒）
   assert_eq!(enemy.status_effects.get_burn_remaining(), 3.0);
   ```

#### 技能系统测试

测试文件：`tests/unit/skill_test.rs`

**必须测试的场景**：

1. **冷却时间准确性**：
   ```rust
   // Test: 技能冷却 5 秒，帧精确
   let mut skill = Skill { cooldown: 5.0, remaining_cooldown: 0.0, .. };
   skill.activate();
   
   assert_eq!(skill.remaining_cooldown, 5.0);
   
   // 模拟 60 帧 = 1 秒（假设 60 FPS）
   for _ in 0..60 {
       skill.update(1.0 / 60.0);
   }
   
   assert!((skill.remaining_cooldown - 4.0).abs() < 0.01); // 误差 <10ms
   ```

2. **资源消耗验证**：
   ```rust
   // Test: 技能消耗 20 MP
   let mut player = Player { mp: 100.0, .. };
   let skill = Skill { mp_cost: 20.0, .. };
   
   assert!(can_use_skill(&player, &skill)); // MP 足够
   
   use_skill(&mut player, &skill);
   
   assert_eq!(player.mp, 80.0);
   ```

3. **技能取消窗口**：
   ```rust
   // Test: 技能动画 30%-70% 可取消
   let skill = Skill { animation_duration: 1.0, cancel_window: 0.3..0.7, .. };
   
   // 动画进行到 0.5 秒（50%）
   skill.update(0.5);
   
   assert!(skill.can_cancel()); // 在取消窗口内
   
   // 取消进入攻击
   skill.cancel_into_attack();
   
   assert_eq!(skill.state, SkillState::Cancelled);
   ```

#### 无敌帧测试

测试文件：`tests/unit/invincibility_test.rs`

**必须测试的场景**：

1. **无敌帧持续时间**：
   ```rust
   // Test: 无敌帧持续 0.5 秒 = 30 帧（60 FPS）
   let mut player = Player { invincible: false, .. };
   
   apply_invincibility(&mut player, 0.5);
   
   assert!(player.invincible);
   
   // 模拟 30 帧
   for _ in 0..30 {
       update_invincibility(&mut player, 1.0 / 60.0);
   }
   
   assert!(!player.invincible); // 无敌帧到期
   ```

2. **无敌帧重叠（取最长）**：
   ```rust
   // Test: 多个无敌帧来源，取最长持续时间
   let mut player = Player::default();
   
   apply_invincibility(&mut player, 0.5); // 来源 1：0.5 秒
   update_invincibility(&mut player, 0.2); // 过了 0.2 秒，剩余 0.3 秒
   
   apply_invincibility(&mut player, 0.6); // 来源 2：0.6 秒（更长）
   
   assert_eq!(player.invincibility_remaining, 0.6); // 应该是 0.6，而不是 0.3
   ```

3. **视觉反馈（闪烁）**：
   ```rust
   // Test: 无敌帧期间，精灵每 0.1 秒闪烁一次
   let mut player = Player::default();
   apply_invincibility(&mut player, 0.5);
   
   let flashes = count_sprite_flashes(&player, 0.5); // 0.5 秒内闪烁次数
   
   assert_eq!(flashes, 5); // 0.5 / 0.1 = 5 次
   ```

#### 边界与漏洞测试

测试文件：`tests/edge_cases/combat_edge_cases_test.rs`

**必须测试的场景**：

1. **零伤害保护**：
   ```rust
   // Test: 伤害为 0 时，应用最小伤害 1
   let result = calculate_damage(0.0, &attacker_stats, &defender_stats, Element::Physical);
   assert_eq!(result.final_damage, 1.0);
   ```

2. **整数溢出保护**：
   ```rust
   // Test: 伤害计算不溢出
   let result = calculate_damage(f32::MAX, &attacker_stats, &defender_stats, Element::Physical);
   assert!(result.final_damage.is_finite());
   assert!(result.final_damage <= 9999.0);
   ```

3. **同时死亡处理**：
   ```rust
   // Test: 玩家和敌人同时死亡，两者都标记为 dead
   let mut player = Player { health: 1.0, .. };
   let mut enemy = Enemy { health: 1.0, .. };
   
   // 相互攻击，同时造成 1 点伤害
   apply_damage(&mut player, 1.0);
   apply_damage(&mut enemy, 1.0);
   
   assert!(player.is_dead());
   assert!(enemy.is_dead());
   ```

4. **技能施法中受击**：
   ```rust
   // Test: 施法中被打断，MP 退还 50%
   let mut player = Player { mp: 100.0, .. };
   let skill = Skill { mp_cost: 20.0, .. };
   
   use_skill(&mut player, &skill); // MP 降至 80
   assert_eq!(player.mp, 80.0);
   
   // 施法中被打断
   interrupt_skill(&mut player, &skill);
   
   assert_eq!(player.mp, 90.0); // 80 + 10 (50% 退还)
   ```

5. **负生命值 clamp**：
   ```rust
   // Test: 生命值不会降至负数
   let mut enemy = Enemy { health: 10.0, .. };
   
   apply_damage(&mut enemy, 999.0); // 过量伤害
   
   assert_eq!(enemy.health, 0.0); // clamp 到 0
   assert!(enemy.is_dead());
   ```

---

## Requirements *(mandatory)*

### 功能需求

#### 伤害计算系统

- **FR-001**: 系统必须实现纯函数伤害计算（`calculate_damage`），零 Bevy 依赖
- **FR-002**: 伤害计算必须支持基础伤害、攻击者属性加成、防御者防御减伤
- **FR-003**: 系统必须支持暴击系统：暴击率（0.0-1.0）、暴击倍率（默认 2.0）
- **FR-004**: 系统必须支持元素伤害类型（Physical, Fire, Ice, Lightning）
- **FR-005**: 元素伤害必须根据敌人弱点/抗性计算修正系数（弱点 1.5x，抗性 0.5x）
- **FR-006**: 伤害值必须 clamp 到 1.0..=9999.0 范围（防止零伤害和溢出）
- **FR-007**: 伤害计算结果必须包含：最终伤害值、是否暴击、元素类型、命中的敌人 ID

#### 碰撞检测系统

- **FR-008**: 系统必须实现 HitBox（攻击判定框）和 HurtBox（受击判定框）组件
- **FR-009**: HitBox 必须包含：矩形区域（Rect）、伤害值、元素类型、持续帧数、是否穿透
- **FR-010**: HurtBox 必须包含：矩形区域（Rect）、所属实体 ID、是否无敌
- **FR-011**: 系统必须每帧检测所有活跃 HitBox 与 HurtBox 的碰撞
- **FR-012**: 碰撞检测必须使用 AABB（轴对齐包围盒）算法，支持矩形与矩形精确碰撞
- **FR-013**: 系统必须支持穿透攻击（同一 HitBox 可命中多个敌人）
- **FR-014**: 系统必须防止同一 HitBox 在同一帧内重复命中同一敌人（使用 HitBox ID + frame counter 去重）
- **FR-015**: 无敌帧状态下的 HurtBox 必须忽略所有 HitBox 碰撞

#### 连击系统

- **FR-016**: 系统必须支持 3 连击：第 1 击（轻击 10 伤害）→ 第 2 击（轻击 10 伤害）→ 第 3 击（重击 20 伤害）
- **FR-017**: 连击窗口必须为 1.0 秒（可配置），超时则重置到第 1 击
- **FR-018**: 第 2/3 击必须可以取消第 1/2 击的动画（combo canceling）
- **FR-019**: 连击计数器必须在 UI 显示当前连击数（如 "3 HIT COMBO"）
- **FR-020**: 连击计数器必须在连击结束后 1 秒淡出
- **FR-021**: 第 3 击（重击）必须对敌人施加击退效果（50 像素向后）

#### 打击感反馈系统

- **FR-022**: 系统必须在伤害确认时触发 Hitfreeze（时间定格）
  - 轻击：3 帧（50ms）
  - 重击：5 帧（83ms）
  - 暴击：7 帧（117ms）
- **FR-023**: 系统必须在重击/暴击时触发屏幕震动
  - 重击：振幅 2-4 像素，持续 100-150ms
  - 暴击：振幅 4-6 像素，持续 150-200ms
- **FR-024**: 系统必须在命中点生成粒子特效
  - 轻击：8×8 白色火花，5-10 个粒子
  - 重击：8×8 橙色火花，15-20 个粒子
  - 暴击：8×8 金色火花，20-30 个粒子
- **FR-025**: 系统必须播放对应音效
  - 轻击：`assets/audio/hit_light.ogg`
  - 重击：`assets/audio/hit_heavy.ogg`
  - 暴击：`assets/audio/hit_critical.ogg`
- **FR-026**: 系统必须在命中点上方显示浮动伤害数字
  - 普通伤害：白色，1.0 倍大小
  - 重击：红色，1.5 倍大小
  - 暴击：黄色，2.0 倍大小
  - 动画：向上飘动 50 像素，1 秒后淡出

#### 技能系统

- **FR-027**: 系统必须支持技能数据配置（RON 格式）：技能 ID、名称、冷却时间、MP 消耗、伤害值
- **FR-028**: 玩家必须有 MP 资源（初始 100，最大 100）
- **FR-029**: 技能使用必须检查：MP 是否足够、冷却是否完毕、玩家是否在可施法状态
- **FR-030**: 技能使用成功后必须：扣除 MP、启动冷却计时器、播放施法动画、生成技能弹道/效果
- **FR-031**: 技能冷却必须在 UI 显示（圆形进度条，剩余秒数）
- **FR-032**: 技能"火球术"必须实现：弹道飞行（速度 300 像素/秒）、碰撞检测、爆炸动画
- **FR-033**: 系统必须支持技能取消窗口（动画 30%-70% 可取消）
- **FR-034**: 技能施法中被打断，MP 必须退还 50%

#### 敌人系统

- **FR-035**: 系统必须实现史莱姆敌人（16×16 像素精灵）
- **FR-036**: 史莱姆必须有以下属性：生命值（初始 30）、移动速度（50 像素/秒）、攻击力（5）
- **FR-037**: 史莱姆必须在生命值条上显示当前生命值（血条，红色填充）
- **FR-038**: 史莱姆受到伤害时必须播放受击动画（闪白效果，持续 100ms）
- **FR-039**: 史莱姆生命值归零时必须播放死亡动画（1 秒），然后 despawn
- **FR-040**: 史莱姆死亡时可选掉落战利品（金币，10% 概率）

#### 无敌帧系统

- **FR-041**: 玩家受到伤害后必须进入无敌帧状态（默认 0.5 秒）
- **FR-042**: 无敌帧期间，玩家精灵必须闪烁（白色闪光，每 0.1 秒切换一次）
- **FR-043**: 无敌帧期间，玩家的 HurtBox 必须忽略所有 HitBox 碰撞
- **FR-044**: 无敌帧期间，玩家仍可正常移动和攻击
- **FR-045**: 多个无敌帧来源时，必须取最长的持续时间

#### 架构约束

- **FR-046**: 领域层（`src/domain/combat/`）必须零 Bevy 依赖，所有逻辑为纯函数
- **FR-047**: 基础设施层（`src/infrastructure/systems/combat_systems.rs`）必须桥接 Bevy ECS
- **FR-048**: 所有战斗事件必须使用 Bevy Events：`DamageDealt`, `ComboExtended`, `EnemyDefeated`, `SkillActivated`
- **FR-049**: 所有战斗组件必须为纯数据结构（无业务逻辑方法）
- **FR-050**: 战斗数据（技能、敌人属性）必须使用 RON 配置文件（`assets/data/skills.ron`, `enemies.ron`）

### 关键实体

#### 战斗领域实体（纯数据，零 Bevy 依赖）

- **DamageResult**: 伤害计算结果
  - `final_damage: f32` - 最终伤害值
  - `is_critical: bool` - 是否暴击
  - `element: Element` - 元素类型
  - `target_id: EntityId` - 命中的目标 ID（可能需要在基础设施层处理）

- **Stats**: 战斗属性
  - `attack: f32` - 攻击力
  - `defense: f32` - 防御力
  - `crit_rate: f32` - 暴击率（0.0-1.0）
  - `crit_multiplier: f32` - 暴击倍率（默认 2.0）
  - `element_resistances: HashMap<Element, f32>` - 元素抗性

- **Element**: 元素类型（枚举）
  - `Physical`, `Fire`, `Ice`, `Lightning`

- **ComboState**: 连击状态（枚举）
  - `Idle` - 无连击
  - `FirstHit` - 第 1 击
  - `SecondHit` - 第 2 击
  - `ThirdHit` - 第 3 击（重击）

#### 基础设施层组件（Bevy Components）

- **HitBox**: 攻击判定框（Component）
  - `rect: Rect` - 矩形区域（像素坐标）
  - `damage: f32` - 基础伤害
  - `element: Element` - 元素类型
  - `lifetime_frames: u32` - 持续帧数
  - `can_pierce: bool` - 是否穿透
  - `hit_entities: HashSet<Entity>` - 已命中的实体（防止重复命中）

- **HurtBox**: 受击判定框（Component）
  - `rect: Rect` - 矩形区域
  - `is_invincible: bool` - 是否无敌

- **Health**: 生命值（Component）
  - `current: f32` - 当前生命值
  - `max: f32` - 最大生命值

- **Skill**: 技能数据（Component）
  - `skill_id: String` - 技能 ID
  - `cooldown: f32` - 冷却时间（秒）
  - `remaining_cooldown: f32` - 剩余冷却（秒）
  - `mp_cost: f32` - MP 消耗
  - `damage: f32` - 伤害值
  - `element: Element` - 元素类型

- **Combo**: 连击状态（Component）
  - `state: ComboState` - 当前连击状态
  - `window_remaining: f32` - 连击窗口剩余时间（秒）
  - `hit_count: u32` - 当前连击数

- **Invincibility**: 无敌帧（Component）
  - `remaining: f32` - 剩余时间（秒）
  - `flash_timer: f32` - 闪烁计时器

---

## Success Criteria *(mandatory)*

### 可衡量成果

#### 性能标准

- **SC-001**: 15 个敌人同时战斗时，游戏稳定保持 60 FPS（帧时间 <16.67ms），≥95% 的帧
- **SC-002**: 战斗系统执行时间 <3ms（帧预算的 18%）
- **SC-003**: 碰撞检测系统执行时间 <3ms（帧预算的 18%）
- **SC-004**: 100 个粒子同时存在时，帧率不低于 55 FPS

#### 代码质量标准

- **SC-005**: 领域层（`src/domain/combat/`）零 Bevy 依赖，`cargo tree` 验证无 bevy crate
- **SC-006**: 战斗核心逻辑测试覆盖率 ≥85%（伤害计算、碰撞检测、连击系统）
- **SC-007**: 所有单元测试和集成测试通过（`cargo test` 零失败）
- **SC-008**: 零 unsafe 代码（或有充分文档和测试）
- **SC-009**: `cargo clippy -- -D warnings` 零警告
- **SC-010**: `cargo fmt --check` 格式检查通过

#### 架构标准

- **SC-011**: 伤害计算、连击判定为纯函数（可在无 Bevy App 情况下测试）
- **SC-012**: 所有战斗事件使用 Bevy Events（`DamageDealt`, `ComboExtended`, `EnemyDefeated`, `SkillActivated`）
- **SC-013**: 所有战斗组件为纯数据结构（无业务逻辑方法）
- **SC-014**: 代码使用英文标识符，文档使用中文（Constitution v1.0.1 Language Separation Rule）

#### 游戏体验标准

- **SC-015**: 5 名测试玩家评分打击感 ≥7/10（"打击感强烈"）
- **SC-016**: 3 连击无卡顿，连击取消窗口准确（测试玩家无抱怨）
- **SC-017**: 打击音效与动画同步（误差 <2 帧，33ms）
- **SC-018**: 玩家可以在 30 秒内理解并成功执行 3 连击

#### 功能完整性标准

- **SC-019**: 玩家可以使用基础攻击（3连击）击败史莱姆敌人
- **SC-020**: 玩家可以使用技能"火球术"击败史莱姆敌人
- **SC-021**: 伤害数字、血条、连击计数器、技能冷却在 UI 正确显示
- **SC-022**: 所有打击感反馈（hitfreeze、屏幕震动、粒子、音效）正常工作
- **SC-023**: 无敌帧正确保护玩家，避免连续被击中

---

## 技术依赖与约束

### 技术栈

- **语言**: Rust 1.91.1 (stable)
- **引擎**: Bevy 0.17.0
- **物理引擎**: bevy_rapier2d 0.29+（用于碰撞检测）
- **音频**: bevy_kira_audio（或 Bevy 内置音频）
- **数据格式**: RON（技能、敌人配置）

### 性能预算

根据项目规范（Section 5），战斗系统占用帧预算：

| 系统分类 | 帧预算 (ms) | 帧预算 (%) |
|---------|------------|-----------|
| 物理与碰撞检测 | 3.0 | 18% |
| 战斗逻辑 | 2.5 | 15% |
| 动画与状态 | 1.5 | 9% |
| 音频 | 0.5 | 3% |
| **战斗总计** | **7.5** | **45%** |

### 资产需求

#### 精灵资产

- `assets/sprites/player_attack.png` - 玩家攻击动画（3 连击，每击 3-4 帧）
- `assets/sprites/enemy_slime.png` - 史莱姆敌人（16×16，待机/受击/死亡动画）
- `assets/sprites/hit_effect.png` - 打击特效粒子（8×8，白色/橙色/金色）
- `assets/sprites/skill_fireball.png` - 火球技能（16×16，飞行/爆炸动画）

#### 音频资产

- `assets/audio/hit_light.ogg` - 轻击音效
- `assets/audio/hit_heavy.ogg` - 重击音效
- `assets/audio/hit_critical.ogg` - 暴击音效
- `assets/audio/skill_fireball.ogg` - 火球施法音效

#### 数据配置

- `assets/data/skills.ron` - 技能配置
- `assets/data/enemies.ron` - 敌人配置

---

## 实施阶段

根据项目计划，M2 实施分为 3 周：

### Week 1: 伤害计算与碰撞检测

**目标**：建立战斗系统的数学和物理基础

- [ ] 实现领域层伤害计算（纯函数）
- [ ] 实现 HitBox / HurtBox 组件
- [ ] 实现碰撞检测系统
- [ ] 编写单元测试（≥85% 覆盖率）
- [ ] 玩家可以攻击并造成伤害

### Week 2: 连击系统与打击感

**目标**：实现核心战斗体验

- [ ] 实现 3 连击系统
- [ ] 实现打击感反馈（hitfreeze、屏幕震动、粒子）
- [ ] 实现伤害数字 UI
- [ ] 实现连击计数器 UI
- [ ] 打击感通过测试玩家评审（≥7/10）

### Week 3: 技能系统与敌人

**目标**：丰富战斗内容，完成可玩演示

- [ ] 实现技能系统（冷却、MP 消耗）
- [ ] 实现火球术技能（弹道、爆炸）
- [ ] 实现史莱姆敌人（AI、血条、死亡）
- [ ] 实现无敌帧系统
- [ ] 完成所有集成测试
- [ ] 性能优化（60 FPS 验证）

---

## 下一步行动

1. **立即执行**: 运行 `/speckit.plan` 创建技术实施计划（`plan.md`）
2. **资产准备**: 设计师开始绘制攻击动画、史莱姆精灵、粒子特效
3. **音效准备**: 音效师制作打击音效（轻击、重击、暴击、技能）
4. **团队对齐**: 所有开发者阅读本规范，确认需求和架构约束

---

**状态**: ✅ 规范已完成，等待技术计划（/speckit.plan）

**批准**: 待用户审核


