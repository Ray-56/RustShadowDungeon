//! Integration tests for loot drop workflow
//!
//! 掉落流程集成测试
//!
//! Tests the complete workflow: EnemyDefeated → ItemDropped → WorldItem spawned

use bevy::ecs::message::MessageWriter;
use bevy::prelude::*;
use rust_shadow_dungeon::infrastructure::components::enemy::{Enemy, EnemyType};
use rust_shadow_dungeon::infrastructure::components::loot::WorldItem;
use rust_shadow_dungeon::infrastructure::events::combat::EnemyDefeated;
use rust_shadow_dungeon::infrastructure::{
    components::loot::LootTableAsset, plugins::loot::LootInventoryPlugin,
};
use std::collections::VecDeque;

// Resource to store messages to be sent in tests
#[derive(Resource, Default)]
struct TestEnemyDefeatedQueue {
    messages: VecDeque<EnemyDefeated>,
}

// System to send queued messages
fn send_queued_enemy_defeated_messages(
    mut queue: ResMut<TestEnemyDefeatedQueue>,
    mut writer: MessageWriter<EnemyDefeated>,
) {
    while let Some(msg) = queue.messages.pop_front() {
        writer.write(msg);
    }
}

#[test]
fn test_loot_drop_system_spawns_world_item() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(bevy::asset::AssetPlugin::default());
    app.add_plugins(bevy::input::InputPlugin); // Needed for ButtonInput resource
    // Manually add message type before plugin (plugin also adds it, but we need it for test system)
    app.add_message::<EnemyDefeated>();
    app.add_plugins(LootInventoryPlugin);
    
    // Add test message queue and system
    app.init_resource::<TestEnemyDefeatedQueue>();
    app.add_systems(Update, send_queued_enemy_defeated_messages);

    // Spawn an enemy
    let enemy_entity = app
        .world_mut()
        .spawn((
            Enemy,
            EnemyType::Slime,
            Transform::from_xyz(100.0, 50.0, 0.0),
        ))
        .id();

    // Manually add a loot table asset (simulating loaded asset)
    let loot_table_asset = LootTableAsset {
        id: "slime".to_string(),
        entries: vec![rust_shadow_dungeon::infrastructure::components::loot::LootTableEntryAsset {
            item_id: 1,
            chance: 1.0, // 100% chance for testing
            quantity_min: 5,
            quantity_max: 10,
        }],
    };

    let mut assets = app.world_mut().resource_mut::<bevy::asset::Assets<LootTableAsset>>();
    let _handle = assets.add(loot_table_asset);

    // Queue EnemyDefeated event
    {
        let mut queue = app.world_mut().resource_mut::<TestEnemyDefeatedQueue>();
        queue.messages.push_back(EnemyDefeated {
            enemy: enemy_entity,
            position: bevy::math::Vec2::new(100.0, 50.0),
            killed_by: None,
        });
    }

    // Run loot drop system
    app.update();

    // Check that WorldItem was spawned
    let mut world_items = Vec::new();
    let world = app.world_mut();
    for item in world.query::<&WorldItem>().iter(world) {
        world_items.push(item);
    }

    assert!(!world_items.is_empty(), "WorldItem should be spawned");
    assert_eq!(world_items[0].item_id, 1);
    assert!(world_items[0].quantity >= 5 && world_items[0].quantity <= 10);
}

#[test]
fn test_loot_drop_system_with_multiple_items() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(bevy::asset::AssetPlugin::default());
    app.add_plugins(bevy::input::InputPlugin); // Needed for ButtonInput resource
    // Manually add message type before plugin (plugin also adds it, but we need it for test system)
    app.add_message::<EnemyDefeated>();
    app.add_plugins(LootInventoryPlugin);
    
    // Add test message queue and system
    app.init_resource::<TestEnemyDefeatedQueue>();
    app.add_systems(Update, send_queued_enemy_defeated_messages);
    
    // Add test message queue and system
    app.init_resource::<TestEnemyDefeatedQueue>();
    app.add_systems(Update, send_queued_enemy_defeated_messages);

    let enemy_entity = app
        .world_mut()
        .spawn((
            Enemy,
            EnemyType::Slime,
            Transform::from_xyz(100.0, 50.0, 0.0),
        ))
        .id();

    // Create loot table with multiple items (both 100% chance)
    let loot_table_asset = LootTableAsset {
        id: "slime".to_string(),
        entries: vec![
            rust_shadow_dungeon::infrastructure::components::loot::LootTableEntryAsset {
                item_id: 1,
                chance: 1.0,
                quantity_min: 1,
                quantity_max: 1,
            },
            rust_shadow_dungeon::infrastructure::components::loot::LootTableEntryAsset {
                item_id: 2,
                chance: 1.0,
                quantity_min: 1,
                quantity_max: 1,
            },
        ],
    };

    let mut assets = app.world_mut().resource_mut::<bevy::asset::Assets<LootTableAsset>>();
    let _handle = assets.add(loot_table_asset);

    // Queue EnemyDefeated event
    {
        let mut queue = app.world_mut().resource_mut::<TestEnemyDefeatedQueue>();
        queue.messages.push_back(EnemyDefeated {
            enemy: enemy_entity,
            position: bevy::math::Vec2::new(100.0, 50.0),
            killed_by: None,
        });
    }

    app.update();

    // Check that multiple WorldItems were spawned
    let mut world_items = Vec::new();
    let world = app.world_mut();
    for item in world.query::<&WorldItem>().iter(world) {
        world_items.push(item);
    }

    assert_eq!(world_items.len(), 2, "Should spawn 2 items");
    assert!(world_items.iter().any(|w| w.item_id == 1));
    assert!(world_items.iter().any(|w| w.item_id == 2));
}

#[test]
fn test_loot_drop_system_fallback_when_asset_not_loaded() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(bevy::asset::AssetPlugin::default());
    app.add_plugins(bevy::input::InputPlugin); // Needed for ButtonInput resource
    // Manually add message type before plugin (plugin also adds it, but we need it for test system)
    app.add_message::<EnemyDefeated>();
    app.add_plugins(LootInventoryPlugin);
    
    // Add test message queue and system
    app.init_resource::<TestEnemyDefeatedQueue>();
    app.add_systems(Update, send_queued_enemy_defeated_messages);
    
    // Add test message queue and system
    app.init_resource::<TestEnemyDefeatedQueue>();
    app.add_systems(Update, send_queued_enemy_defeated_messages);

    let enemy_entity = app
        .world_mut()
        .spawn((
            Enemy,
            EnemyType::Slime,
            Transform::from_xyz(100.0, 50.0, 0.0),
        ))
        .id();

    // Don't add any loot table asset (simulating not loaded)

    // Queue EnemyDefeated event
    {
        let mut queue = app.world_mut().resource_mut::<TestEnemyDefeatedQueue>();
        queue.messages.push_back(EnemyDefeated {
            enemy: enemy_entity,
            position: bevy::math::Vec2::new(100.0, 50.0),
            killed_by: None,
        });
    }

    app.update();

    // Check that fallback item was spawned (5 gold coins)
    let mut world_items = Vec::new();
    let world = app.world_mut();
    for item in world.query::<&WorldItem>().iter(world) {
        world_items.push(item);
    }

    assert!(!world_items.is_empty(), "Fallback item should be spawned");
    assert_eq!(world_items[0].item_id, 1); // Default: gold coin
    assert_eq!(world_items[0].quantity, 5); // Default: 5 coins
}

