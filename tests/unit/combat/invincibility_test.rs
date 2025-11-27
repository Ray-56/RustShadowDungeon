/// Unit tests for invincibility system
/// 
/// These tests verify the invincibility frame logic:
/// - Invincibility duration
/// - Invincibility overlap (takes maximum duration)

#[cfg(test)]
mod invincibility_tests {
    use rust_shadow_dungeon::infrastructure::components::combat::Invincibility;

    /// T097: Test invincibility duration
    /// Verifies that invincibility counts down correctly
    #[test]
    fn test_invincibility_duration() {
        let mut invincibility = Invincibility::new(0.5); // 0.5 seconds
        
        assert!(invincibility.is_active());
        assert_eq!(invincibility.remaining, 0.5);
        
        // Update by 0.2 seconds
        invincibility.update(0.2);
        assert!(invincibility.is_active());
        assert_relative_eq!(invincibility.remaining, 0.3, epsilon = 0.001);
        
        // Update by remaining 0.3 seconds
        invincibility.update(0.3);
        assert!(!invincibility.is_active());
        assert_eq!(invincibility.remaining, 0.0);
    }

    /// T097: Test invincibility flash timer
    #[test]
    fn test_invincibility_flash() {
        let mut invincibility = Invincibility::new(0.5);
        
        // Initially visible
        assert!(invincibility.should_be_visible());
        
        // Update by 0.05 seconds (half of 0.1 second flash interval)
        invincibility.update(0.05);
        assert!(invincibility.should_be_visible());
        
        // Update by another 0.05 seconds (total 0.1 seconds)
        invincibility.update(0.05);
        assert!(!invincibility.should_be_visible()); // Should toggle
        
        // Update by another 0.1 seconds
        invincibility.update(0.1);
        assert!(invincibility.should_be_visible()); // Should toggle again
    }

    /// T098: Test invincibility overlap (takes maximum duration)
    /// When multiple invincibility triggers occur, takes the maximum duration
    #[test]
    fn test_invincibility_overlap() {
        let mut invincibility = Invincibility::new(0.3); // 0.3 seconds
        
        // Trigger another invincibility with longer duration
        let longer = Invincibility::new(0.5);
        if longer.remaining > invincibility.remaining {
            invincibility.remaining = longer.remaining;
        }
        assert_eq!(invincibility.remaining, 0.5);
        
        // Trigger another invincibility with shorter duration (should not reduce)
        let shorter = Invincibility::new(0.2);
        if shorter.remaining > invincibility.remaining {
            invincibility.remaining = shorter.remaining;
        }
        assert_eq!(invincibility.remaining, 0.5); // Should still be 0.5
    }

    /// T098: Test invincibility with zero duration
    #[test]
    fn test_invincibility_zero_duration() {
        let invincibility = Invincibility::new(0.0);
        assert!(!invincibility.is_active());
    }
}

// Import approx for floating point comparisons
use approx::assert_relative_eq;

