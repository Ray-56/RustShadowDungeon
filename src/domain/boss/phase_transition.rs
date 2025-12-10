//! Phase transition logic
//!
//! Pure domain logic for Boss phase transitions.

use super::phase::BossPhase;

/// Phase transition result
#[derive(Debug, Clone, PartialEq)]
pub enum PhaseTransitionResult {
    /// No transition needed
    NoTransition,
    /// Transition to next phase
    Transition {
        /// From phase index
        from_phase: usize,
        /// To phase index
        to_phase: usize,
    },
}

/// Check if phase transition is needed
///
/// # Arguments
/// * `current_phase` - Current phase index
/// * `health_percentage` - Current health percentage (0.0-1.0)
/// * `phases` - All phase configurations
/// * `is_locked` - Whether health is currently locked
///
/// # Returns
/// PhaseTransitionResult indicating if transition should occur
pub fn check_phase_transition(
    current_phase: usize,
    health_percentage: f32,
    phases: &[BossPhase],
    is_locked: bool,
) -> PhaseTransitionResult {
    // If health is locked, no transition
    if is_locked {
        return PhaseTransitionResult::NoTransition;
    }

    // Check if there's a next phase
    if current_phase + 1 >= phases.len() {
        return PhaseTransitionResult::NoTransition;
    }

    // Check if next phase threshold is reached
    let next_phase = &phases[current_phase + 1];
    if next_phase.is_threshold_reached(health_percentage) {
        PhaseTransitionResult::Transition {
            from_phase: current_phase,
            to_phase: current_phase + 1,
        }
    } else {
        PhaseTransitionResult::NoTransition
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_transition_when_locked() {
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
    fn test_no_transition_when_no_next_phase() {
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

    #[test]
    fn test_transition_when_threshold_reached() {
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
    fn test_no_transition_when_above_threshold() {
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
}

