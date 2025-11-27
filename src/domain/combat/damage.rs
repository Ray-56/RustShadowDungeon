/// Damage calculation logic (pure functions)
/// 伤害计算逻辑（纯函数）
use super::element::{get_element_multiplier, Element};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Combat statistics for entities (attackers and defenders)
///
/// 实体战斗属性（攻击者和防御者共用）
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stats {
    /// Attack power (added to base damage)
    pub attack: f32,
    /// Defense value (reduces incoming damage)
    pub defense: f32,
    /// Critical hit rate (0.0 = 0%, 1.0 = 100%)
    pub crit_rate: f32,
    /// Critical hit multiplier (typically 2.0 for 2x damage)
    pub crit_multiplier: f32,
    /// Element resistances (-1.0 = 200% damage, 0.0 = 100%, 1.0 = 0% immune)
    pub element_resistances: HashMap<Element, f32>,
}

impl Stats {
    /// Create new Stats with default values
    pub fn new(attack: f32, defense: f32) -> Self {
        Self {
            attack,
            defense,
            crit_rate: 0.1, // 10% default crit rate
            crit_multiplier: 2.0,
            element_resistances: HashMap::new(),
        }
    }

    /// Create Stats with zero values
    pub fn zero() -> Self {
        Self {
            attack: 0.0,
            defense: 0.0,
            crit_rate: 0.0,
            crit_multiplier: 2.0,
            element_resistances: HashMap::new(),
        }
    }
}

impl Default for Stats {
    fn default() -> Self {
        Self::new(10.0, 0.0)
    }
}

/// Result of damage calculation
///
/// 伤害计算结果
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DamageResult {
    /// Final damage value (after all calculations, clamped to 1..=9999)
    pub final_damage: f32,
    /// Whether this was a critical hit
    pub is_critical: bool,
    /// Element type of the attack
    pub element: Element,
}

/// Calculate damage based on attacker/defender stats
///
/// 根据攻击者/防御者属性计算伤害
///
/// # Formula
/// 1. base_damage + attacker.attack
/// 2. Apply defense reduction: damage * (1 - defense / (defense + 100))
/// 3. Apply element multiplier based on resistances
/// 4. Apply critical hit if triggered
/// 5. Clamp to 1..=9999
///
/// # Arguments
/// * `base` - Base damage of the attack
/// * `attacker_stats` - Attacker's combat stats
/// * `defender_stats` - Defender's combat stats
/// * `element` - Element type of the attack
///
/// # Returns
/// DamageResult containing final damage, crit status, and element
///
/// # Examples
/// ```
/// use rust_shadow_dungeon::domain::combat::{calculate_damage, Stats, Element};
///
/// let attacker = Stats::new(10.0, 0.0);
/// let defender = Stats::new(0.0, 0.0);
///
/// let result = calculate_damage(10.0, &attacker, &defender, Element::Physical);
/// // Result: 10 (base) + 10 (attack) = 20 damage
/// ```
pub fn calculate_damage(
    base: f32,
    attacker_stats: &Stats,
    defender_stats: &Stats,
    element: Element,
) -> DamageResult {
    // Step 1: Base damage + attack power
    let mut damage = base + attacker_stats.attack;

    // Step 2: Apply defense reduction
    // Formula: damage * (1 - defense / (defense + 100))
    // This creates an asymptotic curve where defense never fully negates damage
    damage = apply_defense_reduction(damage, defender_stats.defense);

    // Step 3: Apply element multiplier
    let element_multiplier = get_element_multiplier(element, &defender_stats.element_resistances);
    damage *= element_multiplier;

    // Step 4: Roll for critical hit
    let is_critical = roll_critical_hit(attacker_stats.crit_rate);
    if is_critical {
        damage *= attacker_stats.crit_multiplier;
    }

    // Step 5: Clamp damage to valid range (1..=9999)
    damage = damage.clamp(1.0, 9999.0);

    DamageResult { final_damage: damage, is_critical, element }
}

/// Apply defense reduction to damage
///
/// 应用防御减伤
///
/// Formula: damage * (1 - defense / (defense + 100))
///
/// This creates an asymptotic curve:
/// - 0 defense → 100% damage
/// - 50 defense → 66.7% damage
/// - 100 defense → 50% damage
/// - 200 defense → 33.3% damage
/// - ∞ defense → approaches 0% but never reaches it
fn apply_defense_reduction(damage: f32, defense: f32) -> f32 {
    if defense <= 0.0 {
        return damage;
    }

    let reduction_factor = defense / (defense + 100.0);
    damage * (1.0 - reduction_factor)
}

/// Roll for critical hit based on crit rate
///
/// 根据暴击率判定是否暴击
///
/// # Arguments
/// * `crit_rate` - Critical hit rate (0.0 to 1.0)
///
/// # Returns
/// true if critical hit, false otherwise
fn roll_critical_hit(crit_rate: f32) -> bool {
    if crit_rate <= 0.0 {
        return false;
    }
    if crit_rate >= 1.0 {
        return true;
    }

    let mut rng = rand::thread_rng();
    rng.gen::<f32>() < crit_rate
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stats_new() {
        let stats = Stats::new(50.0, 20.0);
        assert_eq!(stats.attack, 50.0);
        assert_eq!(stats.defense, 20.0);
        assert_eq!(stats.crit_rate, 0.1);
        assert_eq!(stats.crit_multiplier, 2.0);
    }

    #[test]
    fn test_stats_zero() {
        let stats = Stats::zero();
        assert_eq!(stats.attack, 0.0);
        assert_eq!(stats.defense, 0.0);
        assert_eq!(stats.crit_rate, 0.0);
    }

    #[test]
    fn test_apply_defense_reduction() {
        // 50 defense → 66.7% damage
        let damage = apply_defense_reduction(100.0, 50.0);
        assert!((damage - 66.67).abs() < 0.1);

        // 0 defense → 100% damage
        let damage = apply_defense_reduction(100.0, 0.0);
        assert_eq!(damage, 100.0);

        // 100 defense → 50% damage
        let damage = apply_defense_reduction(100.0, 100.0);
        assert_eq!(damage, 50.0);
    }

    #[test]
    fn test_roll_critical_hit_zero_rate() {
        assert!(!roll_critical_hit(0.0));
    }

    #[test]
    fn test_roll_critical_hit_max_rate() {
        assert!(roll_critical_hit(1.0));
    }

    #[test]
    fn test_calculate_damage_basic() {
        let attacker = Stats::new(10.0, 0.0);
        let defender = Stats::new(0.0, 0.0);

        // Force no crit by setting rate to 0
        let mut attacker = attacker;
        attacker.crit_rate = 0.0;

        let result = calculate_damage(10.0, &attacker, &defender, Element::Physical);

        assert!((result.final_damage - 20.0).abs() < 0.01);
        assert!(!result.is_critical);
    }

    #[test]
    fn test_calculate_damage_with_defense() {
        let mut attacker = Stats::new(0.0, 0.0);
        attacker.crit_rate = 0.0;
        let defender = Stats::new(0.0, 50.0);

        let result = calculate_damage(100.0, &attacker, &defender, Element::Physical);

        // 100 * (1 - 50/150) = 66.67
        assert!((result.final_damage - 66.67).abs() < 0.1);
    }

    #[test]
    fn test_calculate_damage_with_element() {
        let mut attacker = Stats::new(0.0, 0.0);
        attacker.crit_rate = 0.0;

        let mut defender = Stats::new(0.0, 0.0);
        defender.element_resistances.insert(Element::Fire, -0.5);

        let result = calculate_damage(10.0, &attacker, &defender, Element::Fire);

        // 10 * 1.5 = 15.0
        assert!((result.final_damage - 15.0).abs() < 0.01);
    }

    #[test]
    fn test_calculate_damage_clamping() {
        let mut attacker = Stats::new(10000.0, 0.0);
        attacker.crit_rate = 1.0;
        attacker.crit_multiplier = 10.0;

        let defender = Stats::new(0.0, 0.0);

        let result = calculate_damage(10000.0, &attacker, &defender, Element::Physical);

        // Should be clamped to 9999
        assert_eq!(result.final_damage, 9999.0);
    }

    #[test]
    fn test_calculate_damage_minimum() {
        let mut attacker = Stats::new(0.0, 0.0);
        attacker.crit_rate = 0.0;

        let defender = Stats::new(0.0, 9999.0);

        let result = calculate_damage(0.1, &attacker, &defender, Element::Physical);

        // Should be clamped to minimum 1.0
        assert_eq!(result.final_damage, 1.0);
    }
}
