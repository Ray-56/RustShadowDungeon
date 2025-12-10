//! Unit tests for loot drop table calculation
//!
//! 掉落表计算单元测试

use rust_shadow_dungeon::domain::loot::drop_table::{
    calculate_item_quantity, calculate_loot_drops, LootTable, LootTableEntry,
};
use rand::thread_rng;

#[test]
fn test_calculate_loot_drops_single_item() {
    let loot_table = LootTable {
        id: "test".to_string(),
        entries: vec![LootTableEntry {
            item_id: 1,
            chance: 1.0, // 100% chance
            quantity_min: 5,
            quantity_max: 10,
        }],
    };

    let mut rng = thread_rng();
    let drops = calculate_loot_drops(&loot_table, &mut rng);

    // Should always drop since chance is 1.0
    assert_eq!(drops.len(), 1);
    assert_eq!(drops[0].item_id, 1);
    assert!(drops[0].quantity >= 5 && drops[0].quantity <= 10);
}

#[test]
fn test_calculate_loot_drops_zero_chance() {
    let loot_table = LootTable {
        id: "test".to_string(),
        entries: vec![LootTableEntry {
            item_id: 1,
            chance: 0.0, // 0% chance
            quantity_min: 5,
            quantity_max: 10,
        }],
    };

    let mut rng = thread_rng();
    let drops = calculate_loot_drops(&loot_table, &mut rng);

    // Should never drop
    assert_eq!(drops.len(), 0);
}

#[test]
fn test_calculate_loot_drops_multiple_items() {
    let loot_table = LootTable {
        id: "test".to_string(),
        entries: vec![
            LootTableEntry {
                item_id: 1,
                chance: 1.0, // 100% chance
                quantity_min: 1,
                quantity_max: 1,
            },
            LootTableEntry {
                item_id: 2,
                chance: 1.0, // 100% chance
                quantity_min: 1,
                quantity_max: 1,
            },
        ],
    };

    let mut rng = thread_rng();
    let drops = calculate_loot_drops(&loot_table, &mut rng);

    // Both items should drop (independent probability)
    assert_eq!(drops.len(), 2);
    assert!(drops.iter().any(|d| d.item_id == 1));
    assert!(drops.iter().any(|d| d.item_id == 2));
}

#[test]
fn test_calculate_item_quantity() {
    let mut rng = thread_rng();
    
    // Test multiple times to ensure range is respected
    for _ in 0..100 {
        let qty = calculate_item_quantity(5, 10, &mut rng);
        assert!(qty >= 5 && qty <= 10);
    }
}

#[test]
fn test_calculate_item_quantity_single_value() {
    let mut rng = thread_rng();
    let qty = calculate_item_quantity(5, 5, &mut rng);
    assert_eq!(qty, 5);
}

