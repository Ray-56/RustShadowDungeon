//! Score tracking resource

use bevy::prelude::*;

/// Game score tracker
#[derive(Resource, Debug, Default)]
pub struct Score {
    /// Current score
    pub points: u32,
    /// Coins collected
    pub coins_collected: u32,
}

impl Score {
    /// Add points to score
    pub fn add_points(&mut self, points: u32) {
        self.points += points;
    }

    /// Collect a coin
    pub fn collect_coin(&mut self, value: u32) {
        self.coins_collected += 1;
        self.add_points(value);
        info!("Coin collected! Score: {} (+{})", self.points, value);
    }
}
