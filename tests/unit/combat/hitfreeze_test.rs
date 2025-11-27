/// Unit tests for hitfreeze (hit stop) system
/// 
/// These tests verify the hitfreeze timer functionality:
/// - Hitfreeze trigger and duration
/// - Hitfreeze timer update
/// - Multiple hitfreeze triggers (max duration)

#[cfg(test)]
mod hitfreeze_tests {
    use rust_shadow_dungeon::infrastructure::resources::hitfreeze::HitfreezeTimer;

    /// T056: Test hitfreeze trigger
    /// Verifies that triggering hitfreeze sets the correct duration
    #[test]
    fn test_hitfreeze_trigger() {
        let mut timer = HitfreezeTimer::new();
        
        // Trigger hitfreeze for 0.05 seconds (light hit, ~3 frames)
        timer.trigger(0.05);
        
        assert!(timer.is_active());
        assert_eq!(timer.remaining, 0.05);
    }

    /// T056: Test hitfreeze duration
    /// Verifies that hitfreeze timer counts down correctly
    #[test]
    fn test_hitfreeze_duration() {
        let mut timer = HitfreezeTimer::new();
        
        // Trigger for 0.05 seconds
        timer.trigger(0.05);
        
        // Update with 0.02 seconds (should still be active)
        let ended = timer.update(0.02);
        assert!(!ended);
        assert!(timer.is_active());
        assert_relative_eq!(timer.remaining, 0.03, epsilon = 0.001);
        
        // Update with remaining 0.03 seconds (should end)
        let ended = timer.update(0.03);
        assert!(ended);
        assert!(!timer.is_active());
        assert_eq!(timer.remaining, 0.0);
    }

    /// T056: Test multiple hitfreeze triggers (max duration)
    /// When multiple hits occur, hitfreeze should take the maximum duration
    #[test]
    fn test_hitfreeze_max_duration() {
        let mut timer = HitfreezeTimer::new();
        
        // Trigger light hit (0.05s)
        timer.trigger(0.05);
        assert_eq!(timer.remaining, 0.05);
        
        // Trigger heavy hit (0.083s) - should extend
        timer.trigger(0.083);
        assert_eq!(timer.remaining, 0.083);
        
        // Trigger light hit again - should NOT reduce
        timer.trigger(0.05);
        assert_eq!(timer.remaining, 0.083); // Still max
        
        // Trigger critical hit (0.117s) - should extend further
        timer.trigger(0.117);
        assert_eq!(timer.remaining, 0.117);
    }

    /// T056: Test hitfreeze progress calculation
    #[test]
    fn test_hitfreeze_progress() {
        let mut timer = HitfreezeTimer::new();
        
        // Trigger for 1.0 second
        timer.trigger(1.0);
        assert_eq!(timer.progress(1.0), 0.0); // Just started
        
        // Update by 0.5 seconds
        timer.update(0.5);
        assert_relative_eq!(timer.progress(1.0), 0.5, epsilon = 0.01); // Halfway
        
        // Update by remaining 0.5 seconds
        timer.update(0.5);
        assert_eq!(timer.progress(1.0), 1.0); // Finished
    }

    /// T056: Test hitfreeze with zero duration
    #[test]
    fn test_hitfreeze_zero_duration() {
        let mut timer = HitfreezeTimer::new();
        
        // Trigger with zero duration should not activate
        timer.trigger(0.0);
        assert!(!timer.is_active());
        
        // Update should not cause issues
        let ended = timer.update(0.1);
        assert!(!ended);
        assert!(!timer.is_active());
    }

    /// T056: Test hitfreeze update with large delta
    #[test]
    fn test_hitfreeze_large_delta() {
        let mut timer = HitfreezeTimer::new();
        
        // Trigger for 0.05 seconds
        timer.trigger(0.05);
        
        // Update with very large delta (should end immediately)
        let ended = timer.update(1.0);
        assert!(ended);
        assert!(!timer.is_active());
        assert_eq!(timer.remaining, 0.0);
    }
}

// Import approx for floating point comparisons
use approx::assert_relative_eq;

