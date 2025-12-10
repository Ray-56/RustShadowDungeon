//! Boss configuration resource
//!
//! Resource for storing Boss definitions loaded from RON files.

use bevy::prelude::*;
use crate::domain::boss::phase::BossPhase;
use serde::{Deserialize, Serialize};

/// Boss definition (loaded from RON file)
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BossDefinition {
    /// Boss ID
    pub id: String,
    /// Boss name
    pub name: String,
    /// Maximum health
    pub max_health: f32,
    /// Phase configurations
    pub phases: Vec<BossPhase>,
}

/// RON file container structure
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BossDefinitionsContainer {
    /// All Boss definitions
    pub bosses: Vec<BossDefinition>,
}

/// Boss configuration resource
#[derive(Resource, Debug)]
pub struct BossConfig {
    /// All Boss definitions
    pub bosses: Vec<BossDefinition>,
}

impl BossConfig {
    /// Create a new empty Boss config
    pub fn new() -> Self {
        Self { bosses: Vec::new() }
    }

    /// Load Boss config from RON file
    ///
    /// # Arguments
    /// * `path` - Path to RON file (e.g., "assets/data/bosses.ron")
    ///
    /// # Returns
    /// Result containing BossConfig or error message
    pub fn load_from_file(path: &str) -> Result<Self, String> {
        let contents = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read Boss config file '{}': {}", path, e))?;

        let container: BossDefinitionsContainer = ron::from_str(&contents)
            .map_err(|e| format!("Failed to parse Boss config file '{}': {}", path, e))?;

        Ok(Self {
            bosses: container.bosses,
        })
    }

    /// Get Boss definition by ID
    pub fn get_boss(&self, boss_id: &str) -> Option<&BossDefinition> {
        self.bosses.iter().find(|b| b.id == boss_id)
    }
}

impl Default for BossConfig {
    fn default() -> Self {
        Self::new()
    }
}

