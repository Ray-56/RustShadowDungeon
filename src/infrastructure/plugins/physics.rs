//! Physics plugin configuration
//!
//! Configures Avian2d physics for 2D side-scrolling gameplay.
//!
//! **Technical Decision**: Using Avian2d instead of `bevy_rapier2d`
//! - Avian2d is the successor to `bevy_xpbd`
//! - Better Bevy 0.17 compatibility
//! - MIT/Apache-2.0 dual license (Constitution compliant)
//! - Active development and community support

use avian2d::prelude::*;
use bevy::prelude::*;

/// Physics plugin for the game
///
/// Configures:
/// - 2D physics simulation with Avian2d
/// - Gravity (-9.8 m/s² downward, scaled to pixels)
/// - Collision layers for Player, Ground, Walls
/// - Debug rendering (enabled in dev mode)
pub struct GamePhysicsPlugin;

impl Plugin for GamePhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            PhysicsPlugins::default().with_length_unit(16.0), // 16 pixels = 1 meter
        )
        .insert_resource(Gravity(Vec2::new(0.0, -9.8 * 16.0))); // -9.8 m/s² in pixels

        // Debug rendering in dev builds
        #[cfg(debug_assertions)]
        app.add_plugins(PhysicsDebugPlugin);
    }
}

/// Collision layers for entity filtering
///
/// Uses Avian2d's `LayerMask` system for collision filtering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollisionLayer {
    /// Player character collision
    Player,
    /// Ground and platforms
    Ground,
    /// Walls and obstacles
    Walls,
}

impl CollisionLayer {
    /// Get layer mask for this collision layer
    #[must_use]
    pub const fn to_layer(self) -> LayerMask {
        match self {
            Self::Player => LayerMask(0b0001),
            Self::Ground => LayerMask(0b0010),
            Self::Walls => LayerMask(0b0100),
        }
    }

    /// Get collision filter (what this layer collides with)
    #[must_use]
    pub fn collision_filter(self) -> CollisionLayers {
        match self {
            Self::Player => CollisionLayers::new(
                self.to_layer(),
                LayerMask(0b0110), // Collides with Ground | Walls
            ),
            Self::Ground | Self::Walls => CollisionLayers::new(
                self.to_layer(),
                LayerMask(0b0001), // Collides with Player
            ),
        }
    }
}
