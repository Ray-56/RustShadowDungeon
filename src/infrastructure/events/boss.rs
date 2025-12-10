//! Boss events
//!
//! Events for Boss encounter system communication.

use bevy::prelude::*;
use bevy::ecs::message::Message;

/// Boss encounter started event
#[derive(Message, Debug, Clone)]
pub struct BossEncounterStarted {
    /// Boss entity
    pub boss_entity: Entity,
    /// Boss ID
    pub boss_id: String,
}

/// Boss phase transition event
#[derive(Message, Debug, Clone)]
pub struct BossPhaseTransition {
    /// Boss entity
    pub boss_entity: Entity,
    /// From phase index
    pub from_phase: usize,
    /// To phase index
    pub to_phase: usize,
}

/// Boss defeated event
#[derive(Message, Debug, Clone)]
pub struct BossDefeated {
    /// Boss entity
    pub boss_entity: Entity,
    /// Boss ID
    pub boss_id: String,
}

