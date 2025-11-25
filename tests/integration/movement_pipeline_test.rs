use bevy::prelude::*;
use rust_shadow_dungeon::infrastructure::components::player::{
    GroundedState, InputState, Player, VelocityComponent,
};
use rust_shadow_dungeon::infrastructure::resources::movement_config::MovementConfig;
use rust_shadow_dungeon::infrastructure::systems::movement::ground_movement_system;

#[test]
fn test_ground_movement_pipeline() {
    let mut app = App::new();

    app.add_plugins(MinimalPlugins)
        .init_resource::<MovementConfig>()
        .add_systems(Update, ground_movement_system);

    // Override config for deterministic test
    let mut config = app.world_mut().resource_mut::<MovementConfig>();
    config.ground_speed = 100.0;
    config.air_control_factor = 0.5;

    // Spawn player
    let player = app
        .world_mut()
        .spawn((
            Player,
            InputState::default(),
            VelocityComponent::default(),
            GroundedState { is_grounded: true, ..default() },
        ))
        .id();

    // Set input
    {
        let mut input = app.world_mut().get_mut::<InputState>(player).unwrap();
        input.move_direction = 1.0; // Right
    }

    // Run schedule
    app.update();

    // Check velocity
    let velocity = app.world().get::<VelocityComponent>(player).unwrap();
    assert_eq!(velocity.0.x, 100.0);
    assert_eq!(velocity.0.y, 0.0);
}

#[test]
fn test_air_movement_pipeline() {
    let mut app = App::new();

    app.add_plugins(MinimalPlugins)
        .init_resource::<MovementConfig>()
        .add_systems(Update, ground_movement_system);

    // Override config
    let mut config = app.world_mut().resource_mut::<MovementConfig>();
    config.ground_speed = 100.0;
    config.air_control_factor = 0.5;

    // Spawn player in air
    let player = app
        .world_mut()
        .spawn((
            Player,
            InputState::default(),
            VelocityComponent::default(),
            GroundedState { is_grounded: false, ..default() },
        ))
        .id();

    // Set input and initial velocity
    {
        let mut input = app.world_mut().get_mut::<InputState>(player).unwrap();
        input.move_direction = 1.0; // Right

        let mut vel = app.world_mut().get_mut::<VelocityComponent>(player).unwrap();
        vel.0.x = 0.0;
    }

    // Run schedule
    app.update();

    // Check velocity - Air control
    // Formula in system: velocity.x = velocity.x * 0.7 + target * 0.3
    // target = 1.0 * 100.0 * 0.5 = 50.0
    // initial = 0.0
    // new = 0.0 * 0.7 + 50.0 * 0.3 = 15.0
    let velocity = app.world().get::<VelocityComponent>(player).unwrap();
    // Allow small float error
    assert!((velocity.0.x - 15.0).abs() < 0.1, "Expected ~15.0, got {}", velocity.0.x);
}
