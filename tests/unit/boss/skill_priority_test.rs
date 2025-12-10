//! Unit tests for Boss skill priority logic
//!
//! Tests the domain layer skill selection logic.

use rust_shadow_dungeon::domain::boss::skill_priority::{
    select_next_skill, SkillCooldown, SkillPriority,
};

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
    let skills = vec![SkillCooldown {
        skill_id: "skill1".to_string(),
        cooldown_remaining: 5.0,
        priority: SkillPriority::High,
    }];
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

#[test]
fn test_select_next_skill_filters_on_cooldown() {
    let skills = vec![
        SkillCooldown {
            skill_id: "on_cooldown".to_string(),
            cooldown_remaining: 5.0,
            priority: SkillPriority::High,
        },
        SkillCooldown {
            skill_id: "available".to_string(),
            cooldown_remaining: 0.0,
            priority: SkillPriority::Low,
        },
    ];
    let selected = select_next_skill(&skills, 0.0);
    assert_eq!(selected, Some("available".to_string()));
}

