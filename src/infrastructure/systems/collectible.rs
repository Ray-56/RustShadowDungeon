//! Collectible systems (coins, powerups)

use bevy::prelude::*;

use crate::infrastructure::{
    components::{Coin, Player},
    resources::Score,
};

/// Collect coins when player touches them
///
/// Optimized: Uses squared distance to avoid sqrt calculation
pub fn coin_collection_system(
    mut commands: Commands,
    mut score: ResMut<Score>,
    player_query: Query<&Transform, With<Player>>,
    coin_query: Query<(Entity, &Transform, &Coin)>,
) {
    // Early exit if no coins
    if coin_query.is_empty() {
        return;
    }

    for player_transform in &player_query {
        let player_pos = player_transform.translation.truncate();
        const COLLECT_DISTANCE_SQ: f32 = 30.0 * 30.0; // 900.0 (squared to avoid sqrt)

        for (coin_entity, coin_transform, coin) in &coin_query {
            let coin_pos = coin_transform.translation.truncate();
            let distance_sq = player_pos.distance_squared(coin_pos);

            // Collect if player is close enough (using squared distance)
            if distance_sq < COLLECT_DISTANCE_SQ {
                score.collect_coin(coin.value);
                commands.entity(coin_entity).despawn();
            }
        }
    }
}
