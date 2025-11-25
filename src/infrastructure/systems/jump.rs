//! Jump systems

use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    domain::movement::apply_variable_jump,
    infrastructure::{
        components::{GroundedState, InputState, Player},
        resources::MovementConfig,
    },
};

/// Jump initiation system
///
/// Triggers jump when conditions are met
/// Supports:
/// - Normal jump (grounded + jump pressed)
/// - Coyote jump (recently left ground + jump pressed)
/// - Buffered jump (jump pressed before landing)
pub fn jump_initiation_system(
    config: Res<MovementConfig>,
    mut query: Query<(&mut InputState, &GroundedState, &mut LinearVelocity), With<Player>>,
) {
    for (mut input, grounded, mut linear_velocity) in &mut query {
        // Check if can jump (normal or coyote)
        let can_perform_jump = grounded.is_grounded || grounded.can_coyote_jump();

        // Try jump from direct input or buffered input
        let wants_to_jump = input.jump_pressed || (can_perform_jump && input.has_buffered_jump());

        if can_perform_jump && wants_to_jump {
            // Set jump velocity directly on physics body
            linear_velocity.y = config.jump_params.initial_velocity;

            // Consume buffered jump
            input.consume_jump_buffer();

            if grounded.can_coyote_jump() {
                info!("Coyote jump! Velocity: {:.1}", linear_velocity.y);
            } else {
                info!("Jump! Velocity: {:.1}", linear_velocity.y);
            }
        }
    }
}

/// Variable jump height system
///
/// Applies variable jump when player releases jump button while ascending
/// Directly modifies Avian2d's `LinearVelocity`
pub fn variable_jump_system(
    config: Res<MovementConfig>,
    mut query: Query<(&InputState, &mut LinearVelocity), With<Player>>,
) {
    for (input, mut linear_velocity) in &mut query {
        // If player releases jump button while moving upward
        if !input.jump_held && linear_velocity.y > 0.0 {
            linear_velocity.y = apply_variable_jump(linear_velocity.y, &config.jump_params);
        }
    }
}

/// Gravity system (disabled - Avian2d handles gravity)
///
/// NOTE: This system is now a no-op. Avian2d's physics engine
/// automatically applies gravity to all Dynamic rigid bodies.
pub const fn gravity_system() {
    // Avian2d handles all gravity automatically
}
