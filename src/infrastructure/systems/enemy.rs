//! Enemy AI systems

use bevy::prelude::*;

use crate::infrastructure::components::{Enemy, PatrolBehavior};

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
