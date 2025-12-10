//! ECS Systems - behavior functions
//!
//! Systems operate on components and call domain layer logic.

/// Animation systems
pub mod animation;
/// Camera systems
pub mod camera;
pub mod collectible;
/// Collision detection systems for combat (HitBox vs HurtBox)
pub mod collision;
pub mod combat;
/// Combat audio system
pub mod combat_audio;
pub mod combo;
/// Damage application and death detection systems
pub mod damage;
pub mod debug;
pub mod dungeon;
pub mod enemy;
/// Combat feedback systems (hitfreeze, screen shake, particles, damage numbers)
pub mod feedback;
pub mod fps_limiter;
pub mod input;
/// Invincibility frame system
pub mod invincibility;
pub mod jump;
/// Loot and inventory systems
pub mod loot;
pub mod movement;
/// Particle effect systems
pub mod particles;
pub mod pixel_snap;
/// Skill system
pub mod skill;
/// Boss systems
pub mod boss_systems;
/// UI system for displaying game information
pub mod ui;

#[cfg(feature = "inventory-ui")]
pub mod inventory_ui;

pub use animation::animation_system;
pub use camera::{camera_follow_system, pixel_snap_camera, setup_camera, GameCamera};
pub use collectible::coin_collection_system;
pub use collision::{collision_detection_system, hitbox_cleanup_system};
pub use combat::{attack_input_system, enemy_collision_system};
pub use combat_audio::combat_audio_system;
pub use combo::{combo_system, combo_timer_system, knockback_system};
pub use damage::{apply_damage_system, death_system};
pub use debug::DebugPlugin;
pub use enemy::{
    enemy_death_animation_system, enemy_death_animation_update_system, enemy_hit_flash_system,
    enemy_hit_flash_update_system, enemy_patrol_system, spawn_slime_system,
};
pub use feedback::{
    apply_screen_shake_system, damage_number_system, damage_number_update_system, hitfreeze_system,
    hitfreeze_timer_system, screen_shake_cleanup_system, screen_shake_system,
};
pub use input::player_input_system;
pub use invincibility::{
    invincibility_flash_system, invincibility_timer_system, invincibility_trigger_system,
};
pub use jump::{gravity_system, jump_initiation_system, variable_jump_system};
pub use movement::{
    apply_velocity_system, ground_detection_system, ground_movement_system, state_transition_system,
};
pub use particles::{hit_particle_system, particle_pool_system, particle_update_system};
pub use pixel_snap::{pixel_snap_system, PixelSnap};
pub use skill::{
    fireball_collision_system, lifetime_system, mp_consumption_system, projectile_system,
    skill_activation_system, skill_cooldown_system, skill_input_system,
};
pub use ui::{
    boss_ui::update_boss_health_ui, combo_ui_fadeout_system, combo_ui_system, setup_ui,
    skill_cooldown_ui_system, update_coin_count_ui, update_fps_ui, update_health_ui, update_mp_ui,
    update_score_ui,
};
