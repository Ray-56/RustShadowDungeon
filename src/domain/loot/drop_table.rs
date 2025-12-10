//! Loot drop table calculation logic
//!
//! 掉落表计算逻辑
//!
//! Pure functions for calculating loot drops based on independent probability.
//! Each item in a loot table rolls independently, allowing multiple items to drop.

use rand::Rng;

/// 物品 ID（类型别名）
pub type ItemId = u32;

/// 掉落物结果
///
/// 表示一次掉落计算的结果，包含物品 ID 和数量
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LootDrop {
    /// 物品 ID
    pub item_id: ItemId,
    /// 数量
    pub quantity: u32,
}

/// 掉落表条目
///
/// 定义单个物品的掉落规则
#[derive(Debug, Clone)]
pub struct LootTableEntry {
    /// 物品 ID
    pub item_id: ItemId,
    /// 掉落概率（0.0 - 1.0）
    pub chance: f32,
    /// 最小数量
    pub quantity_min: u32,
    /// 最大数量
    pub quantity_max: u32,
}

/// 掉落表
///
/// 包含多个掉落条目，使用独立概率计算
#[derive(Debug, Clone)]
pub struct LootTable {
    /// 表 ID（用于配置查找）
    pub id: String,
    /// 掉落条目列表
    pub entries: Vec<LootTableEntry>,
}

/// 计算掉落物列表
///
/// 根据掉落表使用独立概率计算掉落物。
/// 每个条目独立判断是否掉落，可以同时掉落多个物品。
///
/// # Arguments
///
/// * `loot_table` - 掉落表配置
/// * `rng` - 随机数生成器
///
/// # Returns
///
/// 掉落物列表（可能为空）
pub fn calculate_loot_drops<R: Rng>(loot_table: &LootTable, rng: &mut R) -> Vec<LootDrop> {
    loot_table
        .entries
        .iter()
        .filter_map(|entry| {
            if roll_item_drop(entry.chance, rng) {
                Some(LootDrop {
                    item_id: entry.item_id,
                    quantity: calculate_item_quantity(entry.quantity_min, entry.quantity_max, rng),
                })
            } else {
                None
            }
        })
        .collect()
}

/// 判断物品是否掉落
///
/// 根据概率进行随机判断
///
/// # Arguments
///
/// * `chance` - 掉落概率（0.0 - 1.0）
/// * `rng` - 随机数生成器
///
/// # Returns
///
/// 如果随机数小于等于概率，返回 true
fn roll_item_drop<R: Rng>(chance: f32, rng: &mut R) -> bool {
    rng.gen::<f32>() <= chance
}

/// 计算物品数量
///
/// 在最小值和最大值之间随机选择数量
///
/// # Arguments
///
/// * `min` - 最小数量
/// * `max` - 最大数量
/// * `rng` - 随机数生成器
///
/// # Returns
///
/// 随机数量（包含 min 和 max）
pub fn calculate_item_quantity<R: Rng>(min: u32, max: u32, rng: &mut R) -> u32 {
    rng.gen_range(min..=max)
}
