//! Inventory UI system
//!
//! 库存界面系统
//!
//! Handles opening/closing inventory screen and rendering inventory UI.

use bevy::prelude::*;
use crate::infrastructure::resources::loot::InventoryUIData;

/// Marker component for inventory UI root node
#[derive(Component)]
pub struct InventoryUIRoot;

/// Marker component for inventory slot UI elements
#[derive(Component)]
pub struct InventorySlotUI {
    pub slot_index: usize,
}

/// Resource to track inventory UI state
#[derive(Resource, Default)]
pub struct InventoryUIState {
    pub is_open: bool,
}

/// Toggle inventory UI system
///
/// 切换库存界面显示/隐藏
/// 默认按键：Tab 键
pub fn toggle_inventory_ui_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut ui_state: ResMut<InventoryUIState>,
    mut commands: Commands,
    ui_root_query: Query<Entity, With<InventoryUIRoot>>,
    inventory_ui_data: Res<InventoryUIData>,
) {
    // Toggle on Tab key press
    if keyboard.just_pressed(KeyCode::Tab) {
        ui_state.is_open = !ui_state.is_open;

        if ui_state.is_open {
            // Open inventory UI
            spawn_inventory_ui(&mut commands, &inventory_ui_data);
        } else {
            // Close inventory UI
            if let Some(root_entity) = ui_root_query.iter().next() {
                commands.entity(root_entity).despawn();
            }
        }
    }
}

/// Spawn inventory UI
///
/// 创建库存界面 UI 元素
fn spawn_inventory_ui(commands: &mut Commands, inventory_data: &InventoryUIData) {
    // Create root panel
    let root_entity = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            InventoryUIRoot,
        ))
        .id();

    // Create inventory panel (centered) with border
    // Outer container for border effect
    let panel_outer = commands
        .spawn(Node {
            width: Val::Px(420.0),
            height: Val::Px(520.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        })
        .id();

    // Inner panel with background
    let panel_entity = commands
        .spawn(Node {
            width: Val::Px(400.0),
            height: Val::Px(500.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(20.0)),
            // Create border effect using margin (simulating border)
            margin: UiRect::all(Val::Px(10.0)),
            ..default()
        })
        .id();

    commands.entity(panel_outer).add_child(panel_entity);
    commands.entity(root_entity).add_child(panel_outer);

    // Title with better styling (use English to avoid font issues)
    let title_entity = commands
        .spawn((
            Text::new("Inventory"),
            TextFont { font_size: 24.0, ..default() },
            TextColor(Color::srgb(1.0, 1.0, 1.0)),
            Node {
                margin: UiRect::bottom(Val::Px(20.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .id();
    commands.entity(panel_entity).add_child(title_entity);

    // Create grid container for inventory slots (6 columns x 5 rows = 30 slots)
    let grid_entity = commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Row,
            flex_wrap: FlexWrap::Wrap,
            ..default()
        })
        .id();

    commands.entity(panel_entity).add_child(grid_entity);

    // Create 30 inventory slots
    for slot_idx in 0..30 {
        let slot_data = inventory_data.slots.get(slot_idx);

        // Create slot with border effect
        let slot_outer = commands
            .spawn(Node {
                width: Val::Px(64.0),
                height: Val::Px(64.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(2.0)),
                ..default()
            })
            .id();

        let slot_entity = commands
            .spawn((
                Node {
                    width: Val::Px(60.0),
                    height: Val::Px(60.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    // Border effect using margin (simulating 2px border)
                    margin: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                InventorySlotUI { slot_index: slot_idx },
            ))
            .id();

        commands.entity(slot_outer).add_child(slot_entity);

        // Add item info if slot has item
        if let Some(slot) = slot_data {
            if slot.item_id.is_some() {
                // Create a container for item display
                let item_container = commands
                    .spawn(Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(4.0)),
                        ..default()
                    })
                    .id();

                // Item name (truncate if too long)
                let display_name = if slot.item_name.len() > 8 {
                    format!("{}...", &slot.item_name[..8])
                } else {
                    slot.item_name.clone()
                };

                let name_text = commands
                    .spawn((
                        Text::new(display_name),
                        TextFont { font_size: 10.0, ..default() },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    ))
                    .id();
                commands.entity(item_container).add_child(name_text);

                // Quantity display (if > 1)
                if slot.quantity > 1 {
                    let qty_text = commands
                        .spawn((
                            Text::new(format!("×{}", slot.quantity)),
                            TextFont { font_size: 12.0, ..default() },
                            TextColor(Color::srgb(1.0, 0.9, 0.0)), // Gold color for quantity
                        ))
                        .id();
                    commands.entity(item_container).add_child(qty_text);
                }

                commands.entity(slot_entity).add_child(item_container);
            }
        }

        commands.entity(grid_entity).add_child(slot_outer);
    }

    // Stats summary (use English to avoid font issues)
    let filled_slots = inventory_data.slots.iter().filter(|s| s.item_id.is_some()).count();
    let stats_text = format!("Used: {}/30 slots", filled_slots);
    let stats_entity = commands
        .spawn((
            Text::new(stats_text),
            TextFont { font_size: 14.0, ..default() },
            TextColor(Color::srgb(0.7, 0.7, 0.7)),
            Node {
                margin: UiRect::top(Val::Px(10.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .id();
    commands.entity(panel_entity).add_child(stats_entity);

    // Close hint (use English to avoid font issues)
    let hint_entity = commands
        .spawn((
            Text::new("Press Tab to close"),
            TextFont { font_size: 12.0, ..default() },
            TextColor(Color::srgba(0.6, 0.6, 0.6, 0.8)),
            Node {
                margin: UiRect::top(Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .id();
    commands.entity(panel_entity).add_child(hint_entity);
}

/// Update inventory UI when inventory data changes
///
/// 当库存数据变化时更新 UI
pub fn update_inventory_ui_system(
    _inventory_data: Res<InventoryUIData>,
    _ui_state: Res<InventoryUIState>,
) {
    // Only update if UI is open
    if !_ui_state.is_open {
        return;
    }

    // This system would update slot visuals when inventory changes
    // For now, the UI is recreated on toggle, so this is a placeholder
    // In a full implementation, we'd update individual slots here
}
