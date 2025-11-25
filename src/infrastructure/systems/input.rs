//! Input handling systems
//!
//! Manual input handling for keyboard and gamepad.

use bevy::prelude::*;

use crate::infrastructure::components::{InputState, Player};

/// Process input (Keyboard + Gamepad)
pub fn player_input_system(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    gamepads: Query<&Gamepad>,
    mut query: Query<&mut InputState, With<Player>>,
) {
    let dt = time.delta_secs();

    for mut input in &mut query {
        let mut direction = 0.0;

        // Keyboard Input
        if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
            direction -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
            direction += 1.0;
        }

        // Gamepad Input (using first available gamepad)
        if let Some(gamepad) = gamepads.iter().next() {
            // D-Pad
            if gamepad.pressed(GamepadButton::DPadLeft) {
                direction -= 1.0;
            }
            if gamepad.pressed(GamepadButton::DPadRight) {
                direction += 1.0;
            }

            // Analog Stick (Left Stick X)
            let stick_x = gamepad.get(GamepadAxis::LeftStickX).unwrap_or(0.0);
            if stick_x.abs() > 0.1 {
                // Deadzone
                direction += stick_x;
            }
        }

        // Clamp direction
        input.move_direction = direction.clamp(-1.0, 1.0);

        // Jump Input
        let mut jump = false;
        let mut jump_held = false;

        // Keyboard Jump
        if keyboard.just_pressed(KeyCode::Space) {
            jump = true;
        }
        if keyboard.pressed(KeyCode::Space) {
            jump_held = true;
        }

        // Gamepad Jump
        if let Some(gamepad) = gamepads.iter().next() {
            if gamepad.just_pressed(GamepadButton::South) {
                jump = true;
            }
            if gamepad.pressed(GamepadButton::South) {
                jump_held = true;
            }
        }

        input.jump_pressed = jump;
        input.jump_held = jump_held;

        // Buffer jump
        if input.jump_pressed {
            input.buffer_jump();
        }

        // Update buffer
        input.tick_buffer(dt);
    }
}
