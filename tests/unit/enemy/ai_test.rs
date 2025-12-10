//! Unit tests for enemy AI domain logic
//!
//! Tests for pure Rust functions in src/domain/enemy/ai.rs
//! These tests have ZERO Bevy dependencies.

use rust_shadow_dungeon::domain::enemy::ai::*;
use bevy::math::Vec2;

#[test]
fn test_should_transition_to_chase_in_range() {
    assert!(should_transition_to_chase(100.0, 200.0, true));
}

#[test]
fn test_should_transition_to_chase_out_of_range() {
    assert!(!should_transition_to_chase(300.0, 200.0, true));
}

#[test]
fn test_should_transition_to_chase_no_los() {
    assert!(!should_transition_to_chase(100.0, 200.0, false));
}

#[test]
fn test_should_transition_to_attack_in_range() {
    assert!(should_transition_to_attack(30.0, 32.0, true));
}

#[test]
fn test_should_transition_to_attack_cooldown_not_ready() {
    assert!(!should_transition_to_attack(30.0, 32.0, false));
}

#[test]
fn test_should_drop_aggro_out_of_range() {
    assert!(should_drop_aggro(500.0, 400.0, 0.0, 5.0));
}

#[test]
fn test_should_drop_aggro_lost_sight() {
    assert!(should_drop_aggro(200.0, 400.0, 6.0, 5.0));
}

#[test]
fn test_calculate_chase_direction() {
    let enemy_pos = Vec2::new(0.0, 0.0);
    let target_pos = Vec2::new(100.0, 0.0);
    let direction = calculate_chase_direction(enemy_pos, target_pos);
    
    assert!((direction.x - 1.0).abs() < 0.01);
    assert!(direction.y.abs() < 0.01);
}

#[test]
fn test_calculate_patrol_target() {
    let spawn_pos = Vec2::new(0.0, 0.0);
    let patrol_radius = 100.0;
    
    let target = calculate_patrol_target(Vec2::ZERO, spawn_pos, patrol_radius);
    
    // Target should be within patrol radius
    let distance = spawn_pos.distance(target);
    assert!(distance <= patrol_radius);
}

#[test]
fn test_check_line_of_sight_no_obstacles() {
    let enemy_pos = Vec2::new(0.0, 0.0);
    let target_pos = Vec2::new(100.0, 0.0);
    let obstacles: Vec<(f32, f32, f32, f32)> = vec![];
    
    assert!(check_line_of_sight(enemy_pos, target_pos, &obstacles));
}

#[test]
fn test_check_line_of_sight_blocking_obstacle() {
    let enemy_pos = Vec2::new(0.0, 0.0);
    let target_pos = Vec2::new(100.0, 0.0);
    // Obstacle blocking the path (x: 50, y: -10, width: 20, height: 20)
    let blocking_obstacle = vec![(50.0, -10.0, 20.0, 20.0)];
    
    assert!(!check_line_of_sight(enemy_pos, target_pos, &blocking_obstacle));
}

#[test]
fn test_check_line_of_sight_side_obstacle() {
    let enemy_pos = Vec2::new(0.0, 0.0);
    let target_pos = Vec2::new(100.0, 0.0);
    // Obstacle to the side, not blocking
    let side_obstacle = vec![(50.0, 50.0, 20.0, 20.0)];
    
    assert!(check_line_of_sight(enemy_pos, target_pos, &side_obstacle));
}

#[test]
fn test_check_line_of_sight_short_distance() {
    let enemy_pos = Vec2::new(0.0, 0.0);
    let close_target = Vec2::new(1.0, 0.0);
    let obstacles: Vec<(f32, f32, f32, f32)> = vec![];
    
    assert!(check_line_of_sight(enemy_pos, close_target, &obstacles));
}


