//! Unit tests for Boss phase transition logic
//!
//! Tests the domain layer phase transition logic.

use rust_shadow_dungeon::domain::boss::{
    phase::BossPhase,
    phase_transition::{check_phase_transition, PhaseTransitionResult},
};

#[test]
fn test_phase_transition_no_transition_when_locked() {
    let phases = vec![
        BossPhase {
            phase_index: 0,
            health_threshold: 1.0,
            invulnerability_duration: 1.5,
            skill_ids: vec![],
            attack_frequency: 2.0,
            move_speed: 50.0,
        },
        BossPhase {
            phase_index: 1,
            health_threshold: 0.5,
            invulnerability_duration: 2.0,
            skill_ids: vec![],
            attack_frequency: 3.0,
            move_speed: 70.0,
        },
    ];

    let result = check_phase_transition(0, 0.4, &phases, true);
    assert_eq!(result, PhaseTransitionResult::NoTransition);
}

#[test]
fn test_phase_transition_when_threshold_reached() {
    let phases = vec![
        BossPhase {
            phase_index: 0,
            health_threshold: 1.0,
            invulnerability_duration: 1.5,
            skill_ids: vec![],
            attack_frequency: 2.0,
            move_speed: 50.0,
        },
        BossPhase {
            phase_index: 1,
            health_threshold: 0.5,
            invulnerability_duration: 2.0,
            skill_ids: vec![],
            attack_frequency: 3.0,
            move_speed: 70.0,
        },
    ];

    let result = check_phase_transition(0, 0.4, &phases, false);
    assert_eq!(
        result,
        PhaseTransitionResult::Transition {
            from_phase: 0,
            to_phase: 1
        }
    );
}

#[test]
fn test_phase_transition_no_transition_when_above_threshold() {
    let phases = vec![
        BossPhase {
            phase_index: 0,
            health_threshold: 1.0,
            invulnerability_duration: 1.5,
            skill_ids: vec![],
            attack_frequency: 2.0,
            move_speed: 50.0,
        },
        BossPhase {
            phase_index: 1,
            health_threshold: 0.5,
            invulnerability_duration: 2.0,
            skill_ids: vec![],
            attack_frequency: 3.0,
            move_speed: 70.0,
        },
    ];

    let result = check_phase_transition(0, 0.6, &phases, false);
    assert_eq!(result, PhaseTransitionResult::NoTransition);
}

#[test]
fn test_phase_transition_no_next_phase() {
    let phases = vec![BossPhase {
        phase_index: 0,
        health_threshold: 1.0,
        invulnerability_duration: 1.5,
        skill_ids: vec![],
        attack_frequency: 2.0,
        move_speed: 50.0,
    }];

    let result = check_phase_transition(0, 0.4, &phases, false);
    assert_eq!(result, PhaseTransitionResult::NoTransition);
}

