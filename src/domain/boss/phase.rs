//! Boss phase definition
//!
//! Pure domain logic for Boss phase configuration.

use serde::{Deserialize, Serialize};

/// Boss phase definition (pure data structure)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct BossPhase {
    /// Phase index (0-based)
    pub phase_index: usize,
    /// Health threshold (0.0-1.0)
    pub health_threshold: f32,
    /// Invulnerability duration (seconds, 1-2 seconds)
    pub invulnerability_duration: f32,
    /// Available skill IDs
    pub skill_ids: Vec<String>,
    /// Attack frequency (attacks per second)
    pub attack_frequency: f32,
    /// Movement speed (pixels per second)
    pub move_speed: f32,
}

impl BossPhase {
    /// Check if this phase threshold is reached
    pub fn is_threshold_reached(&self, health_percentage: f32) -> bool {
        health_percentage <= self.health_threshold
    }
}

