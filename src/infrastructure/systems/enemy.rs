//! Enemy AI systems

use bevy::prelude::*;

use crate::domain::combat::collision::Rect;
use crate::infrastructure::components::{
    Enemy, Health, HurtBox, PatrolBehavior, Stats,
};
use crate::infrastructure::components::enemy::{DeathAnimation, EnemyType, HitFlash};
use crate::infrastructure::events::combat::{DamageDealt, EnemyDefeated};
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::prelude::*;

/// Enemy spawn system - spawns slime enemies (T036)
///
/// Spawns a slime enemy with Health, Stats, and HurtBox components.
///
/// 生成史莱姆敌人，包含 Health、Stats 和 HurtBox 组件
pub fn spawn_slime_system(
    mut commands: Commands,
    query: Query<Entity, (With<Enemy>, With<crate::infrastructure::components::enemy::EnemyType>)>,
) {
    // Only spawn if no enemies exist
    if !query.is_empty() {
        return;
    }

    // Spawn slime enemy at position (200, 0)
    let slime_entity = commands
        .spawn((
            Enemy,
            crate::infrastructure::components::enemy::EnemyType::Slime,
            crate::infrastructure::components::enemy::EnemyId(1),
            Name::new("Slime"),
            // Stats: attack 5, defense 0
            Stats::new(5.0, 0.0),
            // Health: 30 HP
            Health::new(30.0),
            // HurtBox: 16x16 at center
            HurtBox::new(Rect { x: -8.0, y: -8.0, width: 16.0, height: 16.0 }),
            // Transform
            Transform::from_translation(Vec3::new(200.0, 0.0, 0.0)),
            // Visual representation (placeholder - simple colored sprite)
            Sprite {
                color: Color::srgb(0.2, 0.8, 0.2), // Green color for slime
                custom_size: Some(Vec2::new(16.0, 16.0)),
                ..default()
            },
        ))
        .id();

    info!("Spawned slime enemy at entity {:?}", slime_entity);
}

/// Enemy patrol system - makes enemies move back and forth
///
/// For Kinematic bodies, we directly move the transform instead of using velocity
pub fn enemy_patrol_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut PatrolBehavior), With<Enemy>>,
) {
    let dt = time.delta_secs();

    for (mut transform, mut patrol) in &mut query {
        let current_x = transform.translation.x;

        // Check if we've reached a boundary
        if current_x <= patrol.left_bound {
            patrol.direction = 1.0; // Turn right
        } else if current_x >= patrol.right_bound {
            patrol.direction = -1.0; // Turn left
        }

        // Move the enemy directly (for Kinematic bodies)
        transform.translation.x += patrol.direction * patrol.speed * dt;
    }
}

/// Enemy hit flash system
///
/// 敌人受伤闪烁系统
///
/// Listens to DamageDealt events and applies hit flash effect to enemies.
/// Changes sprite color to red briefly when enemy takes damage.
pub fn enemy_hit_flash_system(
    mut damage_events: MessageReader<DamageDealt>,
    mut commands: Commands,
    mut enemy_query: Query<(Entity, &mut Sprite, Option<&mut HitFlash>), (With<Enemy>, With<EnemyType>)>,
) {
    for event in damage_events.read() {
        // Try to get enemy entity
        match enemy_query.get_mut(event.target) {
            Ok((entity, mut sprite, hit_flash_opt)) => {
                // Store original color if not already stored
                let original_color = if let Some(mut hit_flash) = hit_flash_opt {
                    hit_flash.duration = 0.15; // Reset duration
                    hit_flash.original_color
                } else {
                    let original = sprite.color;
                    // Add HitFlash component
                    commands.entity(entity).insert(HitFlash::new(0.15, original));
                    original
                };

                // Flash to red
                sprite.color = Color::srgb(1.0, 0.3, 0.3); // Red tint
                info!("Enemy {:?} hit flash triggered", entity);
            }
            Err(_) => {
                // Not an enemy, skip
            }
        }
    }
}

/// Update hit flash effect
///
/// 更新受伤闪烁效果
///
/// Gradually fades the flash effect back to original color.
pub fn enemy_hit_flash_update_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut HitFlash, &mut Sprite)>,
    time: Res<Time>,
) {
    let delta = time.delta_secs();

    for (entity, mut hit_flash, mut sprite) in &mut query {
        hit_flash.duration -= delta;

        if hit_flash.duration > 0.0 {
            // Fade back to original color
            let t = (hit_flash.duration / 0.15).clamp(0.0, 1.0);
            // Gradually fade from red back to original color
            // Use a simple linear blend: more red at start, more original at end
            if t > 0.7 {
                // Mostly red
                sprite.color = Color::srgb(1.0, 0.3, 0.3);
            } else if t > 0.3 {
                // Blend between red and original
                sprite.color = Color::srgb(0.7, 0.5, 0.4);
            } else {
                // Mostly original
                sprite.color = hit_flash.original_color;
            }
        } else {
            // Restore original color and remove component
            sprite.color = hit_flash.original_color;
            commands.entity(entity).remove::<HitFlash>();
        }
    }
}

/// Enemy death animation system
///
/// 敌人死亡动画系统
///
/// Listens to EnemyDefeated events and spawns death animation.
/// Instead of immediately despawning, plays a shrink/fade animation.
/// Removes HurtBox to prevent further damage during animation.
pub fn enemy_death_animation_system(
    mut enemy_defeated_events: MessageReader<EnemyDefeated>,
    mut commands: Commands,
    mut enemy_query: Query<(Entity, &mut Transform, &mut Sprite, &mut Visibility), (With<Enemy>, With<EnemyType>)>,
) {
    for event in enemy_defeated_events.read() {
        if let Ok((entity, mut transform, mut sprite, mut visibility)) =
            enemy_query.get_mut(event.enemy)
        {
            // Instead of despawning immediately, add death animation component
            let initial_scale = transform.scale;
            commands.entity(entity).insert(DeathAnimation::new(0.3, initial_scale));

            // Remove HurtBox to prevent further damage during death animation
            commands.entity(entity).remove::<HurtBox>();

            // Change color to darker/dead color
            sprite.color = Color::srgb(0.3, 0.1, 0.1); // Dark red/brown

            info!("Enemy {:?} death animation started", entity);
        }
    }
}

/// Update death animation
///
/// 更新死亡动画
///
/// Shrinks and fades the enemy sprite, then despawns when animation completes.
pub fn enemy_death_animation_update_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut DeathAnimation, &mut Transform, &mut Sprite), With<Enemy>>,
    time: Res<Time>,
) {
    let delta = time.delta_secs();

    for (entity, mut death_anim, mut transform, mut sprite) in &mut query {
        death_anim.duration -= delta;

        if death_anim.duration > 0.0 {
            // Interpolate scale from initial to target (shrink)
            let t = 1.0 - (death_anim.duration / 0.3);
            transform.scale = death_anim
                .initial_scale
                .lerp(death_anim.target_scale, t);

            // Fade out by darkening the color (simpler than alpha)
            let fade = death_anim.duration / 0.3;
            sprite.color = Color::srgb(0.3 * fade, 0.1 * fade, 0.1 * fade);
        } else {
            // Animation complete, despawn
            commands.entity(entity).despawn();
        }
    }
}
