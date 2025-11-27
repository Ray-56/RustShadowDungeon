use serde::{Deserialize, Serialize};
/// Element types for combat system
/// 元素类型定义
use std::collections::HashMap;

/// Element types for damage calculation
///
/// 元素类型：
/// - Physical: 物理伤害（无元素）
/// - Fire: 火元素
/// - Ice: 冰元素  
/// - Lightning: 雷元素
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Element {
    Physical,
    Fire,
    Ice,
    Lightning,
}

impl Default for Element {
    fn default() -> Self {
        Element::Physical
    }
}

/// Calculate element damage multiplier based on resistances
///
/// 根据抗性计算元素伤害修正
///
/// # Arguments
/// * `element` - 攻击元素类型
/// * `resistances` - 目标元素抗性表 (-1.0 = 200% 伤害弱点, 0.0 = 100% 正常, 1.0 = 0% 完全免疫)
///
/// # Returns
/// Damage multiplier (0.0 to 2.0)
///
/// # Examples
/// ```
/// use std::collections::HashMap;
/// use rust_shadow_dungeon::domain::combat::element::*;
///
/// let mut resistances = HashMap::new();
/// resistances.insert(Element::Fire, -0.5); // 火弱点：1.5x 伤害
///
/// let multiplier = get_element_multiplier(Element::Fire, &resistances);
/// assert_eq!(multiplier, 1.5);
/// ```
pub fn get_element_multiplier(element: Element, resistances: &HashMap<Element, f32>) -> f32 {
    match resistances.get(&element) {
        Some(&resistance) => {
            // resistance = -1.0 → multiplier = 2.0 (double damage, weakness)
            // resistance = 0.0 → multiplier = 1.0 (normal damage)
            // resistance = 0.5 → multiplier = 0.5 (half damage, resistance)
            // resistance = 1.0 → multiplier = 0.0 (immune)
            (1.0 - resistance).clamp(0.0, 2.0)
        },
        None => 1.0, // No resistance data = normal damage
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_multiplier_no_resistance() {
        let resistances = HashMap::new();
        let multiplier = get_element_multiplier(Element::Fire, &resistances);
        assert_eq!(multiplier, 1.0);
    }

    #[test]
    fn test_element_multiplier_weakness() {
        let mut resistances = HashMap::new();
        resistances.insert(Element::Fire, -0.5);
        let multiplier = get_element_multiplier(Element::Fire, &resistances);
        assert_eq!(multiplier, 1.5);
    }

    #[test]
    fn test_element_multiplier_resistance() {
        let mut resistances = HashMap::new();
        resistances.insert(Element::Fire, 0.5);
        let multiplier = get_element_multiplier(Element::Fire, &resistances);
        assert_eq!(multiplier, 0.5);
    }

    #[test]
    fn test_element_multiplier_immunity() {
        let mut resistances = HashMap::new();
        resistances.insert(Element::Ice, 1.0);
        let multiplier = get_element_multiplier(Element::Ice, &resistances);
        assert_eq!(multiplier, 0.0);
    }

    #[test]
    fn test_element_multiplier_double_damage() {
        let mut resistances = HashMap::new();
        resistances.insert(Element::Lightning, -1.0);
        let multiplier = get_element_multiplier(Element::Lightning, &resistances);
        assert_eq!(multiplier, 2.0);
    }
}
