//! Combat and damage systems

use bevy::prelude::*;

use crate::infrastructure::components::{Enemy, Health, InvincibilityTimer, Player};

/// Check collision between player and enemies, apply damage
pub fn enemy_collision_system(
    mut commands: Commands,
    mut player_query: Query<
        (Entity, &Transform, &mut Health, Option<&InvincibilityTimer>),
        With<Player>,
    >,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    for (player_entity, player_transform, mut health, invincibility) in &mut player_query {
        // Skip if invincible
        if let Some(timer) = invincibility {
            if timer.is_active() {
                continue;
            }
        }

        let player_pos = player_transform.translation.truncate();
        const COLLISION_DISTANCE_SQ: f32 = 28.0 * 28.0; // ~28 pixels

        for enemy_transform in &enemy_query {
            let enemy_pos = enemy_transform.translation.truncate();
            let distance_sq = player_pos.distance_squared(enemy_pos);

            if distance_sq < COLLISION_DISTANCE_SQ {
                // Take damage
                health.take_damage(1);

                // Add invincibility
                commands.entity(player_entity).insert(InvincibilityTimer::new(1.0)); // 1 second invincibility

                // Check if dead
                if !health.is_alive() {
                    info!("Player died! Respawning...");
                    // Reset health
                    health.reset();
                    // Teleport to spawn point
                    commands.entity(player_entity).insert(Transform::from_xyz(0.0, 100.0, 1.0));
                }

                break; // Only one hit per frame
            }
        }
    }
}

/// Update invincibility timers
pub fn invincibility_timer_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut InvincibilityTimer, &mut Sprite)>,
) {
    let dt = time.delta_secs();

    for (entity, mut timer, mut sprite) in &mut query {
        timer.tick(dt);

        // Flashing effect while invincible
        if timer.is_active() {
            // Flash by toggling alpha (using time)
            let flash = ((timer.remaining * 10.0).sin() + 1.0) / 2.0; // 0.0 to 1.0
            sprite.color.set_alpha(0.5 + flash * 0.5); // 0.5 to 1.0
        } else {
            // Restore full opacity
            sprite.color.set_alpha(1.0);
            // Remove invincibility component
            commands.entity(entity).remove::<InvincibilityTimer>();
        }
    }
}

