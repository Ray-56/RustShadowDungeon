/// Unit tests for combo system logic
///
/// T041-T042: Tests for combo state transitions and window timeout
/// These tests verify the domain layer combo logic:
/// - Combo state advancement (Idle → FirstHit → SecondHit → ThirdHit)
/// - Combo reset logic
/// - Combo window timeout detection

#[cfg(test)]
mod combo_tests {
    use rust_shadow_dungeon::domain::combat::combo::{
        advance_combo, get_combo_count, is_combo_expired, reset_combo, ComboState,
    };

    /// T041: Test combo state advancement
    #[test]
    fn test_combo_advance() {
        // Idle → FirstHit
        let (next_state, window) = advance_combo(ComboState::Idle, 1.0);
        assert_eq!(next_state, ComboState::FirstHit);
        assert_eq!(window, 1.0);

        // FirstHit → SecondHit
        let (next_state, window) = advance_combo(ComboState::FirstHit, 1.0);
        assert_eq!(next_state, ComboState::SecondHit);
        assert_eq!(window, 1.0);

        // SecondHit → ThirdHit
        let (next_state, window) = advance_combo(ComboState::SecondHit, 1.0);
        assert_eq!(next_state, ComboState::ThirdHit);
        assert_eq!(window, 1.0);

        // ThirdHit → Idle (combo completes)
        let (next_state, window) = advance_combo(ComboState::ThirdHit, 1.0);
        assert_eq!(next_state, ComboState::Idle);
        assert_eq!(window, 1.0);
    }

    /// T041: Test combo reset
    #[test]
    fn test_combo_reset() {
        let (state, window, count) = reset_combo();
        assert_eq!(state, ComboState::Idle);
        assert_eq!(window, 0.0);
        assert_eq!(count, 0);
    }

    /// T042: Test combo window timeout
    #[test]
    fn test_combo_window_timeout() {
        // Window expired (time <= 0)
        assert!(is_combo_expired(0.0));
        assert!(is_combo_expired(-0.1));

        // Window still active (time > 0)
        assert!(!is_combo_expired(0.1));
        assert!(!is_combo_expired(1.0));
    }

    /// Test combo count calculation
    #[test]
    fn test_get_combo_count() {
        assert_eq!(get_combo_count(ComboState::Idle), 0);
        assert_eq!(get_combo_count(ComboState::FirstHit), 1);
        assert_eq!(get_combo_count(ComboState::SecondHit), 2);
        assert_eq!(get_combo_count(ComboState::ThirdHit), 3);
    }

    /// Test damage multiplier for each combo state
    #[test]
    fn test_combo_damage_multiplier() {
        assert_eq!(ComboState::Idle.damage_multiplier(), 1.0);
        assert_eq!(ComboState::FirstHit.damage_multiplier(), 1.0);
        assert_eq!(ComboState::SecondHit.damage_multiplier(), 1.0);
        assert_eq!(ComboState::ThirdHit.damage_multiplier(), 2.0);
    }

    /// Test heavy hit detection
    #[test]
    fn test_is_heavy_hit() {
        assert!(!ComboState::Idle.is_heavy_hit());
        assert!(!ComboState::FirstHit.is_heavy_hit());
        assert!(!ComboState::SecondHit.is_heavy_hit());
        assert!(ComboState::ThirdHit.is_heavy_hit());
    }

    /// Test combo state default
    #[test]
    fn test_combo_state_default() {
        let state = ComboState::default();
        assert_eq!(state, ComboState::Idle);
    }

    /// Test advance_combo with different window durations
    #[test]
    fn test_advance_combo_window_duration() {
        let (next_state, window) = advance_combo(ComboState::Idle, 0.5);
        assert_eq!(next_state, ComboState::FirstHit);
        assert_eq!(window, 0.5);

        let (next_state, window) = advance_combo(ComboState::FirstHit, 2.0);
        assert_eq!(next_state, ComboState::SecondHit);
        assert_eq!(window, 2.0);
    }

    /// Test is_combo_expired with various values
    #[test]
    fn test_is_combo_expired_various() {
        assert!(is_combo_expired(0.0));
        assert!(is_combo_expired(-0.0001));
        assert!(is_combo_expired(-1.0));
        assert!(!is_combo_expired(0.0001));
        assert!(!is_combo_expired(0.5));
        assert!(!is_combo_expired(1.0));
        assert!(!is_combo_expired(10.0));
    }

    /// Test get_combo_count for all states
    #[test]
    fn test_get_combo_count_all_states() {
        assert_eq!(get_combo_count(ComboState::Idle), 0);
        assert_eq!(get_combo_count(ComboState::FirstHit), 1);
        assert_eq!(get_combo_count(ComboState::SecondHit), 2);
        assert_eq!(get_combo_count(ComboState::ThirdHit), 3);
    }

    /// Test complete combo cycle
    #[test]
    fn test_complete_combo_cycle() {
        let mut state = ComboState::Idle;
        let window = 1.0;

        // First hit
        let (state, _) = advance_combo(state, window);
        assert_eq!(state, ComboState::FirstHit);
        assert_eq!(get_combo_count(state), 1);

        // Second hit
        let (state, _) = advance_combo(state, window);
        assert_eq!(state, ComboState::SecondHit);
        assert_eq!(get_combo_count(state), 2);

        // Third hit
        let (state, _) = advance_combo(state, window);
        assert_eq!(state, ComboState::ThirdHit);
        assert_eq!(get_combo_count(state), 3);
        assert!(state.is_heavy_hit());

        // Reset to Idle
        let (state, _, count) = reset_combo();
        assert_eq!(state, ComboState::Idle);
        assert_eq!(count, 0);
        assert_eq!(get_combo_count(state), 0);
    }

    /// Test advance_combo with zero window duration
    #[test]
    fn test_advance_combo_zero_window() {
        let (next_state, window) = advance_combo(ComboState::Idle, 0.0);
        assert_eq!(next_state, ComboState::FirstHit);
        assert_eq!(window, 0.0);
    }

    /// Test advance_combo with negative window duration (edge case)
    #[test]
    fn test_advance_combo_negative_window() {
        let (next_state, window) = advance_combo(ComboState::FirstHit, -1.0);
        assert_eq!(next_state, ComboState::SecondHit);
        assert_eq!(window, -1.0);
    }

    /// Test advance_combo with very large window duration
    #[test]
    fn test_advance_combo_large_window() {
        let (next_state, window) = advance_combo(ComboState::SecondHit, 1000.0);
        assert_eq!(next_state, ComboState::ThirdHit);
        assert_eq!(window, 1000.0);
    }

    /// Test is_combo_expired with exactly zero
    #[test]
    fn test_is_combo_expired_exactly_zero() {
        assert!(is_combo_expired(0.0));
    }

    /// Test is_combo_expired with very small positive value
    #[test]
    fn test_is_combo_expired_very_small_positive() {
        assert!(!is_combo_expired(0.000001));
    }

    /// Test is_combo_expired with very large negative value
    #[test]
    fn test_is_combo_expired_very_large_negative() {
        assert!(is_combo_expired(-1000.0));
    }

    /// Test get_combo_count for all states (comprehensive)
    #[test]
    fn test_get_combo_count_comprehensive() {
        assert_eq!(get_combo_count(ComboState::Idle), 0);
        assert_eq!(get_combo_count(ComboState::FirstHit), 1);
        assert_eq!(get_combo_count(ComboState::SecondHit), 2);
        assert_eq!(get_combo_count(ComboState::ThirdHit), 3);
    }

    /// Test damage_multiplier for all states
    #[test]
    fn test_damage_multiplier_all_states() {
        assert_eq!(ComboState::Idle.damage_multiplier(), 1.0);
        assert_eq!(ComboState::FirstHit.damage_multiplier(), 1.0);
        assert_eq!(ComboState::SecondHit.damage_multiplier(), 1.0);
        assert_eq!(ComboState::ThirdHit.damage_multiplier(), 2.0);
    }

    /// Test is_heavy_hit for all states
    #[test]
    fn test_is_heavy_hit_all_states() {
        assert!(!ComboState::Idle.is_heavy_hit());
        assert!(!ComboState::FirstHit.is_heavy_hit());
        assert!(!ComboState::SecondHit.is_heavy_hit());
        assert!(ComboState::ThirdHit.is_heavy_hit());
    }
}
