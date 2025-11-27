/// 战斗系统插件
///
/// 注册战斗系统的所有资源、事件和系统
use bevy::prelude::*;

use crate::infrastructure::events::combat::*;
use crate::infrastructure::resources::{CombatConfig, HitfreezeTimer, ParticlePool};
use crate::infrastructure::systems::particles::particle_pool_system;
use crate::infrastructure::systems::combat_audio::combat_audio_system;
use crate::infrastructure::systems::combo::{combo_system, combo_timer_system, knockback_system};
use crate::infrastructure::systems::feedback::{
    apply_screen_shake_system, damage_number_system, damage_number_update_system, hitfreeze_system,
    hitfreeze_timer_system, screen_shake_cleanup_system, screen_shake_system,
};
use crate::infrastructure::systems::enemy::{
    enemy_death_animation_system, enemy_death_animation_update_system, enemy_hit_flash_system,
    enemy_hit_flash_update_system,
};
use crate::infrastructure::systems::invincibility::{
    invincibility_flash_system, invincibility_timer_system, invincibility_trigger_system,
};
use crate::infrastructure::systems::particles::{hit_particle_system, particle_update_system};

/// 战斗系统插件
pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        // 注册资源
        app.insert_resource(CombatConfig::default())
            .insert_resource(HitfreezeTimer::new())
            .insert_resource(ParticlePool::new());

        // 初始化粒子对象池（在 Startup 阶段运行一次）
        app.add_systems(Startup, particle_pool_system);

        // 注册事件 (Bevy 0.17 uses add_message instead of add_event)
        app.add_message::<DamageDealt>()
            .add_message::<ComboExtended>()
            .add_message::<ComboReset>()
            .add_message::<EnemyDefeated>()
            .add_message::<SkillActivated>()
            .add_message::<InvincibilityStarted>()
            .add_message::<InvincibilityEnded>()
            .add_message::<HitBoxSpawned>();

        // 注册系统
        app.add_systems(
            Update,
            (
                // 1. Combo system (handles attack input and combo state) - T048
                combo_system,
                // 2. Combo timer system (updates combo window) - T049
                combo_timer_system,
                // 3. Collision detection (HitBox vs HurtBox)
                crate::infrastructure::systems::collision_detection_system,
                // 4. Apply damage (updates Health)
                crate::infrastructure::systems::apply_damage_system,
                // 5. Knockback system (applies knockback for third hit) - T050
                knockback_system,
                // 6. Death detection (despawns dead entities)
                crate::infrastructure::systems::death_system,
                // 7. HitBox cleanup (removes expired HitBoxes)
                crate::infrastructure::systems::hitbox_cleanup_system,
                // 8. Hitfreeze system (triggers hit stop on damage) - T058
                hitfreeze_system,
                // 9. Hitfreeze timer update (controls time dilation) - T059
                hitfreeze_timer_system,
                // 10. Screen shake system (adds shake component on heavy/critical hits) - T061
                screen_shake_system,
                // 11. Apply screen shake (updates camera position) - T062
                apply_screen_shake_system,
                // 12. Screen shake cleanup (removes component when expired)
                screen_shake_cleanup_system,
                // 13. Hit particle system (spawns particles on hit) - T064
                hit_particle_system,
                // 14. Particle update system (updates particle position/lifetime) - T065
                particle_update_system,
                // 15. Damage number system (spawns floating damage numbers) - T068
                damage_number_system,
                // 16. Damage number update system (updates position/alpha) - T069
                damage_number_update_system,
                // 17. Combat audio system (plays sound effects) - T070
                combat_audio_system,
                // 18. Invincibility trigger system (adds i-frames on damage) - T101
                invincibility_trigger_system,
                // 19. Invincibility timer system (updates and removes i-frames) - T102
                invincibility_timer_system,
                // 20. Invincibility flash system (visual feedback) - T103
                invincibility_flash_system,
            ),
        )
        // Enemy visual feedback systems (run after damage systems)
        .add_systems(
            Update,
            (
                // 21. Enemy hit flash system (visual feedback when enemy takes damage)
                enemy_hit_flash_system.after(crate::infrastructure::systems::apply_damage_system),
                // 22. Enemy hit flash update system (updates flash effect)
                enemy_hit_flash_update_system,
                // 23. Enemy death animation system (triggers death animation)
                enemy_death_animation_system.after(crate::infrastructure::systems::death_system),
                // 24. Enemy death animation update system (updates death animation)
                enemy_death_animation_update_system,
            ),
        );

        info!("CombatPlugin initialized");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combat_plugin_builds() {
        let mut app = App::new();
        app.add_plugins(CombatPlugin);

        // Verify resources are inserted
        assert!(app.world().get_resource::<CombatConfig>().is_some());
        assert!(app.world().get_resource::<HitfreezeTimer>().is_some());
    }
}
