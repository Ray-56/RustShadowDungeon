//! ECS Systems - behavior functions
//!
//! Systems operate on components and call domain layer logic.

/// Animation systems
pub mod animation;
/// Camera systems
pub mod camera;
pub mod collectible;
pub mod combat;
pub mod debug;
pub mod enemy;
pub mod fps_limiter;
pub mod input;
pub mod jump;
pub mod movement;
pub mod pixel_snap;
/// UI system for displaying game information
pub mod ui;

pub use animation::animation_system;
pub use camera::{camera_follow_system, pixel_snap_camera, setup_camera, GameCamera};
pub use collectible::coin_collection_system;
pub use combat::{enemy_collision_system, invincibility_timer_system};
pub use debug::DebugPlugin;
pub use enemy::enemy_patrol_system;
pub use input::player_input_system;
pub use jump::{gravity_system, jump_initiation_system, variable_jump_system};
pub use movement::{
    apply_velocity_system, ground_detection_system, ground_movement_system, state_transition_system,
};
pub use pixel_snap::{pixel_snap_system, PixelSnap};
