//! Loot and inventory events
//!
//! 战利品与库存事件
//!
//! Events for inter-system communication in the loot/inventory system.

use bevy::ecs::message::Message;
use bevy::math::Vec2;
use bevy::prelude::*;

use crate::infrastructure::components::loot::ItemId;

/// 物品掉落事件
///
/// Emitted when an item is dropped in the world
/// 当物品掉落在场景中时触发
#[derive(Event, Message, Debug, Clone)]
pub struct ItemDropped {
    /// 掉落物实体
    pub world_item_entity: Entity,
    /// 物品 ID
    pub item_id: ItemId,
    /// 数量
    pub quantity: u32,
    /// 掉落位置
    pub position: Vec2,
}

/// 物品拾取事件
///
/// Emitted when a player picks up an item
/// 当玩家拾取物品时触发
#[derive(Event, Message, Debug, Clone)]
pub struct ItemPickedUp {
    /// 玩家实体
    pub player_entity: Entity,
    /// 物品 ID
    pub item_id: ItemId,
    /// 数量
    pub quantity: u32,
}

/// 库存已满事件
///
/// Emitted when inventory is full and cannot accept more items
/// 当库存已满无法接受更多物品时触发
#[derive(Event, Message, Debug, Clone)]
pub struct InventoryFull {
    /// 玩家实体
    pub player_entity: Entity,
    /// 尝试拾取的物品 ID
    pub item_id: ItemId,
}

/// 物品堆叠事件
///
/// Emitted when items are stacked in inventory
/// 当物品在库存中堆叠时触发
#[derive(Event, Message, Debug, Clone)]
pub struct ItemStacked {
    /// 玩家实体
    pub player_entity: Entity,
    /// 物品 ID
    pub item_id: ItemId,
    /// 堆叠后的总数量
    pub total_quantity: u32,
    /// 槽位索引
    pub slot_index: usize,
}
