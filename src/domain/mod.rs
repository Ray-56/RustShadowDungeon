//! Domain layer - pure Rust business logic
//!
//! This module contains the core game logic with zero dependencies on Bevy.
//! All domain logic is implemented as pure functions for maximum testability.

// Movement domain logic (M1 - Player Movement)
pub mod movement;

// Combat domain logic (M2 - Combat System Core)
pub mod combat;
