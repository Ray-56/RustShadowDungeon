/// Integration tests for invincibility system
///
/// T099: Tests that invincibility frames block damage:
/// 1. Player takes damage
/// 2. Invincibility component is added
/// 3. Another attack hits during invincibility
/// 4. Second attack is ignored

#[cfg(test)]
mod invincibility_integration_tests {
    use bevy::ecs::message::MessageWriter;
    use bevy::prelude::*;
    use rust_shadow_dungeon::domain::combat::collision::Rect;
    use rust_shadow_dungeon::domain::combat::{DamageResult, Element};
    use rust_shadow_dungeon::infrastructure::components::combat::{HurtBox, Invincibility, Stats};
    use rust_shadow_dungeon::infrastructure::components::{Health, Player};
    use rust_shadow_dungeon::infrastructure::events::combat::DamageDealt;
    use rust_shadow_dungeon::infrastructure::plugins::combat::CombatPlugin;
    use rust_shadow_dungeon::infrastructure::resources::combat_config::CombatConfig;
    use std::collections::VecDeque;

    // Resource to store messages to be sent in tests
    #[derive(Resource, Default)]
    struct TestMessageQueue {
        messages: VecDeque<DamageDealt>,
    }

    // System to send queued messages
    fn send_queued_messages(
        mut queue: ResMut<TestMessageQueue>,
        mut writer: MessageWriter<DamageDealt>,
    ) {
        while let Some(msg) = queue.messages.pop_front() {
            writer.write(msg);
        }
    }

    /// T099: Test invincibility blocks damage
    /// Verifies that during invincibility frames, attacks are ignored
    #[test]
    fn test_invincibility_blocks_damage() {
        let mut app = App::new();

        // Add minimal plugins with input support
        app.add_plugins(MinimalPlugins);
        // Add input plugin for ButtonInput resource
        app.add_plugins(bevy::input::InputPlugin);
        // Add asset plugin for AssetServer (needed by combat_audio_system)
        app.add_plugins(bevy::asset::AssetPlugin::default());
        app.add_plugins(CombatPlugin);
        // Add DungeonPlugin to register PlayerDeathInRoom message (needed by death_system)
        app.add_plugins(rust_shadow_dungeon::infrastructure::plugins::dungeon::DungeonPlugin);

        // Add test message queue and system
        app.init_resource::<TestMessageQueue>();
        // Ensure send_queued_messages runs before apply_damage_system
        // CombatPlugin already registers apply_damage_system and invincibility_trigger_system
        // We need to ensure send_queued_messages runs first
        use rust_shadow_dungeon::infrastructure::systems::apply_damage_system;
        app.add_systems(Update, send_queued_messages.before(apply_damage_system));

        // Add combat config
        let config = CombatConfig::default();
        let invincibility_duration = config.invincibility_duration;
        app.insert_resource(config);

        // Spawn player
        let player = app
            .world_mut()
            .spawn((
                Name::new("Player"),
                Player,
                Health { current: 100.0, max: 100.0 },
                HurtBox {
                    rect: Rect { x: 0.0, y: 0.0, width: 32.0, height: 32.0 },
                    is_invincible: false,
                },
                Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ))
            .id();

        // First attack - should deal damage
        {
            let mut queue = app.world_mut().resource_mut::<TestMessageQueue>();
            queue.messages.push_back(DamageDealt {
                target: player,
                source: Entity::PLACEHOLDER,
                result: DamageResult {
                    final_damage: 10.0,
                    is_critical: false,
                    element: Element::Physical,
                },
                position: Vec2::new(0.0, 0.0),
            });
        }
        app.update();

        // Check player took damage
        if let Some(health) = app.world().get::<Health>(player) {
            assert_eq!(health.current, 90.0); // 100 - 10
        } else {
            panic!("Player health component not found");
        }

        // Check invincibility was added
        if let Some(invincibility) = app.world().get::<Invincibility>(player) {
            assert!(invincibility.is_active());
            assert_eq!(invincibility.remaining, invincibility_duration);
        } else {
            panic!("Invincibility component not found after taking damage");
        }

        // Second attack during invincibility - should be ignored
        {
            let mut queue = app.world_mut().resource_mut::<TestMessageQueue>();
            queue.messages.push_back(DamageDealt {
                target: player,
                source: Entity::PLACEHOLDER,
                result: DamageResult {
                    final_damage: 10.0,
                    is_critical: false,
                    element: Element::Physical,
                },
                position: Vec2::new(0.0, 0.0),
            });
        }
        app.update();

        // Check player health did NOT decrease (still 90)
        if let Some(health) = app.world().get::<Health>(player) {
            assert_eq!(health.current, 90.0); // Should still be 90
        } else {
            panic!("Player health component not found");
        }
    }
}
