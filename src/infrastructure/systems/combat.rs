//! Combat and damage systems

use crate::domain::combat::{collision::Rect, Element};
use crate::infrastructure::components::{Enemy, Health, HitBox, InvincibilityTimer, Player};
use crate::infrastructure::events::combat::HitBoxSpawned;
use bevy::ecs::message::MessageWriter;
use bevy::prelude::*;

/// Attack input system (T033)
///
/// Listens for J key input and spawns HitBox entities for player attacks.
///
/// 监听 J 键输入并生成玩家攻击的 HitBox 实体
pub fn attack_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    mut commands: Commands,
    mut hitbox_spawned_events: MessageWriter<HitBoxSpawned>,
) {
    // Check if J key was just pressed
    if !keyboard.just_pressed(KeyCode::KeyJ) {
        return;
    }

    // Get player entity and position
    if let Some((player_entity, player_transform)) = player_query.iter().next() {
        let player_pos = player_transform.translation.truncate();

        // Create HitBox in front of player (to the right)
        // HitBox: 32x32, positioned 16 pixels to the right of player center
        let hitbox_rect = Rect {
            x: 0.0,
            y: -16.0, // Center vertically
            width: 32.0,
            height: 32.0,
        };

        let hitbox_entity = commands
            .spawn((
                Name::new("PlayerAttack"),
                HitBox::new(hitbox_rect, 10.0) // 10 base damage
                    .with_element(Element::Physical)
                    .with_lifetime(10), // 10 frames lifetime
                Transform::from_translation(Vec3::new(
                    player_pos.x + 16.0, // Offset to the right
                    player_pos.y,
                    0.0,
                )),
                // Visual debug (optional - can be removed later)
                Sprite {
                    color: Color::srgb(1.0, 0.0, 0.0).with_alpha(0.3), // Semi-transparent red
                    custom_size: Some(Vec2::new(32.0, 32.0)),
                    ..default()
                },
            ))
            .id();

        // Publish HitBoxSpawned event
        hitbox_spawned_events.write(HitBoxSpawned {
            hitbox: hitbox_entity,
            owner: player_entity,
            damage: 10.0,
        });

        info!("Player {:?} attacked! Spawned HitBox {:?}", player_entity, hitbox_entity);
    }
}

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
                health.take_damage(1.0);

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
