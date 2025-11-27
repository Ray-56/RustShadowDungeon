//! Bevy Plugins - system registration and resource initialization
//!
//! Each feature is encapsulated as a Bevy plugin.

pub mod combat; // M2 - Combat System Core
pub mod enemy;
pub mod physics;
pub mod player; // M2 - Enemy System
pub mod skill; // M2 - Skill System

pub use combat::CombatPlugin;
pub use enemy::EnemyPlugin;
pub use physics::GamePhysicsPlugin;
pub use player::PlayerPlugin;
pub use skill::SkillPlugin;
