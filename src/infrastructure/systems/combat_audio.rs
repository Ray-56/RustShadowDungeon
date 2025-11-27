//! Combat audio system
//!
//! 战斗音效系统
//!
//! Plays sound effects for combat actions based on hit type and combo state.
//!
//! NOTE: Audio system implementation requires bevy_kira_audio plugin to be initialized.
//! Add `bevy_kira_audio::AudioPlugin` to your app plugins.

use bevy::prelude::*;

use crate::domain::combat::ComboState;
use crate::infrastructure::components::combat::Combo;
use crate::infrastructure::events::combat::DamageDealt;

/// Combat audio system
///
/// 战斗音效系统
///
/// T070: Listens to DamageDealt events and plays appropriate sound effects:
/// - Light hit: hit_light.ogg
/// - Heavy hit (third combo): hit_heavy.ogg
/// - Critical hit: hit_critical.ogg
///
/// TODO: Implement audio playback once bevy_kira_audio plugin is properly configured.
/// Current implementation is a placeholder that selects the correct sound path.
pub fn combat_audio_system(
    mut damage_events: MessageReader<DamageDealt>,
    // TODO: Add audio resource when bevy_kira_audio is configured
    // audio: Res<bevy_kira_audio::Audio>,
    _asset_server: Res<AssetServer>,
    combo_query: Query<&Combo>,
) {
    for event in damage_events.read() {
        // Determine if this is a heavy hit (third combo hit)
        let is_heavy = combo_query
            .get(event.source)
            .map(|combo| combo.state == ComboState::ThirdHit)
            .unwrap_or(false);

        // Select sound effect based on hit type
        let sound_path = if event.result.is_critical {
            "audio/combat/hit_critical.ogg"
        } else if is_heavy {
            "audio/combat/hit_heavy.ogg"
        } else {
            "audio/combat/hit_light.ogg"
        };

        // TODO: Play sound effect once audio system is configured
        // audio
        //     .play(asset_server.load(sound_path))
        //     .with_volume(0.5);

        // For now, just log the sound that would be played (debug only)
        #[cfg(debug_assertions)]
        info!("Would play sound: {}", sound_path);
    }
}
