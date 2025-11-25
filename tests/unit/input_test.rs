use rust_shadow_dungeon::domain::movement::input::{Input, InputDirection};

#[test]
fn test_input_direction_factors() {
    assert_eq!(InputDirection::Left.to_velocity_factor(), -1.0);
    assert_eq!(InputDirection::Right.to_velocity_factor(), 1.0);
    assert_eq!(InputDirection::None.to_velocity_factor(), 0.0);
}

#[test]
fn test_input_creation() {
    let input = Input::new(InputDirection::Left);
    assert_eq!(input.direction, InputDirection::Left);
}

#[test]
fn test_input_default() {
    let input = Input::default();
    assert_eq!(input.direction, InputDirection::None);
}
