//! ECS Events - system-to-system communication
//!
//! Events enable decoupled communication between systems.

pub mod movement;

pub use movement::{PlayerMoved, StateChanged};
