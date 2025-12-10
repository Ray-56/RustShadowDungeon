//! Obstacle components for line-of-sight detection
//!
//! 障碍物组件，用于视线检测
//!
//! Components that mark entities as obstacles that can block line of sight
//! for enemy AI perception system.

use bevy::prelude::*;

/// Marker component for entities that block line of sight
///
/// 标记阻挡视线的实体组件
///
/// Entities with this component will be considered as obstacles
/// when checking line of sight in the perception system.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Obstacle;

/// Collision rectangle for obstacle (world-space)
///
/// 障碍物的碰撞矩形（世界坐标）
///
/// This component stores the collision rectangle for an obstacle entity.
/// The rectangle is in world-space coordinates (absolute position).
/// If not present, the system will use the entity's Transform and Sprite size.
#[derive(Component, Debug, Clone)]
pub struct ObstacleCollider {
    /// Collision rectangle in world-space coordinates
    pub rect: crate::domain::combat::collision::Rect,
}

impl ObstacleCollider {
    /// Create a new obstacle collider from a Rect
    pub fn new(rect: crate::domain::combat::collision::Rect) -> Self {
        Self { rect }
    }

    /// Create a new obstacle collider from position and size
    pub fn from_pos_size(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            rect: crate::domain::combat::collision::Rect::new(x, y, width, height),
        }
    }
}

