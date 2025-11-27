# Event Contracts: 战斗系统事件

**Feature**: 002-combat-core  
**Date**: 2025-11-25  
**Purpose**: 定义战斗系统 Bevy Events 契约

---

## 事件契约概述

战斗系统使用 Bevy Events 进行系统间通信。所有事件遵循以下原则：

1. **不可变性**: 事件发布后不可修改
2. **解耦**: 事件发布者不关心订阅者
3. **单一职责**: 每个事件表示一个明确的游戏事件
4. **数据完整性**: 事件包含所有必要信息，订阅者无需额外查询

---

## Event 1: DamageDealt

**描述**: 伤害已造成事件（伤害计算完成并应用到目标）

**触发时机**: `apply_damage_system` 完成伤害应用后

**订阅者**:
- `hit_reaction_system`: 播放受击动画
- `hitfreeze_system`: 触发打击定格
- `screen_shake_system`: 触发屏幕震动
- `particle_system`: 生成打击粒子
- `damage_number_system`: 显示伤害数字
- `combat_audio_system`: 播放打击音效

**字段**:

| 字段名 | 类型 | 描述 | 约束 |
|--------|------|------|------|
| `attacker` | `Entity` | 攻击者实体 | - |
| `target` | `Entity` | 目标实体 | - |
| `damage` | `f32` | 最终伤害值 | 1.0..=9999.0 |
| `is_critical` | `bool` | 是否暴击 | - |
| `element` | `Element` | 元素类型 | Physical, Fire, Ice, Lightning |
| `hit_position` | `Vec2` | 命中点（世界坐标） | 用于粒子和伤害数字生成 |

**示例**:
```rust
#[derive(Event)]
pub struct DamageDealt {
    pub attacker: Entity,
    pub target: Entity,
    pub damage: f32,
    pub is_critical: bool,
    pub element: Element,
    pub hit_position: Vec2,
}

// 发布事件
fn apply_damage_system(
    mut events: EventWriter<DamageDealt>,
    // ...
) {
    events.send(DamageDealt {
        attacker: player_entity,
        target: enemy_entity,
        damage: 50.0,
        is_critical: true,
        element: Element::Fire,
        hit_position: Vec2::new(100.0, 100.0),
    });
}

// 订阅事件
fn hitfreeze_system(
    mut events: EventReader<DamageDealt>,
    mut hitfreeze_timer: ResMut<HitfreezeTimer>,
    config: Res<CombatConfig>,
) {
    for event in events.read() {
        let duration = if event.is_critical {
            config.hitfreeze_critical
        } else {
            config.hitfreeze_light
        };
        hitfreeze_timer.trigger(duration);
    }
}
```

---

## Event 2: ComboExtended

**描述**: 连击延续事件（玩家成功执行连击）

**触发时机**: `combo_system` 检测到玩家在连击窗口内按攻击键

**订阅者**:
- `combo_ui_system`: 更新连击计数器 UI
- `combo_audio_system`: 播放连击音效（可选）

**字段**:

| 字段名 | 类型 | 描述 |
|--------|------|------|
| `player` | `Entity` | 玩家实体 |
| `combo_count` | `u32` | 当前连击数（1, 2, 3） |
| `new_state` | `ComboState` | 新的连击状态 |

**示例**:
```rust
#[derive(Event)]
pub struct ComboExtended {
    pub player: Entity,
    pub combo_count: u32,
    pub new_state: ComboState,
}

// 发布事件
fn combo_system(
    mut events: EventWriter<ComboExtended>,
    mut combos: Query<(Entity, &mut Combo)>,
    // ...
) {
    for (entity, mut combo) in combos.iter_mut() {
        combo.advance(1.0);
        events.send(ComboExtended {
            player: entity,
            combo_count: combo.hit_count,
            new_state: combo.state,
        });
    }
}

// 订阅事件
fn combo_ui_system(
    mut events: EventReader<ComboExtended>,
    mut ui_query: Query<&mut Text, With<ComboCounter>>,
) {
    for event in events.read() {
        for mut text in ui_query.iter_mut() {
            text.sections[0].value = format!("{} HIT COMBO", event.combo_count);
        }
    }
}
```

---

## Event 3: EnemyDefeated

**描述**: 敌人死亡事件（生命值归零）

**触发时机**: `death_system` 检测到敌人 Health <= 0

**订阅者**:
- `loot_system`: 生成战利品掉落
- `enemy_despawn_system`: 播放死亡动画后 despawn 敌人
- `score_system`: 更新玩家分数（可选）
- `quest_system`: 更新任务进度（可选）

**字段**:

| 字段名 | 类型 | 描述 |
|--------|------|------|
| `enemy` | `Entity` | 死亡的敌人实体 |
| `position` | `Vec2` | 死亡位置（世界坐标） |
| `enemy_id` | `String` | 敌人 ID（如 "slime"） |
| `killed_by` | `Entity` | 击杀者实体 |

**示例**:
```rust
#[derive(Event)]
pub struct EnemyDefeated {
    pub enemy: Entity,
    pub position: Vec2,
    pub enemy_id: String,
    pub killed_by: Entity,
}

// 发布事件
fn death_system(
    mut commands: Commands,
    mut events: EventWriter<EnemyDefeated>,
    enemies: Query<(Entity, &Transform, &Health, &EnemyId), With<Enemy>>,
) {
    for (entity, transform, health, enemy_id) in enemies.iter() {
        if health.is_dead() {
            events.send(EnemyDefeated {
                enemy: entity,
                position: transform.translation.truncate(),
                enemy_id: enemy_id.0.clone(),
                killed_by: player_entity, // 需要从上下文获取
            });
            
            // 播放死亡动画后 despawn（延迟 despawn）
            commands.entity(entity).insert(DeathAnimation { timer: 1.0 });
        }
    }
}

// 订阅事件
fn loot_system(
    mut commands: Commands,
    mut events: EventReader<EnemyDefeated>,
    enemy_db: Res<EnemyDatabase>,
) {
    for event in events.read() {
        if let Some(enemy_data) = enemy_db.enemies.get(&event.enemy_id) {
            // 10% 概率掉落金币
            if rand::random::<f32>() < 0.1 {
                commands.spawn((
                    Loot,
                    Transform::from_translation(event.position.extend(0.0)),
                    // ... 战利品组件
                ));
            }
        }
    }
}
```

---

## Event 4: SkillActivated

**描述**: 技能激活事件（玩家成功施放技能）

**触发时机**: `skill_input_system` 检查冷却和 MP 后确认技能可用

**订阅者**:
- `projectile_system`: 生成技能弹道（如火球）
- `skill_animation_system`: 播放施法动画
- `skill_audio_system`: 播放施法音效
- `mp_system`: 扣除 MP
- `cooldown_system`: 启动冷却

**字段**:

| 字段名 | 类型 | 描述 |
|--------|------|------|
| `player` | `Entity` | 施法者实体 |
| `skill_id` | `String` | 技能 ID（如 "fireball"） |
| `target_position` | `Vec2` | 目标位置（世界坐标） |
| `direction` | `Vec2` | 施法方向（归一化） |

**示例**:
```rust
#[derive(Event)]
pub struct SkillActivated {
    pub player: Entity,
    pub skill_id: String,
    pub target_position: Vec2,
    pub direction: Vec2,
}

// 发布事件
fn skill_input_system(
    mut events: EventWriter<SkillActivated>,
    players: Query<(Entity, &Transform, &Skill)>,
    mouse_button: Res<ButtonInput<MouseButton>>,
) {
    if mouse_button.just_pressed(MouseButton::Left) {
        for (entity, transform, skill) in players.iter() {
            if skill.is_ready() {
                events.send(SkillActivated {
                    player: entity,
                    skill_id: skill.skill_id.clone(),
                    target_position: get_mouse_world_position(),
                    direction: (get_mouse_world_position() - transform.translation.truncate()).normalize(),
                });
            }
        }
    }
}

// 订阅事件
fn projectile_system(
    mut commands: Commands,
    mut events: EventReader<SkillActivated>,
    skill_db: Res<SkillDatabase>,
) {
    for event in events.read() {
        if let Some(skill_data) = skill_db.skills.get(&event.skill_id) {
            // 生成火球弹道
            commands.spawn((
                Fireball,
                Transform::from_translation(event.target_position.extend(0.0)),
                RigidBody::Dynamic,
                Velocity {
                    linvel: event.direction * 300.0, // 300 像素/秒
                    angvel: 0.0,
                },
                Collider::ball(8.0),
                GravityScale(0.0),
                Sensor,
                Lifetime(3.0),
            ));
        }
    }
}
```

---

## Event 5: ComboReset

**描述**: 连击重置事件（连击窗口超时或玩家受击）

**触发时机**: `combo_system` 检测到连击窗口超时，或 `hit_reaction_system` 检测到玩家受击

**订阅者**:
- `combo_ui_system`: 隐藏连击计数器
- `combo_audio_system`: 播放连击中断音效（可选）

**字段**:

| 字段名 | 类型 | 描述 |
|--------|------|------|
| `player` | `Entity` | 玩家实体 |
| `final_combo_count` | `u32` | 最终连击数 |
| `reason` | `ComboResetReason` | 重置原因 |

**示例**:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComboResetReason {
    Timeout,   // 连击窗口超时
    HitStun,   // 玩家被击中
    Manual,    // 手动重置
}

#[derive(Event)]
pub struct ComboReset {
    pub player: Entity,
    pub final_combo_count: u32,
    pub reason: ComboResetReason,
}

// 发布事件
fn combo_system(
    mut events: EventWriter<ComboReset>,
    mut combos: Query<(Entity, &mut Combo)>,
    time: Res<Time>,
) {
    for (entity, mut combo) in combos.iter_mut() {
        if combo.window_remaining > 0.0 {
            combo.window_remaining -= time.delta_seconds();
            if combo.window_remaining <= 0.0 {
                events.send(ComboReset {
                    player: entity,
                    final_combo_count: combo.hit_count,
                    reason: ComboResetReason::Timeout,
                });
                combo.reset();
            }
        }
    }
}
```

---

## Event 6: InvincibilityStarted

**描述**: 无敌帧开始事件（玩家受击后进入无敌状态）

**触发时机**: `hit_reaction_system` 检测到玩家受到伤害

**订阅者**:
- `invincibility_system`: 启动无敌帧计时器
- `invincibility_flash_system`: 开始闪烁效果

**字段**:

| 字段名 | 类型 | 描述 |
|--------|------|------|
| `entity` | `Entity` | 进入无敌状态的实体 |
| `duration` | `f32` | 无敌帧持续时间（秒） |

**示例**:
```rust
#[derive(Event)]
pub struct InvincibilityStarted {
    pub entity: Entity,
    pub duration: f32,
}

// 发布事件
fn hit_reaction_system(
    mut commands: Commands,
    mut events_in: EventReader<DamageDealt>,
    mut events_out: EventWriter<InvincibilityStarted>,
    config: Res<CombatConfig>,
) {
    for event in events_in.read() {
        events_out.send(InvincibilityStarted {
            entity: event.target,
            duration: config.invincibility_duration,
        });
    }
}
```

---

## Event 7: InvincibilityEnded

**描述**: 无敌帧结束事件

**触发时机**: `invincibility_system` 检测到无敌帧计时器到期

**订阅者**:
- `invincibility_flash_system`: 停止闪烁效果
- `hurtbox_system`: 重新启用 HurtBox

**字段**:

| 字段名 | 类型 | 描述 |
|--------|------|------|
| `entity` | `Entity` | 退出无敌状态的实体 |

**示例**:
```rust
#[derive(Event)]
pub struct InvincibilityEnded {
    pub entity: Entity,
}

// 发布事件
fn invincibility_system(
    mut commands: Commands,
    mut events: EventWriter<InvincibilityEnded>,
    mut query: Query<(Entity, &mut Invincibility)>,
    time: Res<Time>,
) {
    for (entity, mut invincibility) in query.iter_mut() {
        invincibility.update(time.delta_seconds());
        if !invincibility.is_invincible() {
            events.send(InvincibilityEnded { entity });
            commands.entity(entity).remove::<Invincibility>();
        }
    }
}
```

---

## Event 8: HitBoxSpawned

**描述**: HitBox 生成事件（攻击判定框出现）

**触发时机**: `attack_system` 在攻击动画特定帧生成 HitBox

**订阅者**:
- `collision_detection_system`: 开始检测碰撞
- `debug_hitbox_system`: 可视化 HitBox（调试模式）

**字段**:

| 字段名 | 类型 | 描述 |
|--------|------|------|
| `hitbox` | `Entity` | HitBox 实体 |
| `owner` | `Entity` | HitBox 所有者（玩家或敌人） |
| `damage` | `f32` | 基础伤害 |
| `element` | `Element` | 元素类型 |

**示例**:
```rust
#[derive(Event)]
pub struct HitBoxSpawned {
    pub hitbox: Entity,
    pub owner: Entity,
    pub damage: f32,
    pub element: Element,
}

// 发布事件
fn attack_system(
    mut commands: Commands,
    mut events: EventWriter<HitBoxSpawned>,
    players: Query<(Entity, &Transform, &Stats, &Combo)>,
) {
    for (entity, transform, stats, combo) in players.iter() {
        // 生成 HitBox
        let hitbox_entity = commands.spawn((
            HitBox::new(Rect { x: 32.0, y: 0.0, width: 32.0, height: 32.0 }, 10.0),
            Transform::from_translation(transform.translation),
        )).id();
        
        events.send(HitBoxSpawned {
            hitbox: hitbox_entity,
            owner: entity,
            damage: 10.0,
            element: Element::Physical,
        });
    }
}
```

---

## Event System Diagram（事件流图）

```
[Player Input]
      ↓
[skill_input_system]
      ↓
[SkillActivated Event] ──→ [projectile_system] → 生成火球
      ↓                   ──→ [mp_system] → 扣除 MP
      ↓                   ──→ [cooldown_system] → 启动冷却
      ↓
[attack_system]
      ↓
[HitBoxSpawned Event] ──→ [collision_detection_system]
      ↓
[DamageDealt Event] ──→ [apply_damage_system] → 更新 Health
      ↓                ──→ [hitfreeze_system] → 打击定格
      ↓                ──→ [screen_shake_system] → 屏幕震动
      ↓                ──→ [particle_system] → 生成粒子
      ↓                ──→ [damage_number_system] → 显示伤害数字
      ↓                ──→ [combat_audio_system] → 播放音效
      ↓
[death_system]
      ↓
[EnemyDefeated Event] ──→ [loot_system] → 掉落战利品
                       ──→ [enemy_despawn_system] → 死亡动画 + despawn
```

---

## Event Ordering（事件顺序）

Bevy Events 在同一帧内按照发布顺序被读取。以下是战斗系统事件的推荐顺序：

| 顺序 | System Set | 包含的 Systems |
|------|-----------|---------------|
| 1 | `InputSet` | `skill_input_system`, `attack_input_system` |
| 2 | `LogicSet` | `combo_system`, `attack_system` |
| 3 | `CollisionSet` | `collision_detection_system` |
| 4 | `DamageSet` | `apply_damage_system`, `death_system` |
| 5 | `FeedbackSet` | `hitfreeze_system`, `screen_shake_system`, `particle_system` |
| 6 | `UISet` | `damage_number_system`, `combo_ui_system` |
| 7 | `AudioSet` | `combat_audio_system` |

**配置**:
```rust
app.configure_sets(Update, (
    InputSet,
    LogicSet,
    CollisionSet,
    DamageSet,
    FeedbackSet,
    UISet,
    AudioSet,
).chain());
```

---

## Testing Events（事件测试）

### 单元测试（事件数据完整性）

```rust
#[test]
fn test_damage_dealt_event_completeness() {
    let event = DamageDealt {
        attacker: Entity::PLACEHOLDER,
        target: Entity::PLACEHOLDER,
        damage: 50.0,
        is_critical: true,
        element: Element::Fire,
        hit_position: Vec2::new(100.0, 100.0),
    };
    
    assert!(event.damage > 0.0);
    assert_eq!(event.element, Element::Fire);
    assert!(event.is_critical);
}
```

### 集成测试（事件流）

```rust
#[test]
fn test_damage_event_flow() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_event::<DamageDealt>()
       .add_systems(Update, apply_damage_system);
    
    // 发布事件
    app.world.send_event(DamageDealt {
        attacker: Entity::PLACEHOLDER,
        target: enemy_entity,
        damage: 10.0,
        is_critical: false,
        element: Element::Physical,
        hit_position: Vec2::ZERO,
    });
    
    // 运行 1 帧
    app.update();
    
    // 验证事件被处理
    let enemy_health = app.world.get::<Health>(enemy_entity).unwrap();
    assert_eq!(enemy_health.current, 20.0); // 30 - 10 = 20
}
```

---

## Best Practices（最佳实践）

1. **事件不可变**: 事件发布后不应修改，确保所有订阅者看到相同数据
2. **数据完整性**: 事件包含所有必要信息，避免订阅者额外查询
3. **解耦**: 事件发布者不关心订阅者，便于扩展
4. **单一职责**: 每个事件表示一个明确的游戏事件
5. **命名规范**: 事件名使用过去式（DamageDealt, EnemyDefeated），表示已发生的事件
6. **性能考虑**: 避免在单帧内发布大量事件（>1000），可能导致帧率下降

---

**Status**: ✅ Event Contracts Complete  
**Total Events**: 8  
**Next**: 运行 `/speckit.tasks` 生成详细任务列表


