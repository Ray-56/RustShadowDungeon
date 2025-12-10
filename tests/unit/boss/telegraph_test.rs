//! Unit tests for Boss telegraph area calculations
//!
//! Tests the domain layer telegraph geometry calculations.

use rust_shadow_dungeon::domain::boss::telegraph::{TelegraphArea, TelegraphShape};

#[test]
fn test_circle_telegraph_contains_point() {
    let area = TelegraphArea {
        shape: TelegraphShape::Circle { radius: 10.0 },
        center: (0.0, 0.0),
        rotation: 0.0,
    };

    assert!(area.contains_point((0.0, 0.0)));
    assert!(area.contains_point((5.0, 0.0)));
    assert!(area.contains_point((0.0, 10.0)));
    assert!(!area.contains_point((11.0, 0.0)));
    assert!(!area.contains_point((0.0, 11.0)));
}

#[test]
fn test_circle_telegraph_edge_case() {
    let area = TelegraphArea {
        shape: TelegraphShape::Circle { radius: 10.0 },
        center: (0.0, 0.0),
        rotation: 0.0,
    };

    // Point exactly on the edge
    assert!(area.contains_point((10.0, 0.0)));
    assert!(area.contains_point((0.0, 10.0)));
}

#[test]
fn test_rectangle_telegraph_contains_point() {
    let area = TelegraphArea {
        shape: TelegraphShape::Rectangle {
            width: 20.0,
            height: 10.0,
        },
        center: (0.0, 0.0),
        rotation: 0.0,
    };

    assert!(area.contains_point((0.0, 0.0)));
    assert!(area.contains_point((10.0, 5.0)));
    assert!(area.contains_point((-10.0, -5.0)));
    assert!(!area.contains_point((11.0, 0.0)));
    assert!(!area.contains_point((0.0, 6.0)));
}

#[test]
fn test_rectangle_telegraph_edge_case() {
    let area = TelegraphArea {
        shape: TelegraphShape::Rectangle {
            width: 20.0,
            height: 10.0,
        },
        center: (0.0, 0.0),
        rotation: 0.0,
    };

    // Points on the edge
    assert!(area.contains_point((10.0, 0.0)));
    assert!(area.contains_point((0.0, 5.0)));
}

