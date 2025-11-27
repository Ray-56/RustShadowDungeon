//! Combo system implementation
//!
//! T048-T050: Combo system, timer system, and knockback system
//!
//! Handles:
//! - Combo state advancement on attack input
//! - Combo window timer countdown
//! - Knockback effect for third hit

use crate::domain::combat::collision::Rect;
use crate::domain::combat::{combo::ComboState, Element};
use crate::infrastructure::components::{combat::Combo, combat::HitBox, Player};
use crate::infrastructure::events::combat::{
    ComboExtended, ComboReset, ComboResetReason, DamageDealt,
};
use crate::infrastructure::resources::CombatConfig;
use bevy::prelude::*;

/// T048: Combo system
///
/// Listens for attack input (J key) and updates Combo state.
/// Generates HitBox with appropriate damage based on combo state.
///
/// 连击系统：监听攻击输入，更新连击状态，根据连击状态生成不同伤害的 HitBox
pub fn combo_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    config: Res<CombatConfig>,
    mut player_query: Query<(Entity, &Transform, &mut Combo), With<Player>>,
    mut commands: Commands,
    mut combo_extended_events: MessageWriter<ComboExtended>,
    mut combo_reset_events: MessageWriter<ComboReset>,
) {
    // Check if J key was just pressed
    if !keyboard.just_pressed(KeyCode::KeyJ) {
        return;
    }

    for (player_entity, player_transform, mut combo) in &mut player_query {
        // Check if combo window expired
        if combo.is_expired() && combo.state != ComboState::Idle {
            // Reset combo due to timeout
            let final_count = combo.hit_count;
            combo.reset();
            combo_reset_events.write(ComboReset {
                player: player_entity,
                final_combo_count: final_count,
                reason: ComboResetReason::Timeout,
            });
        }

        // Advance combo state
        let old_state = combo.state;
        combo.advance(config.combo_window);
        let new_state = combo.state;
        let hit_count = combo.hit_count;

        // Publish ComboExtended event
        combo_extended_events.write(ComboExtended {
            player: player_entity,
            combo_count: hit_count,
            new_state,
        });

        // Generate HitBox with damage based on combo state
        let base_damage = 10.0;
        let damage_multiplier = new_state.damage_multiplier();
        let final_damage = base_damage * damage_multiplier;

        let player_pos = player_transform.translation.truncate();

        // Create HitBox in front of player (to the right)
        let hitbox_rect = Rect {
            x: 0.0,
            y: -16.0, // Center vertically
            width: 32.0,
            height: 32.0,
        };

        commands.spawn((
            Name::new("PlayerComboAttack"),
            HitBox::new(hitbox_rect, final_damage)
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
        ));

        info!(
            "Player {:?} combo: {:?} → {:?} (hit count: {}, damage: {})",
            player_entity, old_state, new_state, hit_count, final_damage
        );
    }
}

/// T049: Combo timer system
///
/// Updates combo window timer. When window expires, resets combo and publishes ComboReset event.
///
/// 连击窗口计时器系统：更新连击窗口剩余时间，超时则重置连击
pub fn combo_timer_system(
    time: Res<Time>,
    mut player_query: Query<(Entity, &mut Combo), With<Player>>,
    mut combo_reset_events: MessageWriter<ComboReset>,
) {
    let dt = time.delta_secs();

    for (player_entity, mut combo) in &mut player_query {
        // Only update timer if combo is active (not Idle)
        if combo.state == ComboState::Idle {
            continue;
        }

        // Decrease window remaining time
        combo.window_remaining -= dt;

        // Check if window expired
        if combo.is_expired() {
            // Reset combo
            let final_count = combo.hit_count;
            combo.reset();

            // Publish ComboReset event
            combo_reset_events.write(ComboReset {
                player: player_entity,
                final_combo_count: final_count,
                reason: ComboResetReason::Timeout,
            });

            info!("Player {:?} combo expired (timeout)", player_entity);
        }
    }
}

/// T050: Knockback system
///
/// Applies knockback force to enemies when third hit (heavy hit) deals damage.
///
/// 击退效果系统：第三击命中时对敌人施加击退力（50 像素）
pub fn knockback_system(
    mut damage_events: MessageReader<DamageDealt>,
    config: Res<CombatConfig>,
    mut enemy_query: Query<&mut Transform, (Without<Player>, Without<Camera>)>,
    player_query: Query<&Combo, With<Player>>,
) {
    // Get player combo state (if exists)
    let player_combo = player_query.iter().next();
    let is_third_hit =
        player_combo.map(|combo| combo.state == ComboState::ThirdHit).unwrap_or(false);

    if !is_third_hit {
        return;
    }

    // Process damage events
    for event in damage_events.read() {
        // Check if this is a third hit (heavy hit)
        // We can check by looking at the damage value (third hit does 20 damage = 2x)
        // Or we can add a flag to DamageDealt event in the future
        // For now, we'll apply knockback if damage >= 20 (indicating third hit)

        if event.result.final_damage >= 20.0 {
            // Apply knockback to target entity
            if let Ok(mut enemy_transform) = enemy_query.get_mut(event.target) {
                // Knockback direction: away from attacker (to the right for now)
                // In a real game, we'd calculate direction from attacker to target
                let knockback_distance = config.knockback_third_hit;
                enemy_transform.translation.x += knockback_distance;

                info!(
                    "Applied knockback to enemy {:?} (distance: {})",
                    event.target, knockback_distance
                );
            }
        }
    }
}
