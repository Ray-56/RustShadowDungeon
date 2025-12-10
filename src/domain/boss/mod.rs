//! Boss domain logic - pure Rust business logic
//!
//! This module contains Boss-specific domain logic with zero dependencies on Bevy.
//! All domain logic is implemented as pure functions for maximum testability.

pub mod phase;
pub mod phase_transition;
pub mod skill_priority;
pub mod telegraph;

