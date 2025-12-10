//! Loot and inventory resources
//!
//! 战利品与库存资源
//!
//! Global game state resources for loot configuration and pickup settings.

use bevy::prelude::*;

/// 拾取模式
///
/// Pickup mode configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PickupMode {
    /// 手动拾取（按键触发）
    Manual,
    /// 自动拾取（进入范围即拾取）
    Automatic,
}

/// 全局掉落配置
///
/// Global loot configuration resource
#[derive(Resource, Default, Debug)]
pub struct LootConfig {
    /// 默认掉落表（如果敌人没有指定）
    pub default_loot_table_id: Option<String>,
}

/// 拾取配置
///
/// Pickup configuration resource
#[derive(Resource, Debug)]
pub struct PickupConfig {
    /// 拾取模式
    pub mode: PickupMode,
    /// 拾取范围（米）
    pub range: f32,
    /// 拾取按键（手动模式）
    pub pickup_key: KeyCode,
}

impl Default for PickupConfig {
    fn default() -> Self {
        Self {
            mode: PickupMode::Manual,
            range: 3.0, // 3 meters (48 pixels) - increased for better UX
            pickup_key: KeyCode::KeyE,
        }
    }
}

/// 库存 UI 数据
///
/// Inventory UI data resource for UI system consumption
#[derive(Resource, Debug, Clone)]
pub struct InventoryUIData {
    /// 槽位数据（用于 UI 显示）
    pub slots: Vec<InventorySlotUIData>,
}

/// 库存槽位 UI 数据
///
/// Single inventory slot UI data
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InventorySlotUIData {
    /// 物品 ID（None 表示空槽位）
    pub item_id: Option<u32>,
    /// 数量（仅当 item_id 为 Some 时有效）
    pub quantity: u32,
    /// 物品名称（用于显示）
    pub item_name: String,
    /// 图标路径（用于显示）
    pub icon_path: String,
}

impl Default for InventoryUIData {
    fn default() -> Self {
        Self { slots: Vec::with_capacity(30) }
    }
}
