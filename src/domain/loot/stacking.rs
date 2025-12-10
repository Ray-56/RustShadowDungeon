//! Item stacking logic
//!
//! 物品堆叠逻辑
//!
//! Pure functions for calculating item stacking results and validation.

use crate::domain::loot::drop_table::ItemId;

/// 堆叠结果
///
/// 表示堆叠计算的结果
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StackResult {
    /// 完全合并
    ///
    /// 所有物品都可以堆叠到当前槽位
    Merge(u32), // 最终数量
    /// 部分合并，有溢出
    ///
    /// 部分物品堆叠到当前槽位，剩余部分需要新槽位
    Split {
        /// 当前槽位剩余数量（达到 max_stack）
        remaining: u32,
        /// 溢出数量（需要新槽位）
        overflow: u32,
    },
}

/// 检查两个物品是否可以堆叠
///
/// 只有相同物品 ID 且都标记为可堆叠时才能堆叠
///
/// # Arguments
///
/// * `item1_id` - 第一个物品 ID
/// * `item2_id` - 第二个物品 ID
/// * `item1_stackable` - 第一个物品是否可堆叠
/// * `item2_stackable` - 第二个物品是否可堆叠
///
/// # Returns
///
/// 如果可以堆叠，返回 true
pub fn can_stack_items(
    item1_id: ItemId,
    item2_id: ItemId,
    item1_stackable: bool,
    item2_stackable: bool,
) -> bool {
    item1_id == item2_id && item1_stackable && item2_stackable
}

/// 计算堆叠结果
///
/// 计算将指定数量添加到当前数量后的结果，考虑最大堆叠限制
///
/// # Arguments
///
/// * `current_quantity` - 当前数量
/// * `add_quantity` - 要添加的数量
/// * `max_stack` - 最大堆叠数量
///
/// # Returns
///
/// 堆叠结果
pub fn calculate_stack_result(
    current_quantity: u32,
    add_quantity: u32,
    max_stack: u32,
) -> StackResult {
    let total = current_quantity + add_quantity;
    if total <= max_stack {
        StackResult::Merge(total)
    } else {
        StackResult::Split { remaining: max_stack, overflow: total - max_stack }
    }
}

/// 分割堆叠
///
/// 将当前数量分割为达到最大堆叠的部分和剩余部分
///
/// # Arguments
///
/// * `current_quantity` - 当前数量
/// * `max_stack` - 最大堆叠数量
///
/// # Returns
///
/// (达到 max_stack 的数量, 剩余数量)
pub fn split_stack(current_quantity: u32, max_stack: u32) -> (u32, u32) {
    if current_quantity <= max_stack {
        (current_quantity, 0)
    } else {
        (max_stack, current_quantity - max_stack)
    }
}
