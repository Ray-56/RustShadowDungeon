//! Pickup range detection logic
//!
//! 拾取范围检测逻辑
//!
//! Pure functions for checking if items are within pickup range.

/// 默认拾取范围（米）
pub const DEFAULT_PICKUP_RANGE: f32 = 2.0;

/// 检查物品是否在拾取范围内
///
/// 计算物品位置和玩家位置之间的距离，判断是否在拾取范围内
///
/// # Arguments
///
/// * `item_pos` - 物品位置 (Vec2)
/// * `player_pos` - 玩家位置 (Vec2)
/// * `range` - 拾取范围（米）
///
/// # Returns
///
/// 如果在范围内，返回 true
pub fn is_within_pickup_range(item_pos: (f32, f32), player_pos: (f32, f32), range: f32) -> bool {
    let dx = item_pos.0 - player_pos.0;
    let dy = item_pos.1 - player_pos.1;
    let distance = (dx * dx + dy * dy).sqrt();
    distance <= range
}

/// 获取默认拾取范围
///
/// # Returns
///
/// 默认拾取范围（2.0 米）
pub fn calculate_pickup_range() -> f32 {
    DEFAULT_PICKUP_RANGE
}
