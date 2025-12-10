//! Unit tests for dungeon progression domain logic
//!
//! Tests pure functions in src/domain/dungeon/progression.rs

use rust_shadow_dungeon::domain::dungeon::progression::*;
use rust_shadow_dungeon::infrastructure::components::dungeon::{DoorState, RoomId};

#[test]
fn test_should_spawn_enemies() {
    assert!(should_spawn_enemies(RoomState::Uncleared));
    assert!(!should_spawn_enemies(RoomState::Cleared));
    assert!(!should_spawn_enemies(RoomState::Active));
}

#[test]
fn test_check_room_cleared() {
    assert!(check_room_cleared(0));
    assert!(!check_room_cleared(1));
    assert!(!check_room_cleared(5));
}

#[test]
fn test_should_unlock_doors() {
    assert!(should_unlock_doors(RoomState::Cleared));
    assert!(!should_unlock_doors(RoomState::Uncleared));
    assert!(!should_unlock_doors(RoomState::Active));
}

#[test]
fn test_mark_room_cleared() {
    assert_eq!(mark_room_cleared(RoomState::Uncleared), RoomState::Cleared);
    assert_eq!(mark_room_cleared(RoomState::Cleared), RoomState::Cleared);
    assert_eq!(mark_room_cleared(RoomState::Active), RoomState::Active);
}

#[test]
fn test_is_room_cleared() {
    assert!(is_room_cleared(RoomState::Cleared));
    assert!(!is_room_cleared(RoomState::Uncleared));
    assert!(!is_room_cleared(RoomState::Active));
}

#[test]
fn test_get_target_room_entrance() {
    use bevy::math::Vec2;
    let entrance = Vec2::new(100.0, 200.0);
    let result = get_target_room_entrance(entrance);
    assert_eq!(result, entrance);
}

#[test]
fn test_can_transition_to_room() {
    assert!(can_transition_to_room(DoorState::Unlocked));
    assert!(!can_transition_to_room(DoorState::Locked));
}

