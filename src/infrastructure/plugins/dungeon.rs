//! Dungeon system plugin
//!
//! Registers all dungeon-related systems and resources.

use crate::infrastructure::events::dungeon::*;
use crate::infrastructure::resources::dungeon::DungeonSession;
use crate::infrastructure::systems::dungeon::{
    check_room_clear_system, door_interaction_detection_system, ensure_state_consistency_system,
    handle_door_interaction_system, handle_empty_room_system,
    handle_player_death_in_dungeon_system, handle_room_cleared_system, handle_room_entered_system,
    initialize_room_system, load_dungeon_system, load_target_room_system,
    monitor_enemy_deaths_system, restore_room_state_system, show_interaction_prompt_system,
    spawn_enemies_system, track_room_clear_system, transition_to_room_system,
    unload_current_room_system, unlock_doors_system, update_camera_on_room_transition_system,
    update_door_visual_system,
};
use bevy::prelude::*;

/// Plugin for dungeon system
pub struct DungeonPlugin;

impl Plugin for DungeonPlugin {
    fn build(&self, app: &mut App) {
        // Register resources
        app.init_resource::<DungeonSession>();

        // Register events (Bevy 0.17 uses add_message instead of add_event)
        app.add_message::<RoomEntered>()
            .add_message::<RoomCleared>()
            .add_message::<DoorUnlocked>()
            .add_message::<RoomTransitioned>()
            .add_message::<PlayerDeathInRoom>();

        // Register systems for US1 (Enter dungeon and room initialization)
        app.add_systems(Startup, load_dungeon_system).add_systems(
            Update,
            (
                // Initialize room (runs once at start, and after room transitions)
                initialize_room_system,
                // Spawn enemies (runs after room initialization)
                spawn_enemies_system.after(initialize_room_system),
                handle_room_entered_system,
            ),
        );

        // Register systems for US2 (Room clearing and door unlocking)
        // Note: check_room_clear_system runs after enemy death animation completes
        // IMPORTANT: Must run after spawn_enemies_system to avoid false positives (Commands are deferred)
        app.add_systems(
            Update,
            (
                monitor_enemy_deaths_system,
                check_room_clear_system
                    .after(
                        crate::infrastructure::systems::enemy::enemy_death_animation_update_system,
                    )
                    .after(spawn_enemies_system), // Ensure enemies are spawned before checking
                track_room_clear_system.after(check_room_clear_system), // Track cleared rooms in session
                handle_room_cleared_system,
                unlock_doors_system,
                update_door_visual_system,
            ),
        );

        // Register systems for US4 (Progress tracking)
        app.add_systems(
            Update,
            (
                restore_room_state_system.after(initialize_room_system), // Restore state after room initialization
            ),
        );

        // Register systems for Phase 7 (Edge cases and polish)
        // IMPORTANT: handle_empty_room_system must run after spawn_enemies_system to avoid false positives
        app.add_systems(
            Update,
            (
                handle_player_death_in_dungeon_system
                    .after(crate::infrastructure::systems::damage::death_system),
                handle_empty_room_system
                    .after(initialize_room_system)
                    .after(spawn_enemies_system), // Ensure enemies are spawned before checking
                ensure_state_consistency_system.after(unlock_doors_system),
            ),
        );

        // Register systems for US3 (Room transitions)
        // IMPORTANT: handle_door_interaction_system runs after manual_pickup_system
        // so that item pickup takes priority over door interaction when both are possible
        app.add_systems(
            Update,
            (
                door_interaction_detection_system,
                show_interaction_prompt_system,
                handle_door_interaction_system
                    .after(crate::infrastructure::systems::loot::manual_pickup_system), // Run after pickup to prioritize items
                transition_to_room_system.after(handle_door_interaction_system),
                unload_current_room_system.after(transition_to_room_system),
                load_target_room_system.after(transition_to_room_system),
                update_camera_on_room_transition_system.after(transition_to_room_system),
                // Re-initialize room after transition (system already registered above, will run automatically)
                // spawn_enemies_system will also run automatically after initialize_room_system
            ),
        );
    }
}
