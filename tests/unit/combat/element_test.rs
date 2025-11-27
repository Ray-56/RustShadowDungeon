/// Unit tests for element system
/// 
/// These tests verify the domain layer element logic:
/// - Element multiplier calculation
/// - Resistance/weakness effects
/// - Immunity handling
/// - Default element

#[cfg(test)]
mod element_tests {
    use rust_shadow_dungeon::domain::combat::element::{Element, get_element_multiplier};
    use std::collections::HashMap;

    /// Test element multiplier with no resistance
    #[test]
    fn test_element_multiplier_no_resistance() {
        let resistances = HashMap::new();
        let multiplier = get_element_multiplier(Element::Fire, &resistances);
        assert_eq!(multiplier, 1.0);
    }

    /// Test element multiplier with weakness
    #[test]
    fn test_element_multiplier_weakness() {
        let mut resistances = HashMap::new();
        resistances.insert(Element::Fire, -0.5); // -0.5 = 1.5x damage
        let multiplier = get_element_multiplier(Element::Fire, &resistances);
        assert_eq!(multiplier, 1.5);
    }

    /// Test element multiplier with resistance
    #[test]
    fn test_element_multiplier_resistance() {
        let mut resistances = HashMap::new();
        resistances.insert(Element::Fire, 0.5); // 0.5 = 0.5x damage
        let multiplier = get_element_multiplier(Element::Fire, &resistances);
        assert_eq!(multiplier, 0.5);
    }

    /// Test element multiplier with immunity
    #[test]
    fn test_element_multiplier_immunity() {
        let mut resistances = HashMap::new();
        resistances.insert(Element::Ice, 1.0); // 1.0 = 0x damage (immune)
        let multiplier = get_element_multiplier(Element::Ice, &resistances);
        assert_eq!(multiplier, 0.0);
    }

    /// Test element multiplier with double damage (maximum weakness)
    #[test]
    fn test_element_multiplier_double_damage() {
        let mut resistances = HashMap::new();
        resistances.insert(Element::Lightning, -1.0); // -1.0 = 2.0x damage
        let multiplier = get_element_multiplier(Element::Lightning, &resistances);
        assert_eq!(multiplier, 2.0);
    }

    /// Test element multiplier with different element (no resistance data)
    #[test]
    fn test_element_multiplier_different_element() {
        let mut resistances = HashMap::new();
        resistances.insert(Element::Fire, 0.5); // Only Fire resistance
        let multiplier = get_element_multiplier(Element::Ice, &resistances);
        // Ice not in resistances, should return 1.0
        assert_eq!(multiplier, 1.0);
    }

    /// Test Element::default()
    #[test]
    fn test_element_default() {
        let element = Element::default();
        assert_eq!(element, Element::Physical);
    }

    /// Test element multiplier clamping (resistance > 1.0)
    #[test]
    fn test_element_multiplier_clamp_above() {
        let mut resistances = HashMap::new();
        resistances.insert(Element::Fire, 2.0); // > 1.0, should clamp to 0.0
        let multiplier = get_element_multiplier(Element::Fire, &resistances);
        assert_eq!(multiplier, 0.0);
    }

    /// Test element multiplier clamping (weakness < -1.0)
    #[test]
    fn test_element_multiplier_clamp_below() {
        let mut resistances = HashMap::new();
        resistances.insert(Element::Fire, -2.0); // < -1.0, should clamp to 2.0
        let multiplier = get_element_multiplier(Element::Fire, &resistances);
        assert_eq!(multiplier, 2.0);
    }

    /// Test element multiplier with exactly -1.0 (maximum weakness)
    #[test]
    fn test_element_multiplier_exact_max_weakness() {
        let mut resistances = HashMap::new();
        resistances.insert(Element::Fire, -1.0);
        let multiplier = get_element_multiplier(Element::Fire, &resistances);
        assert_eq!(multiplier, 2.0);
    }

    /// Test element multiplier with exactly 1.0 (immunity)
    #[test]
    fn test_element_multiplier_exact_immunity() {
        let mut resistances = HashMap::new();
        resistances.insert(Element::Ice, 1.0);
        let multiplier = get_element_multiplier(Element::Ice, &resistances);
        assert_eq!(multiplier, 0.0);
    }

    /// Test element multiplier with exactly 0.0 (normal damage)
    #[test]
    fn test_element_multiplier_exact_normal() {
        let mut resistances = HashMap::new();
        resistances.insert(Element::Physical, 0.0);
        let multiplier = get_element_multiplier(Element::Physical, &resistances);
        assert_eq!(multiplier, 1.0);
    }

    /// Test element multiplier with all element types
    #[test]
    fn test_element_multiplier_all_elements() {
        let mut resistances = HashMap::new();
        resistances.insert(Element::Physical, 0.0);
        resistances.insert(Element::Fire, -0.5);
        resistances.insert(Element::Ice, 0.5);
        resistances.insert(Element::Lightning, 1.0);

        assert_eq!(get_element_multiplier(Element::Physical, &resistances), 1.0);
        assert_eq!(get_element_multiplier(Element::Fire, &resistances), 1.5);
        assert_eq!(get_element_multiplier(Element::Ice, &resistances), 0.5);
        assert_eq!(get_element_multiplier(Element::Lightning, &resistances), 0.0);
    }

    /// Test element multiplier with partial resistance (0.25)
    #[test]
    fn test_element_multiplier_partial_resistance() {
        let mut resistances = HashMap::new();
        resistances.insert(Element::Fire, 0.25); // 0.75x damage
        let multiplier = get_element_multiplier(Element::Fire, &resistances);
        assert_eq!(multiplier, 0.75);
    }

    /// Test element multiplier with partial weakness (-0.25)
    #[test]
    fn test_element_multiplier_partial_weakness() {
        let mut resistances = HashMap::new();
        resistances.insert(Element::Fire, -0.25); // 1.25x damage
        let multiplier = get_element_multiplier(Element::Fire, &resistances);
        assert_eq!(multiplier, 1.25);
    }

    /// Test Element enum variants
    #[test]
    fn test_element_variants() {
        assert_eq!(Element::Physical, Element::Physical);
        assert_eq!(Element::Fire, Element::Fire);
        assert_eq!(Element::Ice, Element::Ice);
        assert_eq!(Element::Lightning, Element::Lightning);
        assert_ne!(Element::Physical, Element::Fire);
    }
}

