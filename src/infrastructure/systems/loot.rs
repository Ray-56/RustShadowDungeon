//! Loot and inventory systems
//!
//! 战利品与库存系统
//!
//! Bevy ECS systems that bridge domain layer logic to the game engine.
//! All systems call pure functions from the domain layer.

use bevy::ecs::message::MessageReader;
use bevy::ecs::message::MessageWriter;
use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::domain::loot::drop_table::{calculate_loot_drops, LootTable, LootTableEntry};
use crate::domain::loot::inventory::{
    can_add_item, find_empty_slot, find_stackable_slot, Inventory,
};
use crate::domain::loot::pickup::is_within_pickup_range;
use crate::domain::loot::stacking::calculate_stack_result;
use crate::infrastructure::components::enemy::{Enemy, EnemyType};
use crate::infrastructure::assets::ron_loader::ItemDefinitionAssetContainer;
use crate::infrastructure::components::loot::{
    InventoryComponent, InventorySlot, ItemDefinitionAsset, ItemId, LootTableAsset, WorldItem,
};
use crate::infrastructure::components::player::Player;
use crate::infrastructure::events::combat::EnemyDefeated;
use crate::infrastructure::events::loot::{InventoryFull, ItemDropped, ItemPickedUp, ItemStacked};
use crate::infrastructure::resources::loot::{InventorySlotUIData, InventoryUIData};
use crate::infrastructure::resources::loot::{PickupConfig, PickupMode};

/// 加载掉落表系统
///
/// 在游戏启动时加载掉落表配置
pub fn load_loot_tables_system(asset_server: Res<AssetServer>) {
    // Load loot tables from RON files
    // For now, load the default goblin loot table
    // This can be expanded to load multiple tables
    let loot_table_path = "data/loot_tables.ron";
    let _handle: Handle<LootTableAsset> = asset_server.load(loot_table_path);
    
    // Note: Asset loading is async, so the asset may not be available immediately
    // The loot_drop_system will handle cases where assets are not yet loaded
    info!("Loading loot tables from: {}", loot_table_path);
}

/// 加载物品定义系统
///
/// 在游戏启动时加载物品定义配置
pub fn load_item_definitions_system(asset_server: Res<AssetServer>) {
    // Load item definitions from RON files
    let items_path = "data/items.ron";
    let _handle: Handle<ItemDefinitionAssetContainer> = asset_server.load(items_path);
    
    // Note: Asset loading is async, so the asset may not be available immediately
    // The inventory_ui_system will handle cases where assets are not yet loaded
    info!("Loading item definitions from: {}", items_path);
}

/// 根据敌人类型查找掉落表 ID
///
/// 将敌人类型映射到对应的掉落表 ID
fn get_loot_table_id_for_enemy_type(enemy_type: EnemyType) -> &'static str {
    match enemy_type {
        EnemyType::Slime => "slime", // Match the loot table ID in loot_tables.ron
        // Add more enemy types as they are implemented
    }
}

/// 验证掉落位置
///
/// 确保物品掉落在可达区域。如果位置无效，返回最近的有效位置。
/// 当前实现：直接返回原位置（未来可以添加碰撞检测）
fn validate_drop_position(position: Vec2) -> Vec3 {
    // TODO: Add collision detection to find nearest valid position
    // For now, items drop at enemy death position
    // This can be enhanced to check for walls, platforms, etc.
    // Z coordinate set to 0.5 to ensure items appear above ground but below player
    position.extend(0.5)
}

/// 掉落系统
///
/// 监听 EnemyDefeated 事件，计算掉落物并生成 WorldItem 实体
pub fn loot_drop_system(
    mut commands: Commands,
    mut enemy_defeated_events: MessageReader<EnemyDefeated>,
    enemy_query: Query<&EnemyType, (With<Enemy>, Without<WorldItem>)>,
    loot_tables: Res<Assets<LootTableAsset>>,
    mut item_dropped_events: MessageWriter<ItemDropped>,
) {
    for event in enemy_defeated_events.read() {
        // Get enemy type from enemy entity
        let enemy_type = enemy_query.get(event.enemy).ok().copied().unwrap_or(EnemyType::Slime); // Default to Slime if not found

        // Look up loot table ID based on enemy type
        let loot_table_id = get_loot_table_id_for_enemy_type(enemy_type);

        // Find loot table by ID
        let Some(loot_table_asset) = loot_tables
            .iter()
            .find(|(_, asset)| asset.id == loot_table_id)
            .map(|(_, asset)| asset)
        else {
            // Asset not loaded yet or not found - log and skip this drop
            warn!("Loot table '{}' not found for enemy type {:?}. Using fallback drops.", loot_table_id, enemy_type);
            // Fallback: spawn a default item drop even if asset is not loaded
            let fallback_drop = commands.spawn((
                WorldItem { item_id: 1, quantity: 5, picked_up: false }, // Default: 5 gold coins
                Transform::from_translation(validate_drop_position(event.position)),
                Sprite {
                    color: Color::srgb(1.0, 0.84, 0.0), // Yellow for gold
                    custom_size: Some(Vec2::new(32.0, 32.0)),
                    ..default()
                },
                crate::infrastructure::systems::pixel_snap::PixelSnap,
            )).id();
            item_dropped_events.write(ItemDropped {
                world_item_entity: fallback_drop,
                item_id: 1,
                quantity: 5,
                position: event.position,
            });
            continue;
        };

        // Convert asset to domain LootTable
        let loot_table = LootTable {
            id: loot_table_asset.id.clone(),
            entries: loot_table_asset
                .entries
                .iter()
                .map(|e| LootTableEntry {
                    item_id: e.item_id,
                    chance: e.chance,
                    quantity_min: e.quantity_min,
                    quantity_max: e.quantity_max,
                })
                .collect(),
        };

        // Calculate drops using domain layer
        let mut rng = thread_rng();
        let drops = calculate_loot_drops(&loot_table, &mut rng);

        // Spawn WorldItem entities for each drop
        for drop in drops {
            // Validate drop position: ensure item doesn't spawn in unreachable areas
            // Add small random offset to prevent items from stacking exactly on top of each other
            let mut offset_rng = thread_rng();
            let offset_x = offset_rng.gen_range(-8.0..8.0);
            let offset_y = offset_rng.gen_range(-8.0..8.0);
            let drop_pos_2d = event.position + Vec2::new(offset_x, offset_y);
            let drop_position = validate_drop_position(drop_pos_2d);

            // Create visual representation for dropped item
            // For now, use a colored sprite as placeholder
            // TODO: Load actual item sprite from ItemDefinitionAsset when available
            let sprite_color = match drop.item_id {
                1 => Color::srgb(1.0, 0.84, 0.0), // Gold coin - yellow
                2 => Color::srgb(1.0, 0.0, 0.0),  // Health potion - red
                _ => Color::srgb(0.5, 0.5, 0.5),  // Default - gray
            };

            // Create visual representation for dropped item
            // Use a larger, more visible sprite with pixel snap
            // Z coordinate set to 0.5 to ensure items appear above ground but below player
            let mut entity_commands = commands.spawn((
                WorldItem { item_id: drop.item_id, quantity: drop.quantity, picked_up: false },
                Transform::from_translation(drop_position), // Z = 0.5 for visibility (set in validate_drop_position)
                // Visual representation: 32x32 pixel sprite (more visible)
                Sprite {
                    color: sprite_color,
                    custom_size: Some(Vec2::new(32.0, 32.0)),
                    ..default()
                },
                // Add pixel snap for pixel-perfect rendering
                crate::infrastructure::systems::pixel_snap::PixelSnap,
            ));

            // Add quantity label if > 1 (as a child entity)
            if drop.quantity > 1 {
                entity_commands.with_children(|parent| {
                    parent.spawn((
                        Text::new(format!("×{}", drop.quantity)),
                        TextFont { font_size: 12.0, ..default() },
                        TextColor(Color::srgb(1.0, 1.0, 1.0)),
                        Transform::from_translation(Vec3::new(8.0, -8.0, 0.1)),
                    ));
                });
            }

            let world_item_entity = entity_commands.id();

            // Emit ItemDropped event
            item_dropped_events.write(ItemDropped {
                world_item_entity,
                item_id: drop.item_id,
                quantity: drop.quantity,
                position: drop_pos_2d,
            });
        }
    }
}

/// 拾取范围检测系统（节流检查）
///
/// 每 3-5 帧检查一次玩家是否在拾取范围内，减少计算成本
pub fn pickup_range_check_system(
    time: Res<Time>,
    mut last_check: Local<f32>,
    world_item_query: Query<(Entity, &Transform, &WorldItem), (With<WorldItem>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<WorldItem>)>,
    _pickup_config: Res<PickupConfig>,
) {
    // Throttle: Check every 3-5 frames (at 60 FPS, that's ~0.05-0.083 seconds)
    let check_interval = 0.05; // ~3 frames at 60 FPS
    let current_time = time.elapsed_secs();

    if current_time - *last_check < check_interval {
        return;
    }
    *last_check = current_time;

    let Some(player_transform) = player_query.iter().next() else {
        return;
    };
    let _player_pos = player_transform.translation.truncate(); // Used for future visual indicators

        // Check each world item
        // Note: This system is throttled to reduce performance cost
        // Actual pickup logic is handled by manual_pickup_system and automatic_pickup_system
        for (_entity, _item_transform, _world_item) in world_item_query.iter() {
            // Range checking is done in pickup systems
            // This system can be expanded to add visual indicators or other effects
        }
}

/// 手动拾取系统
///
/// 监听拾取按键输入，处理手动拾取
/// 支持并发拾取处理：多个玩家同时拾取同一物品时，只有第一个成功
pub fn manual_pickup_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    pickup_config: Res<PickupConfig>,
    mut world_item_query: Query<(Entity, &Transform, &mut WorldItem), (With<WorldItem>, Without<Player>)>,
    player_query: Query<(Entity, &Transform), (With<Player>, Without<WorldItem>)>,
    mut item_picked_up_events: MessageWriter<ItemPickedUp>,
    mut commands: Commands,
) {
    // Only process if in manual mode
    if pickup_config.mode != PickupMode::Manual {
        return;
    }

    // Check if pickup key was pressed
    // IMPORTANT: This system must run before door_interaction_system to prioritize item pickup
    if !keyboard.just_pressed(pickup_config.pickup_key) {
        return;
    }
    
    info!("Pickup key (E) pressed - checking for nearby items...");

    let Some((player_entity, player_transform)) = player_query.iter().next() else {
        return;
    };
    let player_pos = player_transform.translation.truncate();

    // Find nearest item in range
    let mut nearest_item: Option<(Entity, ItemId, u32, Vec2)> = None;
    let mut nearest_distance = f32::MAX;

    // First pass: find nearest item (read-only check)
    for (item_entity, item_transform, world_item) in world_item_query.iter() {
        if world_item.picked_up {
            continue; // Already picked up
        }

        let item_pos = item_transform.translation.truncate();
        let distance = item_pos.distance(player_pos);

        // Check if in range (convert range from meters to pixels: 1 meter = 16 pixels)
        let range_pixels = pickup_config.range * 16.0;
        
        if distance <= range_pixels && distance < nearest_distance
        {
            nearest_item = Some((item_entity, world_item.item_id, world_item.quantity, item_pos));
            nearest_distance = distance;
        }
    }

    // Pick up nearest item (with concurrent pickup protection)
    if let Some((item_entity, item_id, quantity, _position)) = nearest_item {
        info!("Found item to pick up: item_id={}, quantity={}, distance={:.2}", item_id, quantity, nearest_distance);
        // Atomic check-and-mark: only one player can successfully pick up
        // Use get_mut to get mutable access for atomic check-and-set
        let item_query = world_item_query.get_mut(item_entity);
        if let Ok((_, _, mut world_item)) = item_query {
            if world_item.picked_up {
                // Another player already picked it up (race condition protection)
                return;
            }
            // Mark as picked up atomically
            world_item.picked_up = true;
        } else {
            return;
        }

        // Emit ItemPickedUp event (inventory system will handle adding to inventory)
        item_picked_up_events.write(ItemPickedUp { player_entity, item_id, quantity });

        // Despawn world item entity
        commands.entity(item_entity).despawn();
        info!("Item picked up successfully and despawned");
        // Item was picked up, so we don't want door interaction to trigger
        // The key press is consumed by this system, so door_interaction_system won't see it
        return;
    } else {
        info!("No items found in pickup range (range: {:.2} meters)", pickup_config.range);
        // No item found, so door interaction can proceed (if door_interaction_system runs after this)
    }
}

/// 自动拾取系统
///
/// 自动拾取范围内的物品
/// 支持并发拾取处理：多个玩家同时拾取同一物品时，只有第一个成功
pub fn automatic_pickup_system(
    pickup_config: Res<PickupConfig>,
    mut world_item_query: Query<(Entity, &Transform, &mut WorldItem), (With<WorldItem>, Without<Player>)>,
    player_query: Query<(Entity, &Transform), (With<Player>, Without<WorldItem>)>,
    mut item_picked_up_events: MessageWriter<ItemPickedUp>,
    mut commands: Commands,
) {
    // Only process if in automatic mode
    if pickup_config.mode != PickupMode::Automatic {
        return;
    }

    let Some((player_entity, player_transform)) = player_query.iter().next() else {
        return;
    };
    let player_pos = player_transform.translation.truncate();

    // Pick up all items in range
    for (item_entity, item_transform, mut world_item) in world_item_query.iter_mut() {
        // Atomic check-and-mark: only one player can successfully pick up
        if world_item.picked_up {
            continue; // Already picked up by another player
        }

        let item_pos = item_transform.translation.truncate();
        let distance = item_pos.distance(player_pos);

        // Check if in range (convert range from meters to pixels: 1 meter = 16 pixels)
        let range_pixels = pickup_config.range * 16.0;
        
        if distance <= range_pixels {
            // Mark as picked up atomically (double-check after range check)
            if world_item.picked_up {
                // Another player already picked it up (race condition protection)
                continue;
            }
            world_item.picked_up = true;

            // Emit ItemPickedUp event
            item_picked_up_events.write(ItemPickedUp {
                player_entity,
                item_id: world_item.item_id,
                quantity: world_item.quantity,
            });

            // Despawn world item entity
            commands.entity(item_entity).despawn();
        }
    }
}

/// 库存管理系统
///
/// 处理 ItemPickedUp 事件，将物品添加到玩家库存
pub fn inventory_management_system(
    mut item_picked_up_events: MessageReader<ItemPickedUp>,
    mut inventory_query: Query<&mut InventoryComponent, With<Player>>,
    item_definitions: Res<Assets<ItemDefinitionAssetContainer>>,
    mut inventory_full_events: MessageWriter<InventoryFull>,
    mut item_stacked_events: MessageWriter<ItemStacked>,
) {
    for event in item_picked_up_events.read() {
        let Ok(mut inventory) = inventory_query.get_mut(event.player_entity) else {
            continue;
        };

        // Get item definition (for stacking rules)
        let item_def = item_definitions
            .iter()
            .flat_map(|(_, container)| container.items.iter())
            .find(|def| def.id == event.item_id);
        let (_stackable, max_stack) =
            item_def.map(|def| (def.stackable, def.max_stack)).unwrap_or((true, 99)); // Default: stackable, max 99

        // Convert InventoryComponent to domain Inventory
        let mut domain_inventory = Inventory {
            slots: std::array::from_fn(|i| {
                inventory.slots[i]
                    .as_ref()
                    .map(|s| crate::domain::loot::inventory::InventorySlot {
                        item_id: s.item_id,
                        quantity: s.quantity,
                    })
            }),
        };

        // Check if we can add the item
        if !can_add_item(&domain_inventory, event.item_id, event.quantity) {
            // Inventory full
            inventory_full_events.write(InventoryFull {
                player_entity: event.player_entity,
                item_id: event.item_id,
            });
            continue;
        }

        // Try to find stackable slot first
        if let Some(slot_idx) = find_stackable_slot(&domain_inventory, event.item_id) {
            let current_slot = &mut domain_inventory.slots[slot_idx].as_mut().unwrap();

            // Calculate stack result
            let stack_result =
                calculate_stack_result(current_slot.quantity, event.quantity, max_stack);

            match stack_result {
                crate::domain::loot::stacking::StackResult::Merge(total) => {
                    // Fully merged
                    current_slot.quantity = total;
                    inventory.slots[slot_idx].as_mut().unwrap().quantity = total;

                    item_stacked_events.write(ItemStacked {
                        player_entity: event.player_entity,
                        item_id: event.item_id,
                        total_quantity: total,
                        slot_index: slot_idx,
                    });
                },
                crate::domain::loot::stacking::StackResult::Split { remaining, overflow } => {
                    // Partially merged, need new slot for overflow
                    current_slot.quantity = remaining;
                    inventory.slots[slot_idx].as_mut().unwrap().quantity = remaining;

                    // Try to add overflow to new slot
                    if let Some(empty_slot_idx) = find_empty_slot(&domain_inventory) {
                        domain_inventory.slots[empty_slot_idx] =
                            Some(crate::domain::loot::inventory::InventorySlot {
                                item_id: event.item_id,
                                quantity: overflow,
                            });
                        inventory.slots[empty_slot_idx] =
                            Some(InventorySlot { item_id: event.item_id, quantity: overflow });
                    }
                },
            }
        } else {
            // No stackable slot, find empty slot
            if let Some(slot_idx) = find_empty_slot(&domain_inventory) {
                domain_inventory.slots[slot_idx] =
                    Some(crate::domain::loot::inventory::InventorySlot {
                        item_id: event.item_id,
                        quantity: event.quantity,
                    });
                inventory.slots[slot_idx] =
                    Some(InventorySlot { item_id: event.item_id, quantity: event.quantity });
            }
        }
    }
}

/// 库存 UI 数据系统
///
/// 查询 InventoryComponent 并转换为 UI 数据格式
/// 供 UI 系统消费以显示库存内容
pub fn inventory_ui_system(
    inventory_query: Query<&InventoryComponent, With<Player>>,
    item_definitions: Res<Assets<ItemDefinitionAssetContainer>>,
    mut ui_data: ResMut<InventoryUIData>,
) {
    let Some(inventory) = inventory_query.iter().next() else {
        // No player inventory found, clear UI data
        ui_data.slots.clear();
        return;
    };

    // Convert InventoryComponent to UI data format
    ui_data.slots = inventory
        .slots
        .iter()
        .map(|slot_opt| {
            if let Some(slot) = slot_opt {
                // Get item definition for name and icon
                let item_def = item_definitions
                    .iter()
                    .flat_map(|(_, container)| container.items.iter())
                    .find(|def| def.id == slot.item_id);

                InventorySlotUIData {
                    item_id: Some(slot.item_id),
                    quantity: slot.quantity,
                    item_name: item_def
                        .map(|def| def.name.clone())
                        .unwrap_or_else(|| format!("Item #{}", slot.item_id)),
                    icon_path: item_def
                        .map(|def| def.icon_path.clone())
                        .unwrap_or_else(|| "sprites/items/unknown.png".to_string()),
                }
            } else {
                // Empty slot
                InventorySlotUIData {
                    item_id: None,
                    quantity: 0,
                    item_name: String::new(),
                    icon_path: String::new(),
                }
            }
        })
        .collect();
}
