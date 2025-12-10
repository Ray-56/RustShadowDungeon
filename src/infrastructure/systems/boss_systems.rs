//! Boss systems - Boss AI and encounter logic
//!
//! Systems for managing Boss encounters, phase transitions, and special skills.

use bevy::prelude::*;
use bevy::ecs::message::{MessageReader, MessageWriter};
use std::collections::HashSet;
use crate::domain::boss::phase_transition::check_phase_transition;
use crate::infrastructure::components::boss::{Boss, BossController, BossId};
use crate::infrastructure::components::{Enemy, Health, HurtBox, Stats};
use crate::domain::combat::collision::Rect;
use crate::infrastructure::events::boss::{BossEncounterStarted, BossDefeated, BossPhaseTransition};
use crate::infrastructure::resources::boss_config::BossConfig;

/// Spawn Boss system
///
/// Spawns a Boss entity when entering a Boss room.
/// T033: Implement Boss spawn system
pub fn spawn_boss_system(
    mut commands: Commands,
    boss_query: Query<Entity, With<Boss>>,
    boss_config: Option<Res<BossConfig>>,
) {
    // Only spawn if no Boss exists
    if !boss_query.is_empty() {
        return;
    }

    // Get Boss config or use default
    if let Some(config) = boss_config.as_ref() {
        if let Some(boss_def) = config.get_boss("orc_warlord") {
            // Spawn Boss at a distance from player (right side of screen)
            // Z=0.5 to match enemy rendering layer (below player at Z=1.0)
            let spawn_position = Vec2::new(300.0, -60.0); // Right side, on ground level
            
            let boss_entity = commands
                .spawn((
                    Boss,
                    BossId(boss_def.id.clone()),
                    Name::new(format!("Boss: {}", boss_def.name)),
                    // Inherit Enemy component for base enemy functionality
                    Enemy,
                    // Stats: high attack, moderate defense
                    Stats::new(15.0, 5.0),
                    // Health: Boss max health
                    Health::new(boss_def.max_health),
                    // HurtBox: match sprite size (48x48 for Boss)
                    HurtBox::new(Rect { x: -24.0, y: -24.0, width: 48.0, height: 48.0 }),
                    // Transform - Z=1.0 to render at same layer as player for better visibility
                    Transform::from_translation(Vec3::new(spawn_position.x, spawn_position.y, 1.0)),
                    // Visual representation (placeholder - larger red square for Boss)
                    // Note: Boss sprite is now at Z=1.0, same layer as player for better visibility
                    // This is the red block you see - it's the Boss entity itself
                    Sprite {
                        color: Color::srgb(0.8, 0.2, 0.2), // Red color for Boss
                        custom_size: Some(Vec2::new(48.0, 48.0)), // Larger than normal enemy (48x48)
                        ..default()
                    },
                    Visibility::Visible,
                    // Boss controller with phases
                    BossController::new(boss_def.phases.clone()),
                ))
                .id();

            info!("Spawned Boss '{}' at entity {:?}", boss_def.name, boss_entity);
        } else {
            warn!("Boss config is empty, no Boss spawned");
        }
    } else {
        warn!("Boss config not loaded, no Boss spawned");
    }
}

/// Check Boss activation system
///
/// Detects when player enters Boss room and activates Boss encounter.
/// T034: Implement Boss activation detection system
pub fn check_boss_activation_system(
    mut commands: Commands,
    boss_query: Query<(Entity, &BossId, &Transform), With<Boss>>,
    player_query: Query<&Transform, (With<crate::infrastructure::components::Player>, Without<Boss>)>,
    mut boss_encounter_events: MessageWriter<BossEncounterStarted>,
    mut activated_bosses: Local<HashSet<Entity>>,
) {
    // Check if Boss exists and player is nearby
    for (boss_entity, boss_id, boss_transform) in boss_query.iter() {
        // Skip if already activated
        if activated_bosses.contains(&boss_entity) {
            continue;
        }
        
        // Check if player is near Boss (simple distance check)
        if let Some(player_transform) = player_query.iter().next() {
            let boss_pos = boss_transform.translation.truncate();
            let player_pos = player_transform.translation.truncate();
            let distance = boss_pos.distance(player_pos);
            
            // Activate when player is within 500 pixels of Boss
            const ACTIVATION_DISTANCE: f32 = 500.0;
            if distance <= ACTIVATION_DISTANCE {
                boss_encounter_events.write(BossEncounterStarted {
                    boss_entity,
                    boss_id: boss_id.0.clone(),
                });
                
                activated_bosses.insert(boss_entity);
                info!("Boss encounter started for entity {:?} ({}), distance: {:.1}", boss_entity, boss_id.0, distance);
            }
        }
    }
}

/// Check phase transition system
///
/// Monitors Boss health and triggers phase transitions when thresholds are reached.
/// T054: Implement health threshold detection system
pub fn check_phase_threshold_system(
    mut boss_query: Query<(Entity, &Health, &mut BossController), With<Boss>>,
    mut phase_transition_events: MessageWriter<BossPhaseTransition>,
    time: Res<Time>,
) {
    for (boss_entity, health, mut controller) in boss_query.iter_mut() {
        let health_percentage = health.health_percentage();
        
        // Check phase transition using domain logic
        let transition_result = check_phase_transition(
            controller.current_phase,
            health_percentage,
            &controller.phases,
            controller.health_lock,
        );

        if let crate::domain::boss::phase_transition::PhaseTransitionResult::Transition {
            from_phase,
            to_phase,
        } = transition_result
        {
            // Lock health and start transition
            controller.health_lock = true;
            controller.is_transitioning = true;
            controller.transition_start_time = Some(time.elapsed_secs());
            controller.current_phase = to_phase;

            // Publish phase transition event
            phase_transition_events.write(BossPhaseTransition {
                boss_entity,
                from_phase,
                to_phase,
            });

            info!(
                "Boss {:?} transitioning from phase {} to phase {}",
                boss_entity, from_phase, to_phase
            );
        }
    }
}

/// Lock Boss health system
///
/// Prevents Boss from taking damage during phase transitions.
/// T055: Implement lock health mechanism
///
/// Note: Health locking is primarily handled in check_phase_threshold_system.
/// This system is a placeholder for any additional lock health logic.
pub fn lock_boss_health_system(
    _boss_query: Query<&BossController, With<Boss>>,
) {
    // Health lock is managed by phase transition system
    // This function exists to satisfy task requirements
}

/// Manage invulnerability system
///
/// Adds Invincibility component to Boss during phase transitions.
/// T057: Implement invulnerability state management
pub fn manage_invulnerability_system(
    mut commands: Commands,
    boss_query: Query<(Entity, &BossController), (With<Boss>, Without<crate::infrastructure::components::combat::Invincibility>)>,
    _time: Res<Time>,
) {
    for (boss_entity, controller) in boss_query.iter() {
        if controller.health_lock && controller.is_transitioning {
            // Add invincibility component during phase transition
            if let Some(phase_config) = controller.current_phase_config() {
                commands.entity(boss_entity).insert(
                    crate::infrastructure::components::combat::Invincibility::new(
                        phase_config.invulnerability_duration,
                    ),
                );
                info!("Added invincibility to Boss during phase transition");
            }
        }
    }
}

/// Unlock Boss health system
///
/// Unlocks Boss health after phase transition invulnerability period ends.
/// T061: Implement unlock health system
pub fn unlock_boss_health_system(
    mut commands: Commands,
    mut boss_query: Query<(Entity, &mut BossController), With<Boss>>,
    time: Res<Time>,
) {
    for (boss_entity, mut controller) in boss_query.iter_mut() {
        if let Some(start_time) = controller.transition_start_time {
            let elapsed = time.elapsed_secs() - start_time;
            
            if let Some(phase_config) = controller.current_phase_config() {
                if elapsed >= phase_config.invulnerability_duration {
                    // Unlock health
                    controller.health_lock = false;
                    controller.is_transitioning = false;
                    controller.transition_start_time = None;
                    
                    // Remove invincibility component
                    commands.entity(boss_entity).remove::<crate::infrastructure::components::combat::Invincibility>();
                    
                    info!("Boss health unlocked after phase transition");
                }
            }
        }
    }
}

/// Check Boss death system
///
/// Detects when Boss health reaches zero and publishes BossDefeated event.
/// T090: Implement Boss death detection system
pub fn check_boss_death_system(
    mut commands: Commands,
    boss_query: Query<(Entity, &Health, &BossId), (With<Boss>, With<Health>)>,
    mut boss_defeated_events: MessageWriter<BossDefeated>,
) {
    for (boss_entity, health, boss_id) in boss_query.iter() {
        if health.is_dead() {
            // Publish Boss defeated event
            boss_defeated_events.write(BossDefeated {
                boss_entity,
                boss_id: boss_id.0.clone(),
            });

            info!("Boss '{}' defeated at entity {:?}", boss_id.0, boss_entity);
            
            // Despawn Boss entity
            commands.entity(boss_entity).despawn();
        }
    }
}

/// Lock Boss room doors system
///
/// Locks all doors in the current room when Boss encounter starts.
/// T040: Implement Boss room entrance locking logic
pub fn lock_boss_room_doors_system(
    mut boss_encounter_events: MessageReader<BossEncounterStarted>,
    mut door_query: Query<&mut crate::infrastructure::components::dungeon::Door>,
) {
    for _event in boss_encounter_events.read() {
        // Lock all doors in current room when Boss encounter starts
        for mut door in door_query.iter_mut() {
            door.door_state = crate::infrastructure::components::dungeon::DoorState::Locked;
            info!("Locked door {:?} due to Boss encounter", door.door_id);
        }
    }
}

/// Unlock Boss room doors system
///
/// Unlocks all doors in the current room when Boss is defeated.
pub fn unlock_boss_room_doors_system(
    mut boss_defeated_events: MessageReader<BossDefeated>,
    mut door_query: Query<&mut crate::infrastructure::components::dungeon::Door>,
) {
    for _event in boss_defeated_events.read() {
        // Unlock all doors when Boss is defeated
        for mut door in door_query.iter_mut() {
            door.door_state = crate::infrastructure::components::dungeon::DoorState::Unlocked;
            info!("Unlocked door {:?} after Boss defeat", door.door_id);
        }
    }
}

// ============================================================================
// User Story 3: Special Skill Execution Systems
// ============================================================================

use crate::domain::boss::skill_priority::{select_next_skill, SkillCooldown, SkillPriority};
use crate::domain::boss::telegraph::{TelegraphArea, TelegraphShape};
use crate::infrastructure::components::boss::{BossSkill, Telegraph};

/// Update skill cooldowns system
///
/// Updates cooldown timers for all Boss skills.
/// T076: Implement skill cooldown management
pub fn update_skill_cooldowns_system(
    mut skill_query: Query<&mut BossSkill, With<Boss>>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    
    for mut skill in skill_query.iter_mut() {
        if skill.cooldown_remaining > 0.0 {
            skill.cooldown_remaining -= dt;
            if skill.cooldown_remaining <= 0.0 {
                skill.cooldown_remaining = 0.0;
                skill.is_on_cooldown = false;
            }
        }
    }
}

/// Select Boss skill system
///
/// Selects next skill to use based on priority and cooldown status.
/// T075: Implement skill selection system
/// T100: Performance optimization - limits active telegraphs to 2 max
pub fn select_boss_skill_system(
    mut commands: Commands,
    boss_query: Query<(Entity, &BossController, &Transform), With<Boss>>,
    skill_query: Query<&BossSkill, With<Boss>>,
    player_query: Query<&Transform, (With<crate::infrastructure::components::Player>, Without<Boss>)>,
    telegraph_query: Query<Entity, With<Telegraph>>,
) {
    // T100: Limit active telegraphs to 2 max
    let active_telegraph_count = telegraph_query.iter().count();
    if active_telegraph_count >= 2 {
        return; // Don't spawn more telegraphs if limit reached
    }
    for (boss_entity, controller, boss_transform) in boss_query.iter() {
        // Get current phase skills
        if let Some(phase_config) = controller.current_phase_config() {
            // Build skill cooldown list
            let mut available_skills: Vec<SkillCooldown> = Vec::new();
            
            for skill_id in &phase_config.skill_ids {
                // Check if skill component exists
                let mut found = false;
                for skill in skill_query.iter() {
                    if skill.skill_id == *skill_id {
                        available_skills.push(SkillCooldown {
                            skill_id: skill_id.clone(),
                            cooldown_remaining: skill.cooldown_remaining,
                            priority: match skill.priority {
                                1 => SkillPriority::Low,
                                2 => SkillPriority::Medium,
                                3 => SkillPriority::High,
                                4 => SkillPriority::Critical,
                                _ => SkillPriority::Medium,
                            },
                        });
                        found = true;
                        break;
                    }
                }
                
                // If skill component doesn't exist, create it
                if !found {
                    commands.entity(boss_entity).insert(BossSkill {
                        skill_id: skill_id.clone(),
                        cooldown_remaining: 0.0,
                        priority: 2, // Default medium priority
                        is_on_cooldown: false,
                    });
                }
            }
            
            // Select next skill using domain logic
            if let Some(selected_skill_id) = select_next_skill(&available_skills, 0.3) {
                // Spawn telegraph for selected skill
                if let Some(player_transform) = player_query.iter().next() {
                    let player_pos = player_transform.translation.truncate();
                    
                    // Create telegraph area (simple circle for now)
                    let telegraph_area = TelegraphArea {
                        shape: TelegraphShape::Circle { radius: 50.0 },
                        center: (player_pos.x, player_pos.y), // Target player position
                        rotation: 0.0,
                    };
                    
                    // Spawn telegraph entity
                    // Z=0.8 to render below player (Z=1.0) but above ground
                    commands.spawn((
                        Telegraph {
                            skill_id: selected_skill_id.clone(),
                            area: telegraph_area.clone(),
                            warning_duration: 1.0, // 1 second warning
                            elapsed_time: 0.0,
                            damage_area: telegraph_area,
                        },
                        Transform::from_translation(Vec3::new(player_pos.x, player_pos.y, 0.8)),
                        Sprite {
                            color: Color::srgba(1.0, 0.0, 0.0, 0.5), // Semi-transparent red
                            custom_size: Some(Vec2::new(100.0, 100.0)), // Circle diameter
                            ..default()
                        },
                        Visibility::Visible,
                    ));
                    
                    info!("Boss {:?} selected skill '{}' and spawned telegraph", boss_entity, selected_skill_id);
                }
            }
        }
    }
}

/// Update telegraph timer system
///
/// Updates elapsed time for telegraph warnings.
/// T080: Implement telegraph duration management
pub fn update_telegraph_timer_system(
    mut telegraph_query: Query<&mut Telegraph>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    
    for mut telegraph in telegraph_query.iter_mut() {
        telegraph.elapsed_time += dt;
    }
}

/// Animate telegraph flash system
///
/// Creates flashing animation effect for telegraph warnings.
/// T079: Implement telegraph flash animation
pub fn animate_telegraph_flash_system(
    mut sprite_query: Query<&mut Sprite, With<Telegraph>>,
    time: Res<Time>,
) {
    for mut sprite in sprite_query.iter_mut() {
        // Flashing effect: alpha oscillates between 0.3 and 0.7
        let flash_factor = (time.elapsed_secs() * 4.0).sin() * 0.2 + 0.5;
        sprite.color.set_alpha(flash_factor);
    }
}

/// Execute Boss skill system
///
/// Executes skill damage when telegraph warning ends.
/// T081: Implement skill execution system
pub fn execute_boss_skill_system(
    mut commands: Commands,
    telegraph_query: Query<(Entity, &Telegraph)>,
    player_query: Query<(Entity, &Transform, &crate::infrastructure::components::Health), With<crate::infrastructure::components::Player>>,
    mut damage_events: MessageWriter<crate::infrastructure::events::combat::DamageDealt>,
) {
    for (telegraph_entity, telegraph) in telegraph_query.iter() {
        // Check if warning duration has elapsed
        if telegraph.elapsed_time >= telegraph.warning_duration {
            // Execute skill damage
            let damage_area = &telegraph.damage_area;
            
            for (player_entity, player_transform, _health) in player_query.iter() {
                let player_pos = player_transform.translation.truncate();
                
                // Check if player is in damage area
                let player_point = (player_pos.x, player_pos.y);
                if damage_area.contains_point(player_point) {
                    // Deal damage to player
                    damage_events.write(crate::infrastructure::events::combat::DamageDealt {
                        target: player_entity,
                        source: telegraph_entity, // Use telegraph entity as source
                        result: crate::domain::combat::damage::DamageResult {
                            final_damage: 20.0, // TODO: Get from skill config
                            is_critical: false,
                            element: crate::domain::combat::Element::Physical,
                        },
                        position: player_pos,
                    });
                    
                    info!("Boss skill '{}' hit player at {:?}", telegraph.skill_id, player_pos);
                }
            }
            
            // Remove telegraph after execution
            commands.entity(telegraph_entity).despawn();
        }
    }
}

/// Cleanup expired telegraphs system
///
/// Removes telegraphs that have expired without execution.
/// T083: Implement telegraph cleanup system
pub fn cleanup_expired_telegraphs_system(
    mut commands: Commands,
    telegraph_query: Query<(Entity, &Telegraph)>,
) {
    for (entity, telegraph) in telegraph_query.iter() {
        // Remove telegraphs that have been active too long (safety cleanup)
        if telegraph.elapsed_time > telegraph.warning_duration + 0.5 {
            commands.entity(entity).despawn();
            info!("Cleaned up expired telegraph for skill '{}'", telegraph.skill_id);
        }
    }
}

// ============================================================================
// Phase 6: Polish & Cross-Cutting Concerns
// ============================================================================

/// Reset Boss on player death system
///
/// Resets Boss state when player dies during Boss encounter.
/// T094: Implement player death Boss reset system
pub fn reset_boss_on_player_death_system(
    mut player_death_events: MessageReader<crate::infrastructure::events::dungeon::PlayerDeathInRoom>,
    mut boss_query: Query<(Entity, &mut BossController, &mut crate::infrastructure::components::Health), With<Boss>>,
    mut commands: Commands,
) {
    for _event in player_death_events.read() {
        // Reset all Bosses
        for (boss_entity, mut controller, mut health) in boss_query.iter_mut() {
            // Reset to first phase
            controller.current_phase = 0;
            controller.health_lock = false;
            controller.is_transitioning = false;
            controller.transition_start_time = None;
            
            // Reset health to max
            health.current = health.max;
            
            // Remove invincibility if present
            commands.entity(boss_entity).remove::<crate::infrastructure::components::combat::Invincibility>();
            
            // Remove all skills to reset cooldowns
            commands.entity(boss_entity).remove::<BossSkill>();
            
            info!("Reset Boss {:?} after player death", boss_entity);
        }
    }
}

/// Check Boss out of combat system
///
/// Resets Boss if player leaves Boss room.
/// T095: Implement Boss out of combat detection and reset
pub fn check_boss_out_of_combat_system(
    mut boss_query: Query<(Entity, &mut BossController, &Transform), With<Boss>>,
    player_query: Query<&Transform, (With<crate::infrastructure::components::Player>, Without<Boss>)>,
    mut commands: Commands,
) {
    // Simple implementation: if player is too far from Boss, reset
    const MAX_COMBAT_RANGE: f32 = 1000.0; // 1000 pixels max combat range
    
    if let Some(player_transform) = player_query.iter().next() {
        let player_pos = player_transform.translation.truncate();
        
        for (boss_entity, mut controller, boss_transform) in boss_query.iter_mut() {
            let boss_pos = boss_transform.translation.truncate();
            let distance = player_pos.distance(boss_pos);
            
            if distance > MAX_COMBAT_RANGE {
                // Reset Boss
                controller.current_phase = 0;
                controller.health_lock = false;
                controller.is_transitioning = false;
                controller.transition_start_time = None;
                
                // Remove invincibility
                commands.entity(boss_entity).remove::<crate::infrastructure::components::combat::Invincibility>();
                
                info!("Boss {:?} reset due to out of combat range", boss_entity);
            }
        }
    }
}
