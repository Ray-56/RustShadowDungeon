use crate::infrastructure::{
    components::{Health, Player},
    resources::Score,
};
use bevy::prelude::*;

/// Marker for the score text UI element
#[derive(Component)]
pub struct ScoreText;

/// Marker for the health text UI element
#[derive(Component)]
pub struct HealthText;

/// Marker for the coin counter text UI element
#[derive(Component)]
pub struct CoinCountText;

/// Marker for the FPS counter text UI element
#[derive(Component)]
pub struct FpsText;

/// Setup the game UI (HUD)
pub fn setup_ui(mut commands: Commands) {
    // Root UI container (full screen, top-left anchored)
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceBetween,
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(20.0)),
            ..default()
        })
        .with_children(|parent| {
            // Top bar (score, health, coins)
            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    height: Val::Auto,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::FlexStart,
                    ..default()
                })
                .with_children(|parent| {
                    // Left side (Score + Coins)
                    parent
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(8.0),
                            ..default()
                        })
                        .with_children(|parent| {
                            // Score display
                            parent.spawn((
                                Text::new("SCORE: 0"),
                                TextFont { font_size: 32.0, ..default() },
                                TextColor(Color::srgb(1.0, 0.95, 0.3)), // Gold color
                                ScoreText,
                            ));

                            // Coin count display
                            parent.spawn((
                                Text::new("COINS: 0/7"),
                                TextFont { font_size: 28.0, ..default() },
                                TextColor(Color::srgb(1.0, 0.84, 0.0)), // Gold
                                CoinCountText,
                            ));
                        });

                    // Right side (Health)
                    parent
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::FlexEnd,
                            row_gap: Val::Px(8.0),
                            ..default()
                        })
                        .with_children(|parent| {
                            // Health display (using ASCII characters for compatibility)
                            parent.spawn((
                                Text::new("HP: [|||]"),
                                TextFont { font_size: 32.0, ..default() },
                                TextColor(Color::srgb(1.0, 0.2, 0.2)), // Red
                                HealthText,
                            ));

                            // FPS counter (debug)
                            parent.spawn((
                                Text::new("FPS: --"),
                                TextFont { font_size: 24.0, ..default() },
                                TextColor(Color::srgb(0.5, 0.8, 0.5)), // Green
                                FpsText,
                            ));
                        });
                });
        });

    info!("UI system initialized");
}

/// Update score display
pub fn update_score_ui(score: Res<Score>, mut query: Query<&mut Text, With<ScoreText>>) {
    if score.is_changed() {
        for mut text in &mut query {
            **text = format!("SCORE: {}", score.points);
        }
    }
}

/// Update health display
pub fn update_health_ui(
    player_query: Query<&Health, (With<Player>, Changed<Health>)>,
    mut ui_query: Query<&mut Text, With<HealthText>>,
) {
    for health in &player_query {
        for mut text in &mut ui_query {
            // Display health bars using ASCII characters
            let bars = match health.current {
                2 => "[|| ]",
                1 => "[|  ]",
                0 => "[   ]",
                _ => "[|||]",
            };
            **text = format!("HP: {bars}");
        }
    }
}

/// Update coin counter
pub fn update_coin_count_ui(score: Res<Score>, mut query: Query<&mut Text, With<CoinCountText>>) {
    if score.is_changed() {
        for mut text in &mut query {
            // Display collected coins count
            let total_coins = 7;
            let collected = score.coins_collected.min(total_coins);
            **text = format!("COINS: {collected}/{total_coins}");
        }
    }
}

/// Update FPS counter
pub fn update_fps_ui(
    diagnostics: Res<bevy::diagnostic::DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(&bevy::diagnostic::FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                **text = format!("FPS: {value:.0}");
            }
        }
    }
}
