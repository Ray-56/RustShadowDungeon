//! Enemy AI resources
//!
//! Global resources for enemy AI system configuration.
//!
//! 敌人 AI 系统全局资源配置

use bevy::prelude::*;

/// AI 全局配置资源（可选）
///
/// Global AI configuration resource for default settings.
/// 全局 AI 配置资源，用于默认设置
#[derive(Resource, Debug)]
pub struct AIConfig {
    /// 默认检测范围（像素）
    pub default_detection_range: f32,
    /// 默认攻击范围（像素）
    pub default_attack_range: f32,
    /// 默认脱战范围（像素）
    pub default_aggro_drop_range: f32,
    /// 默认感知检查间隔（秒）
    pub default_perception_check_interval: f32,
    /// 是否启用调试可视化
    pub debug_visualization: bool,
}

impl Default for AIConfig {
    fn default() -> Self {
        Self {
            default_detection_range: 200.0,
            default_attack_range: 32.0,
            default_aggro_drop_range: 400.0,
            default_perception_check_interval: 0.1, // ~3-5 frames at 60 FPS
            debug_visualization: false,
        }
    }
}
