//! Inventory management logic
//!
//! 库存管理逻辑
//!
//! Pure functions for inventory slot management, item addition, and validation.

use crate::domain::loot::drop_table::ItemId;

/// 库存槽位
///
/// 表示一个库存槽位，包含物品 ID 和数量
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InventorySlot {
    /// 物品 ID
    pub item_id: ItemId,
    /// 数量
    pub quantity: u32,
}

/// 库存
///
/// 固定 30 个槽位的库存
#[derive(Debug, Clone)]
pub struct Inventory {
    /// 固定 30 个槽位
    pub slots: [Option<InventorySlot>; 30],
}

impl Default for Inventory {
    fn default() -> Self {
        Self { slots: std::array::from_fn(|_| None) }
    }
}

/// 库存错误
#[derive(Debug, PartialEq, Eq)]
pub enum InventoryError {
    /// 库存已满
    Full,
    /// 槽位索引无效
    InvalidSlot(usize),
    /// 堆叠失败（物品不匹配或不可堆叠）
    StackingFailed,
}

/// 检查是否可以添加物品
///
/// 检查库存是否有空间添加指定数量的物品
///
/// # Arguments
///
/// * `inventory` - 库存
/// * `item_id` - 物品 ID
/// * `quantity` - 数量
///
/// # Returns
///
/// 如果可以添加，返回 true
pub fn can_add_item(inventory: &Inventory, item_id: ItemId, _quantity: u32) -> bool {
    // 首先检查是否有可堆叠的槽位
    if find_stackable_slot(inventory, item_id).is_some() {
        return true;
    }

    // 检查是否有空槽位
    find_empty_slot(inventory).is_some()
}

/// 查找空槽位
///
/// # Arguments
///
/// * `inventory` - 库存
///
/// # Returns
///
/// 第一个空槽位的索引，如果没有则返回 None
pub fn find_empty_slot(inventory: &Inventory) -> Option<usize> {
    inventory.slots.iter().position(|slot| slot.is_none())
}

/// 查找可堆叠的槽位
///
/// 查找包含相同物品 ID 的槽位（用于堆叠）
///
/// # Arguments
///
/// * `inventory` - 库存
/// * `item_id` - 物品 ID
///
/// # Returns
///
/// 可堆叠槽位的索引，如果没有则返回 None
pub fn find_stackable_slot(inventory: &Inventory, item_id: ItemId) -> Option<usize> {
    inventory
        .slots
        .iter()
        .position(|slot| slot.as_ref().map_or(false, |s| s.item_id == item_id))
}

/// 添加物品到槽位
///
/// 将物品添加到指定槽位。如果槽位已有相同物品，会尝试堆叠。
///
/// # Arguments
///
/// * `inventory` - 库存（可变引用）
/// * `slot_idx` - 槽位索引
/// * `item_id` - 物品 ID
/// * `quantity` - 数量
///
/// # Returns
///
/// 成功返回 Ok(())，失败返回 InventoryError
pub fn add_item_to_slot(
    inventory: &mut Inventory,
    slot_idx: usize,
    item_id: ItemId,
    quantity: u32,
) -> Result<(), InventoryError> {
    if slot_idx >= 30 {
        return Err(InventoryError::InvalidSlot(slot_idx));
    }

    match &mut inventory.slots[slot_idx] {
        Some(slot) => {
            // 如果槽位已有物品，检查是否可以堆叠
            if slot.item_id == item_id {
                slot.quantity += quantity;
                Ok(())
            } else {
                Err(InventoryError::StackingFailed)
            }
        },
        None => {
            // 空槽位，直接添加
            inventory.slots[slot_idx] = Some(InventorySlot { item_id, quantity });
            Ok(())
        },
    }
}
