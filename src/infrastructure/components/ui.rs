//! UI-related components
//!
//! UI 相关组件

use bevy::prelude::*;

/// Damage number component
///
/// 伤害数字组件
///
/// T067: Displays floating damage numbers above hit targets.
/// Numbers float upward and fade out over time.
#[derive(Component, Debug, Clone)]
pub struct DamageNumber {
    /// Damage value to display
    /// 显示的伤害值
    pub value: f32,
    /// Remaining lifetime (seconds)
    /// 剩余存活时间（秒）
    pub lifetime: f32,
    /// Velocity for floating animation (pixels per second)
    /// 浮动动画速度（像素/秒）
    pub velocity: Vec2,
    /// Whether this is a critical hit (affects color/size)
    /// 是否为暴击（影响颜色/大小）
    pub is_critical: bool,
    /// Maximum lifetime (for alpha calculation)
    /// 最大存活时间（用于透明度计算）
    pub max_lifetime: f32,
}

impl DamageNumber {
    /// Create a new damage number
    pub fn new(value: f32, lifetime: f32, is_critical: bool) -> Self {
        Self {
            value,
            lifetime,
            velocity: Vec2::new(0.0, 50.0), // Float upward at 50 pixels/second
            is_critical,
            max_lifetime: lifetime,
        }
    }

    /// Update lifetime (returns true if still alive)
    pub fn update(&mut self, delta: f32) -> bool {
        self.lifetime -= delta;
        self.lifetime > 0.0
    }

    /// Get alpha value based on remaining lifetime (fades out)
    pub fn get_alpha(&self) -> f32 {
        if self.max_lifetime <= 0.0 {
            return 1.0;
        }
        (self.lifetime / self.max_lifetime).clamp(0.0, 1.0)
    }

    /// Get text color based on hit type
    pub fn get_color(&self) -> Color {
        if self.is_critical {
            Color::srgb(1.0, 0.84, 0.0) // Gold for critical
        } else {
            Color::WHITE // White for normal
        }
    }
}

/// T092: Skill cooldown UI component
///
/// 技能冷却 UI 组件
///
/// Displays skill cooldown progress as a circular progress bar with remaining seconds.
#[derive(Component, Debug, Clone)]
pub struct SkillCooldownUI {
    /// Skill ID this UI represents
    /// 此 UI 代表的技能 ID
    pub skill_id: String,
    /// Entity reference to the skill component (optional, for direct access)
    /// 技能组件的实体引用（可选，用于直接访问）
    pub skill_entity: Option<Entity>,
}
