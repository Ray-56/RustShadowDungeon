use crate::infrastructure::systems::enemy::{enemy_patrol_system, spawn_slime_system};
/// Enemy system plugin
/// 敌人系统插件
use bevy::prelude::*;

/// Enemy plugin - registers enemy systems
///
/// 敌人插件 - 注册敌人相关系统
///
/// Note: Enemy visual feedback systems (hit flash, death animation) are registered
/// in CombatPlugin to ensure proper execution order with damage systems.
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_slime_system)
            .add_systems(Update, enemy_patrol_system);

        info!("EnemyPlugin initialized");
    }
}
