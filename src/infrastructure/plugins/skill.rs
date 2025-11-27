/// Skill system plugin
///
/// 技能系统插件
///
/// Registers all skill-related systems and resources.
use bevy::prelude::*;

use crate::infrastructure::resources::SkillDatabase;
use crate::infrastructure::systems::skill::{
    fireball_collision_system, fireball_movement_system, lifetime_system, mp_consumption_system,
    mp_regeneration_system, projectile_system, skill_activation_system, skill_cooldown_system,
    skill_input_system,
};

/// Skill system plugin
///
/// 技能系统插件
///
/// T095: Registers skill-related systems and resources.
pub struct SkillPlugin;

impl Plugin for SkillPlugin {
    fn build(&self, app: &mut App) {
        // Register resources
        app.insert_resource(SkillDatabase::default());

        // Register systems
        // Note: skill_input_system should run after player systems to ensure player components exist
        app.add_systems(
            Update,
            (
                // 1. Skill input system (listens for K key) - T082
                // Run after player input system to ensure input is processed correctly
                skill_input_system
                    .after(crate::infrastructure::systems::input::player_input_system),
                // 2. Skill cooldown system (updates cooldown timers) - T083
                skill_cooldown_system,
                // 3. MP regeneration system (restores MP over time)
                mp_regeneration_system,
                // 4. Skill activation system (sets cooldown on activation) - T085
                skill_activation_system,
                // 5. MP consumption system (deducts MP on skill use) - T084
                mp_consumption_system,
                // 6. Projectile system (spawns fireballs) - T087
                projectile_system,
                // 7. Fireball movement system (updates fireball position) - T087
                fireball_movement_system,
                // 8. Fireball collision system (detects hits) - T088
                fireball_collision_system,
                // 9. Lifetime system (despawns expired entities) - T089
                lifetime_system,
            )
                .chain(),
        );

        info!("SkillPlugin initialized");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skill_plugin_builds() {
        let mut app = App::new();
        app.add_plugins(SkillPlugin);

        // Verify resource is inserted
        assert!(app.world().get_resource::<SkillDatabase>().is_some());
    }
}
