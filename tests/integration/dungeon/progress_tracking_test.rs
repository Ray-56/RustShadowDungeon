//! Integration tests for progress tracking system
//!
//! Tests that cleared rooms remain cleared when player returns.

use bevy::prelude::*;
use rust_shadow_dungeon::infrastructure::{
    components::dungeon::{Room, RoomId, RoomState},
    plugins::dungeon::DungeonPlugin,
    resources::dungeon::DungeonSession,
};

#[test]
fn test_progress_tracking() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_plugins(DungeonPlugin)
        .init_resource::<DungeonSession>();

    // Test would require full setup - placeholder for now
    // This test would verify:
    // 1. Cleared rooms are recorded in DungeonSession
    // 2. Returning to cleared room doesn't spawn enemies
    // 3. Doors remain unlocked in cleared rooms
}

