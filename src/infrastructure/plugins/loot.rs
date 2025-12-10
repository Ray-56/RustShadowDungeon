//! Loot and inventory plugin
//!
//! 战利品与库存插件
//!
//! Registers all loot and inventory systems, components, events, and resources.

use bevy::prelude::*;

use crate::infrastructure::assets::ron_loader::{
    ItemDefinitionAssetContainer, ItemDefinitionAssetLoader, LootTableAssetLoader,
};
use crate::infrastructure::components::loot::{ItemDefinitionAsset, LootTableAsset};
use crate::infrastructure::events::loot::{InventoryFull, ItemDropped, ItemPickedUp, ItemStacked};
use crate::infrastructure::resources::loot::{InventoryUIData, LootConfig, PickupConfig};
use crate::infrastructure::systems::loot::{
    automatic_pickup_system, inventory_management_system, inventory_ui_system,
    load_item_definitions_system, load_loot_tables_system, loot_drop_system, manual_pickup_system,
    pickup_range_check_system,
};
use crate::infrastructure::systems::ui::inventory::{
    toggle_inventory_ui_system, update_inventory_ui_system, InventoryUIState,
};

/// Loot and inventory plugin
///
/// 战利品与库存插件
pub struct LootInventoryPlugin;

impl Plugin for LootInventoryPlugin {
    fn build(&self, app: &mut App) {
        // Register asset types and loaders
        // In Bevy 0.17, asset types must be registered using init_asset before they can be loaded
        // init_asset automatically creates the Assets<T> resource, so we don't need to init_resource separately
        app.init_asset::<LootTableAsset>()
            .init_asset::<ItemDefinitionAssetContainer>()
            .register_asset_loader(LootTableAssetLoader::default())
            .register_asset_loader(ItemDefinitionAssetLoader::default());

        // Register events (messages)
        app.add_message::<ItemDropped>()
            .add_message::<ItemPickedUp>()
            .add_message::<InventoryFull>()
            .add_message::<ItemStacked>();

        // Register resources
        app.init_resource::<LootConfig>()
            .init_resource::<PickupConfig>()
            .init_resource::<InventoryUIData>()
            .init_resource::<InventoryUIState>();

        // Register systems
        // Load assets at startup
        app.add_systems(Startup, (load_loot_tables_system, load_item_definitions_system));

        // Loot and pickup systems run in Update
        app.add_systems(
            Update,
            (
                // Loot drop system (runs when enemy dies)
                loot_drop_system,
                // Pickup range check (throttled, every 3-5 frames)
                pickup_range_check_system,
                // Manual pickup (runs when pickup key is pressed)
                manual_pickup_system,
                // Automatic pickup (runs every frame if in auto mode)
                automatic_pickup_system,
                // Inventory management (processes ItemPickedUp events)
                inventory_management_system
                    .after(manual_pickup_system)
                    .after(automatic_pickup_system),
                // Inventory UI data system (updates UI data for display)
                inventory_ui_system.after(inventory_management_system),
                // Inventory UI toggle and update
                toggle_inventory_ui_system,
                update_inventory_ui_system.after(inventory_ui_system),
            ),
        );
    }
}
