use crate::infrastructure::systems::enemy::{
    ai_state_machine_system, attack_cooldown_system, attack_system, chase_system,
    enemy_patrol_system, patrol_system, perception_system, return_to_patrol_system,
    spawn_slime_system,
};
/// Enemy system plugin
/// 敌人系统插件
use bevy::prelude::*;

use crate::infrastructure::events::enemy::{
    EnemyAttackTriggered, EnemyDetectedPlayer, EnemyLostTarget,
};
use crate::infrastructure::resources::enemy::AIConfig;

/// Enemy plugin - registers enemy systems
///
/// 敌人插件 - 注册敌人相关系统
///
/// Note: Enemy visual feedback systems (hit flash, death animation) are registered
/// in CombatPlugin to ensure proper execution order with damage systems.
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        // Register events (messages)
        app.add_message::<EnemyDetectedPlayer>()
            .add_message::<EnemyLostTarget>()
            .add_message::<EnemyAttackTriggered>();

        // Register resources (optional)
        app.init_resource::<AIConfig>();

        // Register systems
        app.add_systems(Startup, spawn_slime_system)
            // Legacy patrol system (old implementation)
            .add_systems(Update, enemy_patrol_system)
            // New AI systems (004-enemy-ai)
            .add_systems(
                Update,
                (
                    // Perception system runs first (throttled, every 3-5 frames)
                    perception_system,
                    // State machine updates based on perception
                    ai_state_machine_system,
                    // Behavior systems based on current state
                    (patrol_system, chase_system, attack_system, return_to_patrol_system)
                        .chain(),
                    // Cooldown management
                    attack_cooldown_system,
                )
                    .chain(),
            );

        info!("EnemyPlugin initialized with AI systems");
    }
}
