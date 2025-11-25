//! Minimal Bevy 0.17 test - just a red square
//! Run with: cargo run --example minimal_test

use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.5))) // Blue background
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Spawn camera
    commands.spawn(Camera2d);

    // Spawn a HUGE red square using Sprite
    commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 0.0, 0.0),
            custom_size: Some(Vec2::new(600.0, 600.0)),
            ..default()
        },
        Transform::default(),
    ));

    info!("Minimal test: Camera and HUGE red 600x600 sprite spawned");
}
