//! Invincibility frame system
//!
//! 无敌帧系统
//!
//! Handles invincibility frames after taking damage, including visual flashing.

use bevy::prelude::*;

use crate::infrastructure::components::combat::Invincibility;
use crate::infrastructure::components::Player;
use crate::infrastructure::events::combat::{
    DamageDealt, InvincibilityEnded, InvincibilityStarted,
};
use crate::infrastructure::resources::CombatConfig;

/// Invincibility trigger system
///
/// 无敌帧触发系统
///
/// T101: Listens to DamageDealt events for players and adds Invincibility component.
/// If invincibility already exists, takes the maximum duration.
pub fn invincibility_trigger_system(
    mut damage_events: MessageReader<DamageDealt>,
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    mut invincibility_query: Query<&mut Invincibility>,
    combat_config: Res<CombatConfig>,
    mut invincibility_started_events: MessageWriter<InvincibilityStarted>,
) {
    for event in damage_events.read() {
        // Check if target is a player
        if player_query.get(event.target).is_ok() {
            // Check if player already has invincibility
            if let Ok(mut existing) = invincibility_query.get_mut(event.target) {
                // Take maximum duration
                let new_duration = combat_config.invincibility_duration;
                if new_duration > existing.remaining {
                    existing.remaining = new_duration;
                    existing.flash_timer = 0.0; // Reset flash timer

                    // Publish event
                    invincibility_started_events.write(InvincibilityStarted {
                        entity: event.target,
                        duration: new_duration,
                    });
                }
            } else {
                // Add new invincibility component
                let duration = combat_config.invincibility_duration;
                commands.entity(event.target).insert(Invincibility::new(duration));

                // Publish event
                invincibility_started_events
                    .write(InvincibilityStarted { entity: event.target, duration });
            }
        }
    }
}

/// Invincibility timer system
///
/// 无敌帧计时器系统
///
/// T102: Updates Invincibility.remaining, removes component when expired,
/// and publishes InvincibilityEnded event.
pub fn invincibility_timer_system(
    mut commands: Commands,
    mut invincibility_query: Query<(Entity, &mut Invincibility)>,
    time: Res<Time>,
    mut invincibility_ended_events: MessageWriter<InvincibilityEnded>,
) {
    let delta = time.delta_secs();

    for (entity, mut invincibility) in &mut invincibility_query {
        let was_active = invincibility.is_active();
        invincibility.update(delta);

        // Check if invincibility just ended
        if was_active && !invincibility.is_active() {
            // Remove component
            commands.entity(entity).remove::<Invincibility>();

            // Publish event
            invincibility_ended_events.write(InvincibilityEnded { entity });
        }
    }
}

/// Invincibility flash system
///
/// 无敌帧闪烁系统
///
/// T103: Toggles sprite visibility every 0.1 seconds for visual feedback.
pub fn invincibility_flash_system(mut sprite_query: Query<(&mut Visibility, &Invincibility)>) {
    for (mut visibility, invincibility) in &mut sprite_query {
        // Toggle visibility based on flash timer
        if invincibility.should_be_visible() {
            *visibility = Visibility::Visible;
        } else {
            // Flash effect: make semi-transparent or invisible
            *visibility = Visibility::Hidden;
        }
    }
}
