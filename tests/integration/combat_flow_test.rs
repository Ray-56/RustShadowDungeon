/// Integration tests for complete combat flow
/// 
/// T023: Tests the full combat cycle:
/// 1. Player attacks (generates HitBox)
/// 2. Collision detection (HitBox vs HurtBox)
/// 3. Damage calculation and application
/// 4. Enemy health reduction
/// 5. Enemy death and despawn

#[cfg(test)]
mod combat_flow_integration_tests {
    use bevy::prelude::*;
    use rust_shadow_dungeon::domain::combat::{Stats, Element};
    use rust_shadow_dungeon::infrastructure::components::Health;
    use rust_shadow_dungeon::infrastructure::components::combat::{HitBox, HurtBox};
    use rust_shadow_dungeon::infrastructure::events::combat::DamageDealt;
    use rust_shadow_dungeon::infrastructure::plugins::combat::CombatPlugin;
    use rust_shadow_dungeon::domain::combat::collision::Rect;

    /// T023: Test player can defeat enemy (complete combat flow)
    #[test]
    fn test_player_can_defeat_enemy() {
        let mut app = App::new();
        
        // Add minimal plugins for testing
        app.add_plugins(MinimalPlugins);
        app.add_plugins(CombatPlugin);

        // Spawn player entity
        let player = app.world.spawn((
            Name::new("Player"),
            Stats {
                attack: 10.0,
                defense: 0.0,
                crit_rate: 0.0,
                crit_multiplier: 2.0,
                element_resistances: std::collections::HashMap::new(),
            },
            Health {
                current: 100.0,
                max: 100.0,
            },
            HurtBox {
                rect: Rect {
                    x: 0.0,
                    y: 0.0,
                    width: 32.0,
                    height: 32.0,
                },
                is_invincible: false,
            },
            Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        )).id();

        // Spawn enemy entity (slime with 30 HP)
        let enemy = app.world.spawn((
            Name::new("Slime"),
            Stats {
                attack: 5.0,
                defense: 0.0,
                crit_rate: 0.0,
                crit_multiplier: 2.0,
                element_resistances: std::collections::HashMap::new(),
            },
            Health {
                current: 30.0,
                max: 30.0,
            },
            HurtBox {
                rect: Rect {
                    x: 0.0,
                    y: 0.0,
                    width: 16.0,
                    height: 16.0,
                },
                is_invincible: false,
            },
            Transform::from_translation(Vec3::new(50.0, 0.0, 0.0)),
        )).id();

        // Generate HitBox overlapping with enemy (at x=50)
        app.world.spawn((
            Name::new("PlayerAttack"),
            HitBox {
                rect: Rect {
                    x: 0.0,
                    y: 0.0,
                    width: 64.0,  // Wide enough to hit enemy at x=50
                    height: 32.0,
                },
                damage: 10.0,
                element: Element::Physical,
                lifetime_frames: 10,
                can_pierce: false,
                hit_entities: std::collections::HashSet::new(),
            },
            Transform::from_translation(Vec3::new(32.0, 0.0, 0.0)),
        ));

        // Run 1 frame to process collision and damage
        app.update();

        // Verify enemy took damage
        if let Ok(enemy_health) = app.world.get::<Health>(enemy) {
            // Expected: 30 - (10 base + 0 attack) = 20
            assert_eq!(enemy_health.current, 20.0, "Enemy should have taken 10 damage");
        } else {
            panic!("Enemy entity not found after first attack");
        }

        // Attack 2 more times to kill the enemy
        for i in 0..2 {
            app.world.spawn((
                Name::new(format!("PlayerAttack{}", i + 2)),
                HitBox {
                    rect: Rect {
                        x: 0.0,
                        y: 0.0,
                        width: 64.0,
                        height: 32.0,
                    },
                    damage: 10.0,
                    element: Element::Physical,
                    lifetime_frames: 10,
                    can_pierce: false,
                    hit_entities: std::collections::HashSet::new(),
                },
                Transform::from_translation(Vec3::new(32.0, 0.0, 0.0)),
            ));
            app.update();
        }

        // Verify enemy is dead (despawned or health <= 0)
        match app.world.get::<Health>(enemy) {
            Ok(health) => {
                assert!(health.is_dead(), "Enemy should be dead (health <= 0)");
            }
            Err(_) => {
                // Enemy despawned (acceptable outcome)
            }
        }
    }

    /// Test HitBox lifetime expiration
    #[test]
    fn test_hitbox_lifetime() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(CombatPlugin);

        // Spawn HitBox with 1 frame lifetime
        let hitbox_id = app.world.spawn((
            Name::new("ShortLivedHitBox"),
            HitBox {
                rect: Rect {
                    x: 0.0,
                    y: 0.0,
                    width: 32.0,
                    height: 32.0,
                },
                damage: 10.0,
                element: Element::Physical,
                lifetime_frames: 1,
                can_pierce: false,
                hit_entities: std::collections::HashSet::new(),
            },
            Transform::default(),
        )).id();

        // Run 1 frame
        app.update();

        // HitBox should be despawned after 1 frame
        assert!(
            app.world.get_entity(hitbox_id).is_none(),
            "HitBox should be despawned after lifetime expires"
        );
    }

    /// Test multiple HitBoxes hitting the same entity (deduplication)
    #[test]
    fn test_multiple_hitbox_deduplication() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(CombatPlugin);

        // Spawn enemy
        let enemy = app.world.spawn((
            Name::new("Enemy"),
            Stats {
                attack: 0.0,
                defense: 0.0,
                crit_rate: 0.0,
                crit_multiplier: 2.0,
                element_resistances: std::collections::HashMap::new(),
            },
            Health {
                current: 100.0,
                max: 100.0,
            },
            HurtBox {
                rect: Rect {
                    x: 0.0,
                    y: 0.0,
                    width: 16.0,
                    height: 16.0,
                },
                is_invincible: false,
            },
            Transform::from_translation(Vec3::new(50.0, 0.0, 0.0)),
        )).id();

        // Spawn same HitBox entity persisting for 5 frames
        app.world.spawn((
            Name::new("LongHitBox"),
            HitBox {
                rect: Rect {
                    x: 0.0,
                    y: 0.0,
                    width: 100.0,
                    height: 32.0,
                },
                damage: 10.0,
                element: Element::Physical,
                lifetime_frames: 5,
                can_pierce: false,
                hit_entities: std::collections::HashSet::new(),
            },
            Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ));

        // Run 5 frames
        for _ in 0..5 {
            app.update();
        }

        // Enemy should only take damage once (deduplication)
        if let Ok(enemy_health) = app.world.get::<Health>(enemy) {
            assert_eq!(
                enemy_health.current, 90.0,
                "Enemy should only take damage once despite HitBox persisting"
            );
        }
    }

    /// Test DamageDealt event is published
    #[test]
    fn test_damage_dealt_event() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(CombatPlugin);

        // Spawn enemy
        let enemy = app.world.spawn((
            Name::new("Enemy"),
            Stats {
                attack: 0.0,
                defense: 0.0,
                crit_rate: 0.0,
                crit_multiplier: 2.0,
                element_resistances: std::collections::HashMap::new(),
            },
            Health {
                current: 50.0,
                max: 50.0,
            },
            HurtBox {
                rect: Rect {
                    x: 0.0,
                    y: 0.0,
                    width: 16.0,
                    height: 16.0,
                },
                is_invincible: false,
            },
            Transform::default(),
        )).id();

        // Spawn HitBox
        app.world.spawn((
            Name::new("Attack"),
            HitBox {
                rect: Rect {
                    x: 0.0,
                    y: 0.0,
                    width: 32.0,
                    height: 32.0,
                },
                damage: 15.0,
                element: Element::Fire,
                lifetime_frames: 10,
                can_pierce: false,
                hit_entities: std::collections::HashSet::new(),
            },
            Transform::default(),
        ));

        // Run frame
        app.update();

        // Check if DamageDealt event was published
        let mut event_reader = app.world.resource_mut::<Events<DamageDealt>>();
        let events: Vec<_> = event_reader.drain().collect();

        assert!(!events.is_empty(), "DamageDealt event should be published");
        
        let event = &events[0];
        assert_eq!(event.target, enemy);
        assert_eq!(event.result.element, Element::Fire);
    }
}

