use crate::infrastructure::{
    components::combat::Skill,
    components::ui::SkillCooldownUI,
    components::{Health, Player, MP},
    resources::Score,
};
use bevy::prelude::*;

/// Inventory UI system
pub mod inventory;
/// Boss UI system
pub mod boss_ui;

/// Marker for the score text UI element
#[derive(Component)]
pub struct ScoreText;

/// Marker for the health text UI element
#[derive(Component)]
pub struct HealthText;

/// Marker for the MP text UI element
#[derive(Component)]
pub struct MPText;

/// Marker for the coin counter text UI element
#[derive(Component)]
pub struct CoinCountText;

/// Marker for the FPS counter text UI element
#[derive(Component)]
pub struct FpsText;

/// Marker for the Boss health bar UI element
#[derive(Component)]
pub struct BossHealthBar;

/// Marker for the Boss name text UI element
#[derive(Component)]
pub struct BossNameText;

/// Marker for the Boss phase indicator UI element
#[derive(Component)]
pub struct BossPhaseIndicator;

/// T053: Combo counter UI component
///
/// Displays combo count (e.g., "2 HIT COMBO", "3 HIT COMBO")
#[derive(Component)]
pub struct ComboCounter {
    /// Lifetime remaining (seconds) - fades out after combo ends
    pub lifetime: f32,
}

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

                            // MP display (using ASCII characters for compatibility)
                            parent.spawn((
                                Text::new("MP: [|||]"),
                                TextFont { font_size: 32.0, ..default() },
                                TextColor(Color::srgb(0.2, 0.6, 1.0)), // Blue
                                MPText,
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

            // Center combo display (T053)
            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    height: Val::Auto,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        Text::new(""),
                        TextFont { font_size: 48.0, ..default() },
                        TextColor(Color::srgb(1.0, 0.9, 0.0)), // Gold
                        ComboCounter { lifetime: 0.0 },
                        Visibility::Hidden, // Hidden by default
                    ));
                });

            // Boss health bar (T036) - displayed at top center when Boss is active
            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    height: Val::Auto,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    top: Val::Px(20.0),
                    ..default()
                })
                .with_children(|parent| {
                    // Boss name
                    parent.spawn((
                        Text::new(""),
                        TextFont { font_size: 36.0, ..default() },
                        TextColor(Color::srgb(1.0, 0.3, 0.3)), // Red
                        BossNameText,
                        Visibility::Hidden,
                    ));
                    
                    // Boss health bar (large, prominent)
                    parent.spawn((
                        Text::new(""),
                        TextFont { font_size: 32.0, ..default() },
                        TextColor(Color::srgb(1.0, 0.2, 0.2)), // Red
                        BossHealthBar,
                        Visibility::Hidden,
                    ));
                    
                    // Boss phase indicator
                    parent.spawn((
                        Text::new(""),
                        TextFont { font_size: 24.0, ..default() },
                        TextColor(Color::srgb(1.0, 0.8, 0.0)), // Gold
                        BossPhaseIndicator,
                        Visibility::Hidden,
                    ));
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
            // Display health bars using ASCII characters (M2: updated for f32 health)
            let percentage = health.health_percentage();
            let bars = if percentage >= 0.75 {
                "[|||]"
            } else if percentage >= 0.5 {
                "[|| ]"
            } else if percentage >= 0.25 {
                "[|  ]"
            } else if percentage > 0.0 {
                "[|  ]"
            } else {
                "[   ]"
            };
            **text = format!("HP: {bars} ({:.0}/{:.0})", health.current, health.max);
        }
    }
}

/// Update MP display
///
/// T094: MP bar UI system - displays current MP / max MP
pub fn update_mp_ui(
    player_query: Query<&MP, (With<Player>, Changed<MP>)>,
    mut ui_query: Query<&mut Text, With<MPText>>,
) {
    for mp in &player_query {
        for mut text in &mut ui_query {
            // Display MP bars using ASCII characters
            let percentage = mp.percentage();
            let bars = if percentage >= 0.75 {
                "[|||]"
            } else if percentage >= 0.5 {
                "[|| ]"
            } else if percentage >= 0.25 {
                "[|  ]"
            } else if percentage > 0.0 {
                "[|  ]"
            } else {
                "[   ]"
            };
            **text = format!("MP: {bars} ({:.0}/{:.0})", mp.current, mp.max);
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

/// T054: Combo UI update system
///
/// Listens for ComboExtended events and displays combo count (e.g., "2 HIT COMBO")
pub fn combo_ui_system(
    mut combo_extended_events: MessageReader<crate::infrastructure::events::combat::ComboExtended>,
    mut combo_query: Query<(&mut Text, &mut ComboCounter, &mut Visibility)>,
) {
    for event in combo_extended_events.read() {
        // Only show for 2+ hit combos
        if event.combo_count >= 2 {
            for (mut text, mut combo_counter, mut visibility) in &mut combo_query {
                **text = format!("{} HIT COMBO", event.combo_count);
                combo_counter.lifetime = 1.0; // 1 second lifetime
                *visibility = Visibility::Visible;
            }
        }
    }
}

/// T055: Combo UI fadeout system
///
/// Fades out combo display after 1 second when combo ends
pub fn combo_ui_fadeout_system(
    time: Res<Time>,
    mut combo_query: Query<(&mut ComboCounter, &mut Visibility, &mut TextColor)>,
) {
    let dt = time.delta_secs();

    for (mut combo_counter, mut visibility, mut text_color) in &mut combo_query {
        if combo_counter.lifetime > 0.0 {
            // Update lifetime
            combo_counter.lifetime -= dt;

            // Fade out alpha as lifetime decreases
            let alpha = (combo_counter.lifetime / 1.0).clamp(0.0, 1.0);
            text_color.0.set_alpha(alpha);

            // Hide when lifetime expires
            if combo_counter.lifetime <= 0.0 {
                *visibility = Visibility::Hidden;
                combo_counter.lifetime = 0.0;
            }
        }
    }
}

/// T093: Skill cooldown UI system
///
/// 技能冷却 UI 系统
///
/// Updates skill cooldown UI to display circular progress bar and remaining seconds.
/// For simplicity, we use text-based display showing cooldown progress and time remaining.
pub fn skill_cooldown_ui_system(
    player_query: Query<(Entity, &Skill), (With<Player>, Changed<Skill>)>,
    mut ui_query: Query<(&SkillCooldownUI, &mut Text), Without<Player>>,
) {
    for (_player_entity, skill) in &player_query {
        // Find matching UI element for this skill
        for (cooldown_ui, mut text) in &mut ui_query {
            if cooldown_ui.skill_id == skill.skill_id {
                // Update text to show cooldown progress
                if skill.is_ready() {
                    **text = format!("{}: READY", skill.skill_id);
                } else {
                    let progress = skill.cooldown_progress();
                    let remaining = skill.remaining_cooldown;
                    // Display as progress bar using ASCII: [====     ] 2.5s
                    let filled = (progress * 10.0) as usize;
                    let empty = 10 - filled;
                    let bar = "=".repeat(filled) + &" ".repeat(empty);
                    **text = format!("{}: [{bar}] {:.1}s", skill.skill_id, remaining);
                }
            }
        }
    }
}
