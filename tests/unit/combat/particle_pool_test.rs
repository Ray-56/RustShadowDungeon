/// Unit tests for particle pool system
/// 
/// T106: Tests for particle pool initialization and entity reuse
/// These tests verify that the particle pool correctly pre-allocates
/// 200 particles and manages entity reuse.

#[cfg(test)]
mod particle_pool_tests {
    use bevy::prelude::*;
    use rust_shadow_dungeon::infrastructure::components::combat::Particle;
    use rust_shadow_dungeon::infrastructure::resources::particle_pool::{ParticlePool, particle_pool_init_system};

    /// Test ParticlePool creation
    #[test]
    fn test_particle_pool_new() {
        let pool = ParticlePool::new();
        assert_eq!(pool.available_count(), 0);
        assert_eq!(pool.pool_size(), 0);
    }

    /// Test ParticlePool default
    #[test]
    fn test_particle_pool_default() {
        let pool = ParticlePool::default();
        assert_eq!(pool.available_count(), 0);
        assert_eq!(pool.pool_size(), 0);
    }

    /// Test adding entities to pool
    #[test]
    fn test_particle_pool_add_entity() {
        let mut pool = ParticlePool::new();
        let mut app = App::new();
        
        // Spawn a test entity
        let entity = app.world_mut().spawn_empty().id();
        
        pool.add_entity(entity);
        assert_eq!(pool.available_count(), 1);
        assert_eq!(pool.pool_size(), 1);
    }

    /// Test getting entities from pool
    #[test]
    fn test_particle_pool_get() {
        let mut pool = ParticlePool::new();
        let mut app = App::new();
        
        let entity1 = app.world_mut().spawn_empty().id();
        let entity2 = app.world_mut().spawn_empty().id();
        
        pool.add_entity(entity1);
        pool.add_entity(entity2);
        
        assert_eq!(pool.available_count(), 2);
        
        let retrieved = pool.get();
        assert_eq!(retrieved, Some(entity1));
        assert_eq!(pool.available_count(), 1);
        
        let retrieved2 = pool.get();
        assert_eq!(retrieved2, Some(entity2));
        assert_eq!(pool.available_count(), 0);
        
        // Pool is empty, should return None
        let retrieved3 = pool.get();
        assert_eq!(retrieved3, None);
    }

    /// Test returning entities to pool
    #[test]
    fn test_particle_pool_return_entity() {
        let mut pool = ParticlePool::new();
        let mut app = App::new();
        
        let entity = app.world_mut().spawn_empty().id();
        
        // Return entity to pool
        pool.return_entity(entity);
        assert_eq!(pool.available_count(), 1);
        
        // Get it back
        let retrieved = pool.get();
        assert_eq!(retrieved, Some(entity));
        assert_eq!(pool.available_count(), 0);
    }

    /// Test particle pool initialization system
    #[test]
    fn test_particle_pool_init_system() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(ParticlePool::new());
        
        // Run initialization system
        app.add_systems(Startup, particle_pool_init_system);
        app.update();
        
        // Verify pool was initialized
        let pool = app.world().get_resource::<ParticlePool>().unwrap();
        assert_eq!(pool.pool_size(), 200);
        assert_eq!(pool.available_count(), 200);
    }

    /// Test pool reuse workflow
    #[test]
    fn test_particle_pool_reuse_workflow() {
        let mut pool = ParticlePool::new();
        let mut app = App::new();
        
        // Add 5 entities to pool
        for _ in 0..5 {
            let entity = app.world_mut().spawn_empty().id();
            pool.add_entity(entity);
        }
        
        assert_eq!(pool.available_count(), 5);
        
        // Get 3 entities (simulate using particles)
        let entities: Vec<_> = (0..3).map(|_| pool.get().unwrap()).collect();
        assert_eq!(pool.available_count(), 2);
        
        // Return 2 entities (simulate particles expiring)
        pool.return_entity(entities[0]);
        pool.return_entity(entities[1]);
        assert_eq!(pool.available_count(), 4);
        
        // Get one more
        let retrieved = pool.get();
        assert!(retrieved.is_some());
        assert_eq!(pool.available_count(), 3);
    }
}

