//! Boss plugin
//!
//! Bevy plugin for Boss encounter system.

use bevy::prelude::*;
use crate::infrastructure::systems::boss_systems;

/// Boss encounter system plugin
pub struct BossPlugin;

impl Plugin for BossPlugin {
    fn build(&self, app: &mut App) {
        // Initialize BossConfig resource
        app.init_resource::<crate::infrastructure::resources::boss_config::BossConfig>();
        
        // Load Boss config from file in Startup
        app.add_systems(Startup, load_boss_config_system);
        
        // Register events (Bevy 0.17 uses add_message instead of add_event)
        app.add_message::<crate::infrastructure::events::boss::BossEncounterStarted>()
            .add_message::<crate::infrastructure::events::boss::BossPhaseTransition>()
            .add_message::<crate::infrastructure::events::boss::BossDefeated>();

        // Register systems
        app.add_systems(
            Update,
            (
                // Phase 1 & 2: Setup and Foundational
                boss_systems::spawn_boss_system,
                boss_systems::check_boss_activation_system,
                // Phase 3: User Story 1
                boss_systems::check_boss_death_system,
                boss_systems::lock_boss_room_doors_system,
                boss_systems::unlock_boss_room_doors_system,
                // Phase 4: User Story 2
                boss_systems::check_phase_threshold_system,
                boss_systems::lock_boss_health_system,
                boss_systems::manage_invulnerability_system,
                boss_systems::unlock_boss_health_system,
                // Phase 5: User Story 3 - Skill systems
                boss_systems::update_skill_cooldowns_system,
                boss_systems::select_boss_skill_system,
                boss_systems::update_telegraph_timer_system,
                boss_systems::animate_telegraph_flash_system,
                boss_systems::execute_boss_skill_system,
                boss_systems::cleanup_expired_telegraphs_system,
                // Phase 6: Polish
                boss_systems::reset_boss_on_player_death_system,
                boss_systems::check_boss_out_of_combat_system,
            ),
        )
        .add_systems(Update, crate::infrastructure::systems::ui::boss_ui::update_boss_health_ui);
    }
}

/// Load Boss config from RON file
fn load_boss_config_system(mut boss_config: ResMut<crate::infrastructure::resources::boss_config::BossConfig>) {
    match crate::infrastructure::resources::boss_config::BossConfig::load_from_file("assets/data/bosses.ron") {
        Ok(config) => {
            *boss_config = config;
            info!("Boss config loaded successfully: {} boss(es)", boss_config.bosses.len());
        }
        Err(e) => {
            warn!("Failed to load Boss config: {}. Using empty config.", e);
        }
    }
}

