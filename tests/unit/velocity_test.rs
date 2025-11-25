use rust_shadow_dungeon::domain::movement::input::InputDirection;
use rust_shadow_dungeon::domain::movement::velocity::{calculate_ground_velocity, Velocity};

#[test]
fn test_velocity_creation() {
    let v = Velocity::new(10.0, -5.0);
    assert_eq!(v.x, 10.0);
    assert_eq!(v.y, -5.0);
}

#[test]
fn test_velocity_zero() {
    let v = Velocity::zero();
    assert_eq!(v.x, 0.0);
    assert_eq!(v.y, 0.0);
}

#[test]
fn test_velocity_magnitude() {
    let v = Velocity::new(3.0, 4.0);
    assert_eq!(v.magnitude(), 5.0);

    let v_zero = Velocity::zero();
    assert_eq!(v_zero.magnitude(), 0.0);
}

#[test]
fn test_velocity_normalize() {
    let v = Velocity::new(3.0, 4.0);
    let normalized = v.normalize();

    assert!((normalized.magnitude() - 1.0).abs() < 1e-6);
    assert!((normalized.x - 0.6).abs() < 1e-6);
    assert!((normalized.y - 0.8).abs() < 1e-6);
}

#[test]
fn test_velocity_normalize_zero() {
    let v = Velocity::zero();
    let normalized = v.normalize();
    assert_eq!(normalized.x, 0.0);
    assert_eq!(normalized.y, 0.0);
}

#[test]
fn test_calculate_ground_velocity_left() {
    let current = Velocity::zero();
    let speed = 100.0;
    let new_vel = calculate_ground_velocity(current, InputDirection::Left, speed);
    assert_eq!(new_vel.x, -100.0);
    // Y should be preserved (0.0)
    assert_eq!(new_vel.y, 0.0);
}

#[test]
fn test_calculate_ground_velocity_right() {
    let current = Velocity::zero();
    let speed = 48.0;
    let new_vel = calculate_ground_velocity(current, InputDirection::Right, speed);
    assert_eq!(new_vel.x, 48.0);
}

#[test]
fn test_calculate_ground_velocity_none() {
    let current = Velocity::new(50.0, 0.0);
    let speed = 48.0;
    let new_vel = calculate_ground_velocity(current, InputDirection::None, speed);
    assert_eq!(new_vel.x, 0.0);
}

#[test]
fn test_calculate_ground_velocity_preserves_y() {
    let current = Velocity::new(0.0, -50.0);
    let speed = 100.0;
    let new_vel = calculate_ground_velocity(current, InputDirection::Right, speed);
    assert_eq!(new_vel.x, 100.0);
    assert_eq!(new_vel.y, -50.0);
}
