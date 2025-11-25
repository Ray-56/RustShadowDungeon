//! Player plugin - registers all player-related systems

use avian2d::prelude::*;
use bevy::prelude::*;

use crate::infrastructure::plugins::physics::CollisionLayer;

use crate::infrastructure::{
    components::{
        AnimationState, Coin, Enemy, GroundedState, Health, InputState, MovementStateComponent,
        PatrolBehavior, PixelSnap, Player, VelocityComponent,
    },
    events::{PlayerMoved, StateChanged},
    resources::{MovementConfig, PlayerAnimations, Score},
    systems::{
        animation_system, apply_velocity_system, coin_collection_system, enemy_collision_system,
        enemy_patrol_system, gravity_system, ground_detection_system, ground_movement_system,
        input::player_input_system,
        invincibility_timer_system, jump_initiation_system, pixel_snap_system,
        state_transition_system,
        ui::{setup_ui, update_coin_count_ui, update_fps_ui, update_health_ui, update_score_ui},
        variable_jump_system,
    },
};

/// Player plugin
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .insert_resource(MovementConfig::default())
            .insert_resource(Score::default())
            .insert_resource(PlayerAnimations::default())
            // Events
            .add_message::<PlayerMoved>()
            .add_message::<StateChanged>()
            // Startup systems
            .add_systems(Startup, spawn_player)
            .add_systems(Startup, spawn_test_ground)
            .add_systems(Startup, spawn_coins)
            .add_systems(Startup, spawn_enemies)
            .add_systems(Startup, setup_ui)
            // Update systems (order matters!)
            .add_systems(
                Update,
                (
                    // 1. Input processing
                    player_input_system,
                    // 2. Ground detection (before jump logic)
                    ground_detection_system,
                    // 3. Jump logic
                    jump_initiation_system,
                    variable_jump_system,
                    // 4. Movement logic
                    ground_movement_system,
                    // 5. Physics
                    gravity_system,
                    // 6. State updates
                    state_transition_system,
                    animation_system,
                    // 7. Apply velocity to position
                    apply_velocity_system,
                    // 8. Enemy AI
                    enemy_patrol_system,
                    // 9. Combat (damage and invincibility)
                    enemy_collision_system,
                    invincibility_timer_system,
                    // 10. Collectibles
                    coin_collection_system,
                    // 11. UI updates
                    update_score_ui,
                    update_health_ui,
                    update_coin_count_ui,
                    update_fps_ui,
                    // 12. Pixel snap for pixel-perfect rendering
                    pixel_snap_system,
                )
                    .chain(), // Run in sequence
            );
    }
}

use crate::domain::movement::MovementState;

/// Spawn player entity
fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut animations: ResMut<PlayerAnimations>,
) {
    // Load assets (T047, T068)
    let idle_texture: Handle<Image> = asset_server.load("sprites/player_idle.png");
    let walk_texture: Handle<Image> = asset_server.load("sprites/player_walk.png");
    let jump_texture: Handle<Image> = asset_server.load("sprites/player_jump.png");
    let fall_texture: Handle<Image> = asset_server.load("sprites/player_fall.png");

    // Configure animations
    animations.add(MovementState::Idle, vec![idle_texture.clone()]);
    animations.add(MovementState::Walking, vec![walk_texture]); // Only 1 frame for now
    animations.add(MovementState::Jumping, vec![jump_texture]);
    animations.add(MovementState::Falling, vec![fall_texture]);
    animations.frame_duration = 0.1; // 100ms per frame

    commands.spawn((
        // Sprite component
        Sprite {
            image: idle_texture,
            custom_size: Some(Vec2::new(32.0, 32.0)), // 32x32 pixels (2 tiles)
            ..default()
        },
        // Transform component
        Transform::from_xyz(0.0, 100.0, 1.0), // Start above ground
        // Physics components (Avian2d)
        RigidBody::Dynamic,                        // Dynamic physics body
        Collider::rectangle(32.0, 32.0),           // Box collider (32x32 pixels)
        CollisionLayer::Player.collision_filter(), // Collision filtering
        LockedAxes::ROTATION_LOCKED,               // Prevent rotation (stay upright)
        // Player marker
        Player,
        // Health
        Health::new(3), // 3 HP
        // Movement components
        InputState::default(),
        MovementStateComponent::default(),
        VelocityComponent::default(),
        GroundedState::default(),
        // Animation state
        AnimationState::new(0.1),
        // Pixel snap for pixel-perfect rendering
        PixelSnap,
    ));

    info!("Player spawned with 3 HP");
}

/// Spawn test ground platform
fn spawn_test_ground(mut commands: Commands) {
    // Main ground platform (large, at bottom)
    commands.spawn((
        Sprite {
            color: Color::srgb(0.5, 0.5, 0.5), // Medium gray
            custom_size: Some(Vec2::new(640.0, 32.0)),
            ..default()
        },
        Transform::from_xyz(0.0, -200.0, 0.0), // Bottom of screen
        RigidBody::Static,                     // Static platform (doesn't move)
        Collider::rectangle(640.0, 32.0),      // Match sprite size
        CollisionLayer::Ground.collision_filter(),
        PixelSnap,
    ));

    // Left floating platform
    commands.spawn((
        Sprite {
            color: Color::srgb(0.6, 0.8, 0.6), // Light green
            custom_size: Some(Vec2::new(128.0, 16.0)),
            ..default()
        },
        Transform::from_xyz(-200.0, -80.0, 0.0),
        RigidBody::Static,
        Collider::rectangle(128.0, 16.0),
        CollisionLayer::Ground.collision_filter(),
        PixelSnap,
    ));

    // Right floating platform
    commands.spawn((
        Sprite {
            color: Color::srgb(0.6, 0.6, 0.8), // Light blue
            custom_size: Some(Vec2::new(128.0, 16.0)),
            ..default()
        },
        Transform::from_xyz(200.0, -80.0, 0.0),
        RigidBody::Static,
        Collider::rectangle(128.0, 16.0),
        CollisionLayer::Ground.collision_filter(),
        PixelSnap,
    ));

    // Center high platform
    commands.spawn((
        Sprite {
            color: Color::srgb(0.8, 0.6, 0.6), // Light red
            custom_size: Some(Vec2::new(96.0, 16.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 40.0, 0.0),
        RigidBody::Static,
        Collider::rectangle(96.0, 16.0),
        CollisionLayer::Ground.collision_filter(),
        PixelSnap,
    ));

    info!("Test platforms spawned (4 platforms with Avian2d physics)");
}

/// Spawn coins in the level
fn spawn_coins(mut commands: Commands) {
    // Coins on left platform
    for i in 0..3 {
        commands.spawn((
            Sprite {
                color: Color::srgb(1.0, 0.9, 0.0), // Gold color
                custom_size: Some(Vec2::new(16.0, 16.0)),
                ..default()
            },
            Transform::from_xyz((i as f32).mul_add(30.0, -200.0), -50.0, 0.5),
            Coin::standard(),
            PixelSnap,
        ));
    }

    // Coins on right platform
    for i in 0..3 {
        commands.spawn((
            Sprite {
                color: Color::srgb(1.0, 0.9, 0.0),
                custom_size: Some(Vec2::new(16.0, 16.0)),
                ..default()
            },
            Transform::from_xyz((i as f32).mul_add(30.0, 200.0), -50.0, 0.5),
            Coin::standard(),
            PixelSnap,
        ));
    }

    // Valuable coin on high platform
    commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 0.5, 0.0), // Orange (more valuable)
            custom_size: Some(Vec2::new(20.0, 20.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 70.0, 0.5),
        Coin::valuable(), // Worth 5 points
        PixelSnap,
    ));

    info!("Coins spawned: 6 standard + 1 valuable");
}

/// Spawn enemies in the level
fn spawn_enemies(mut commands: Commands) {
    // Enemy on bottom ground (patrolling left/right)
    commands.spawn((
        Sprite {
            color: Color::srgb(0.8, 0.2, 0.2), // Red enemy
            custom_size: Some(Vec2::new(24.0, 24.0)),
            ..default()
        },
        Transform::from_xyz(0.0, -170.0, 0.5),
        RigidBody::Kinematic, // Kinematic = not affected by gravity
        Collider::rectangle(24.0, 24.0),
        CollisionLayer::Ground.collision_filter(),
        Enemy,
        PatrolBehavior::new(0.0, 150.0, 50.0), // Patrol 150px left/right at 50px/s
        PixelSnap,
    ));

    // Enemy on left platform
    commands.spawn((
        Sprite {
            color: Color::srgb(0.8, 0.2, 0.2),
            custom_size: Some(Vec2::new(24.0, 24.0)),
            ..default()
        },
        Transform::from_xyz(-200.0, -60.0, 0.5),
        RigidBody::Kinematic, // Kinematic = not affected by gravity
        Collider::rectangle(24.0, 24.0),
        CollisionLayer::Ground.collision_filter(),
        Enemy,
        PatrolBehavior::new(-200.0, 50.0, 40.0), // Smaller patrol on platform
        PixelSnap,
    ));

    // Enemy on right platform
    commands.spawn((
        Sprite {
            color: Color::srgb(0.8, 0.2, 0.2),
            custom_size: Some(Vec2::new(24.0, 24.0)),
            ..default()
        },
        Transform::from_xyz(200.0, -60.0, 0.5),
        RigidBody::Kinematic,
        Collider::rectangle(24.0, 24.0),
        CollisionLayer::Ground.collision_filter(),
        Enemy,
        PatrolBehavior::new(200.0, 50.0, 40.0),
        PixelSnap,
    ));

    info!("Enemies spawned: 3 patrolling enemies");
}
