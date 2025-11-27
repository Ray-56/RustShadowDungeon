//! Skill system
//!
//! 技能系统
//!
//! Handles skill input, cooldown, MP consumption, and projectile spawning.

use avian2d::prelude::*;
use bevy::prelude::*;

use crate::infrastructure::components::combat::{Fireball, Lifetime, Skill};
use crate::infrastructure::components::{Player, MP};
use crate::infrastructure::events::combat::{DamageDealt, SkillActivated};
use crate::infrastructure::plugins::physics::CollisionLayer;
use crate::infrastructure::resources::SkillDatabase;

/// Skill input system
///
/// 技能输入系统
///
/// T082: Listens for K key input, checks cooldown and MP, publishes SkillActivated event.
pub fn skill_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    player_query: Query<(Entity, &Skill, &MP), With<Player>>,
    mut skill_activated_events: MessageWriter<SkillActivated>,
    player_transform_query: Query<
        &Transform,
        (With<Player>, Without<crate::infrastructure::systems::GameCamera>),
    >,
) {
    // Check if K key was just pressed
    if !keyboard.just_pressed(KeyCode::KeyK) {
        return;
    }

    // Debug: Log that K key was pressed
    info!("K key pressed - checking player query...");

    // Get player entity, skill, and MP
    if let Some((player_entity, skill, mp)) = player_query.iter().next() {
        info!(
            "Player found: {:?}, Skill: {:?}, MP: {:.0}/{:.0}",
            player_entity, skill.skill_id, mp.current, mp.max
        );
        // Check if skill is ready (cooldown finished)
        if !skill.is_ready() {
            info!(
                "Skill {} not ready, cooldown remaining: {:.2}s",
                skill.skill_id, skill.remaining_cooldown
            );
            return; // Skill still on cooldown
        }

        // Check if player has enough MP
        if !mp.has_enough(skill.mp_cost) {
            info!(
                "Not enough MP for skill {}: need {:.0}, have {:.0}",
                skill.skill_id, skill.mp_cost, mp.current
            );
            return; // Not enough MP
        }

        info!("Activating skill: {} (cost: {:.0} MP)", skill.skill_id, skill.mp_cost);

        // Get target position (mouse position or forward direction)
        // For now, use player's forward direction (to the right)
        let target_position = if let Some(player_transform) = player_transform_query.iter().next() {
            let player_pos = player_transform.translation.truncate();
            Vec2::new(player_pos.x + 200.0, player_pos.y) // 200 pixels to the right
        } else {
            Vec2::ZERO
        };

        // Publish SkillActivated event
        skill_activated_events.write(SkillActivated {
            player: player_entity,
            skill_id: skill.skill_id.clone(),
            target_position,
        });

        info!("SkillActivated event published for skill: {}", skill.skill_id);
    } else {
        // Debug: More detailed error message
        let player_count = player_query.iter().count();
        info!(
            "No player with Skill and MP components found. Query returned {} results",
            player_count
        );
    }
}

/// Skill cooldown system
///
/// 技能冷却系统
///
/// T083: Updates all Skill components' remaining_cooldown.
pub fn skill_cooldown_system(mut skill_query: Query<&mut Skill>, time: Res<Time>) {
    let delta = time.delta_secs();

    for mut skill in &mut skill_query {
        skill.update(delta);
    }
}

/// MP regeneration system
///
/// MP 恢复系统
///
/// Gradually restores MP over time (e.g., 10 MP per second).
pub fn mp_regeneration_system(mut player_query: Query<&mut MP, With<Player>>, time: Res<Time>) {
    let delta = time.delta_secs();
    let regen_rate = 10.0; // MP per second

    for mut mp in &mut player_query {
        if mp.current < mp.max {
            mp.restore(regen_rate * delta);
        }
    }
}

/// Skill activation system
///
/// 技能激活系统
///
/// T085: Listens to SkillActivated events and sets skill cooldown.
/// This system should run before mp_consumption_system to ensure cooldown is set.
pub fn skill_activation_system(
    mut skill_activated_events: MessageReader<SkillActivated>,
    mut skill_query: Query<&mut Skill>,
) {
    for event in skill_activated_events.read() {
        // Get player skill component
        if let Ok(mut skill) = skill_query.get_mut(event.player) {
            // Set cooldown if skill ID matches
            if skill.skill_id == event.skill_id {
                skill.remaining_cooldown = skill.cooldown;
                info!(
                    "Skill {} activated, cooldown set to {:.2}s",
                    skill.skill_id, skill.cooldown
                );
            }
        } else {
            warn!("Skill component not found for player {:?}", event.player);
        }
    }
}

/// MP consumption system
///
/// MP 扣除系统
///
/// T084: Listens to SkillActivated events and deducts MP from player.
pub fn mp_consumption_system(
    mut skill_activated_events: MessageReader<SkillActivated>,
    mut player_query: Query<&mut MP, With<Player>>,
    skill_query: Query<&Skill>,
) {
    for event in skill_activated_events.read() {
        // Get player MP
        if let Ok(mut mp) = player_query.get_mut(event.player) {
            // Get skill to get MP cost
            if let Ok(skill) = skill_query.get(event.player) {
                // Consume MP
                let consumed = mp.consume(skill.mp_cost);
                if consumed {
                    info!(
                        "MP consumed: {:.0} MP (remaining: {:.0}/{:.0})",
                        skill.mp_cost, mp.current, mp.max
                    );
                } else {
                    warn!("Failed to consume MP: not enough MP");
                }
            } else {
                warn!("Skill component not found for player {:?}", event.player);
            }
        } else {
            warn!("MP component not found for player {:?}", event.player);
        }
    }
}

/// Projectile system (fireball generation)
///
/// 弹道系统（火球生成）
///
/// T087: Listens to SkillActivated events and spawns fireball projectiles.
/// Fireball has velocity, lifetime, and collision detection.
pub fn projectile_system(
    mut skill_activated_events: MessageReader<SkillActivated>,
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    _skill_query: Query<&Skill, With<Player>>,
    skill_database: Res<SkillDatabase>,
) {
    for event in skill_activated_events.read() {
        info!(
            "Processing SkillActivated event: {} for player {:?}",
            event.skill_id, event.player
        );

        // Get skill data from database
        let skill_data = match skill_database.get(&event.skill_id) {
            Some(data) => data,
            None => {
                warn!("Skill {} not found in database", event.skill_id);
                continue; // Skill not found in database
            },
        };

        // Only handle fireball for now
        if event.skill_id != "fireball" {
            continue;
        }

        // Get player position and direction
        if let Some(player_transform) = player_query.iter().next() {
            let player_pos = player_transform.translation.truncate();

            // Calculate direction to target
            let direction = (event.target_position - player_pos).normalize_or_zero();
            let speed = 300.0; // pixels per second
            let velocity_vec = direction * speed;

            // Spawn fireball entity with visual and physics
            let _fireball_entity = commands
                .spawn((
                    Name::new("Fireball"),
                    Fireball {
                        velocity: velocity_vec, // Store velocity in Fireball component for movement
                    },
                    Skill {
                        skill_id: event.skill_id.clone(),
                        cooldown: skill_data.cooldown,
                        remaining_cooldown: 0.0,
                        mp_cost: skill_data.mp_cost,
                        damage: skill_data.damage,
                        element: skill_data.element,
                    },
                    Lifetime::new(3.0), // 3 second lifetime
                    // Visual: Orange-red fireball sprite
                    Sprite {
                        color: Color::srgb(1.0, 0.3, 0.0), // Orange-red fireball color
                        custom_size: Some(Vec2::new(16.0, 16.0)), // 16x16 pixels
                        ..default()
                    },
                    Transform::from_translation(Vec3::new(
                        player_pos.x,
                        player_pos.y,
                        10.0, // Higher z-index to ensure visibility above other entities
                    )),
                    // Physics: Kinematic body (velocity handled by system)
                    // Note: We use Kinematic but don't use LinearVelocity - we update Transform directly
                    // This is because we want precise control over fireball movement
                    RigidBody::Kinematic,  // Kinematic = not affected by gravity
                    Collider::circle(8.0), // 8 pixel radius (16x16 sprite)
                    CollisionLayer::Player.collision_filter(), // Use player collision layer
                                           // Note: No PixelSnap - fireballs should move smoothly, not snap to grid
                                           // Note: No LinearVelocity - we update Transform directly in fireball_movement_system
                ))
                .id();

            info!(
                "Fireball spawned at ({:.1}, {:.1}) with velocity ({:.1}, {:.1})",
                player_pos.x, player_pos.y, velocity_vec.x, velocity_vec.y
            );
        }
    }
}

/// Fireball collision system
///
/// 火球碰撞系统
///
/// T088: Detects fireball collision with enemies, publishes DamageDealt event,
/// plays explosion animation, and despawns fireball.
pub fn fireball_collision_system(
    mut commands: Commands,
    fireball_query: Query<(Entity, &Transform, &Skill), With<Fireball>>,
    enemy_query: Query<
        (Entity, &Transform),
        (With<crate::infrastructure::components::Enemy>, Without<Fireball>),
    >,
    mut damage_events: MessageWriter<DamageDealt>,
) {
    for (fireball_entity, fireball_transform, skill) in &fireball_query {
        let fireball_pos = fireball_transform.translation.truncate();
        let fireball_radius = 8.0; // Fireball collision radius

        // Check collision with enemies
        for (enemy_entity, enemy_transform) in &enemy_query {
            let enemy_pos = enemy_transform.translation.truncate();
            let distance = fireball_pos.distance(enemy_pos);

            // Simple distance-based collision (in real implementation, use proper collision detection)
            if distance < fireball_radius + 8.0 {
                // Enemy radius ~8.0
                // Publish damage event
                damage_events.write(DamageDealt {
                    target: enemy_entity,
                    source: fireball_entity,
                    result: crate::domain::combat::DamageResult {
                        final_damage: skill.damage,
                        is_critical: false,
                        element: skill.element,
                    },
                    position: enemy_pos,
                });

                // Despawn fireball (exploded)
                commands.entity(fireball_entity).despawn();
                break; // Only hit one enemy per fireball
            }
        }
    }
}

/// Fireball movement system
///
/// 火球移动系统
///
/// Updates fireball position based on velocity stored in Fireball component.
/// Note: For Kinematic rigidbodies, we directly update Transform.
/// The physics engine will sync the position on the next frame.
pub fn fireball_movement_system(
    mut fireball_query: Query<(&Fireball, &mut Transform)>,
    time: Res<Time>,
) {
    let delta = time.delta_secs();

    for (fireball, mut transform) in &mut fireball_query {
        // Update position based on velocity
        // For Kinematic bodies, we can directly modify Transform
        transform.translation.x += fireball.velocity.x * delta;
        transform.translation.y += fireball.velocity.y * delta;
    }
}

/// Lifetime system
///
/// 生命周期系统
///
/// T089: Updates Lifetime components, despawns entities when lifetime expires.
pub fn lifetime_system(
    mut commands: Commands,
    mut lifetime_query: Query<(Entity, &mut Lifetime)>,
    time: Res<Time>,
) {
    let delta = time.delta_secs();

    for (entity, mut lifetime) in &mut lifetime_query {
        if !lifetime.update(delta) {
            // Lifetime expired, despawn entity
            commands.entity(entity).despawn();
        }
    }
}
