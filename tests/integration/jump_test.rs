use avian2d::prelude::LinearVelocity;
use bevy::prelude::*;
use rust_shadow_dungeon::infrastructure::components::player::{
    GroundedState, InputState, Player, VelocityComponent,
};
use rust_shadow_dungeon::infrastructure::resources::movement_config::MovementConfig;
use rust_shadow_dungeon::infrastructure::systems::jump::jump_initiation_system;

#[test]
fn test_jump_pipeline() {
    let mut app = App::new();

    app.add_plugins(MinimalPlugins)
        .init_resource::<MovementConfig>()
        .add_systems(Update, jump_initiation_system);

    // Spawn player grounded
    let player = app
        .world_mut()
        .spawn((
            Player,
            InputState::default(),
            GroundedState { is_grounded: true, ..default() },
            LinearVelocity::default(),
        ))
        .id();

    // Trigger jump
    {
        let mut input = app.world_mut().get_mut::<InputState>(player).unwrap();
        input.jump_pressed = true;
    }

    // Run schedule
    app.update();

    // Check velocity (should be positive Y)
    let velocity = app.world().get::<LinearVelocity>(player).unwrap();
    let config = app.world().resource::<MovementConfig>();

    assert!(velocity.y > 0.0);
    assert_eq!(velocity.y, config.jump_params.initial_velocity);
}
