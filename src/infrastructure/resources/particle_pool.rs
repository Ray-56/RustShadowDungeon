//! Particle pool resource for object pooling
//!
//! 粒子对象池资源
//!
//! Pre-allocates 200 particle entities to avoid frequent spawn/despawn operations.
//! Particles are reused by activating/deactivating them instead of spawning/despawning.

use bevy::prelude::*;
use std::collections::VecDeque;

/// Particle pool resource
///
/// 粒子对象池
///
/// Maintains a pool of pre-allocated particle entities that can be reused.
/// When a particle is needed, it's taken from the pool. When it expires,
/// it's returned to the pool instead of being despawned.
#[derive(Resource)]
pub struct ParticlePool {
    /// Queue of available particle entities (ready to be reused)
    available: VecDeque<Entity>,
    /// Total pool size (should be 200)
    pool_size: usize,
}

impl ParticlePool {
    /// Create a new particle pool
    ///
    /// 创建新的粒子对象池
    pub fn new() -> Self {
        Self { available: VecDeque::new(), pool_size: 0 }
    }

    /// Get an available particle entity from the pool
    ///
    /// 从池中获取一个可用的粒子实体
    ///
    /// Returns None if pool is empty (should spawn new particle instead)
    pub fn get(&mut self) -> Option<Entity> {
        self.available.pop_front()
    }

    /// Return a particle entity to the pool
    ///
    /// 将粒子实体返回到池中
    pub fn return_entity(&mut self, entity: Entity) {
        self.available.push_back(entity);
    }

    /// Add a new entity to the pool (during initialization)
    ///
    /// 向池中添加新实体（初始化时使用）
    pub fn add_entity(&mut self, entity: Entity) {
        self.available.push_back(entity);
        self.pool_size += 1;
    }

    /// Get the number of available particles in the pool
    ///
    /// 获取池中可用粒子的数量
    pub fn available_count(&self) -> usize {
        self.available.len()
    }

    /// Get the total pool size
    ///
    /// 获取池的总大小
    pub fn pool_size(&self) -> usize {
        self.pool_size
    }
}

impl Default for ParticlePool {
    fn default() -> Self {
        Self::new()
    }
}
