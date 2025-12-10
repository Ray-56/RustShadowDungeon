//! Dungeon system resources
//!
//! Global resources for dungeon state and configuration.

use crate::infrastructure::components::dungeon::RoomId;
use bevy::prelude::*;
use std::collections::HashSet;

/// Resource tracking dungeon session state
///
/// Maintains a record of cleared rooms during the current dungeon run.
#[derive(Resource, Debug, Default)]
pub struct DungeonSession {
    /// Set of room IDs that have been cleared
    pub cleared_rooms: HashSet<RoomId>,
}

/// Configuration for loading dungeon data from RON files
#[derive(Resource, Debug, Clone, serde::Deserialize)]
pub struct DungeonConfig {
    /// ID of the dungeon
    pub dungeon_id: String,
    /// Starting room ID
    pub start_room_id: u32,
    /// Room configurations
    pub rooms: Vec<RoomConfig>,
}

/// Configuration for a single room
#[derive(Debug, Clone, serde::Deserialize)]
pub struct RoomConfig {
    /// Room ID
    pub room_id: u32,
    /// Spawn points for enemies
    pub spawn_points: Vec<[f32; 2]>,
    /// Door configurations
    pub doors: Vec<DoorConfig>,
}

/// Configuration for a door
#[derive(Debug, Clone, serde::Deserialize)]
pub struct DoorConfig {
    /// Door ID
    pub door_id: u32,
    /// Connected room ID
    pub connected_room_id: u32,
    /// Entrance position in target room
    pub entrance_position: [f32; 2],
}

impl DungeonConfig {
    /// Load dungeon config from RON file
    ///
    /// # Arguments
    /// * `path` - Path to RON file (e.g., "assets/data/dungeons/test_dungeon.ron")
    ///
    /// # Returns
    /// Result containing DungeonConfig or error message
    pub fn load_from_file(path: &str) -> Result<Self, String> {
        let contents = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read dungeon config file '{}': {}", path, e))?;

        let config: Self = ron::from_str(&contents)
            .map_err(|e| format!("Failed to parse dungeon config file '{}': {}", path, e))?;

        Ok(config)
    }
}

/// Resource pool for entity reuse optimization
///
/// 实体复用资源池
///
/// Stores inactive entities that can be reused instead of spawning new ones.
/// This reduces allocation overhead and improves performance.
#[derive(Resource, Debug, Default)]
pub struct EntityPool {
    /// Pool of inactive enemy entities (can be reused)
    /// 非活跃敌人实体池（可复用）
    pub inactive_enemies: Vec<bevy::prelude::Entity>,
    /// Maximum pool size to prevent unbounded growth
    /// 最大池大小，防止无限增长
    pub max_pool_size: usize,
}

impl EntityPool {
    /// Create a new entity pool with default settings
    ///
    /// 创建新的实体池，使用默认设置
    pub fn new() -> Self {
        Self {
            inactive_enemies: Vec::new(),
            max_pool_size: 20, // Reasonable limit for enemy pool
        }
    }

    /// Get an entity from the pool if available, otherwise return None
    ///
    /// 从池中获取实体（如果可用），否则返回 None
    pub fn pop_enemy(&mut self) -> Option<bevy::prelude::Entity> {
        self.inactive_enemies.pop()
    }

    /// Return an entity to the pool for reuse
    ///
    /// 将实体返回到池中以供复用
    pub fn push_enemy(&mut self, entity: bevy::prelude::Entity) {
        if self.inactive_enemies.len() < self.max_pool_size {
            self.inactive_enemies.push(entity);
        }
        // If pool is full, entity will be despawned normally
    }

    /// Clear the pool (useful for cleanup)
    ///
    /// 清空池（用于清理）
    pub fn clear(&mut self) {
        self.inactive_enemies.clear();
    }
}
