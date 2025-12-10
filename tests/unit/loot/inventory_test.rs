//! Unit tests for inventory management
//!
//! 库存管理单元测试

use rust_shadow_dungeon::domain::loot::inventory::{
    add_item_to_slot, can_add_item, find_empty_slot, find_stackable_slot, Inventory, InventorySlot,
};

#[test]
fn test_find_empty_slot_empty_inventory() {
    let inventory = Inventory::default();
    let slot = find_empty_slot(&inventory);
    
    assert_eq!(slot, Some(0)); // First slot should be empty
}

#[test]
fn test_find_empty_slot_partially_filled() {
    let mut inventory = Inventory::default();
    inventory.slots[0] = Some(InventorySlot { item_id: 1, quantity: 5 });
    inventory.slots[1] = Some(InventorySlot { item_id: 2, quantity: 3 });
    
    let slot = find_empty_slot(&inventory);
    
    assert_eq!(slot, Some(2)); // Third slot should be empty
}

#[test]
fn test_find_empty_slot_full_inventory() {
    let mut inventory = Inventory::default();
    for i in 0..30 {
        inventory.slots[i] = Some(InventorySlot { item_id: 1, quantity: 1 });
    }
    
    let slot = find_empty_slot(&inventory);
    
    assert_eq!(slot, None); // No empty slots
}

#[test]
fn test_find_stackable_slot_exists() {
    let mut inventory = Inventory::default();
    inventory.slots[0] = Some(InventorySlot { item_id: 1, quantity: 5 });
    inventory.slots[1] = Some(InventorySlot { item_id: 2, quantity: 3 });
    
    let slot = find_stackable_slot(&inventory, 1);
    
    assert_eq!(slot, Some(0)); // Slot 0 has item_id 1
}

#[test]
fn test_find_stackable_slot_not_exists() {
    let mut inventory = Inventory::default();
    inventory.slots[0] = Some(InventorySlot { item_id: 1, quantity: 5 });
    
    let slot = find_stackable_slot(&inventory, 2);
    
    assert_eq!(slot, None); // No slot with item_id 2
}

#[test]
fn test_find_stackable_slot_empty_inventory() {
    let inventory = Inventory::default();
    let slot = find_stackable_slot(&inventory, 1);
    
    assert_eq!(slot, None); // No items in inventory
}

#[test]
fn test_can_add_item_empty_inventory() {
    let inventory = Inventory::default();
    
    assert!(can_add_item(&inventory, 1, 5));
}

#[test]
fn test_can_add_item_stackable_slot_exists() {
    let mut inventory = Inventory::default();
    inventory.slots[0] = Some(InventorySlot { item_id: 1, quantity: 5 });
    
    // Can add more of item_id 1 (will stack)
    assert!(can_add_item(&inventory, 1, 10));
}

#[test]
fn test_can_add_item_full_inventory() {
    let mut inventory = Inventory::default();
    for i in 0..30 {
        inventory.slots[i] = Some(InventorySlot { item_id: i as u32 + 1, quantity: 1 });
    }
    
    // Cannot add new item (inventory full)
    assert!(!can_add_item(&inventory, 100, 1));
}

#[test]
fn test_can_add_item_stackable_slot_available() {
    let mut inventory = Inventory::default();
    inventory.slots[0] = Some(InventorySlot { item_id: 1, quantity: 5 });
    // Slot 1 is empty, but item_id 1 can stack in slot 0
    
    assert!(can_add_item(&inventory, 1, 10)); // Can stack
}

#[test]
fn test_add_item_to_slot_empty_slot() {
    let mut inventory = Inventory::default();
    
    let result = add_item_to_slot(&mut inventory, 0, 1, 5);
    
    assert!(result.is_ok());
    assert_eq!(inventory.slots[0], Some(InventorySlot { item_id: 1, quantity: 5 }));
}

#[test]
fn test_add_item_to_slot_invalid_index() {
    let mut inventory = Inventory::default();
    
    let result = add_item_to_slot(&mut inventory, 30, 1, 5); // Index 30 is out of bounds
    
    assert!(result.is_err());
    if let Err(err) = result {
        assert!(matches!(err, rust_shadow_dungeon::domain::loot::inventory::InventoryError::InvalidSlot(30)));
    }
}

#[test]
fn test_add_item_to_slot_occupied_slot() {
    let mut inventory = Inventory::default();
    inventory.slots[0] = Some(InventorySlot { item_id: 1, quantity: 5 });
    
    // Try to add different item to occupied slot
    let result = add_item_to_slot(&mut inventory, 0, 2, 3);
    
    assert!(result.is_err());
    if let Err(err) = result {
        assert!(matches!(err, rust_shadow_dungeon::domain::loot::inventory::InventoryError::StackingFailed));
    }
}

#[test]
fn test_add_item_to_slot_same_item_stack() {
    let mut inventory = Inventory::default();
    inventory.slots[0] = Some(InventorySlot { item_id: 1, quantity: 5 });
    
    // Try to add same item to same slot (should stack)
    // The current implementation supports stacking in add_item_to_slot
    let result = add_item_to_slot(&mut inventory, 0, 1, 3);
    
    // Should succeed and stack the items
    assert!(result.is_ok());
    assert_eq!(inventory.slots[0].as_ref().unwrap().quantity, 8); // 5 + 3
}

