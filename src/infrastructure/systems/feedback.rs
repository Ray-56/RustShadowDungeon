//! Combat feedback systems
//!
//! 战斗反馈系统
//!
//! Provides visual and audio feedback for combat:
//! - Hitfreeze (hit stop / time freeze)
//! - Screen shake
//! - Particle effects
//! - Damage numbers
//! - Audio triggers

use bevy::prelude::*;
use bevy::time::Time;
use rand::Rng;

use crate::domain::combat::ComboState;
use crate::infrastructure::components::combat::Combo;
use crate::infrastructure::components::{DamageNumber, ScreenShake};
use crate::infrastructure::events::combat::DamageDealt;
use crate::infrastructure::resources::{CombatConfig, HitfreezeTimer};
use crate::infrastructure::systems::GameCamera;

/// Hitfreeze system
///
/// 打击定格系统
///
/// T058: Listens to DamageDealt events and triggers hitfreeze based on hit type.
/// - Light hit: 3 frames (~0.05s)
/// - Heavy hit: 5 frames (~0.083s)
/// - Critical hit: 7 frames (~0.117s)
pub fn hitfreeze_system(
    mut damage_events: MessageReader<DamageDealt>,
    mut hitfreeze_timer: ResMut<HitfreezeTimer>,
    combat_config: Res<CombatConfig>,
    combo_query: Query<&Combo>,
) {
    for event in damage_events.read() {
        // Determine if this is a heavy hit (third combo hit)
        let is_heavy = combo_query
            .get(event.source)
            .map(|combo| combo.state == ComboState::ThirdHit)
            .unwrap_or(false);

        // Get hitfreeze duration based on hit type
        let duration = combat_config.get_hitfreeze_duration(event.result.is_critical, is_heavy);

        // Trigger hitfreeze (takes max if already active)
        hitfreeze_timer.trigger(duration);
    }
}

/// Hitfreeze timer update system
///
/// 打击定格计时器更新系统
///
/// T059: Updates hitfreeze timer using Time<Real> and controls Time<Virtual> pause.
/// When hitfreeze is active, game time is paused (except UI/audio).
pub fn hitfreeze_timer_system(
    mut hitfreeze_timer: ResMut<HitfreezeTimer>,
    time: Res<Time<Real>>,
    mut virtual_time: ResMut<Time<Virtual>>,
) {
    // Update timer using real time (not affected by time dilation)
    let delta = time.delta_secs();
    let just_ended = hitfreeze_timer.update(delta);

    // Control virtual time based on hitfreeze state
    if hitfreeze_timer.is_active() {
        // Pause game time (hitfreeze effect)
        virtual_time.pause();
    } else if just_ended {
        // Resume game time when hitfreeze ends
        virtual_time.unpause();
    }
}

/// Screen shake system
///
/// 屏幕震动系统
///
/// T061: Listens to DamageDealt events and adds ScreenShake component to camera
/// for heavy hits and critical hits.
pub fn screen_shake_system(
    mut damage_events: MessageReader<DamageDealt>,
    camera_query: Query<Entity, (With<GameCamera>, Without<ScreenShake>)>,
    mut commands: Commands,
    combat_config: Res<CombatConfig>,
    combo_query: Query<&Combo>,
) {
    for event in damage_events.read() {
        // Only shake for heavy hits or critical hits
        let is_heavy = combo_query
            .get(event.source)
            .map(|combo| combo.state == ComboState::ThirdHit)
            .unwrap_or(false);

        if event.result.is_critical || is_heavy {
            // Get shake amplitude based on hit type
            let amplitude =
                combat_config.get_screen_shake_amplitude(event.result.is_critical, is_heavy);

            // Add ScreenShake component to camera
            if let Some(camera_entity) = camera_query.iter().next() {
                commands
                    .entity(camera_entity)
                    .insert(ScreenShake::new(amplitude, combat_config.screen_shake_duration));
            }
        }
    }
}

/// Apply screen shake system
///
/// 应用屏幕震动系统
///
/// T062: Updates camera position with random offset based on ScreenShake component.
/// Shake intensity decays over time.
pub fn apply_screen_shake_system(
    mut camera_query: Query<(&mut Transform, &mut ScreenShake), With<GameCamera>>,
    time: Res<Time>,
) {
    let delta = time.delta_secs();
    let mut rng = rand::thread_rng();

    for (mut transform, mut shake) in &mut camera_query {
        // Update shake timer
        let still_active = shake.update(delta);

        if still_active {
            // Calculate current intensity (decays over time)
            let intensity = shake.intensity();

            // Apply random offset based on intensity
            let offset_x = rng.gen_range(-intensity..=intensity);
            let offset_y = rng.gen_range(-intensity..=intensity);

            // Apply offset to camera transform
            // Note: This is added to the base camera position
            // The camera follow system will handle the base position
            transform.translation.x += offset_x;
            transform.translation.y += offset_y;
        } else {
            // Remove ScreenShake component when duration expires
            // (This will be handled by a cleanup system or despawn)
        }
    }
}

/// Damage number generation system
///
/// 伤害数字生成系统
///
/// T068: Listens to DamageDealt events and spawns floating damage numbers
/// above hit targets. Numbers display damage value and float upward.
pub fn damage_number_system(
    mut damage_events: MessageReader<DamageDealt>,
    mut commands: Commands,
    combat_config: Res<CombatConfig>,
) {
    for event in damage_events.read() {
        // Create damage number component
        let damage_number = DamageNumber::new(
            event.result.final_damage,
            combat_config.damage_number_lifetime,
            event.result.is_critical,
        );

        // Spawn damage number entity above hit position
        // Note: In a real implementation, you'd add a Text2dBundle here
        // For now, we just create the component
        commands.spawn((
            damage_number,
            Transform::from_translation(Vec3::new(
                event.position.x,
                event.position.y + 20.0, // Spawn 20 pixels above hit point
                100.0,                   // High Z to appear above other entities
            )),
        ));
    }
}

/// Damage number update system
///
/// 伤害数字更新系统
///
/// T069: Updates damage number position (floats upward), lifetime, and alpha.
/// Numbers fade out and despawn when lifetime expires.
pub fn damage_number_update_system(
    mut commands: Commands,
    mut damage_number_query: Query<(Entity, &mut DamageNumber, &mut Transform)>,
    time: Res<Time>,
) {
    let delta = time.delta_secs();

    for (entity, mut damage_number, mut transform) in &mut damage_number_query {
        // Update lifetime
        let still_alive = damage_number.update(delta);

        if still_alive {
            // Float upward
            transform.translation.x += damage_number.velocity.x * delta;
            transform.translation.y += damage_number.velocity.y * delta;

            // Note: In a real implementation, you'd update the Text2dBundle's
            // color alpha based on damage_number.get_alpha()
        } else {
            // Lifetime expired, despawn
            commands.entity(entity).despawn();
        }
    }
}

/// Screen shake cleanup system
///
/// 屏幕震动清理系统
///
/// Removes ScreenShake component when duration expires.
pub fn screen_shake_cleanup_system(
    mut commands: Commands,
    mut camera_query: Query<(Entity, &mut ScreenShake), With<GameCamera>>,
    time: Res<Time>,
) {
    let delta = time.delta_secs();

    for (entity, mut shake) in &mut camera_query {
        if !shake.update(delta) {
            // Duration expired, remove component
            commands.entity(entity).remove::<ScreenShake>();
        }
    }
}
