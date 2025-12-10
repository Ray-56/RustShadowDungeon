//! Loot and inventory components
//!
//! 战利品与库存组件
//!
//! ECS components for loot tables, inventory, items, and world items.
//! These are pure data structures with no behavior.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// 物品 ID（类型别名）
pub type ItemId = u32;

/// 掉落表配置（从 RON 文件加载）
///
/// Loot table asset loaded from RON files
#[derive(Asset, TypePath, Deserialize, Serialize, Clone, Debug)]
pub struct LootTableAsset {
    /// 表 ID
    pub id: String,
    /// 掉落条目
    pub entries: Vec<LootTableEntryAsset>,
}

/// 掉落表条目（资产格式）
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct LootTableEntryAsset {
    /// 物品 ID
    pub item_id: u32,
    /// 掉落概率（0.0 - 1.0）
    pub chance: f32,
    /// 最小数量
    pub quantity_min: u32,
    /// 最大数量
    pub quantity_max: u32,
}

/// 库存槽位（基础设施层）
///
/// Inventory slot component for ECS
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InventorySlot {
    /// 物品 ID
    pub item_id: ItemId,
    /// 数量
    pub quantity: u32,
}

/// 库存组件（附着于玩家实体）
///
/// Inventory component attached to player entities
#[derive(Component, Clone, Debug)]
pub struct InventoryComponent {
    /// 固定 30 个槽位
    pub slots: [Option<InventorySlot>; 30],
}

impl Default for InventoryComponent {
    fn default() -> Self {
        Self { slots: std::array::from_fn(|_| None) }
    }
}

/// 物品定义资源（从 RON 文件加载）
///
/// Item definition asset loaded from RON files
#[derive(Asset, TypePath, Deserialize, Serialize, Clone, Debug)]
pub struct ItemDefinitionAsset {
    /// 物品 ID
    pub id: u32,
    /// 物品名称（用于显示）
    pub name: String,
    /// 图标路径（用于 UI）
    pub icon_path: String,
    /// 是否可堆叠
    pub stackable: bool,
    /// 最大堆叠数量（默认 99）
    pub max_stack: u32,
}

/// 场景中的物品实体
///
/// World item component for items dropped in the game world
#[derive(Component, Clone, Debug)]
pub struct WorldItem {
    /// 物品 ID
    pub item_id: ItemId,
    /// 数量
    pub quantity: u32,
    /// 是否已被拾取（用于清理）
    pub picked_up: bool,
}
