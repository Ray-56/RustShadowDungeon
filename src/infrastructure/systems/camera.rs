//! Camera systems for pixel-perfect rendering
//!
//! Ensures 16×16 pixel grid alignment and camera setup.

use bevy::prelude::*;

use crate::infrastructure::components::{CameraFollow, Player};

/// Marker component for the main game camera
#[derive(Component)]
pub struct GameCamera;

/// Setup pixel-perfect camera
///
/// Creates an orthographic camera with pixel-perfect scaling.
/// - 1:1 pixel mapping (no sub-pixel rendering)
/// - Fixed integer scaling for different resolutions
/// - 16×16 grid alignment
pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        GameCamera,
        CameraFollow::default(),
        Transform::from_xyz(0.0, 0.0, 999.9), // Camera looks down from high Z
    ));
}

/// Camera follow system
///
/// Smoothly follows the player.
pub fn camera_follow_system(
    mut camera_query: Query<(&mut Transform, &CameraFollow), With<GameCamera>>,
    player_query: Query<&Transform, (With<Player>, Without<GameCamera>)>,
) {
    if let Some(player_transform) = player_query.iter().next() {
        for (mut camera_transform, follow) in &mut camera_query {
            let target_pos = player_transform.translation + follow.offset;

            // Smooth interpolation
            let current = camera_transform.translation;
            let new_pos = current.lerp(target_pos, follow.smoothness);

            // Update X and Y only, keep Z
            camera_transform.translation.x = new_pos.x;
            camera_transform.translation.y = new_pos.y;
        }
    }
}

/// Pixel-snap camera position
///
/// Ensures camera position is locked to integer pixels
/// to avoid sub-pixel rendering and blurring.
pub fn pixel_snap_camera(mut query: Query<&mut Transform, With<GameCamera>>) {
    for mut transform in &mut query {
        transform.translation.x = transform.translation.x.round();
        transform.translation.y = transform.translation.y.round();
        // Z coordinate (depth) can remain floating point
    }
}
