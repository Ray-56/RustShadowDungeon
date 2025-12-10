use crate::infrastructure::components::combat::Invincibility;
use crate::infrastructure::components::{enemy::Enemy, Health, Player};
use crate::infrastructure::events::combat::{DamageDealt, EnemyDefeated};
use crate::infrastructure::events::dungeon::PlayerDeathInRoom;
use bevy::ecs::message::{MessageReader, MessageWriter};
/// Damage application and death systems
/// 伤害应用和死亡系统
use bevy::prelude::*;

/// Apply damage system (T031)
///
/// Listens for DamageDealt events and applies damage to Health components.
/// Skips damage if target has active Invincibility component.
///
/// 监听 DamageDealt 事件并应用伤害到 Health 组件
/// 如果目标有激活的 Invincibility 组件，则跳过伤害
pub fn apply_damage_system(
    mut damage_events: MessageReader<DamageDealt>,
    mut health_query: Query<&mut Health>,
    invincibility_query: Query<&Invincibility>,
    boss_query: Query<&crate::infrastructure::components::boss::BossController, With<crate::infrastructure::components::boss::Boss>>,
) {
    for event in damage_events.read() {
        // Check if target is invincible
        if let Ok(invincibility) = invincibility_query.get(event.target) {
            if invincibility.is_active() {
                info!("Entity {:?} is invincible, damage ignored", event.target);
                continue; // Skip damage if invincible
            }
        }

        // T059: Check if Boss health is locked (phase transition)
        if let Ok(controller) = boss_query.get(event.target) {
            if controller.health_lock {
                info!("Boss {:?} health is locked during phase transition, damage ignored", event.target);
                continue; // Skip damage if health is locked
            }
        }

        if let Ok(mut health) = health_query.get_mut(event.target) {
            health.take_damage(event.result.final_damage);

            info!(
                "Entity {:?} took {:.1} damage (crit: {}). Health: {:.1}/{:.1}",
                event.target,
                event.result.final_damage,
                event.result.is_critical,
                health.current,
                health.max
            );
        }
    }
}

/// Death system (T032)
///
/// Checks for entities with Health <= 0 and handles their death.
/// Publishes EnemyDefeated event. For enemies, death animation system will handle despawning.
/// For players, publishes PlayerDeathInRoom event for dungeon system to handle respawn.
///
/// 检查 Health <= 0 的实体并处理死亡
/// 发布 EnemyDefeated 事件。对于敌人，死亡动画系统将处理销毁。
/// 对于玩家，发布 PlayerDeathInRoom 事件供地下城系统处理重生。
pub fn death_system(
    mut commands: Commands,
    health_query: Query<
        (Entity, &Health, &Transform),
        Without<crate::infrastructure::components::enemy::DeathAnimation>,
    >,
    enemy_query: Query<&Enemy>,
    player_query: Query<&Player>,
    mut enemy_defeated_events: MessageWriter<EnemyDefeated>,
    mut player_death_events: MessageWriter<PlayerDeathInRoom>,
    dungeon_manager_query: Query<&crate::infrastructure::components::dungeon::DungeonManager>,
) {
    for (entity, health, transform) in &health_query {
        if health.is_dead() {
            info!("Entity {:?} died at position {:?}", entity, transform.translation);

            // Check if this is an enemy
            let is_enemy = enemy_query.get(entity).is_ok();
            let is_player = player_query.get(entity).is_ok();

            if is_enemy {
                // Publish enemy defeated event (death animation system will handle despawning)
                enemy_defeated_events.write(EnemyDefeated {
                    enemy: entity,
                    position: transform.translation.truncate(),
                    killed_by: None, // TODO: Track killer entity
                });
            } else if is_player {
                // Publish player death event for dungeon system to handle
                if let Ok(dungeon_manager) = dungeon_manager_query.single() {
                    player_death_events.write(PlayerDeathInRoom {
                        room_id: dungeon_manager.current_room_id,
                        death_position: transform.translation.truncate(),
                    });
                }
                // Don't despawn player - dungeon system will handle respawn
            } else {
                // For other entities, despawn immediately
                commands.entity(entity).despawn();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_damage_system_compiles() {
        let mut app = App::new();
        app.add_systems(Update, apply_damage_system);
    }

    #[test]
    fn test_death_system_compiles() {
        let mut app = App::new();
        app.add_systems(Update, death_system);
    }
}
