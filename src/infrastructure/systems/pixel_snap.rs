//! Pixel snapping system for pixel-perfect rendering
//!
//! Ensures all entity positions are locked to integer pixels.

use bevy::prelude::*;

/// Marker component for entities that should snap to pixel grid
#[derive(Component)]
pub struct PixelSnap;

/// Snap entity transforms to pixel grid
///
/// Rounds all Transform translations to nearest integer pixel.
/// This prevents sub-pixel rendering which causes sprite blurring.
///
/// Applied to all entities with PixelSnap component.
pub fn pixel_snap_system(mut query: Query<&mut Transform, With<PixelSnap>>) {
    for mut transform in &mut query {
        transform.translation.x = transform.translation.x.round();
        transform.translation.y = transform.translation.y.round();
        // Z coordinate (depth/layer) can remain floating point
    }
}





