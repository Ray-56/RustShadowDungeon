//! Unit tests for item stacking logic
//!
//! 物品堆叠逻辑单元测试

use rust_shadow_dungeon::domain::loot::stacking::{
    calculate_stack_result, can_stack_items, split_stack, StackResult,
};

#[test]
fn test_can_stack_items_same_id_stackable() {
    // Same item ID, both stackable
    assert!(can_stack_items(1, 1, true, true));
}

#[test]
fn test_can_stack_items_different_id() {
    // Different item IDs
    assert!(!can_stack_items(1, 2, true, true));
}

#[test]
fn test_can_stack_items_not_stackable() {
    // Same item ID but not stackable
    assert!(!can_stack_items(1, 1, false, true));
    assert!(!can_stack_items(1, 1, true, false));
    assert!(!can_stack_items(1, 1, false, false));
}

#[test]
fn test_calculate_stack_result_merge() {
    // Total quantity fits within max_stack
    let result = calculate_stack_result(5, 3, 99);
    
    assert!(matches!(result, StackResult::Merge(8)));
    if let StackResult::Merge(total) = result {
        assert_eq!(total, 8);
    }
}

#[test]
fn test_calculate_stack_result_exact_max() {
    // Total quantity equals max_stack
    let result = calculate_stack_result(50, 49, 99);
    
    assert!(matches!(result, StackResult::Merge(99)));
}

#[test]
fn test_calculate_stack_result_split() {
    // Total quantity exceeds max_stack
    let result = calculate_stack_result(50, 50, 99);
    
    assert!(matches!(result, StackResult::Split { remaining: 99, overflow: 1 }));
    if let StackResult::Split { remaining, overflow } = result {
        assert_eq!(remaining, 99);
        assert_eq!(overflow, 1);
    }
}

#[test]
fn test_calculate_stack_result_large_overflow() {
    // Large overflow case
    let result = calculate_stack_result(80, 30, 99);
    
    assert!(matches!(result, StackResult::Split { remaining: 99, overflow: 11 }));
    if let StackResult::Split { remaining, overflow } = result {
        assert_eq!(remaining, 99);
        assert_eq!(overflow, 11);
    }
}

#[test]
fn test_calculate_stack_result_zero_current() {
    // Starting from zero
    let result = calculate_stack_result(0, 10, 99);
    
    assert!(matches!(result, StackResult::Merge(10)));
}

#[test]
fn test_calculate_stack_result_zero_add() {
    // Adding zero quantity
    let result = calculate_stack_result(5, 0, 99);
    
    assert!(matches!(result, StackResult::Merge(5)));
}

#[test]
fn test_split_stack_below_max() {
    // Quantity below max_stack
    let (filled, remaining) = split_stack(50, 99);
    
    assert_eq!(filled, 50);
    assert_eq!(remaining, 0);
}

#[test]
fn test_split_stack_at_max() {
    // Quantity exactly at max_stack
    let (filled, remaining) = split_stack(99, 99);
    
    assert_eq!(filled, 99);
    assert_eq!(remaining, 0);
}

#[test]
fn test_split_stack_above_max() {
    // Quantity above max_stack
    let (filled, remaining) = split_stack(150, 99);
    
    assert_eq!(filled, 99);
    assert_eq!(remaining, 51);
}

#[test]
fn test_split_stack_zero() {
    // Zero quantity
    let (filled, remaining) = split_stack(0, 99);
    
    assert_eq!(filled, 0);
    assert_eq!(remaining, 0);
}

