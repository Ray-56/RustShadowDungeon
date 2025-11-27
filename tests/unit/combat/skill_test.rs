/// Unit tests for skill system
/// 
/// These tests verify the skill cooldown and MP consumption logic:
/// - Skill cooldown system
/// - MP consumption validation

#[cfg(test)]
mod skill_tests {
    use rust_shadow_dungeon::infrastructure::components::combat::Skill;
    use rust_shadow_dungeon::domain::combat::Element;

    /// T075: Test skill cooldown system
    /// Verifies that skills enter cooldown after activation and count down correctly
    #[test]
    fn test_skill_cooldown() {
        let mut skill = Skill::new(
            "fireball".to_string(),
            5.0,  // 5 second cooldown
            20.0, // 20 MP cost
            30.0, // 30 damage
            Element::Fire,
        );

        // Initially ready
        assert!(skill.is_ready());
        assert_eq!(skill.remaining_cooldown, 0.0);

        // Activate skill
        skill.activate();
        assert!(!skill.is_ready());
        assert_eq!(skill.remaining_cooldown, 5.0);
        assert_eq!(skill.cooldown_progress(), 1.0); // Just activated

        // Update cooldown (2 seconds pass)
        skill.update(2.0);
        assert!(!skill.is_ready());
        assert_eq!(skill.remaining_cooldown, 3.0);
        assert_relative_eq!(skill.cooldown_progress(), 0.6, epsilon = 0.01); // 60% remaining

        // Update cooldown (remaining 3 seconds)
        skill.update(3.0);
        assert!(skill.is_ready());
        assert_eq!(skill.remaining_cooldown, 0.0);
        assert_eq!(skill.cooldown_progress(), 0.0); // Ready
    }

    /// T075: Test skill cooldown with large delta
    #[test]
    fn test_skill_cooldown_large_delta() {
        let mut skill = Skill::new(
            "fireball".to_string(),
            5.0,
            20.0,
            30.0,
            Element::Fire,
        );

        skill.activate();
        skill.update(10.0); // Update with very large delta

        assert!(skill.is_ready());
        assert_eq!(skill.remaining_cooldown, 0.0); // Should clamp to 0
    }

    /// T076: Test MP consumption validation
    /// Verifies that MP cost is correctly stored and can be checked
    #[test]
    fn test_mp_consumption() {
        let skill = Skill::new(
            "fireball".to_string(),
            5.0,
            20.0, // 20 MP cost
            30.0,
            Element::Fire,
        );

        assert_eq!(skill.mp_cost, 20.0);

        // Test different MP costs
        let expensive_skill = Skill::new(
            "meteor".to_string(),
            10.0,
            50.0, // 50 MP cost
            100.0,
            Element::Fire,
        );

        assert_eq!(expensive_skill.mp_cost, 50.0);
    }

    /// T076: Test skill with zero MP cost
    #[test]
    fn test_zero_mp_cost() {
        let skill = Skill::new(
            "free_skill".to_string(),
            3.0,
            0.0, // Free skill
            10.0,
            Element::Physical,
        );

        assert_eq!(skill.mp_cost, 0.0);
    }
}

// Import approx for floating point comparisons
use approx::assert_relative_eq;

