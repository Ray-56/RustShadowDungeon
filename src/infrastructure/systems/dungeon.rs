//! Dungeon system implementations
//!
//! Systems for managing dungeon rooms, doors, and progression.

use crate::domain::combat::collision::Rect;
use crate::domain::dungeon::progression::{should_spawn_enemies, RoomState as DomainRoomState};
use crate::infrastructure::components::dungeon::{
    Door, DoorId, DoorState, DungeonManager, EnemySpawnPoint, Room, RoomId,
};
use crate::infrastructure::components::enemy::EnemyId;
use crate::infrastructure::components::enemy::PatrolBehavior;
use crate::infrastructure::components::loot::WorldItem;
use crate::infrastructure::components::ui::InteractPrompt;
use crate::infrastructure::components::Player;
use crate::infrastructure::resources::loot::PickupConfig;
use crate::infrastructure::components::{Enemy, EnemyType, Health, HurtBox, Stats};
use crate::infrastructure::events::dungeon::{
    DoorUnlocked, PlayerDeathInRoom, RoomCleared, RoomEntered, RoomTransitioned,
};
use crate::infrastructure::resources::dungeon::DungeonSession;
use crate::infrastructure::resources::dungeon::{DungeonConfig, RoomConfig};
use crate::infrastructure::systems::camera::GameCamera;
use bevy::ecs::message::MessageReader;
use bevy::ecs::message::MessageWriter;
use bevy::prelude::*;

/// Load dungeon system - loads dungeon configuration and initializes first room
///
/// 加载地下城系统
///
/// Loads dungeon configuration from RON file and initializes the starting room.
/// This system runs once at startup.
pub fn load_dungeon_system(mut commands: Commands, _asset_server: Res<AssetServer>) {
    // Load dungeon config from RON file
    // Try to load from assets directory (relative to executable or project root)
    let config_path = "assets/data/dungeons/test_dungeon.ron";

    let dungeon_config = match DungeonConfig::load_from_file(config_path) {
        Ok(config) => {
            info!("✅ Loaded dungeon config: {} from {}", config.dungeon_id, config_path);
            config
        },
        Err(e) => {
            warn!(
                "⚠️  Failed to load dungeon config from '{}': {}. Using fallback hardcoded config.",
                config_path, e
            );
            // Fallback to hardcoded config if file loading fails
            // This ensures the game can still run even if config file is missing
            DungeonConfig {
                dungeon_id: "fallback_dungeon".to_string(),
                start_room_id: 1,
                rooms: vec![
                    RoomConfig {
                        room_id: 1,
                        spawn_points: vec![[100.0, 0.0], [200.0, 0.0], [300.0, 0.0]],
                        doors: vec![crate::infrastructure::resources::dungeon::DoorConfig {
                            door_id: 1,
                            connected_room_id: 2,
                            entrance_position: [100.0, 0.0],
                        }],
                    },
                    RoomConfig {
                        room_id: 2,
                        spawn_points: vec![[150.0, 0.0], [250.0, 0.0]],
                        doors: vec![
                            crate::infrastructure::resources::dungeon::DoorConfig {
                                door_id: 1,
                                connected_room_id: 1,
                                entrance_position: [100.0, 0.0],
                            },
                            crate::infrastructure::resources::dungeon::DoorConfig {
                                door_id: 2,
                                connected_room_id: 3,
                                entrance_position: [100.0, 0.0],
                            },
                        ],
                    },
                    RoomConfig {
                        room_id: 3,
                        spawn_points: vec![[200.0, 0.0]],
                        doors: vec![crate::infrastructure::resources::dungeon::DoorConfig {
                            door_id: 2,
                            connected_room_id: 2,
                            entrance_position: [100.0, 0.0],
                        }],
                    },
                ],
            }
        },
    };

    let start_room_id = RoomId(dungeon_config.start_room_id);

    // Build room graph from config
    // Room graph maps RoomId -> Vec<(DoorId, connected_room_id)>
    let mut room_graph = std::collections::HashMap::new();

    for room_config in &dungeon_config.rooms {
        let room_id = RoomId(room_config.room_id);
        let mut connections = Vec::new();

        for door_config in &room_config.doors {
            connections.push((DoorId(door_config.door_id), RoomId(door_config.connected_room_id)));
        }

        room_graph.insert(room_id, connections);
    }

    // Store dungeon config as a resource for later use (e.g., room initialization)
    commands.insert_resource(dungeon_config);

    // Create dungeon manager
    let _dungeon_manager_entity = commands
        .spawn((
            DungeonManager { current_room_id: start_room_id, room_graph: room_graph.clone() },
            Name::new("DungeonManager"),
        ))
        .id();

    info!("🏛️  Dungeon loaded, starting room: {:?}", start_room_id);

    // Initialize first room
    // This will be handled by initialize_room_system after room entity is created
}

/// Initialize room system - initializes a room and sets doors to locked
///
/// 初始化房间系统
///
/// Creates room entity with spawn points and doors, sets all doors to locked state.
/// This system will create a room if it doesn't exist, or if the current room ID doesn't match.
/// Restores room state from DungeonSession if room was previously cleared.
///
/// Performance optimizations:
/// - Lazy loading: Only creates entities when room is actually entered
/// - Batch processing: Collects entities before spawning to reduce command queue operations
/// 性能优化：
/// - 延迟加载：只在房间真正进入时创建实体
/// - 批量处理：在生成前收集实体以减少命令队列操作
pub fn initialize_room_system(
    mut commands: Commands,
    dungeon_manager_query: Query<&DungeonManager>,
    room_query: Query<(Entity, &Room)>,
    dungeon_session: Res<DungeonSession>,
    dungeon_config: Option<Res<DungeonConfig>>,
) {
    let Ok(dungeon_manager) = dungeon_manager_query.single() else {
        return;
    };

    let current_room_id = dungeon_manager.current_room_id;

    // Check if room already exists and matches current room ID
    if let Some((_room_entity, room)) = room_query.iter().next() {
        if room.room_id == current_room_id {
            // Room already exists and matches, no need to recreate
            return;
        }
        // Room exists but ID doesn't match (shouldn't happen if unload works correctly)
        // But we'll handle it anyway by despawn and recreate
        commands.entity(_room_entity).despawn();
    }

    // Check if room was previously cleared
    let is_cleared = dungeon_session.cleared_rooms.contains(&current_room_id);
    let initial_state =
        if is_cleared { DomainRoomState::Cleared } else { DomainRoomState::Uncleared };

    // Get spawn points and door configs from config if available, otherwise use fallback
    // 从配置获取生成点和门配置（如果可用），否则使用后备方案
    let (spawn_points, room_door_configs) = if let Some(config) = dungeon_config.as_ref() {
        // Find room config for current room
        if let Some(room_config) = config.rooms.iter().find(|r| r.room_id == current_room_id.0) {
            // Convert spawn points from [f32; 2] to Vec2
            let spawns: Vec<Vec2> =
                room_config.spawn_points.iter().map(|sp| Vec2::new(sp[0], sp[1])).collect();
            // Get door configs for this room
            let doors: Vec<&crate::infrastructure::resources::dungeon::DoorConfig> =
                room_config.doors.iter().collect();
            (spawns, doors)
        } else {
            // Room not found in config, use default
            warn!("Room {:?} not found in config, using default spawn points", current_room_id);
            (
                vec![Vec2::new(100.0, 0.0), Vec2::new(200.0, 0.0), Vec2::new(300.0, 0.0)],
                Vec::new(),
            )
        }
    } else {
        // Config not loaded, use hardcoded fallback based on room ID
        let spawns = match current_room_id.0 {
            1 => {
                vec![Vec2::new(100.0, 0.0), Vec2::new(200.0, 0.0), Vec2::new(300.0, 0.0)]
            },
            2 => {
                vec![Vec2::new(80.0, 0.0), Vec2::new(200.0, -50.0), Vec2::new(320.0, 0.0)]
            },
            _ => {
                vec![Vec2::new(100.0, 0.0), Vec2::new(200.0, 0.0), Vec2::new(300.0, 0.0)]
            },
        };
        (spawns, Vec::new())
    };

    let _room_entity = commands
        .spawn((
            Room {
                room_id: current_room_id,
                state: initial_state,
                spawn_points: spawn_points.clone(),
            },
            Name::new(format!("Room_{}", current_room_id.0)),
        ))
        .id();

    // Lazy loading: Create doors only when room is actually loaded
    // 延迟加载：只在房间真正加载时创建门
    if let Some(connections) = dungeon_manager.room_graph.get(&current_room_id) {
        // Batch create doors to reduce command queue operations
        // 批量创建门以减少命令队列操作
        for (door_id, connected_room_id) in connections {
            // Get entrance position from config if available, otherwise use default
            // 从配置获取入口位置（如果可用），否则使用默认值
            let (entrance_position, door_position) = if let Some(door_config) =
                room_door_configs.iter().find(|d| d.door_id == door_id.0)
            {
                // Use entrance position from config (this is where player appears in target room)
                // 使用配置中的入口位置（这是玩家在目标房间出现的位置）
                let entrance =
                    Vec2::new(door_config.entrance_position[0], door_config.entrance_position[1]);
                // Door position in current room (use entrance position as fallback, or place at default location)
                // 当前房间中的门位置（使用入口位置作为后备，或放置在默认位置）
                let door_pos = Vec3::new(entrance.x, entrance.y, 0.0);
                (entrance, door_pos)
            } else {
                // Fallback: Place doors closer to player spawn (200 pixels to the right, at ground level)
                // 后备方案：将门放置在玩家生成点附近（右侧200像素，地面高度）
                let entrance = Vec2::new(200.0, -170.0);
                let door_pos = Vec3::new(200.0, -170.0, 0.0);
                (entrance, door_pos)
            };

            // If room is cleared, doors should be unlocked
            let door_state = if is_cleared { DoorState::Unlocked } else { DoorState::Locked };

            commands.spawn((
                Door {
                    door_id: *door_id,
                    connected_room_id: *connected_room_id,
                    door_state,
                    entrance_position,
                },
                Name::new(format!("Door_{}", door_id.0)),
                Transform::from_translation(door_position),
                Sprite {
                    color: if door_state == DoorState::Unlocked {
                        Color::srgb(0.7, 0.5, 0.3) // Lighter color for unlocked
                    } else {
                        Color::srgb(0.5, 0.3, 0.1) // Brown color for locked
                    },
                    custom_size: Some(Vec2::new(32.0, 64.0)),
                    ..default()
                },
            ));
        }
    }

    // Lazy loading: Create spawn points only when room is actually loaded
    // 延迟加载：只在房间真正加载时创建生成点
    // Batch create spawn points to reduce command queue operations
    // 批量创建生成点以减少命令队列操作
    for (idx, spawn_point) in spawn_points.iter().enumerate() {
        commands.spawn((
            EnemySpawnPoint {
                position: *spawn_point,
                enemy_type: "slime".to_string(),
                spawn_on_activate: true,
            },
            Name::new(format!("SpawnPoint_{}", idx)),
            Transform::from_translation(Vec3::new(spawn_point.x, spawn_point.y, 0.0)),
        ));
    }

    if is_cleared {
        info!("🏠 Room {:?} loaded (previously cleared, no enemies)", current_room_id);
    } else {
        info!("🏠 Room {:?} loaded (enemies will spawn)", current_room_id);
    }
}

/// Spawn enemies system - spawns enemies based on room state and configuration
///
/// 生成敌人系统
///
/// Spawns enemies at spawn points when room is activated and should spawn enemies.
/// Optimized with lazy loading - only spawns when room is actually entered.
/// 使用延迟加载优化 - 只在房间真正进入时生成
pub fn spawn_enemies_system(
    mut commands: Commands,
    room_query: Query<&Room>,
    spawn_point_query: Query<(Entity, &EnemySpawnPoint)>,
    enemy_query: Query<Entity, With<Enemy>>,
    dungeon_session: Res<DungeonSession>,
) {
    let Ok(room) = room_query.single() else {
        return;
    };

    // Check if room is already cleared (don't spawn enemies) - lazy loading optimization
    // 检查房间是否已清理（不生成敌人）- 延迟加载优化
    if dungeon_session.cleared_rooms.contains(&room.room_id) {
        return;
    }

    // Check if enemies should spawn
    if !should_spawn_enemies(room.state) {
        return;
    }

    // Check if enemies already exist (simple check - can be improved to check per-room)
    let existing_enemies = enemy_query.iter().count();
    if existing_enemies > 0 {
        return;
    }

    // Lazy loading: Only spawn enemies when room is actually loaded and activated
    // 延迟加载：只在房间真正加载并激活时生成敌人
    // Collect spawn points first to batch process
    // 首先收集生成点以进行批量处理
    let spawn_points: Vec<_> = spawn_point_query
        .iter()
        .filter(|(_, spawn_point)| spawn_point.spawn_on_activate)
        .collect();

    // Batch spawn enemies to reduce command queue operations
    // 批量生成敌人以减少命令队列操作
    let mut spawned_count = 0;
    for (_spawn_entity, spawn_point) in spawn_points {
        // Spawn slime enemy (for now, hardcoded - can be extended to load from config)
        let _enemy_entity = commands
            .spawn((
                Enemy,
                EnemyType::Slime,
                EnemyId(rand::random::<u32>()),
                Name::new("Slime"),
                Stats::new(5.0, 0.0),
                Health::new(30.0),
                HurtBox::new(Rect { x: -8.0, y: -8.0, width: 16.0, height: 16.0 }),
                PatrolBehavior {
                    left_bound: spawn_point.position.x - 50.0,
                    right_bound: spawn_point.position.x + 50.0,
                    speed: 50.0,
                    direction: 1.0,
                },
                Transform::from_translation(Vec3::new(
                    spawn_point.position.x,
                    spawn_point.position.y,
                    0.0,
                )),
                Sprite {
                    color: Color::srgb(0.2, 0.8, 0.2), // Green color for slime
                    custom_size: Some(Vec2::new(16.0, 16.0)),
                    ..default()
                },
            ))
            .id();

        spawned_count += 1;
    }

    if spawned_count > 0 {
        info!("👾 Spawned {} enemy/enemies in room {:?}", spawned_count, room.room_id);
    }
}

/// Handle room entered system - listens to RoomEntered events and triggers enemy spawning
///
/// 处理房间进入系统
///
/// When player enters a room, this system ensures enemies are spawned if needed.
pub fn handle_room_entered_system(mut room_entered_events: MessageReader<RoomEntered>) {
    for _event in room_entered_events.read() {
        // Player entered room
        // Enemy spawning will be handled by spawn_enemies_system
    }
}

/// Monitor enemy deaths system - listens to EnemyDefeated events and tracks enemy count
///
/// 监控敌人死亡系统
///
/// Listens to EnemyDefeated events and checks if room should be cleared.
/// Note: This system triggers room clear check, but actual clearing happens after death animation.
pub fn monitor_enemy_deaths_system(
    mut enemy_defeated_events: MessageReader<crate::infrastructure::events::combat::EnemyDefeated>,
) {
    for _event in enemy_defeated_events.read() {
        // Enemy defeated (room clear check will be handled by check_room_clear_system)
    }
}

/// Check room clear system - checks if room should be marked as cleared
///
/// 检查房间清理系统
///
/// Checks if all enemies in the current room have been defeated (after death animation completes).
/// When room is cleared, publishes RoomCleared event.
/// Also handles empty rooms (no enemies configured).
pub fn check_room_clear_system(
    mut commands: Commands,
    room_query: Query<(Entity, &Room)>,
    enemy_query: Query<
        Entity,
        (With<Enemy>, Without<crate::infrastructure::components::enemy::DeathAnimation>),
    >,
    spawn_point_query: Query<&EnemySpawnPoint>,
    mut room_cleared_events: bevy::ecs::message::MessageWriter<RoomCleared>,
    time: Res<Time>,
) {
    for (room_entity, room) in room_query.iter() {
        // Only check uncleared rooms
        if !matches!(room.state, DomainRoomState::Uncleared) {
            continue;
        }

        // Count living enemies (not in death animation, and not despawned)
        let enemy_count = enemy_query.iter().count();

        // Count configured enemy spawn points
        let spawn_point_count =
            spawn_point_query.iter().filter(|spawn| spawn.spawn_on_activate).count();

        // Handle empty room case (no enemies configured)
        // Only mark as empty if there are no spawn points AND no enemies exist
        // This check happens after spawn_enemies_system, so if spawn points exist, enemies should have spawned
        if spawn_point_count == 0 && enemy_count == 0 {
            // Empty room - mark as cleared immediately
            commands.entity(room_entity).insert(Room {
                room_id: room.room_id,
                state: DomainRoomState::Cleared,
                spawn_points: room.spawn_points.clone(),
            });

            room_cleared_events
                .write(RoomCleared { room_id: room.room_id, cleared_at: time.elapsed_secs_f64() });

            info!("✅ Room {:?} cleared (empty)", room.room_id);
            continue;
        }

        // Check if room should be cleared using domain logic
        // Only mark as cleared if enemies were actually defeated (not just not spawned yet)
        if spawn_point_count > 0
            && crate::domain::dungeon::progression::check_room_cleared(enemy_count)
        {
            // Update room state
            commands.entity(room_entity).insert(Room {
                room_id: room.room_id,
                state: DomainRoomState::Cleared,
                spawn_points: room.spawn_points.clone(),
            });

            // Publish RoomCleared event
            room_cleared_events
                .write(RoomCleared { room_id: room.room_id, cleared_at: time.elapsed_secs_f64() });

            info!("✅ Room {:?} cleared", room.room_id);
        }
    }
}

/// Track room clear system - records cleared rooms to DungeonSession
///
/// 追踪房间清理系统
///
/// When room is cleared, records the room ID to DungeonSession for progress tracking.
pub fn track_room_clear_system(
    mut room_cleared_events: MessageReader<RoomCleared>,
    mut dungeon_session: ResMut<DungeonSession>,
) {
    for event in room_cleared_events.read() {
        // Record room as cleared in session
        dungeon_session.cleared_rooms.insert(event.room_id);
        // Room recorded in session
    }
}

/// Handle room cleared system - listens to RoomCleared events and unlocks doors
///
/// 处理房间清理系统
///
/// When room is cleared, this system unlocks all doors in the room.
pub fn handle_room_cleared_system(mut room_cleared_events: MessageReader<RoomCleared>) {
    for _event in room_cleared_events.read() {
        // Room cleared event handled (unlock_doors_system will unlock doors)
    }
}

/// Unlock doors system - unlocks doors when room is cleared
///
/// 解锁门系统
///
/// Updates door state to Unlocked when room is cleared.
pub fn unlock_doors_system(
    room_query: Query<&Room>,
    mut door_query: Query<(Entity, &mut Door)>,
    mut door_unlocked_events: bevy::ecs::message::MessageWriter<DoorUnlocked>,
) {
    let Ok(room) = room_query.single() else {
        return;
    };

    // Only unlock doors if room is cleared
    if !matches!(room.state, DomainRoomState::Cleared) {
        return;
    }

    // Unlock all doors in the current room
    let mut unlocked_count = 0;
    for (_door_entity, mut door) in door_query.iter_mut() {
        if door.door_state == DoorState::Locked {
            door.door_state = DoorState::Unlocked;
            unlocked_count += 1;

            // Publish DoorUnlocked event
            door_unlocked_events
                .write(DoorUnlocked { door_id: door.door_id, room_id: room.room_id });

            // Door unlocked (silent - visual will show it)
        }
    }

    if unlocked_count > 0 {
        info!("🔓 {} door(s) unlocked in room {:?}", unlocked_count, room.room_id);
    }
}

/// Update door visual system - updates door visual appearance based on state
///
/// 更新门视觉系统
///
/// Updates door sprite color and visual appearance based on door state.
pub fn update_door_visual_system(mut door_query: Query<(&Door, &mut Sprite), Changed<Door>>) {
    for (door, mut sprite) in door_query.iter_mut() {
        match door.door_state {
            DoorState::Locked => {
                // Brown/dark color for locked door
                sprite.color = Color::srgb(0.5, 0.3, 0.1);
            },
            DoorState::Unlocked => {
                // Lighter color for unlocked door
                sprite.color = Color::srgb(0.7, 0.5, 0.3);
            },
        }
    }
}

/// Door interaction detection system - detects when player is near an unlocked door
///
/// 门交互检测系统
///
/// Detects when player is close to an unlocked door and shows interaction prompt.
pub fn door_interaction_detection_system(
    mut _commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    door_query: Query<(Entity, &Door, &Transform), (With<Door>, Without<Player>)>,
    mut _prompt_query: Query<Entity, (With<InteractPrompt>, Without<Door>)>,
    _keyboard: Res<ButtonInput<KeyCode>>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    let player_pos = player_transform.translation.truncate();
    const INTERACTION_DISTANCE: f32 = 150.0; // Distance threshold for interaction (increased for easier access)

    // Check all doors
    for (_door_entity, door, door_transform) in door_query.iter() {
        let door_pos = door_transform.translation.truncate();
        let distance = player_pos.distance(door_pos);

        // Only show prompt for unlocked doors
        if door.door_state == DoorState::Unlocked && distance < INTERACTION_DISTANCE {
            // Show interaction prompt if not already shown (silent - UI will show it)
        }
    }
}

/// Handle door interaction system - processes player interaction with doors
///
/// 处理门交互系统
///
/// When player presses interaction key near an unlocked door, triggers room transition.
pub fn handle_door_interaction_system(
    mut _commands: Commands,
    player_query: Query<(Entity, &Transform), With<Player>>,
    door_query: Query<(Entity, &Door, &Transform), (With<Door>, Without<Player>)>,
    world_item_query: Query<(&Transform, &WorldItem), (With<WorldItem>, Without<Player>)>,
    mut room_transitioned_events: MessageWriter<RoomTransitioned>,
    keyboard: Res<ButtonInput<KeyCode>>,
    dungeon_manager_query: Query<&DungeonManager>,
    pickup_config: Option<Res<PickupConfig>>,
) {
    let Ok((_player_entity, player_transform)) = player_query.single() else {
        return;
    };

    // Check if interaction key is pressed (E key)
    if !keyboard.just_pressed(KeyCode::KeyE) {
        return;
    }

    let player_pos = player_transform.translation.truncate();
    
    // Check if there are any items in pickup range - if so, don't process door interaction
    // This ensures item pickup takes priority even if both systems run
    if let Some(pickup_config) = pickup_config.as_ref() {
        let range_pixels = pickup_config.range * 16.0;
        for (item_transform, world_item) in world_item_query.iter() {
            if world_item.picked_up {
                continue;
            }
            let item_pos = item_transform.translation.truncate();
            let distance = item_pos.distance(player_pos);
            if distance <= range_pixels {
                // Item is in pickup range, let pickup system handle it
                return;
            }
        }
    }

    let Ok(dungeon_manager) = dungeon_manager_query.single() else {
        return;
    };

    const INTERACTION_DISTANCE: f32 = 150.0;

    // Find nearby unlocked door
    let mut found_door = false;
    for (_door_entity, door, door_transform) in door_query.iter() {
        let door_pos = door_transform.translation.truncate();
        let distance = player_pos.distance(door_pos);

        if door.door_state == DoorState::Unlocked && distance < INTERACTION_DISTANCE {
            found_door = true;
            // Check if transition is allowed using domain logic
            if crate::domain::dungeon::progression::can_transition_to_room(door.door_state) {
                let from_room_id = dungeon_manager.current_room_id;
                let to_room_id = door.connected_room_id;

                // Get target entrance position
                let target_position = crate::domain::dungeon::progression::get_target_room_entrance(
                    door.entrance_position,
                );

                // Publish room transition event
                room_transitioned_events.write(RoomTransitioned {
                    from_room_id,
                    to_room_id,
                    player_position: target_position,
                });

                info!("🚪 Room transition: {:?} → {:?}", from_room_id, to_room_id);
                return;
            }
        }
    }

    // Only log if E was pressed but no door found (for debugging)
    if !found_door {
        info!("⚠️  E pressed but no nearby unlocked door (check distance and door state)");
    }
}

/// Transition to room system - teleports player to target room entrance
///
/// 房间过渡系统
///
/// Teleports player to the target room's entrance position when room transition occurs.
pub fn transition_to_room_system(
    mut room_transitioned_events: MessageReader<RoomTransitioned>,
    mut player_query: Query<&mut Transform, With<Player>>,
    mut dungeon_manager_query: Query<&mut DungeonManager>,
) {
    for event in room_transitioned_events.read() {
        // Update dungeon manager current room
        if let Ok(mut dungeon_manager) = dungeon_manager_query.single_mut() {
            dungeon_manager.current_room_id = event.to_room_id;
        }

        // Teleport player to target position
        if let Ok(mut player_transform) = player_query.single_mut() {
            player_transform.translation.x = event.player_position.x;
            player_transform.translation.y = event.player_position.y;
            info!("📍 Player moved to {:?} in room {:?}", event.player_position, event.to_room_id);
        }
    }
}

/// Unload current room system - removes enemies and temporary entities from current room
///
/// 卸载当前房间系统
///
/// Removes all enemies, doors, spawn points, and room entities when leaving a room.
/// Optimized with batch processing to reduce command queue operations.
/// 使用批量处理优化，减少命令队列操作次数
pub fn unload_current_room_system(
    mut commands: Commands,
    mut room_transitioned_events: MessageReader<RoomTransitioned>,
    enemy_query: Query<
        Entity,
        (With<Enemy>, Without<crate::infrastructure::components::enemy::DeathAnimation>),
    >,
    spawn_point_query: Query<Entity, With<EnemySpawnPoint>>,
    room_query: Query<Entity, With<Room>>,
    door_query: Query<Entity, With<Door>>,
) {
    // Only unload if there's a room transition
    if room_transitioned_events.read().next().is_some() {
        // Collect all entities to despawn for batch processing
        // 收集所有需要销毁的实体，进行批量处理
        let mut entities_to_despawn: Vec<Entity> = Vec::new();

        // Collect enemies
        entities_to_despawn.extend(enemy_query.iter());

        // Collect spawn points
        entities_to_despawn.extend(spawn_point_query.iter());

        // Collect doors
        entities_to_despawn.extend(door_query.iter());

        // Collect room entities
        entities_to_despawn.extend(room_query.iter());

        // Batch despawn all entities at once
        // 批量销毁所有实体
        let entity_count = entities_to_despawn.len();
        for entity in entities_to_despawn {
            commands.entity(entity).despawn();
        }

        info!("🗑️  Previous room unloaded ({} entities)", entity_count);
    }
}

/// Restore room state system - restores room state when player returns to a room
///
/// 恢复房间状态系统
///
/// When player returns to a previously visited room, restores its state from DungeonSession.
/// This ensures cleared rooms remain cleared and doors remain unlocked.
pub fn restore_room_state_system(
    mut commands: Commands,
    room_query: Query<(Entity, &Room)>,
    mut door_query: Query<&mut Door>,
    dungeon_session: Res<DungeonSession>,
) {
    for (room_entity, room) in room_query.iter() {
        // Check if room is in session as cleared
        let is_cleared_in_session = dungeon_session.cleared_rooms.contains(&room.room_id);

        // If room should be cleared but isn't, update it
        if is_cleared_in_session && !matches!(room.state, DomainRoomState::Cleared) {
            commands.entity(room_entity).insert(Room {
                room_id: room.room_id,
                state: DomainRoomState::Cleared,
                spawn_points: room.spawn_points.clone(),
            });

            // Room state restored
        }

        // Also ensure doors are unlocked for cleared rooms
        if is_cleared_in_session {
            // Unlock all doors in this room
            for mut door in door_query.iter_mut() {
                if door.door_state == DoorState::Locked {
                    door.door_state = DoorState::Unlocked;
                    // Door restored
                }
            }
        }
    }
}

/// Load target room system - loads target room and spawns enemies if needed
///
/// 加载目标房间系统
///
/// Loads the target room and spawns enemies based on room state.
pub fn load_target_room_system(
    mut _commands: Commands,
    mut room_transitioned_events: MessageReader<RoomTransitioned>,
    dungeon_manager_query: Query<&DungeonManager>,
    dungeon_session: Res<DungeonSession>,
) {
    for event in room_transitioned_events.read() {
        let Ok(_dungeon_manager) = dungeon_manager_query.single() else {
            continue;
        };

        let _target_room_id = event.to_room_id;

        // Check if room is already cleared
        let _is_cleared = dungeon_session.cleared_rooms.contains(&_target_room_id);

        // Loading target room

        // Room initialization and enemy spawning will be handled by existing systems
    }
}

/// Update camera on room transition system - updates camera context for new room
///
/// 更新相机房间过渡系统
///
/// Updates camera position and context when transitioning to a new room.
pub fn update_camera_on_room_transition_system(
    mut room_transitioned_events: MessageReader<RoomTransitioned>,
    mut camera_query: Query<&mut Transform, (With<GameCamera>, Without<Player>)>,
) {
    for _event in room_transitioned_events.read() {
        // Camera will automatically follow player (handled by camera_follow_system)
        // This system just ensures camera updates for the new room context
        if let Ok(_camera_transform) = camera_query.single_mut() {
            // Camera updated
        }
    }
}

/// Show interaction prompt system - displays/hides interaction prompt UI
///
/// 显示交互提示系统
///
/// Shows or hides the "Press E to interact" prompt when player is near doors.
pub fn show_interaction_prompt_system(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    door_query: Query<(&Door, &Transform), (With<Door>, Without<Player>)>,
    prompt_query: Query<Entity, With<InteractPrompt>>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    let player_pos = player_transform.translation.truncate();
    const INTERACTION_DISTANCE: f32 = 150.0; // Increased for easier interaction

    // Check if player is near any unlocked door
    let mut near_door = false;
    for (door, door_transform) in door_query.iter() {
        if door.door_state == DoorState::Unlocked {
            let door_pos = door_transform.translation.truncate();
            let distance = player_pos.distance(door_pos);
            if distance < INTERACTION_DISTANCE {
                near_door = true;
                break;
            }
        }
    }

    // Show or hide prompt based on proximity
    if near_door {
        // Show prompt if not already shown
        if prompt_query.iter().next().is_none() {
            // For now, just log - full UI implementation can be added later
            // Show interaction prompt (UI will handle display)
        }
    } else {
        // Hide prompt if player moved away
        for prompt_entity in prompt_query.iter() {
            commands.entity(prompt_entity).despawn();
        }
    }
}

/// Handle player death in dungeon system - respawns player at room entrance
///
/// 处理玩家在地下城中死亡系统
///
/// When player dies in a dungeon room, respawns them at the room entrance.
/// Room state remains unchanged (enemies stay dead, doors stay unlocked).
pub fn handle_player_death_in_dungeon_system(
    mut player_death_events: MessageReader<PlayerDeathInRoom>,
    mut player_query: Query<(&mut Transform, &mut Health), With<Player>>,
    _room_query: Query<&Room>,
) {
    for event in player_death_events.read() {
        info!("Player died in room {:?} at position {:?}", event.room_id, event.death_position);

        // Respawn player at room entrance (use first spawn point or default position)
        if let Ok((mut player_transform, mut player_health)) = player_query.single_mut() {
            // Get room entrance position (use default spawn position for now)
            let respawn_position = Vec2::new(0.0, 0.0); // Default room entrance

            // Teleport player to entrance
            player_transform.translation.x = respawn_position.x;
            player_transform.translation.y = respawn_position.y;

            // Restore player health
            player_health.reset();

            info!(
                "Player respawned at room entrance {:?} in room {:?}",
                respawn_position, event.room_id
            );
        }
    }
}

/// Handle empty room system - marks empty rooms as cleared immediately
///
/// 处理空房间系统
///
/// If a room has no enemies configured, marks it as cleared and unlocks doors immediately.
pub fn handle_empty_room_system(
    mut commands: Commands,
    room_query: Query<(Entity, &Room)>,
    spawn_point_query: Query<&EnemySpawnPoint>,
    mut room_cleared_events: bevy::ecs::message::MessageWriter<RoomCleared>,
    time: Res<Time>,
) {
    for (room_entity, room) in room_query.iter() {
        // Only check uncleared rooms
        if !matches!(room.state, DomainRoomState::Uncleared) {
            continue;
        }

        // Count spawn points that should spawn enemies
        let enemy_spawn_count =
            spawn_point_query.iter().filter(|spawn| spawn.spawn_on_activate).count();

        // If no enemies should spawn, mark room as cleared
        if enemy_spawn_count == 0 {
            commands.entity(room_entity).insert(Room {
                room_id: room.room_id,
                state: DomainRoomState::Cleared,
                spawn_points: room.spawn_points.clone(),
            });

            // Publish RoomCleared event
            room_cleared_events
                .write(RoomCleared { room_id: room.room_id, cleared_at: time.elapsed_secs_f64() });

            // Empty room cleared
        }
    }
}

/// Ensure state consistency system - ensures room and door states are consistent
///
/// 确保状态一致性系统
///
/// Ensures that room state and door states are consistent (cleared rooms have unlocked doors).
pub fn ensure_state_consistency_system(
    room_query: Query<(Entity, &Room)>,
    mut door_query: Query<&mut Door>,
) {
    for (_room_entity, room) in room_query.iter() {
        // If room is cleared, ensure all doors are unlocked
        if matches!(room.state, DomainRoomState::Cleared) {
            for mut door in door_query.iter_mut() {
                if door.door_state == DoorState::Locked {
                    door.door_state = DoorState::Unlocked;
                    // State fixed
                }
            }
        }

        // If room is uncleared, ensure all doors are locked
        if matches!(room.state, DomainRoomState::Uncleared) {
            for mut door in door_query.iter_mut() {
                if door.door_state == DoorState::Unlocked {
                    door.door_state = DoorState::Locked;
                    // State fixed
                }
            }
        }
    }
}
