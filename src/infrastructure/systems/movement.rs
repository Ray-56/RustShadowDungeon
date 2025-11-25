//! Movement systems

use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    domain::movement::{
        calculate_air_velocity, calculate_ground_velocity, transition_state, InputDirection,
    },
    infrastructure::{
        components::{
            GroundedState, InputState, MovementStateComponent, Player, VelocityComponent,
        },
        events::StateChanged,
        resources::MovementConfig,
    },
};

/// Apply ground movement based on input
///
/// Optimized for responsive feel:
/// - Instant ground acceleration (no smoothing)
/// - Better air control
pub fn ground_movement_system(
    config: Res<MovementConfig>,
    mut query: Query<(&InputState, &mut VelocityComponent, &GroundedState), With<Player>>,
) {
    for (input, mut velocity, grounded) in &mut query {
        // Convert input to domain direction
        let direction = if input.move_direction > 0.1 {
            InputDirection::Right
        } else if input.move_direction < -0.1 {
            InputDirection::Left
        } else {
            InputDirection::None
        };

        if grounded.is_grounded {
            // Call domain logic for ground movement
            velocity.0 = calculate_ground_velocity(velocity.0, direction, config.ground_speed);
        } else {
            // Call domain logic for air movement
            velocity.0 = calculate_air_velocity(
                velocity.0,
                direction,
                config.ground_speed,
                config.air_control_factor,
            );
        }
    }
}

/// Update movement state based on current conditions
pub fn state_transition_system(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &LinearVelocity,
            &GroundedState,
            &InputState,
            &mut MovementStateComponent,
        ),
        With<Player>,
    >,
) {
    for (entity, velocity, grounded, input, mut state) in &mut query {
        let has_move_input = input.move_direction.abs() > 0.1;
        let new_state = transition_state(
            state.0,
            grounded.is_grounded,
            has_move_input,
            input.jump_pressed, // Pass jump input
            velocity.y,         // Use physics velocity for state transition
        );

        if state.0 != new_state {
            // Emit event
            commands.trigger(StateChanged { entity, from_state: state.0, to_state: new_state });

            state.0 = new_state;
        }
    }
}

/// Apply horizontal velocity to Avian2d physics body
///
/// Only controls horizontal movement, letting Avian2d handle all vertical physics.
pub fn apply_velocity_system(
    mut query: Query<(&VelocityComponent, &mut LinearVelocity), With<Player>>,
) {
    for (velocity, mut linear_velocity) in &mut query {
        // Only control horizontal velocity
        // Avian2d physics engine handles all vertical velocity (gravity + jumps)
        linear_velocity.x = velocity.0.x;
    }
}

/// Ground detection using Avian2d collision events
///
/// Detects when player is touching ground by checking downward raycasts
/// Also tracks coyote time for better jump feel
pub fn ground_detection_system(
    time: Res<Time>,
    mut query: Query<(&Transform, &mut GroundedState), With<Player>>,
    spatial_query: SpatialQuery,
) {
    let dt = time.delta_secs();

    for (transform, mut grounded) in &mut query {
        // Cast a short ray downward from player center
        let ray_origin = transform.translation.truncate();
        let ray_direction = Dir2::NEG_Y; // Downward
        let max_distance = 18.0; // Slightly more than half player height (16 + margin)

        // Perform raycast
        let was_grounded = grounded.is_grounded;
        if let Some(hit) = spatial_query.cast_ray(
            ray_origin,
            ray_direction,
            max_distance,
            true, // Solid hits only
            &SpatialQueryFilter::default(),
        ) {
            // Check if we hit something close enough to be "grounded"
            grounded.is_grounded = hit.distance < 18.0;
        } else {
            grounded.is_grounded = false;
        }

        // Update coyote time
        if grounded.is_grounded {
            grounded.time_since_grounded = 0.0;
        } else if was_grounded {
            // Just left ground, start coyote timer
            grounded.time_since_grounded = 0.0;
        } else {
            // In air, increment timer
            grounded.time_since_grounded += dt;
        }
    }
}
