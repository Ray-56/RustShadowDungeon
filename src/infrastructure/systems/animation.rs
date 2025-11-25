use crate::infrastructure::{
    components::{AnimationState, MovementStateComponent, Player},
    resources::PlayerAnimations,
};
use bevy::prelude::*;

/// Animation system
///
/// Updates sprite texture based on movement state and timer.
pub fn animation_system(
    time: Res<Time>,
    animations: Res<PlayerAnimations>,
    mut query: Query<(&mut Sprite, &mut AnimationState, &MovementStateComponent), With<Player>>,
) {
    for (mut sprite, mut anim_state, movement_state) in &mut query {
        // Check for state change
        if movement_state.0 != anim_state.current_state {
            anim_state.current_state = movement_state.0;
            anim_state.current_frame = 0;
            anim_state.timer.reset();
        }

        // Tick timer
        anim_state.timer.tick(time.delta());

        // Advance frame
        if anim_state.timer.just_finished() {
            if let Some(frames) = animations.get_frames(anim_state.current_state) {
                if !frames.is_empty() {
                    anim_state.current_frame = (anim_state.current_frame + 1) % frames.len();
                }
            }
        }

        // Update sprite
        if let Some(frames) = animations.get_frames(anim_state.current_state) {
            if let Some(texture) = frames.get(anim_state.current_frame) {
                sprite.image = texture.clone();
            }
        }
    }
}
