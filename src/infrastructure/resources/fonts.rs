//! Font resources for UI text rendering

use bevy::prelude::*;

/// Font handles for UI text
#[derive(Resource)]
pub struct GameFonts {
    /// Main UI font (supports Chinese characters)
    pub ui_font: Handle<Font>,
}

impl GameFonts {
    /// Load fonts from asset server
    pub fn load(asset_server: &AssetServer) -> Self {
        Self {
            // Try to load a Chinese font
            // Bevy's default font doesn't support Chinese, so we load system font
            ui_font: asset_server.load("fonts/ui_font.ttf"),
        }
    }
}



