/// Skill database resource
/// 技能数据库资源
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::domain::combat::Element;

/// Skill data structure (loaded from RON file)
///
/// 技能数据结构（从 RON 文件加载）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillData {
    /// Skill ID (e.g., "fireball")
    /// 技能 ID
    pub id: String,
    /// Cooldown duration (seconds)
    /// 冷却时间（秒）
    pub cooldown: f32,
    /// MP cost to activate
    /// MP 消耗
    pub mp_cost: f32,
    /// Base damage
    /// 基础伤害
    pub damage: f32,
    /// Element type
    /// 元素类型
    pub element: Element,
}

/// Skill database resource
///
/// 技能数据库资源
///
/// T079: Loads skill configurations from `assets/data/skills.ron`.
/// Provides lookup by skill ID.
#[derive(Resource, Debug, Clone)]
pub struct SkillDatabase {
    /// Map of skill ID to skill data
    /// 技能 ID 到技能数据的映射
    skills: HashMap<String, SkillData>,
}

impl SkillDatabase {
    /// Create a new empty skill database
    pub fn new() -> Self {
        Self { skills: HashMap::new() }
    }

    /// Load skill database from RON file
    ///
    /// # Arguments
    /// * `path` - Path to RON file (e.g., "skills.ron")
    ///
    /// # Returns
    /// Result containing SkillDatabase or error message
    pub fn load_from_file(path: &str) -> Result<Self, String> {
        let contents = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read skill database file '{}': {}", path, e))?;

        let skills: Vec<SkillData> = ron::from_str(&contents)
            .map_err(|e| format!("Failed to parse skill database file '{}': {}", path, e))?;

        let mut skill_map = HashMap::new();
        for skill in skills {
            skill_map.insert(skill.id.clone(), skill);
        }

        Ok(Self { skills: skill_map })
    }

    /// Get skill data by ID
    ///
    /// # Arguments
    /// * `skill_id` - Skill ID to look up
    ///
    /// # Returns
    /// Option containing SkillData if found
    pub fn get(&self, skill_id: &str) -> Option<&SkillData> {
        self.skills.get(skill_id)
    }

    /// Check if skill exists
    pub fn has_skill(&self, skill_id: &str) -> bool {
        self.skills.contains_key(skill_id)
    }

    /// Get all skill IDs
    pub fn skill_ids(&self) -> Vec<&String> {
        self.skills.keys().collect()
    }
}

impl Default for SkillDatabase {
    fn default() -> Self {
        // Create default database with fireball skill
        let mut skills = HashMap::new();
        skills.insert(
            "fireball".to_string(),
            SkillData {
                id: "fireball".to_string(),
                cooldown: 5.0,
                mp_cost: 20.0,
                damage: 30.0,
                element: Element::Fire,
            },
        );
        Self { skills }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skill_database_new() {
        let db = SkillDatabase::new();
        assert!(db.skills.is_empty());
    }

    #[test]
    fn test_skill_database_default() {
        let db = SkillDatabase::default();
        assert!(db.has_skill("fireball"));

        if let Some(skill) = db.get("fireball") {
            assert_eq!(skill.cooldown, 5.0);
            assert_eq!(skill.mp_cost, 20.0);
            assert_eq!(skill.damage, 30.0);
            assert_eq!(skill.element, Element::Fire);
        } else {
            panic!("Fireball skill not found in default database");
        }
    }

    #[test]
    fn test_skill_database_get() {
        let db = SkillDatabase::default();

        assert!(db.get("fireball").is_some());
        assert!(db.get("nonexistent").is_none());
    }
}
