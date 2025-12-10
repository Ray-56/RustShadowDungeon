//! Unit tests for pickup range detection
//!
//! 拾取范围检测单元测试

use rust_shadow_dungeon::domain::loot::pickup::is_within_pickup_range;

#[test]
fn test_is_within_pickup_range_exact_boundary() {
    // Test at exact range boundary
    let item_pos = (0.0, 0.0);
    let player_pos = (2.0, 0.0); // Exactly 2.0 units away
    let range = 2.0;
    
    assert!(is_within_pickup_range(item_pos, player_pos, range));
}

#[test]
fn test_is_within_pickup_range_inside() {
    // Test well inside range
    let item_pos = (0.0, 0.0);
    let player_pos = (1.0, 0.0); // 1.0 unit away
    let range = 2.0;
    
    assert!(is_within_pickup_range(item_pos, player_pos, range));
}

#[test]
fn test_is_within_pickup_range_outside() {
    // Test outside range
    let item_pos = (0.0, 0.0);
    let player_pos = (3.0, 0.0); // 3.0 units away
    let range = 2.0;
    
    assert!(!is_within_pickup_range(item_pos, player_pos, range));
}

#[test]
fn test_is_within_pickup_range_diagonal() {
    // Test diagonal distance (Pythagorean theorem)
    let item_pos = (0.0, 0.0);
    let player_pos = (1.0, 1.0); // sqrt(2) ≈ 1.414 units away
    let range = 2.0;
    
    assert!(is_within_pickup_range(item_pos, player_pos, range));
}

#[test]
fn test_is_within_pickup_range_diagonal_outside() {
    // Test diagonal distance outside range
    let item_pos = (0.0, 0.0);
    let player_pos = (2.0, 2.0); // sqrt(8) ≈ 2.828 units away
    let range = 2.0;
    
    assert!(!is_within_pickup_range(item_pos, player_pos, range));
}

#[test]
fn test_is_within_pickup_range_zero_distance() {
    // Test at same position
    let item_pos = (10.0, 20.0);
    let player_pos = (10.0, 20.0);
    let range = 2.0;
    
    assert!(is_within_pickup_range(item_pos, player_pos, range));
}

#[test]
fn test_is_within_pickup_range_negative_coordinates() {
    // Test with negative coordinates
    let item_pos = (-5.0, -3.0);
    let player_pos = (-4.0, -3.0); // 1.0 unit away
    let range = 2.0;
    
    assert!(is_within_pickup_range(item_pos, player_pos, range));
}

