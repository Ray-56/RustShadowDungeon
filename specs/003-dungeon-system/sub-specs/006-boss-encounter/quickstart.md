# Boss 遭遇战系统快速入门

**Created**: 2025-12-03
**Feature**: 006-boss-encounter
**Purpose**: 开发者快速入门指南

---

## 快速开始

本指南帮助开发者快速理解和使用 Boss 遭遇战系统。

---

## 1. 系统架构概述

Boss 遭遇战系统采用三层架构：

```
领域层 (Domain)
    ↓ 纯逻辑函数
基础设施层 (Infrastructure)  
    ↓ Bevy ECS 集成
配置层 (Configuration)
    ↓ RON 配置文件
```

### 核心组件

- **BossController**: Boss 核心控制组件（阶段管理、血量锁定）
- **BossPhase**: 阶段定义（技能列表、攻击频率、阈值）
- **Telegraph**: 技能预警组件（半透明红色渐变区域）
- **BossConfig**: 从 RON 文件加载的 Boss 定义

---

## 2. 创建第一个 Boss

### 步骤 1: 创建 RON 配置文件

在 `assets/data/bosses.ron` 中定义 Boss：

```ron
(
    bosses: [
        (
            id: "test_boss",
            name: "测试 Boss",
            max_health: 500.0,
            phases: [
                (
                    phase_index: 0,
                    health_threshold: 1.0,
                    invulnerability_duration: 1.5,
                    skill_ids: ["basic_attack"],
                    attack_frequency: 2.0,
                    move_speed: 50.0,
                ),
                (
                    phase_index: 1,
                    health_threshold: 0.5,
                    invulnerability_duration: 2.0,
                    skill_ids: ["frenzy_attack"],
                    attack_frequency: 3.0,
                    move_speed: 70.0,
                ),
            ],
        ),
    ],
)
```

### 步骤 2: 生成 Boss 实体

```rust
use bevy::prelude::*;
use rust_shadow_dungeon::infrastructure::components::boss::{Boss, BossController};
use rust_shadow_dungeon::infrastructure::resources::BossConfig;

fn spawn_boss(
    mut commands: Commands,
    config: Res<BossConfig>,
) {
    let boss_def = config.get_boss("test_boss").unwrap();
    let controller = BossController::new(boss_def.phases.clone());
    
    commands.spawn((
        Boss,
        BossController(controller),
        Health {
            current: boss_def.max_health,
            max: boss_def.max_health,
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        // ... 其他组件
    ));
}
```

---

## 3. 阶段转换机制

### 工作原理

1. **血量检测**: 伤害系统检测 Boss 血量是否达到阶段阈值
2. **立即锁定**: 血量降至阈值时，立即设置 `health_lock = true`
3. **触发转换**: 发布 `BossPhaseTransition` 事件
4. **无敌状态**: Boss 进入无敌状态（1-2 秒，可配置）
5. **解锁血量**: 无敌时间结束后，解锁血量，切换到新阶段

### 代码示例

```rust
// 在伤害系统中检测阶段转换
fn check_boss_phase_transition(
    mut boss_query: Query<(&mut BossController, &Health), With<Boss>>,
    mut events: EventWriter<BossPhaseTransition>,
    time: Res<Time>,
) {
    for (mut controller, health) in boss_query.iter_mut() {
        let health_percentage = health.current / health.max;
        
        // 检查是否需要阶段转换
        if let Some(next_threshold) = controller.get_next_phase_threshold() {
            if health_percentage <= next_threshold && !controller.health_lock {
                // 锁定血量
                controller.health_lock = true;
                controller.transition_start_time = Some(time.elapsed_seconds());
                
                // 触发阶段转换
                events.send(BossPhaseTransition {
                    boss_entity,
                    from_phase: controller.current_phase,
                    to_phase: controller.current_phase + 1,
                });
            }
        }
    }
}
```

---

## 4. 技能预警（Telegraph）系统

### 创建预警区域

```rust
use rust_shadow_dungeon::infrastructure::components::boss::Telegraph;
use rust_shadow_dungeon::domain::boss::{TelegraphArea, TelegraphShape};

fn create_telegraph(
    commands: &mut Commands,
    skill_id: String,
    center: (f32, f32),
    radius: f32,
    duration: f32,
) {
    let area = TelegraphArea {
        shape: TelegraphShape::Circle { radius },
        center,
        rotation: 0.0,
    };
    
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(1.0, 0.0, 0.0, 0.5), // 半透明红色
                custom_size: Some(Vec2::new(radius * 2.0, radius * 2.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(center.0, center.1, 0.0)),
            ..default()
        },
        Telegraph {
            skill_id,
            area: area.clone(),
            warning_duration: duration,
            elapsed_time: 0.0,
            damage_area: area,
        },
    ));
}
```

### 闪烁动画

```rust
fn update_telegraph_flash(
    mut query: Query<&mut Sprite, With<Telegraph>>,
    time: Res<Time>,
) {
    for mut sprite in query.iter_mut() {
        // 使用 sin 函数实现周期性闪烁
        let flash_factor = (time.elapsed_seconds() * 4.0).sin() * 0.15 + 0.65;
        sprite.color.set_alpha(flash_factor);
    }
}
```

---

## 5. 测试 Boss 系统

### 单元测试示例

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_phase_transition_threshold() {
        let phases = vec![
            BossPhase {
                phase_index: 0,
                health_threshold: 1.0,
                // ...
            },
            BossPhase {
                phase_index: 1,
                health_threshold: 0.5,
                // ...
            },
        ];
        
        // 测试阶段转换逻辑
        let result = check_phase_transition(0, 0.4, &phases, false);
        assert!(matches!(result, PhaseTransitionResult::Transition { .. }));
    }
    
    #[test]
    fn test_telegraph_area_contains() {
        let area = TelegraphArea {
            shape: TelegraphShape::Circle { radius: 10.0 },
            center: (0.0, 0.0),
            rotation: 0.0,
        };
        
        assert!(area.contains_point((5.0, 0.0)));  // 在圆内
        assert!(!area.contains_point((15.0, 0.0))); // 在圆外
    }
}
```

### 集成测试示例

```rust
#[test]
fn test_full_boss_encounter() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugins(BossPlugin);
    
    // 生成 Boss
    let boss_entity = app.world.spawn((Boss, BossController::new(/* ... */))).id();
    
    // 模拟伤害
    app.world.send_event(DamageDealt {
        target: boss_entity,
        amount: 300.0,
    });
    
    // 更新系统
    app.update();
    
    // 验证阶段转换
    let controller = app.world.get::<BossController>(boss_entity).unwrap();
    assert_eq!(controller.current_phase, 1);
}
```

---

## 6. 性能优化提示

1. **限制预警数量**: 同时存在的预警不超过 2 个
2. **阶段转换缓存**: 缓存下一阶段阈值，避免重复计算
3. **技能冷却池**: 使用对象池管理技能冷却状态，减少分配

---

## 7. 常见问题

### Q: 如何修改 Boss 参数？

A: 修改 `assets/data/bosses.ron` 配置文件，重新加载资源（开发模式支持热重载）。

### Q: Boss 快速击杀时如何处理？

A: 系统自动处理 - 血量锁定机制确保所有阶段转换都能完整执行，不会跳过阶段。

### Q: 如何添加新的预警形状？

A: 在 `TelegraphShape` 枚举中添加新形状，在 `TelegraphArea::contains_point` 中实现检测逻辑。

---

## 8. 下一步

- 查看 `data-model.md` 了解完整的数据结构
- 查看 `research.md` 了解技术决策细节
- 查看 `plan.md` 了解系统架构和设计

---

**Document Status**: ✅ 快速入门指南完成






