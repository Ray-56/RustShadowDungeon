//! Particle effect systems
//!
//! 粒子特效系统
//!
//! Generates and updates visual effect particles for combat feedback.

use bevy::prelude::*;
use rand::Rng;

use crate::domain::combat::ComboState;
use crate::infrastructure::components::combat::{Combo, Particle};
use crate::infrastructure::events::combat::DamageDealt;
use crate::infrastructure::resources::{CombatConfig, ParticlePool};

/// Particle pool initialization system
///
/// 粒子对象池初始化系统
///
/// T106: Pre-allocates 200 particle entities and adds them to the pool.
/// This system should run once at startup (in Startup schedule).
/// Particles are reused by activating/deactivating them instead of spawning/despawning.
pub fn particle_pool_system(
    mut commands: Commands,
    mut particle_pool: ResMut<ParticlePool>,
) {
    const POOL_SIZE: usize = 200;

    // Only initialize if pool is empty (avoid re-initialization)
    if particle_pool.pool_size() > 0 {
        return;
    }

    // Spawn 200 pre-allocated particle entities
    for _ in 0..POOL_SIZE {
        let entity = commands
            .spawn((
                Particle::new(0.0, Vec2::ZERO, Color::WHITE),
                Transform::default(),
                // Mark as inactive initially (lifetime = 0 means inactive)
                Visibility::Hidden, // Hide inactive particles
            ))
            .id();

        particle_pool.add_entity(entity);
    }

    info!(
        "Particle pool initialized with {} particles",
        particle_pool.pool_size()
    );
}

/// Hit particle system
///
/// 打击粒子生成系统
///
/// T064: Listens to DamageDealt events and spawns particles at hit position.
/// T106: Uses particle pool to reuse entities instead of spawning new ones.
/// Particle count and color vary based on hit type:
/// - Light hit: 5-10 white particles
/// - Heavy hit: 15-20 orange particles
/// - Critical hit: 20-30 golden particles
pub fn hit_particle_system(
    mut damage_events: MessageReader<DamageDealt>,
    mut commands: Commands,
    mut particle_pool: ResMut<ParticlePool>,
    combat_config: Res<CombatConfig>,
    combo_query: Query<&Combo>,
    mut particle_query: Query<(&mut Particle, &mut Transform, &mut Visibility)>,
) {
    let mut rng = rand::thread_rng();

    for event in damage_events.read() {
        // Determine if this is a heavy hit (third combo hit)
        let is_heavy = combo_query
            .get(event.source)
            .map(|combo| combo.state == ComboState::ThirdHit)
            .unwrap_or(false);

        // Get particle count based on hit type
        let base_count = combat_config.get_particle_count(event.result.is_critical, is_heavy);

        // Add some randomness (±20%)
        let count =
            rng.gen_range((base_count as f32 * 0.8) as u32..=(base_count as f32 * 1.2) as u32);

        // Determine particle color based on hit type
        let color = if event.result.is_critical {
            Color::srgb(1.0, 0.84, 0.0) // Gold for critical
        } else if is_heavy {
            Color::srgb(1.0, 0.5, 0.0) // Orange for heavy
        } else {
            Color::WHITE // White for light
        };

        // Spawn particles at hit position (using pool if available)
        for _ in 0..count {
            // Random velocity direction (outward from hit point)
            let angle = rng.gen_range(0.0..std::f32::consts::TAU);
            let speed = rng.gen_range(50.0..150.0); // pixels per second
            let velocity = Vec2::new(angle.cos() * speed, angle.sin() * speed);

            // Try to get a particle from the pool
            if let Some(entity) = particle_pool.get() {
                // Reuse existing particle entity
                if let Ok((mut particle, mut transform, mut visibility)) =
                    particle_query.get_mut(entity)
                {
                    // Reset particle properties
                    *particle = Particle::new(combat_config.particle_lifetime, velocity, color);
                    transform.translation = Vec3::new(
                        event.position.x,
                        event.position.y,
                        10.0, // Above other entities
                    );
                    *visibility = Visibility::Visible;
                }
            } else {
                // Pool is empty, spawn new particle (fallback)
                commands.spawn((
                    Particle::new(combat_config.particle_lifetime, velocity, color),
                    Transform::from_translation(Vec3::new(
                        event.position.x,
                        event.position.y,
                        10.0, // Above other entities
                    )),
                    Visibility::Visible,
                    // Note: In a real implementation, you'd add a SpriteBundle here
                    // For now, we just create the particle component
                ));
            }
        }
    }
}

/// Particle update system
///
/// 粒子更新系统
///
/// T065: Updates particle position, lifetime, and alpha.
/// T106: Returns expired particles to the pool instead of despawning them.
/// Particles move based on velocity, fade out over time, and return to pool when lifetime expires.
pub fn particle_update_system(
    mut commands: Commands,
    mut particle_pool: ResMut<ParticlePool>,
    mut particle_query: Query<(Entity, &mut Particle, &mut Transform, &mut Visibility)>,
    time: Res<Time>,
) {
    let delta = time.delta_secs();
    let mut expired_entities = Vec::new();

    for (entity, mut particle, mut transform, mut visibility) in &mut particle_query {
        // Update particle lifetime
        let still_alive = particle.update(delta);

        if still_alive {
            // Update position based on velocity
            transform.translation.x += particle.velocity.x * delta;
            transform.translation.y += particle.velocity.y * delta;

            // Apply gravity (optional, for more realistic particle physics)
            // particle.velocity.y -= 200.0 * delta; // Gravity effect
        } else {
            // Lifetime expired, return to pool instead of despawning
            *visibility = Visibility::Hidden;
            expired_entities.push(entity);
        }
    }

    // Return expired particles to the pool
    for entity in expired_entities {
        // Check if this entity is part of the pool (has pool marker or is in pool size range)
        // For simplicity, we'll try to return it to the pool
        // In a more robust implementation, we'd track which entities belong to the pool
        if particle_pool.pool_size() > 0 {
            // Only return to pool if pool was initialized
            particle_pool.return_entity(entity);
        } else {
            // Pool not initialized, despawn normally
            commands.entity(entity).despawn();
        }
    }
}
