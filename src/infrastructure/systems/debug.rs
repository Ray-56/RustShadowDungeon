//! Debug overlay systems
//!
//! FPS counter and other debug information.
//! Only active in debug builds.

use bevy::prelude::*;

#[cfg(debug_assertions)]
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};

/// Marker component for FPS display text
#[derive(Component)]
pub struct FpsText;

/// Setup debug overlay
///
/// Creates UI elements for debug information:
/// - FPS counter
/// - Frame time
#[cfg(debug_assertions)]
pub fn setup_debug_overlay(mut commands: Commands) {
    commands.spawn((
        Text::new("FPS: --"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        FpsText,
    ));
}

/// Update FPS counter
///
/// Displays current FPS and frame time in debug builds.
#[cfg(debug_assertions)]
pub fn update_fps_counter(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                text.0 = format!("FPS: {value:.1}");

                // Color code: Green (60+), Yellow (30-60), Red (<30)
                // Note: Text styling would be done here
            }
        }
    }
}

/// Debug plugin
///
/// Adds debug overlays and diagnostics.
/// Only compiled in debug builds.
#[cfg(debug_assertions)]
pub struct DebugPlugin;

#[cfg(debug_assertions)]
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_debug_overlay)
            .add_systems(Update, update_fps_counter);
    }
}

// No-op plugin for release builds
#[cfg(not(debug_assertions))]
/// Debug plugin (Disabled in release)
pub struct DebugPlugin;

#[cfg(not(debug_assertions))]
impl Plugin for DebugPlugin {
    fn build(&self, _app: &mut App) {
        // Debug systems disabled in release builds
    }
}
