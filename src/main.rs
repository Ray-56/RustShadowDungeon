//! Game entry point for Rust Shadow Dungeon

use bevy::prelude::*;
use rust_shadow_dungeon::infrastructure::{
    plugins::{
        BossPlugin, CombatPlugin, DungeonPlugin, EnemyPlugin, GamePhysicsPlugin,
        LootInventoryPlugin, PlayerPlugin, SkillPlugin,
    },
    systems::DebugPlugin,
};

fn main() {
    App::new()
        // Set clear color (dark blue background for better visibility)
        .insert_resource(ClearColor(Color::srgb(0.1, 0.15, 0.2)))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "锈影地下城 | Rust Shadow Dungeon".to_string(),
                        resolution: (1280, 720).into(),
                        resizable: false,
                        present_mode: bevy::window::PresentMode::AutoNoVsync, // No VSync (will cap manually)
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()), // Nearest neighbor for pixel art
        )
        .add_plugins((
            bevy::diagnostic::FrameTimeDiagnosticsPlugin::default(), // Always enable FPS counter
            GamePhysicsPlugin,
            DebugPlugin,
            PlayerPlugin, // Player movement system
            CombatPlugin, // Combat system (M2)
            EnemyPlugin,  // Enemy system (M2)
            SkillPlugin,  // Skill system (M2)
            DungeonPlugin,
            LootInventoryPlugin, // Loot and inventory system (M3) // Dungeon system (M3)
            BossPlugin,   // Boss encounter system (M3)
        ))
        // FPS limiting
        .init_resource::<rust_shadow_dungeon::infrastructure::systems::fps_limiter::FpsLimiter>()
        .add_systems(Startup, rust_shadow_dungeon::infrastructure::systems::setup_camera)
        .add_systems(
            Update,
            (
                rust_shadow_dungeon::infrastructure::systems::pixel_snap_camera,
                rust_shadow_dungeon::infrastructure::systems::fps_limiter::fps_limit_system
                    .after(rust_shadow_dungeon::infrastructure::systems::pixel_snap_camera),
            ),
        )
        .run();
}
