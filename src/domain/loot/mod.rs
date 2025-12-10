//! Loot and inventory domain logic
//!
//! 战利品与库存领域逻辑
//!
//! This module contains pure Rust functions for loot drop calculation,
//! inventory management, item stacking, and pickup range detection.
//! All functions have ZERO dependencies on Bevy or any game engine.

pub mod drop_table;
pub mod inventory;
pub mod pickup;
pub mod stacking;
