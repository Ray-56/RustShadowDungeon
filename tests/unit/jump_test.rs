use rust_shadow_dungeon::domain::movement::jump::{
    apply_gravity, apply_jump, apply_variable_jump, can_jump, JumpParams,
};

#[test]
fn test_default_jump_params() {
    let params = JumpParams::default_params();
    // Default: ~2 tiles (32px) height
    // v = sqrt(2 * 980 * 32) ≈ 250.4
    assert!(params.initial_velocity > 200.0);
    assert_eq!(params.gravity, 980.0);
}

#[test]
fn test_can_jump() {
    // Grounded + Pressed = True
    assert!(can_jump(true, true));

    // Not Grounded + Pressed = False
    assert!(!can_jump(false, true));

    // Grounded + Not Pressed = False
    assert!(!can_jump(true, false));
}

#[test]
fn test_apply_jump() {
    let params = JumpParams::default_params();
    let current_vy = -100.0; // Falling

    let new_vy = apply_jump(current_vy, &params);

    // Should override current velocity with initial jump velocity
    assert_eq!(new_vy, params.initial_velocity);
}

#[test]
fn test_variable_jump() {
    let params = JumpParams::default_params();
    let high_velocity = params.initial_velocity;

    // Release button (variable jump check usually implies button release)
    // The function apply_variable_jump assumes it is called WHEN button is released
    // and we want to cap velocity.

    let capped_velocity = apply_variable_jump(high_velocity, &params);

    // Should be capped to min_velocity
    assert_eq!(capped_velocity, params.min_velocity);

    // If already slower than min, should not change
    let slow_velocity = params.min_velocity - 10.0;
    assert_eq!(apply_variable_jump(slow_velocity, &params), slow_velocity);
}

#[test]
fn test_gravity() {
    let initial_vy = 0.0;
    let gravity = 100.0;
    let dt = 1.0;

    let new_vy = apply_gravity(initial_vy, gravity, dt);
    assert_eq!(new_vy, -100.0);
}
