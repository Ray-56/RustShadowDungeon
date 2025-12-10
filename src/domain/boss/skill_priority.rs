//! Skill priority logic
//!
//! Pure domain logic for Boss skill selection and priority management.

/// Skill priority level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SkillPriority {
    /// Low priority
    Low = 1,
    /// Medium priority
    Medium = 2,
    /// High priority
    High = 3,
    /// Critical priority
    Critical = 4,
}

/// Skill cooldown state
#[derive(Debug, Clone)]
pub struct SkillCooldown {
    /// Skill ID
    pub skill_id: String,
    /// Remaining cooldown time (seconds)
    pub cooldown_remaining: f32,
    /// Priority level
    pub priority: SkillPriority,
}

impl SkillCooldown {
    /// Check if skill is available
    pub fn is_available(&self) -> bool {
        self.cooldown_remaining <= 0.0
    }
}

/// Select next available skill based on priority and randomness
///
/// # Arguments
/// * `available_skills` - List of available skills with cooldown info
/// * `randomness_factor` - Randomness factor (0.0 = fully priority-based, 1.0 = fully random)
///
/// # Returns
/// Selected skill ID, or None if no skills available
pub fn select_next_skill(
    available_skills: &[SkillCooldown],
    randomness_factor: f32,
) -> Option<String> {
    // Filter to only available skills
    let available: Vec<_> = available_skills
        .iter()
        .filter(|s| s.is_available())
        .collect();

    if available.is_empty() {
        return None;
    }

    // Simple implementation: if randomness is high, pick random; otherwise pick highest priority
    // TODO: Implement weighted random selection based on priority
    if randomness_factor > 0.5 {
        // Random selection (simplified - always picks first for now)
        available.first().map(|s| s.skill_id.clone())
    } else {
        // Priority-based selection (highest priority first)
        available
            .iter()
            .max_by_key(|s| s.priority)
            .map(|s| s.skill_id.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skill_cooldown_available() {
        let skill = SkillCooldown {
            skill_id: "test_skill".to_string(),
            cooldown_remaining: 0.0,
            priority: SkillPriority::Medium,
        };
        assert!(skill.is_available());

        let skill_on_cooldown = SkillCooldown {
            skill_id: "test_skill".to_string(),
            cooldown_remaining: 5.0,
            priority: SkillPriority::Medium,
        };
        assert!(!skill_on_cooldown.is_available());
    }

    #[test]
    fn test_select_next_skill_no_available() {
        let skills = vec![
            SkillCooldown {
                skill_id: "skill1".to_string(),
                cooldown_remaining: 5.0,
                priority: SkillPriority::High,
            },
        ];
        assert_eq!(select_next_skill(&skills, 0.0), None);
    }

    #[test]
    fn test_select_next_skill_priority_based() {
        let skills = vec![
            SkillCooldown {
                skill_id: "low_skill".to_string(),
                cooldown_remaining: 0.0,
                priority: SkillPriority::Low,
            },
            SkillCooldown {
                skill_id: "high_skill".to_string(),
                cooldown_remaining: 0.0,
                priority: SkillPriority::High,
            },
        ];
        let selected = select_next_skill(&skills, 0.0);
        assert_eq!(selected, Some("high_skill".to_string()));
    }
}

