use rust_shadow_dungeon::domain::movement::input::InputDirection;

// Mocking leafwing input behavior or just testing normalization logic
// Since I haven't implemented normalize_gamepad_input yet, I'll define test first.

#[test]
fn test_normalize_gamepad_axis() {
    // This function handles deadzone
    let deadzone = 0.1;

    // Value below deadzone -> 0
    assert_eq!(normalize_axis(0.05, deadzone), 0.0);
    assert_eq!(normalize_axis(-0.05, deadzone), 0.0);

    // Value above deadzone -> Scaled 0..1
    // (0.5 - 0.1) / (1.0 - 0.1) = 0.4 / 0.9 = 0.444...
    let val = 0.55; // 0.55 - 0.1 = 0.45. 0.45 / 0.9 = 0.5
    assert!((normalize_axis(val, deadzone) - 0.5).abs() < 0.001);

    // Full range
    assert_eq!(normalize_axis(1.0, deadzone), 1.0);
    assert_eq!(normalize_axis(-1.0, deadzone), -1.0);
}

fn normalize_axis(value: f32, deadzone: f32) -> f32 {
    if value.abs() < deadzone {
        0.0
    } else {
        let sign = value.signum();
        let mag = value.abs();
        let normalized = (mag - deadzone) / (1.0 - deadzone);
        sign * normalized.clamp(0.0, 1.0)
    }
}
