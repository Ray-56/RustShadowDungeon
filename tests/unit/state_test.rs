use rust_shadow_dungeon::domain::movement::state::{transition_state, MovementState};

#[test]
fn test_idle_to_walking() {
    // On ground, with move input -> Walking
    let next_state = transition_state(MovementState::Idle, true, true, false, 0.0);
    assert_eq!(next_state, MovementState::Walking);
}

#[test]
fn test_walking_to_idle() {
    // On ground, no move input -> Idle
    let next_state = transition_state(MovementState::Walking, true, false, false, 0.0);
    assert_eq!(next_state, MovementState::Idle);
}

#[test]
fn test_walking_continues() {
    // On ground, with move input -> Walking
    let next_state = transition_state(MovementState::Walking, true, true, false, 0.0);
    assert_eq!(next_state, MovementState::Walking);
}

#[test]
fn test_idle_continues() {
    // On ground, no move input -> Idle
    let next_state = transition_state(MovementState::Idle, true, false, false, 0.0);
    assert_eq!(next_state, MovementState::Idle);
}

#[test]
fn test_idle_to_falling() {
    // Not on ground, negative velocity -> Falling
    let next_state = transition_state(MovementState::Idle, false, false, false, -1.0);
    assert_eq!(next_state, MovementState::Falling);
}

#[test]
fn test_grounded_to_jumping() {
    // Idle -> Jumping (with jump input)
    let next_state = transition_state(MovementState::Idle, true, false, true, 0.0);
    assert_eq!(next_state, MovementState::Jumping);

    // Walking -> Jumping (with jump input)
    let next_state = transition_state(MovementState::Walking, true, true, true, 0.0);
    assert_eq!(next_state, MovementState::Jumping);
}
