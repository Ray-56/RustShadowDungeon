/// Unit tests for damage calculation
/// 
/// These tests verify the domain layer damage calculation logic:
/// - Basic damage (base + attack)
/// - Defense reduction formula
/// - Element multipliers
/// - Critical hits
/// - Damage clamping (1..=9999)

#[cfg(test)]
mod damage_tests {
    use rust_shadow_dungeon::domain::combat::{
        calculate_damage, Stats, Element, DamageResult
    };
    use std::collections::HashMap;
    use approx::assert_relative_eq;

    /// T017: Test basic damage calculation
    /// Formula: base_damage + attacker.attack
    #[test]
    fn test_basic_damage_calculation() {
        let attacker = Stats {
            attack: 10.0,
            defense: 0.0,
            crit_rate: 0.0, // No crit for predictable test
            crit_multiplier: 2.0,
            element_resistances: HashMap::new(),
        };
        let defender = Stats {
            attack: 0.0,
            defense: 0.0,
            crit_rate: 0.0,
            crit_multiplier: 2.0,
            element_resistances: HashMap::new(),
        };

        let result = calculate_damage(10.0, &attacker, &defender, Element::Physical);

        // Expected: 10 (base) + 10 (attack) = 20
        assert_relative_eq!(result.final_damage, 20.0, epsilon = 0.01);
        assert!(!result.is_critical);
        assert_eq!(result.element, Element::Physical);
    }

    /// T018: Test defense reduction
    /// Formula: damage * (1 - defense / (defense + 100))
    #[test]
    fn test_defense_reduction() {
        let attacker = Stats {
            attack: 0.0,
            defense: 0.0,
            crit_rate: 0.0,
            crit_multiplier: 2.0,
            element_resistances: HashMap::new(),
        };
        let defender = Stats {
            attack: 0.0,
            defense: 50.0, // 50 defense
            crit_rate: 0.0,
            crit_multiplier: 2.0,
            element_resistances: HashMap::new(),
        };

        let result = calculate_damage(100.0, &attacker, &defender, Element::Physical);

        // Expected: 100 * (1 - 50/(50+100)) = 100 * 0.666... ≈ 66.67
        assert_relative_eq!(result.final_damage, 66.67, epsilon = 0.1);
    }

    /// T018: Test high defense (asymptotic behavior)
    #[test]
    fn test_high_defense() {
        let attacker = Stats {
            attack: 0.0,
            defense: 0.0,
            crit_rate: 0.0,
            crit_multiplier: 2.0,
            element_resistances: HashMap::new(),
        };
        let defender = Stats {
            attack: 0.0,
            defense: 900.0, // Very high defense
            crit_rate: 0.0,
            crit_multiplier: 2.0,
            element_resistances: HashMap::new(),
        };

        let result = calculate_damage(100.0, &attacker, &defender, Element::Physical);

        // Expected: 100 * (1 - 900/(900+100)) = 100 * 0.1 = 10
        // But damage is clamped to minimum 1.0
        assert_relative_eq!(result.final_damage, 10.0, epsilon = 0.1);
    }

    /// T019: Test element multiplier (weakness)
    #[test]
    fn test_element_multiplier() {
        let attacker = Stats {
            attack: 0.0,
            defense: 0.0,
            crit_rate: 0.0,
            crit_multiplier: 2.0,
            element_resistances: HashMap::new(),
        };
        let mut resistances = HashMap::new();
        resistances.insert(Element::Fire, -0.5); // -0.5 = 1.5x damage (weakness)

        let defender = Stats {
            attack: 0.0,
            defense: 0.0,
            crit_rate: 0.0,
            crit_multiplier: 2.0,
            element_resistances: resistances,
        };

        let result = calculate_damage(10.0, &attacker, &defender, Element::Fire);

        // Expected: 10 * 1.5 = 15.0
        assert_relative_eq!(result.final_damage, 15.0, epsilon = 0.01);
        assert_eq!(result.element, Element::Fire);
    }

    /// T019: Test element resistance (reduced damage)
    #[test]
    fn test_element_resistance() {
        let attacker = Stats {
            attack: 0.0,
            defense: 0.0,
            crit_rate: 0.0,
            crit_multiplier: 2.0,
            element_resistances: HashMap::new(),
        };
        let mut resistances = HashMap::new();
        resistances.insert(Element::Fire, 0.5); // 0.5 = 0.5x damage (resistance)

        let defender = Stats {
            attack: 0.0,
            defense: 0.0,
            crit_rate: 0.0,
            crit_multiplier: 2.0,
            element_resistances: resistances,
        };

        let result = calculate_damage(10.0, &attacker, &defender, Element::Fire);

        // Expected: 10 * 0.5 = 5.0
        assert_relative_eq!(result.final_damage, 5.0, epsilon = 0.01);
    }

    /// T020: Test critical hit
    #[test]
    fn test_critical_hit() {
        let attacker = Stats {
            attack: 10.0,
            defense: 0.0,
            crit_rate: 1.0, // 100% crit rate for predictable test
            crit_multiplier: 2.0,
            element_resistances: HashMap::new(),
        };
        let defender = Stats {
            attack: 0.0,
            defense: 0.0,
            crit_rate: 0.0,
            crit_multiplier: 2.0,
            element_resistances: HashMap::new(),
        };

        let result = calculate_damage(10.0, &attacker, &defender, Element::Physical);

        // Expected: (10 + 10) * 2.0 = 40.0
        assert_relative_eq!(result.final_damage, 40.0, epsilon = 0.01);
        assert!(result.is_critical);
    }

    /// T020: Test no critical hit
    #[test]
    fn test_no_critical_hit() {
        let attacker = Stats {
            attack: 10.0,
            defense: 0.0,
            crit_rate: 0.0, // 0% crit rate
            crit_multiplier: 2.0,
            element_resistances: HashMap::new(),
        };
        let defender = Stats {
            attack: 0.0,
            defense: 0.0,
            crit_rate: 0.0,
            crit_multiplier: 2.0,
            element_resistances: HashMap::new(),
        };

        let result = calculate_damage(10.0, &attacker, &defender, Element::Physical);

        // Expected: 10 + 10 = 20.0 (no crit)
        assert_relative_eq!(result.final_damage, 20.0, epsilon = 0.01);
        assert!(!result.is_critical);
    }

    /// T021: Test damage minimum clamp
    #[test]
    fn test_damage_clamp_minimum() {
        let attacker = Stats {
            attack: 0.0,
            defense: 0.0,
            crit_rate: 0.0,
            crit_multiplier: 2.0,
            element_resistances: HashMap::new(),
        };
        let defender = Stats {
            attack: 0.0,
            defense: 9999.0, // Extremely high defense
            crit_rate: 0.0,
            crit_multiplier: 2.0,
            element_resistances: HashMap::new(),
        };

        let result = calculate_damage(1.0, &attacker, &defender, Element::Physical);

        // Expected: Clamped to minimum 1.0
        assert_relative_eq!(result.final_damage, 1.0, epsilon = 0.01);
    }

    /// T021: Test damage maximum clamp
    #[test]
    fn test_damage_clamp_maximum() {
        let attacker = Stats {
            attack: 10000.0, // Very high attack
            defense: 0.0,
            crit_rate: 1.0,
            crit_multiplier: 5.0, // High crit multiplier
            element_resistances: HashMap::new(),
        };
        let defender = Stats {
            attack: 0.0,
            defense: 0.0,
            crit_rate: 0.0,
            crit_multiplier: 2.0,
            element_resistances: HashMap::new(),
        };

        let result = calculate_damage(10000.0, &attacker, &defender, Element::Physical);

        // Expected: Clamped to maximum 9999.0
        assert_relative_eq!(result.final_damage, 9999.0, epsilon = 0.01);
    }

    /// T021: Test zero base damage (edge case)
    #[test]
    fn test_zero_base_damage() {
        let attacker = Stats {
            attack: 10.0,
            defense: 0.0,
            crit_rate: 0.0,
            crit_multiplier: 2.0,
            element_resistances: HashMap::new(),
        };
        let defender = Stats {
            attack: 0.0,
            defense: 0.0,
            crit_rate: 0.0,
            crit_multiplier: 2.0,
            element_resistances: HashMap::new(),
        };

        let result = calculate_damage(0.0, &attacker, &defender, Element::Physical);

        // Expected: 0 + 10 = 10.0
        assert_relative_eq!(result.final_damage, 10.0, epsilon = 0.01);
    }

    /// Test combined effects (defense + element + crit)
    #[test]
    fn test_combined_damage_calculation() {
        let attacker = Stats {
            attack: 50.0,
            defense: 0.0,
            crit_rate: 1.0, // 100% crit
            crit_multiplier: 2.0,
            element_resistances: HashMap::new(),
        };
        let mut resistances = HashMap::new();
        resistances.insert(Element::Fire, -0.5); // 1.5x weakness

        let defender = Stats {
            attack: 0.0,
            defense: 25.0,
            crit_rate: 0.0,
            crit_multiplier: 2.0,
            element_resistances: resistances,
        };

        let result = calculate_damage(10.0, &attacker, &defender, Element::Fire);

        // Expected: 
        // 1. Base + Attack: 10 + 50 = 60
        // 2. Defense reduction: 60 * (1 - 25/(25+100)) = 60 * 0.8 = 48
        // 3. Element multiplier: 48 * 1.5 = 72
        // 4. Critical: 72 * 2.0 = 144
        assert_relative_eq!(result.final_damage, 144.0, epsilon = 0.1);
        assert!(result.is_critical);
        assert_eq!(result.element, Element::Fire);
    }

    /// Test Stats::new() with default values
    #[test]
    fn test_stats_new() {
        let stats = Stats::new(50.0, 20.0);
        assert_eq!(stats.attack, 50.0);
        assert_eq!(stats.defense, 20.0);
        assert_eq!(stats.crit_rate, 0.1);
        assert_eq!(stats.crit_multiplier, 2.0);
        assert!(stats.element_resistances.is_empty());
    }

    /// Test Stats::zero()
    #[test]
    fn test_stats_zero() {
        let stats = Stats::zero();
        assert_eq!(stats.attack, 0.0);
        assert_eq!(stats.defense, 0.0);
        assert_eq!(stats.crit_rate, 0.0);
        assert_eq!(stats.crit_multiplier, 2.0);
        assert!(stats.element_resistances.is_empty());
    }

    /// Test Stats::default()
    #[test]
    fn test_stats_default() {
        let stats = Stats::default();
        assert_eq!(stats.attack, 10.0);
        assert_eq!(stats.defense, 0.0);
        assert_eq!(stats.crit_rate, 0.1);
        assert_eq!(stats.crit_multiplier, 2.0);
    }

    /// Test element immunity (100% resistance)
    #[test]
    fn test_element_immunity() {
        let attacker = Stats {
            attack: 0.0,
            defense: 0.0,
            crit_rate: 0.0,
            crit_multiplier: 2.0,
            element_resistances: HashMap::new(),
        };
        let mut resistances = HashMap::new();
        resistances.insert(Element::Fire, 1.0); // 100% resistance = immune

        let defender = Stats {
            attack: 0.0,
            defense: 0.0,
            crit_rate: 0.0,
            crit_multiplier: 2.0,
            element_resistances: resistances,
        };

        let result = calculate_damage(100.0, &attacker, &defender, Element::Fire);

        // Expected: 100 * 0.0 = 0, but clamped to minimum 1.0
        assert_relative_eq!(result.final_damage, 1.0, epsilon = 0.01);
    }

    /// Test negative defense (edge case)
    #[test]
    fn test_negative_defense() {
        let attacker = Stats {
            attack: 0.0,
            defense: 0.0,
            crit_rate: 0.0,
            crit_multiplier: 2.0,
            element_resistances: HashMap::new(),
        };
        let defender = Stats {
            attack: 0.0,
            defense: -10.0, // Negative defense (should be treated as 0)
            crit_rate: 0.0,
            crit_multiplier: 2.0,
            element_resistances: HashMap::new(),
        };

        let result = calculate_damage(100.0, &attacker, &defender, Element::Physical);

        // Expected: No defense reduction, so 100.0 damage
        assert_relative_eq!(result.final_damage, 100.0, epsilon = 0.01);
    }

    /// Test calculate_damage with all elements
    #[test]
    fn test_calculate_damage_all_elements() {
        let attacker = Stats {
            attack: 0.0,
            defense: 0.0,
            crit_rate: 0.0,
            crit_multiplier: 2.0,
            element_resistances: HashMap::new(),
        };
        let defender = Stats {
            attack: 0.0,
            defense: 0.0,
            crit_rate: 0.0,
            crit_multiplier: 2.0,
            element_resistances: HashMap::new(),
        };

        // Test all element types
        let physical_result = calculate_damage(10.0, &attacker, &defender, Element::Physical);
        assert_relative_eq!(physical_result.final_damage, 10.0, epsilon = 0.01);
        assert_eq!(physical_result.element, Element::Physical);

        let fire_result = calculate_damage(10.0, &attacker, &defender, Element::Fire);
        assert_relative_eq!(fire_result.final_damage, 10.0, epsilon = 0.01);
        assert_eq!(fire_result.element, Element::Fire);

        let ice_result = calculate_damage(10.0, &attacker, &defender, Element::Ice);
        assert_relative_eq!(ice_result.final_damage, 10.0, epsilon = 0.01);
        assert_eq!(ice_result.element, Element::Ice);

        let lightning_result = calculate_damage(10.0, &attacker, &defender, Element::Lightning);
        assert_relative_eq!(lightning_result.final_damage, 10.0, epsilon = 0.01);
        assert_eq!(lightning_result.element, Element::Lightning);
    }

    /// Test calculate_damage with very small base damage
    #[test]
    fn test_calculate_damage_very_small_base() {
        let mut attacker = Stats::new(0.0, 0.0);
        attacker.crit_rate = 0.0;
        let defender = Stats::new(0.0, 0.0);

        let result = calculate_damage(0.01, &attacker, &defender, Element::Physical);
        // Should be clamped to minimum 1.0
        assert_relative_eq!(result.final_damage, 1.0, epsilon = 0.01);
    }

    /// Test calculate_damage with attack power only
    #[test]
    fn test_calculate_damage_attack_only() {
        let mut attacker = Stats::new(50.0, 0.0);
        attacker.crit_rate = 0.0;
        let defender = Stats::new(0.0, 0.0);

        let result = calculate_damage(0.0, &attacker, &defender, Element::Physical);
        // 0 (base) + 50 (attack) = 50.0
        assert_relative_eq!(result.final_damage, 50.0, epsilon = 0.01);
    }

    /// Test calculate_damage with multiple element resistances
    #[test]
    fn test_calculate_damage_multiple_resistances() {
        let mut attacker = Stats::new(0.0, 0.0);
        attacker.crit_rate = 0.0;

        let mut resistances = HashMap::new();
        resistances.insert(Element::Fire, -0.5); // Weakness
        resistances.insert(Element::Ice, 0.5);    // Resistance
        resistances.insert(Element::Lightning, 1.0); // Immune

        let defender = Stats {
            attack: 0.0,
            defense: 0.0,
            crit_rate: 0.0,
            crit_multiplier: 2.0,
            element_resistances: resistances,
        };

        // Fire should do 1.5x damage
        let fire_result = calculate_damage(10.0, &attacker, &defender, Element::Fire);
        assert_relative_eq!(fire_result.final_damage, 15.0, epsilon = 0.01);

        // Ice should do 0.5x damage
        let ice_result = calculate_damage(10.0, &attacker, &defender, Element::Ice);
        assert_relative_eq!(ice_result.final_damage, 5.0, epsilon = 0.01);

        // Lightning should be immune (clamped to 1.0 minimum)
        let lightning_result = calculate_damage(10.0, &attacker, &defender, Element::Lightning);
        assert_relative_eq!(lightning_result.final_damage, 1.0, epsilon = 0.01);
    }

    /// Test calculate_damage with fractional crit rate
    #[test]
    fn test_calculate_damage_fractional_crit_rate() {
        // This test verifies that fractional crit rates work
        // We can't test exact behavior due to randomness, but we can test
        // that the function doesn't panic and returns valid results
        let mut attacker = Stats::new(10.0, 0.0);
        attacker.crit_rate = 0.5; // 50% crit rate
        attacker.crit_multiplier = 2.0;
        let defender = Stats::new(0.0, 0.0);

        // Run multiple times to verify it doesn't panic
        for _ in 0..10 {
            let result = calculate_damage(10.0, &attacker, &defender, Element::Physical);
            // Should be either 20 (no crit) or 40 (crit)
            assert!(result.final_damage == 20.0 || result.final_damage == 40.0);
            assert!(result.final_damage >= 1.0 && result.final_damage <= 9999.0);
        }
    }

    /// Test calculate_damage with custom crit multiplier
    #[test]
    fn test_calculate_damage_custom_crit_multiplier() {
        let mut attacker = Stats::new(0.0, 0.0);
        attacker.crit_rate = 1.0; // 100% crit
        attacker.crit_multiplier = 3.0; // 3x crit multiplier
        let defender = Stats::new(0.0, 0.0);

        let result = calculate_damage(10.0, &attacker, &defender, Element::Physical);
        // Expected: 10 * 3.0 = 30.0
        assert_relative_eq!(result.final_damage, 30.0, epsilon = 0.01);
        assert!(result.is_critical);
    }

    /// Test calculate_damage with very high defense (asymptotic behavior)
    #[test]
    fn test_calculate_damage_extreme_defense() {
        let mut attacker = Stats::new(0.0, 0.0);
        attacker.crit_rate = 0.0;
        let defender = Stats::new(0.0, 10000.0); // Extremely high defense

        let result = calculate_damage(100.0, &attacker, &defender, Element::Physical);
        // Should be reduced significantly but clamped to minimum 1.0
        assert!(result.final_damage >= 1.0 && result.final_damage <= 100.0);
    }

    /// Test calculate_damage with zero crit multiplier (edge case)
    #[test]
    fn test_calculate_damage_zero_crit_multiplier() {
        let mut attacker = Stats::new(10.0, 0.0);
        attacker.crit_rate = 1.0; // 100% crit
        attacker.crit_multiplier = 0.0; // Zero multiplier
        let defender = Stats::new(0.0, 0.0);

        let result = calculate_damage(10.0, &attacker, &defender, Element::Physical);
        // Base damage: 10 + 10 = 20, crit with 0x multiplier = 0, clamped to 1.0
        assert_relative_eq!(result.final_damage, 1.0, epsilon = 0.01);
        assert!(result.is_critical);
    }

    /// Test calculate_damage with very small base damage
    #[test]
    fn test_calculate_damage_tiny_base() {
        let mut attacker = Stats::new(0.0, 0.0);
        attacker.crit_rate = 0.0;
        let defender = Stats::new(0.0, 0.0);

        let result = calculate_damage(0.001, &attacker, &defender, Element::Physical);
        // Should be clamped to minimum 1.0
        assert_relative_eq!(result.final_damage, 1.0, epsilon = 0.01);
    }

    /// Test calculate_damage with negative base damage (edge case)
    #[test]
    fn test_calculate_damage_negative_base() {
        let mut attacker = Stats::new(5.0, 0.0);
        attacker.crit_rate = 0.0;
        let defender = Stats::new(0.0, 0.0);

        let result = calculate_damage(-10.0, &attacker, &defender, Element::Physical);
        // Base: -10 + 5 = -5, clamped to minimum 1.0
        assert_relative_eq!(result.final_damage, 1.0, epsilon = 0.01);
    }

    /// Test calculate_damage with negative attack (edge case)
    #[test]
    fn test_calculate_damage_negative_attack() {
        let mut attacker = Stats::new(-10.0, 0.0);
        attacker.crit_rate = 0.0;
        let defender = Stats::new(0.0, 0.0);

        let result = calculate_damage(5.0, &attacker, &defender, Element::Physical);
        // Base: 5 + (-10) = -5, clamped to minimum 1.0
        assert_relative_eq!(result.final_damage, 1.0, epsilon = 0.01);
    }

    /// Test calculate_damage with all elements and various resistances
    #[test]
    fn test_calculate_damage_all_elements_various_resistances() {
        let mut attacker = Stats::new(0.0, 0.0);
        attacker.crit_rate = 0.0;

        // Test Physical (no resistance data)
        let defender = Stats::new(0.0, 0.0);
        let result = calculate_damage(10.0, &attacker, &defender, Element::Physical);
        assert_relative_eq!(result.final_damage, 10.0, epsilon = 0.01);

        // Test Fire with weakness
        let mut resistances = HashMap::new();
        resistances.insert(Element::Fire, -0.25); // 1.25x damage
        let defender = Stats {
            attack: 0.0,
            defense: 0.0,
            crit_rate: 0.0,
            crit_multiplier: 2.0,
            element_resistances: resistances,
        };
        let result = calculate_damage(10.0, &attacker, &defender, Element::Fire);
        assert_relative_eq!(result.final_damage, 12.5, epsilon = 0.01);

        // Test Ice with resistance
        let mut resistances = HashMap::new();
        resistances.insert(Element::Ice, 0.25); // 0.75x damage
        let defender = Stats {
            attack: 0.0,
            defense: 0.0,
            crit_rate: 0.0,
            crit_multiplier: 2.0,
            element_resistances: resistances,
        };
        let result = calculate_damage(10.0, &attacker, &defender, Element::Ice);
        assert_relative_eq!(result.final_damage, 7.5, epsilon = 0.01);
    }

    /// Test calculate_damage with defense exactly at 0
    #[test]
    fn test_calculate_damage_zero_defense() {
        let mut attacker = Stats::new(10.0, 0.0);
        attacker.crit_rate = 0.0;
        let defender = Stats::new(0.0, 0.0);

        let result = calculate_damage(10.0, &attacker, &defender, Element::Physical);
        // No defense reduction: 10 + 10 = 20
        assert_relative_eq!(result.final_damage, 20.0, epsilon = 0.01);
    }

    /// Test calculate_damage with very small defense
    #[test]
    fn test_calculate_damage_small_defense() {
        let mut attacker = Stats::new(0.0, 0.0);
        attacker.crit_rate = 0.0;
        let defender = Stats::new(0.0, 0.1); // Very small defense

        let result = calculate_damage(100.0, &attacker, &defender, Element::Physical);
        // Should have minimal reduction
        assert!(result.final_damage > 99.0 && result.final_damage < 100.0);
    }

    /// Test calculate_damage with element resistance exactly at 0.0
    #[test]
    fn test_calculate_damage_element_resistance_zero() {
        let mut attacker = Stats::new(0.0, 0.0);
        attacker.crit_rate = 0.0;
        let mut resistances = HashMap::new();
        resistances.insert(Element::Fire, 0.0); // 0.0 = 1.0x damage (normal)
        let defender = Stats {
            attack: 0.0,
            defense: 0.0,
            crit_rate: 0.0,
            crit_multiplier: 2.0,
            element_resistances: resistances,
        };

        let result = calculate_damage(10.0, &attacker, &defender, Element::Fire);
        assert_relative_eq!(result.final_damage, 10.0, epsilon = 0.01);
    }

    /// Test calculate_damage with crit rate exactly at 0.5
    #[test]
    fn test_calculate_damage_crit_rate_half() {
        let mut attacker = Stats::new(10.0, 0.0);
        attacker.crit_rate = 0.5; // 50% crit rate
        attacker.crit_multiplier = 2.0;
        let defender = Stats::new(0.0, 0.0);

        // Run multiple times to verify both outcomes are possible
        let mut crit_count = 0;
        let mut normal_count = 0;
        for _ in 0..100 {
            let result = calculate_damage(10.0, &attacker, &defender, Element::Physical);
            if result.is_critical {
                crit_count += 1;
                assert_relative_eq!(result.final_damage, 40.0, epsilon = 0.01);
            } else {
                normal_count += 1;
                assert_relative_eq!(result.final_damage, 20.0, epsilon = 0.01);
            }
        }
        // Should have both crits and normal hits (with some randomness)
        assert!(crit_count > 0 && normal_count > 0);
    }
}

