//! Bevy Plugins - system registration and resource initialization
//!
//! Each feature is encapsulated as a Bevy plugin.

pub mod combat; // M2 - Combat System Core
pub mod dungeon; // M3 - Dungeon System
pub mod enemy;
pub mod loot;
pub mod physics;
pub mod player; // M2 - Enemy System
pub mod skill; // M2 - Skill System // M3 - Loot and Inventory System
pub mod boss; // M3 - Boss Encounter System

pub use combat::CombatPlugin;
pub use dungeon::DungeonPlugin;
pub use enemy::EnemyPlugin;
pub use loot::LootInventoryPlugin;
pub use physics::GamePhysicsPlugin;
pub use player::PlayerPlugin;
pub use skill::SkillPlugin;
pub use boss::BossPlugin;
