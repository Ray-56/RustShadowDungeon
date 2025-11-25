use bevy::prelude::*;
use criterion::{criterion_group, criterion_main, Criterion};
use rust_shadow_dungeon::infrastructure::components::player::{
    GroundedState, InputState, Player, VelocityComponent,
};
use rust_shadow_dungeon::infrastructure::resources::movement_config::MovementConfig;
use rust_shadow_dungeon::infrastructure::systems::movement::ground_movement_system;

fn movement_benchmark(c: &mut Criterion) {
    let mut app = App::new();

    app.add_plugins(MinimalPlugins)
        .init_resource::<MovementConfig>()
        .add_systems(Update, ground_movement_system);

    // Spawn player
    app.world_mut().spawn((
        Player,
        InputState { move_direction: 1.0, ..default() },
        VelocityComponent::default(),
        GroundedState { is_grounded: true, ..default() },
    ));

    c.bench_function("ground_movement_system", |b| {
        b.iter(|| {
            app.update();
        });
    });
}

criterion_group!(benches, movement_benchmark);
criterion_main!(benches);
