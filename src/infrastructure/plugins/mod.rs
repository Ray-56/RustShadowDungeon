//! Bevy Plugins - system registration and resource initialization
//!
//! Each feature is encapsulated as a Bevy plugin.

pub mod physics;
pub mod player;

pub use physics::GamePhysicsPlugin;
pub use player::PlayerPlugin;
