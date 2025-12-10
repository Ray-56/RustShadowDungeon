//! Boss UI systems
//!
//! UI systems for displaying Boss health bar and phase information.

use bevy::prelude::*;
use bevy::ecs::message::MessageReader;
use crate::infrastructure::components::{Health, boss::{Boss, BossController, BossId}};
use crate::infrastructure::events::boss::{BossEncounterStarted, BossPhaseTransition};
use crate::infrastructure::resources::boss_config::BossConfig;
use super::{BossHealthBar, BossNameText, BossPhaseIndicator};

/// T036: Boss health bar UI system
///
/// Displays large Boss health bar at top center when Boss encounter is active.
/// Shows Boss name, health percentage, and current phase.
pub fn update_boss_health_ui(
    mut boss_encounter_events: MessageReader<BossEncounterStarted>,
    mut _boss_phase_events: MessageReader<BossPhaseTransition>,
    boss_query: Query<(&Health, &BossController, &BossId), With<Boss>>,
    mut name_query: Query<&mut Visibility, (With<BossNameText>, Without<BossPhaseIndicator>)>,
    mut health_query: Query<&mut Text, (With<BossHealthBar>, Without<BossPhaseIndicator>)>,
    mut phase_query: Query<&mut Text, (With<BossPhaseIndicator>, Without<BossHealthBar>)>,
    mut phase_visibility: Query<&mut Visibility, (With<BossPhaseIndicator>, Without<BossNameText>)>,
    boss_config: Option<Res<BossConfig>>,
) {
    // Show UI when Boss encounter starts
    let mut should_show = false;
    for _event in boss_encounter_events.read() {
        should_show = true;
        info!("BossEncounterStarted event received, showing Boss UI");
    }

    // Update health bar for active Boss
    if !boss_query.is_empty() {
        should_show = true;
    }
    
    if should_show {
        for mut vis in &mut name_query {
            *vis = Visibility::Visible;
        }
        for mut vis in &mut phase_visibility {
            *vis = Visibility::Visible;
        }
    }
    
    for (health, controller, boss_id) in boss_query.iter() {
        // Show health bar
        for mut vis in &mut name_query {
            *vis = Visibility::Visible;
        }
        for mut vis in &mut phase_visibility {
            *vis = Visibility::Visible;
        }

        // Get Boss name from config
        let boss_name = boss_config
            .as_ref()
            .and_then(|c| c.get_boss(&boss_id.0))
            .map(|b| b.name.clone())
            .unwrap_or_else(|| boss_id.0.clone());

        // Update health bar (large, prominent display)
        for mut text in &mut health_query {
            let percentage = health.health_percentage();
            let filled = (percentage * 20.0) as usize; // 20 character bar
            let empty = 20 - filled;
            let bar = "█".repeat(filled) + &"░".repeat(empty);
            let health_text = format!("{}: [{bar}] {:.0}/{:.0} ({:.0}%)", boss_name, health.current, health.max, percentage * 100.0);
            **text = health_text.clone();
            info!("Updated Boss health bar: {}", health_text);
        }

        // Update phase indicator
        for mut text in &mut phase_query {
            let phase_num = controller.current_phase + 1;
            let total_phases = controller.phases.len();
            **text = format!("Phase {}/{}", phase_num, total_phases);
        }
    }

    // Hide UI if no Boss exists
    if boss_query.is_empty() {
        for mut vis in &mut name_query {
            *vis = Visibility::Hidden;
        }
        for mut vis in &mut phase_visibility {
            *vis = Visibility::Hidden;
        }
    }
}

