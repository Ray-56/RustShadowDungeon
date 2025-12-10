//! Enemy AI systems

use bevy::prelude::*;

use crate::domain::combat::collision::Rect;
use crate::infrastructure::components::enemy::{DeathAnimation, EnemyType, HitFlash};
use crate::infrastructure::components::obstacle::{Obstacle, ObstacleCollider};
use crate::infrastructure::components::{Enemy, Health, HurtBox, PatrolBehavior, Stats};
use crate::infrastructure::events::combat::{DamageDealt, EnemyDefeated};
use bevy::ecs::message::MessageReader;

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
    let spawn_position = Vec2::new(200.0, 0.0);
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
            Transform::from_translation(Vec3::new(spawn_position.x, spawn_position.y, 0.0)),
            // Visual representation (placeholder - simple colored sprite)
            Sprite {
                color: Color::srgb(0.2, 0.8, 0.2), // Green color for slime
                custom_size: Some(Vec2::new(16.0, 16.0)),
                ..default()
            },
            // AI Components (004-enemy-ai)
            EnemyAI::new(),                      // Initial state: Patrol
            Perception::new(200.0, 32.0, 400.0), // detection_range, attack_range, aggro_drop_range
            AggroTarget::default(),              // Default max_time_without_sight: 5.0
            PatrolConfig::new_random(spawn_position, 100.0, 2.0), // Random patrol, radius 100, wait 2s
            AttackConfig::new(
                32.0, // attack_range
                1.5,  // attack_cooldown (from enemies.ron)
                5.0,  // attack_damage (from enemies.ron)
                crate::infrastructure::components::enemy::AttackType::Melee,
                0.5, // attack_animation_duration
                0.2, // attack_hit_frame (from enemies.ron)
            ),
        ))
        .id();

    info!("Spawned slime enemy at entity {:?} with AI components", slime_entity);
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
    mut enemy_query: Query<
        (Entity, &mut Sprite, Option<&mut HitFlash>),
        (With<Enemy>, With<EnemyType>),
    >,
) {
    for event in damage_events.read() {
        // Try to get enemy entity
        match enemy_query.get_mut(event.target) {
            Ok((entity, mut sprite, hit_flash_opt)) => {
                // Store original color if not already stored
                if let Some(mut hit_flash) = hit_flash_opt {
                    hit_flash.duration = 0.15; // Reset duration
                } else {
                    let original = sprite.color;
                    // Add HitFlash component
                    commands.entity(entity).insert(HitFlash::new(0.15, original));
                }

                // Flash to red
                sprite.color = Color::srgb(1.0, 0.3, 0.3); // Red tint
                info!("Enemy {:?} hit flash triggered", entity);
            },
            Err(_) => {
                // Not an enemy, skip
            },
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
    mut enemy_query: Query<
        (Entity, &mut Transform, &mut Sprite, &mut Visibility),
        (With<Enemy>, With<EnemyType>),
    >,
) {
    for event in enemy_defeated_events.read() {
        if let Ok((entity, transform, mut sprite, _visibility)) = enemy_query.get_mut(event.enemy) {
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
            transform.scale = death_anim.initial_scale.lerp(death_anim.target_scale, t);

            // Fade out by darkening the color (simpler than alpha)
            let fade = death_anim.duration / 0.3;
            sprite.color = Color::srgb(0.3 * fade, 0.1 * fade, 0.1 * fade);
        } else {
            // Animation complete, despawn
            commands.entity(entity).despawn();
        }
    }
}

// ============================================================================
// Enemy AI Systems (004-enemy-ai)
// ============================================================================

use crate::domain::enemy::ai::{
    check_line_of_sight, should_drop_aggro, should_transition_to_chase, AIState,
};
use crate::infrastructure::components::enemy::{
    AggroTarget, AttackConfig, EnemyAI, PatrolConfig, Perception,
};
use crate::infrastructure::components::Player;
use crate::infrastructure::events::enemy::{
    EnemyAttackTriggered, EnemyDetectedPlayer, EnemyLostTarget, TargetLossReason,
};

/// 感知系统 - 检测玩家并更新感知状态
///
/// Perception system - detects players and updates perception state.
/// 感知系统，每 3-5 帧检测一次（约 50-100ms 间隔），更新 Perception 组件
pub fn perception_system(
    mut query: Query<
        (Entity, &Transform, &mut Perception, &mut AggroTarget),
        (With<Enemy>, With<EnemyAI>),
    >,
    player_query: Query<(Entity, &Transform), (With<Player>, Without<Enemy>)>,
    obstacle_query: Query<(&Transform, Option<&ObstacleCollider>, Option<&Sprite>), (With<Obstacle>, Without<Enemy>, Without<Player>)>,
    time: Res<Time>,
    mut event_writer: MessageWriter<EnemyDetectedPlayer>,
    mut lost_target_writer: MessageWriter<EnemyLostTarget>,
) {
    let current_time = time.elapsed_secs();

    // Collect obstacles once per frame (cached for all enemies)
    let obstacles: Vec<(f32, f32, f32, f32)> = obstacle_query
        .iter()
        .map(|(transform, collider_opt, sprite_opt)| {
            if let Some(collider) = collider_opt {
                // Use explicit collider if provided
                (collider.rect.x, collider.rect.y, collider.rect.width, collider.rect.height)
            } else if let Some(sprite) = sprite_opt {
                // Fallback to sprite size if no explicit collider
                let pos = transform.translation.truncate();
                let size = sprite.custom_size.unwrap_or(Vec2::new(16.0, 16.0));
                (pos.x - size.x / 2.0, pos.y - size.y / 2.0, size.x, size.y)
            } else {
                // Fallback to default 16x16 at transform position
                let pos = transform.translation.truncate();
                (pos.x - 8.0, pos.y - 8.0, 16.0, 16.0)
            }
        })
        .collect();

    for (enemy_entity, enemy_transform, mut perception, mut aggro) in query.iter_mut() {
        // Check if should drop aggro (runs every frame, not throttled)
        // This ensures we can drop aggro even if perception check is throttled
        if let Some(target_entity) = aggro.current_target {
            // Calculate actual distance to target entity (not just target_position, which may be stale)
            // This ensures we use the current player position, not a cached position
            let distance = if let Some((_, target_transform)) = player_query
                .iter()
                .find(|(entity, _)| *entity == target_entity)
            {
                // Target is still in world, calculate actual distance from current position
                let actual_dist = enemy_transform
                    .translation
                    .truncate()
                    .distance(target_transform.translation.truncate());
                #[cfg(test)]
                eprintln!("[perception_system] Using actual distance: {} (target at {:?})", actual_dist, target_transform.translation.truncate());
                actual_dist
            } else {
                // Target entity no longer exists, use distance from last known position
                let fallback_dist = perception
                    .target_position
                    .map(|pos| enemy_transform.translation.truncate().distance(pos))
                    .unwrap_or(f32::MAX);
                #[cfg(test)]
                eprintln!("[perception_system] Using fallback distance: {} (target_position: {:?})", fallback_dist, perception.target_position);
                fallback_dist
            };

            #[cfg(test)]
            eprintln!("[perception_system] Checking aggro drop: distance={}, aggro_drop_range={}, time_since_last_seen={}, max_time_without_sight={}", 
                distance, perception.aggro_drop_range, aggro.time_since_last_seen, aggro.max_time_without_sight);

            if should_drop_aggro(
                distance,
                perception.aggro_drop_range,
                aggro.time_since_last_seen,
                aggro.max_time_without_sight,
            ) {
                #[cfg(test)]
                eprintln!("[perception_system] Dropping aggro for enemy {:?}", enemy_entity);
                
                // Emit lost target event
                lost_target_writer.write(EnemyLostTarget {
                    enemy: enemy_entity,
                    lost_target: target_entity,
                    reason: if distance > perception.aggro_drop_range {
                        TargetLossReason::OutOfRange
                    } else {
                        TargetLossReason::LostSight
                    },
                });

                // Clear aggro
                aggro.current_target = None;
                aggro.last_seen_position = None;
                aggro.time_since_last_seen = 0.0;
                perception.target_position = None;
            }
        }

        // Throttle checks: only check every check_interval seconds (~3-5 frames)
        if current_time - perception.last_check_time < perception.check_interval {
            continue;
        }

        perception.last_check_time = current_time;

        // Find nearest player
        let mut nearest_player: Option<(Entity, Vec2, f32)> = None;
        for (player_entity, player_transform) in player_query.iter() {
            let distance = enemy_transform
                .translation
                .truncate()
                .distance(player_transform.translation.truncate());

            if nearest_player.is_none() || distance < nearest_player.unwrap().2 {
                // Line of sight check with step-based raycast
                // Now queries actual obstacles from the world
                let has_los = check_line_of_sight(
                    enemy_transform.translation.truncate(),
                    player_transform.translation.truncate(),
                    &obstacles,
                );

                if has_los {
                    nearest_player =
                        Some((player_entity, player_transform.translation.truncate(), distance));
                }
            }
        }

        // Update perception state
        if let Some((player_entity, player_pos, distance)) = nearest_player {
            // nearest_player already has LOS (checked above), so has_los is true
            let has_los = true;
            
            // Check if should detect player
            if should_transition_to_chase(distance, perception.detection_range, has_los) {
                perception.target_position = Some(player_pos);
                perception.has_line_of_sight = has_los;

                // Update aggro target
                if aggro.current_target != Some(player_entity) {
                    aggro.current_target = Some(player_entity);
                    aggro.last_seen_position = Some(player_pos);
                    aggro.time_since_last_seen = 0.0;

                    // Emit detection event
                    event_writer.write(EnemyDetectedPlayer {
                        enemy: enemy_entity,
                        player: player_entity,
                        detection_position: player_pos,
                        distance,
                    });
                } else {
                    // Update last seen position
                    aggro.last_seen_position = Some(player_pos);
                    aggro.time_since_last_seen = 0.0;
                }
            } else {
                // Out of detection range
                perception.target_position = None;
                perception.has_line_of_sight = false;
            }
        } else {
            // No player in range or no line of sight
            perception.has_line_of_sight = false;
            aggro.time_since_last_seen += perception.check_interval;
        }
    }
}

/// AI 状态机系统 - 处理状态转换
///
/// AI state machine system - handles state transitions.
/// 处理状态转换，调用领域层函数，使用实际 Transform 位置计算距离
pub fn ai_state_machine_system(
    mut query: Query<
        (
            Entity,
            &Transform,
            &mut EnemyAI,
            &Perception,
            &AggroTarget,
            Option<&AttackConfig>,
        ),
        With<Enemy>,
    >,
    time: Res<Time>,
) {
    let current_time = time.elapsed_secs();

    for (_entity, transform, mut ai, perception, aggro, attack_config) in query.iter_mut() {
        // Update state timer
        ai.state_timer = current_time - ai.state_entered_at;

        // Get actual enemy position
        let enemy_pos = transform.translation.truncate();

        // State transition logic
        let new_state = match ai.state {
            AIState::Patrol => {
                // Check if should transition to Chase
                if let Some(target_pos) = perception.target_position {
                    // Calculate actual distance
                    let distance = enemy_pos.distance(target_pos);

                    if should_transition_to_chase(
                        distance,
                        perception.detection_range,
                        perception.has_line_of_sight,
                    ) {
                        AIState::Chase
                    } else {
                        AIState::Patrol
                    }
                } else {
                    AIState::Patrol
                }
            },
            AIState::Chase => {
                // Check if should transition to Attack
                if let Some(attack_cfg) = attack_config {
                    if let Some(target_pos) = perception.target_position {
                        // Calculate actual distance
                        let distance = enemy_pos.distance(target_pos);

                        // Use should_transition_to_attack from domain layer
                        use crate::domain::enemy::ai::should_transition_to_attack;
                        if should_transition_to_attack(
                            distance,
                            attack_cfg.attack_range,
                            attack_cfg.is_cooldown_ready(),
                        ) {
                            AIState::Attack
                        } else {
                            AIState::Chase
                        }
                    } else {
                        // No target, check if should return
                        if aggro.current_target.is_none() {
                            AIState::Return
                        } else {
                            AIState::Chase
                        }
                    }
                } else {
                    AIState::Chase
                }
            },
            AIState::Attack => {
                // Attack state handled by attack_system
                // Transition back to Chase after attack (handled in attack_system)
                AIState::Attack
            },
            AIState::Return => {
                // Check if returned to spawn (simplified - would need spawn position)
                // For now, transition back to Patrol after a short time
                if ai.state_timer > 1.0 {
                    AIState::Patrol
                } else {
                    AIState::Return
                }
            },
        };

        // Update state if changed
        if new_state != ai.state {
            ai.previous_state = ai.state;
            ai.state = new_state;
            ai.state_entered_at = current_time;
            ai.state_timer = 0.0;
        }
    }
}

/// 追逐系统 - 处理追逐行为
///
/// Chase system - handles chase behavior.
/// 处理追逐行为，实际移动敌人朝向玩家
pub fn chase_system(
    mut query: Query<
        (&mut Transform, &EnemyAI, &Perception, &AggroTarget),
        (With<Enemy>, With<EnemyAI>),
    >,
    time: Res<Time>,
) {
    let dt = time.delta_secs();

    for (mut transform, ai, perception, _aggro) in query.iter_mut() {
        // Only process enemies in Chase state
        if ai.state != AIState::Chase {
            continue;
        }

        // Move towards target if we have one
        if let Some(target_pos) = perception.target_position {
            let current_pos = transform.translation.truncate();
            let direction = (target_pos - current_pos).normalize_or_zero();
            let speed = 80.0; // Chase speed (pixels per second, faster than patrol)
            let movement = direction * speed * dt;

            transform.translation.x += movement.x;
            transform.translation.y += movement.y;
        }
    }
}

/// 返回巡逻系统 - 处理脱战，返回生成位置
///
/// Return to patrol system - handles aggro drop, returns to spawn.
/// 处理脱战，返回生成位置，实际移动敌人回到生成点
pub fn return_to_patrol_system(
    mut query: Query<
        (&mut Transform, &EnemyAI, Option<&PatrolConfig>),
        (With<Enemy>, With<EnemyAI>),
    >,
    time: Res<Time>,
) {
    let dt = time.delta_secs();

    for (mut transform, ai, patrol_config) in query.iter_mut() {
        // Only process enemies in Return state
        if ai.state != AIState::Return {
            continue;
        }

        // Move back to spawn position
        if let Some(patrol) = patrol_config {
            let current_pos = transform.translation.truncate();
            let spawn_pos = patrol.spawn_position;
            let distance = current_pos.distance(spawn_pos);

            if distance < 5.0 {
                // Reached spawn, will transition to Patrol in state machine
                // No movement needed
            } else {
                // Move towards spawn
                let direction = (spawn_pos - current_pos).normalize_or_zero();
                let speed = 50.0; // Return speed (pixels per second, same as patrol)
                let movement = direction * speed * dt;

                transform.translation.x += movement.x;
                transform.translation.y += movement.y;
            }
        }
    }
}

/// 攻击系统 - 处理攻击行为
///
/// Attack system - handles attack behavior.
/// 处理攻击行为，触发攻击动画和伤害判定
pub fn attack_system(
    mut query: Query<
        (Entity, &Transform, &mut EnemyAI, &AttackConfig, &AggroTarget),
        (With<Enemy>, With<EnemyAI>),
    >,
    time: Res<Time>,
    mut event_writer: MessageWriter<EnemyAttackTriggered>,
) {
    let current_time = time.elapsed_secs();

    for (enemy_entity, enemy_transform, mut ai, attack_config, aggro) in query.iter_mut() {
        if ai.state != AIState::Attack {
            continue;
        }

        // Check if attack animation is at hit frame
        if ai.state_timer >= attack_config.attack_hit_frame
            && ai.state_timer < attack_config.attack_hit_frame + 0.05
        {
            // Emit attack event
            if let Some(target_entity) = aggro.current_target {
                event_writer.write(EnemyAttackTriggered {
                    enemy: enemy_entity,
                    target: target_entity,
                    damage: attack_config.attack_damage,
                    attack_type: attack_config.attack_type,
                    attack_position: enemy_transform.translation.truncate(),
                });
            }
        }

        // Check if attack animation is complete
        if ai.state_timer >= attack_config.attack_animation_duration {
            // Start cooldown
            // Note: Cooldown is managed by attack_cooldown_system, but we need to set it here
            // The attack_cooldown_system will decrement it each frame

            // Transition back to Chase (or Return if no target)
            ai.previous_state = ai.state;
            if aggro.current_target.is_some() {
                ai.state = AIState::Chase;
            } else {
                ai.state = AIState::Return;
            }
            ai.state_entered_at = current_time;
            ai.state_timer = 0.0;
        }
    }
}

/// 攻击冷却系统 - 管理攻击冷却时间
///
/// Attack cooldown system - manages attack cooldown timers.
/// 管理攻击冷却时间
pub fn attack_cooldown_system(
    mut query: Query<(&mut AttackConfig, &EnemyAI), With<Enemy>>,
    time: Res<Time>,
) {
    let delta = time.delta_secs();

    for (mut attack_config, ai) in query.iter_mut() {
        // Start cooldown when attack animation completes
        if ai.state == AIState::Attack && ai.state_timer >= attack_config.attack_animation_duration
        {
            attack_config.current_cooldown = attack_config.attack_cooldown;
        }

        // Decrement cooldown
        if attack_config.current_cooldown > 0.0 {
            attack_config.current_cooldown -= delta;
            if attack_config.current_cooldown < 0.0 {
                attack_config.current_cooldown = 0.0;
            }
        }
    }
}

/// 巡逻系统 - 处理巡逻行为
///
/// Patrol system - handles patrol behavior.
/// 处理巡逻行为，支持随机点和预设路径点两种模式，并实际移动敌人
pub fn patrol_system(
    mut query: Query<
        (&mut Transform, &mut EnemyAI, &mut PatrolConfig),
        (With<Enemy>, With<EnemyAI>),
    >,
    time: Res<Time>,
) {
    let delta = time.delta_secs();

    for (mut enemy_transform, _ai, mut patrol_config) in query.iter_mut() {
        if _ai.state != AIState::Patrol {
            continue;
        }

        let current_pos = enemy_transform.translation.truncate();

        // Check if reached current target or need new target
        if let Some(target_pos) = patrol_config.current_target {
            let distance = current_pos.distance(target_pos);

            if distance < 5.0 {
                // Reached target, wait
                patrol_config.wait_timer += delta;
                if patrol_config.wait_timer >= patrol_config.wait_time {
                    // Wait complete, choose new target
                    patrol_config.current_target = None;
                    patrol_config.wait_timer = 0.0;
                }
                // Enemy stays in place while waiting
            } else {
                // Move towards target
                let direction = (target_pos - current_pos).normalize_or_zero();
                let speed = 50.0; // Patrol speed (pixels per second)
                let movement = direction * speed * delta;

                enemy_transform.translation.x += movement.x;
                enemy_transform.translation.y += movement.y;
            }
        } else {
            // Need new target
            if patrol_config.use_waypoints {
                // Waypoint mode
                if !patrol_config.waypoints.is_empty() {
                    let next_index =
                        (patrol_config.current_waypoint_index + 1) % patrol_config.waypoints.len();
                    patrol_config.current_waypoint_index = next_index;
                    patrol_config.current_target = Some(patrol_config.waypoints[next_index]);
                }
            } else {
                // Random point mode
                use crate::domain::enemy::ai::calculate_patrol_target;
                patrol_config.current_target = Some(calculate_patrol_target(
                    current_pos,
                    patrol_config.spawn_position,
                    patrol_config.patrol_radius,
                ));
            }
        }
    }
}
