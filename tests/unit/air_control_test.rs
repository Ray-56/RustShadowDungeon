use rust_shadow_dungeon::domain::movement::input::InputDirection;
use rust_shadow_dungeon::domain::movement::velocity::{calculate_air_velocity, Velocity};

#[test]
fn test_air_control_no_input() {
    let current = Velocity::new(100.0, -50.0);
    let new_vel = calculate_air_velocity(current, InputDirection::None, 100.0, 0.5);

    // Target x is 0. Interpolate 100 -> 0 by 0.3
    // 100 * 0.7 + 0 * 0.3 = 70.0
    assert!((new_vel.x - 70.0).abs() < 0.001);
    assert_eq!(new_vel.y, -50.0);
}

#[test]
fn test_air_control_with_input() {
    let current = Velocity::new(0.0, 0.0);
    let new_vel = calculate_air_velocity(current, InputDirection::Right, 100.0, 0.5);

    // Target x is 1.0 * 100 * 0.5 = 50.0
    // 0 * 0.7 + 50 * 0.3 = 15.0
    assert!((new_vel.x - 15.0).abs() < 0.001);
}

#[test]
fn test_air_control_preserves_y() {
    let current = Velocity::new(0.0, 100.0); // Jumping
    let new_vel = calculate_air_velocity(current, InputDirection::Left, 100.0, 1.0);

    assert_eq!(new_vel.y, 100.0);
}
