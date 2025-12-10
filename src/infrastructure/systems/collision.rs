use crate::domain::combat::calculate_damage;
use crate::infrastructure::components::combat::Invincibility;
use crate::infrastructure::components::{HitBox, HurtBox, Stats};
use crate::infrastructure::events::combat::DamageDealt;
use bevy::ecs::message::MessageWriter;
/// Collision detection system for HitBox vs HurtBox
/// HitBox vs HurtBox 碰撞检测系统
use bevy::prelude::*;

/// Collision detection system (T029)
///
/// Detects collision between HitBox (attacks) and HurtBox (damageable entities).
/// Publishes DamageDealt events when hits occur.
///
/// 检测 HitBox（攻击）与 HurtBox（可受伤实体）的碰撞
/// 命中时发布 DamageDealt 事件
/// T104: Modified to check Invincibility component instead of HurtBox.is_invincible
pub fn collision_detection_system(
    mut hitbox_query: Query<(Entity, &mut HitBox, &Transform)>,
    hurtbox_query: Query<
        (Entity, &HurtBox, &Transform, &Stats),
        (
            Without<HitBox>,
            Without<Invincibility>,
            Without<crate::infrastructure::components::enemy::DeathAnimation>,
        ),
    >,
    invincible_query: Query<&Invincibility>,
    mut damage_events: MessageWriter<DamageDealt>,
) {
    for (hitbox_entity, mut hitbox, hitbox_transform) in &mut hitbox_query {
        let hitbox_pos = hitbox_transform.translation.truncate();

        // Get world-space hitbox rect
        let hitbox_world = crate::domain::combat::collision::Rect {
            x: hitbox_pos.x + hitbox.rect.x,
            y: hitbox_pos.y + hitbox.rect.y,
            width: hitbox.rect.width,
            height: hitbox.rect.height,
        };

        for (target_entity, hurtbox, hurtbox_transform, defender_stats) in &hurtbox_query {
            // Skip if already hit this entity (deduplication)
            if hitbox.has_hit(target_entity) && !hitbox.can_pierce {
                continue;
            }

            // T104: Skip if entity has Invincibility component (i-frames)
            if invincible_query.get(target_entity).is_ok() {
                continue;
            }

            let hurtbox_pos = hurtbox_transform.translation.truncate();

            // Get world-space hurtbox rect
            let hurtbox_world = crate::domain::combat::collision::Rect {
                x: hurtbox_pos.x + hurtbox.rect.x,
                y: hurtbox_pos.y + hurtbox.rect.y,
                width: hurtbox.rect.width,
                height: hurtbox.rect.height,
            };

            // Check collision
            if hitbox_world.intersects(&hurtbox_world) {
                // Mark as hit
                hitbox.mark_hit(target_entity);

                // Get attacker stats (default if not found)
                let attacker_stats = crate::domain::combat::damage::Stats::default();

                // Convert component Stats to domain Stats
                let defender_domain_stats = defender_stats.to_domain();

                // Calculate damage
                let result = calculate_damage(
                    hitbox.damage,
                    &attacker_stats,
                    &defender_domain_stats,
                    hitbox.element,
                );

                info!(
                    "HitBox {:?} hit HurtBox {:?} for {:.1} damage (crit: {})",
                    hitbox_entity, target_entity, result.final_damage, result.is_critical
                );

                // Publish damage event
                damage_events.write(DamageDealt {
                    target: target_entity,
                    source: hitbox_entity,
                    result,
                    position: hurtbox_pos,
                });
            }
        }
    }
}

/// HitBox cleanup system (T030)
///
/// Automatically despawns HitBox entities after their lifetime expires.
///
/// 自动销毁生命周期结束的 HitBox 实体
pub fn hitbox_cleanup_system(
    mut commands: Commands,
    mut hitbox_query: Query<(Entity, &mut HitBox)>,
) {
    for (entity, mut hitbox) in &mut hitbox_query {
        if hitbox.lifetime_frames == 0 {
            commands.entity(entity).despawn();
        } else {
            hitbox.lifetime_frames = hitbox.lifetime_frames.saturating_sub(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collision_detection_system_compiles() {
        // This test just verifies the system function signature is correct
        let mut app = App::new();
        app.add_systems(Update, collision_detection_system);
    }

    #[test]
    fn test_hitbox_cleanup_system_compiles() {
        let mut app = App::new();
        app.add_systems(Update, hitbox_cleanup_system);
    }
}
