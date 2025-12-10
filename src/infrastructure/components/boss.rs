//! Boss components
//!
//! Boss-specific ECS components for the Boss encounter system.

use bevy::prelude::*;
use crate::domain::boss::{phase::BossPhase, telegraph::TelegraphArea};

/// Boss marker component
#[derive(Component, Debug)]
pub struct Boss;

/// Boss ID for identification
#[derive(Component, Debug, Clone, PartialEq, Eq, Hash)]
pub struct BossId(pub String);

/// Boss controller component - core AI controller for Boss entities
#[derive(Component, Debug, Clone)]
pub struct BossController {
    /// Current phase index (0-based)
    pub current_phase: usize,
    /// All phase configurations
    pub phases: Vec<BossPhase>,
    /// Health lock flag (prevents damage during phase transitions)
    pub health_lock: bool,
    /// Phase transition start time (seconds)
    pub transition_start_time: Option<f32>,
    /// Whether currently transitioning phases
    pub is_transitioning: bool,
}

impl BossController {
    /// Create a new Boss controller
    pub fn new(phases: Vec<BossPhase>) -> Self {
        Self {
            current_phase: 0,
            phases,
            health_lock: false,
            transition_start_time: None,
            is_transitioning: false,
        }
    }

    /// Get current phase configuration
    pub fn current_phase_config(&self) -> Option<&BossPhase> {
        self.phases.get(self.current_phase)
    }

    /// Get next phase threshold
    pub fn get_next_phase_threshold(&self) -> Option<f32> {
        if self.current_phase + 1 < self.phases.len() {
            Some(self.phases[self.current_phase + 1].health_threshold)
        } else {
            None
        }
    }
}

/// Telegraph component - skill warning area marker
#[derive(Component, Debug, Clone)]
pub struct Telegraph {
    /// Associated skill ID
    pub skill_id: String,
    /// Warning area definition
    pub area: TelegraphArea,
    /// Warning duration (seconds, at least 0.5s)
    pub warning_duration: f32,
    /// Elapsed time (seconds)
    pub elapsed_time: f32,
    /// Actual damage area (for verification)
    pub damage_area: TelegraphArea,
}

/// Boss skill state component (attached to Boss entity)
#[derive(Component, Debug, Clone)]
pub struct BossSkill {
    /// Skill ID
    pub skill_id: String,
    /// Remaining cooldown time (seconds)
    pub cooldown_remaining: f32,
    /// Priority (1-4)
    pub priority: u8,
    /// Whether on cooldown
    pub is_on_cooldown: bool,
}

impl BossSkill {
    /// Check if skill is available
    pub fn is_available(&self) -> bool {
        !self.is_on_cooldown && self.cooldown_remaining <= 0.0
    }
}

