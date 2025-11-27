//! Collectible items (coins, powerups, etc.)

use bevy::prelude::*;

/// Coin collectible marker
#[derive(Component, Debug)]
pub struct Coin {
    /// Point value of this coin
    pub value: u32,
}

impl Coin {
    /// Create a standard coin (worth 1 point)
    #[must_use]
    pub const fn standard() -> Self {
        Self { value: 1 }
    }

    /// Create a valuable coin (worth 5 points)
    #[must_use]
    pub const fn valuable() -> Self {
        Self { value: 5 }
    }
}





