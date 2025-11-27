/// Combat configuration resource
/// 战斗配置资源
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Combat system configuration
///
/// 战斗系统配置
///
/// Loaded from `assets/data/combat_config.ron`.
/// Contains tunable parameters for combat feel and balance.
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct CombatConfig {
    /// Combo window duration (seconds)
    /// 连击窗口时间（秒）
    pub combo_window: f32,

    /// Hitfreeze duration for light hits (seconds)
    /// 轻击定格时长（秒）- 约 3 帧
    pub hitfreeze_light: f32,

    /// Hitfreeze duration for heavy hits (seconds)
    /// 重击定格时长（秒）- 约 5 帧
    pub hitfreeze_heavy: f32,

    /// Hitfreeze duration for critical hits (seconds)
    /// 暴击定格时长（秒）- 约 7 帧
    pub hitfreeze_critical: f32,

    /// Invincibility duration after taking damage (seconds)
    /// 受击后无敌帧时长（秒）
    pub invincibility_duration: f32,

    /// Damage number lifetime (seconds)
    /// 伤害数字显示时长（秒）
    pub damage_number_lifetime: f32,

    /// Screen shake amplitude for light hits (pixels)
    /// 轻击屏幕震动幅度（像素）
    pub screen_shake_light: f32,

    /// Screen shake amplitude for heavy hits (pixels)
    /// 重击屏幕震动幅度（像素）
    pub screen_shake_heavy: f32,

    /// Screen shake amplitude for critical hits (pixels)
    /// 暴击屏幕震动幅度（像素）
    pub screen_shake_critical: f32,

    /// Screen shake duration (seconds)
    /// 屏幕震动持续时间（秒）
    pub screen_shake_duration: f32,

    /// Particle count for light hits
    /// 轻击粒子数量
    pub particle_count_light: u32,

    /// Particle count for heavy hits
    /// 重击粒子数量
    pub particle_count_heavy: u32,

    /// Particle count for critical hits
    /// 暴击粒子数量
    pub particle_count_critical: u32,

    /// Particle lifetime (seconds)
    /// 粒子存活时间（秒）
    pub particle_lifetime: f32,

    /// Knockback force for third hit (pixels)
    /// 第三击击退力（像素）
    pub knockback_third_hit: f32,
}

impl Default for CombatConfig {
    fn default() -> Self {
        Self {
            combo_window: 1.0,
            hitfreeze_light: 0.05,     // ~3 frames at 60 FPS
            hitfreeze_heavy: 0.083,    // ~5 frames at 60 FPS
            hitfreeze_critical: 0.117, // ~7 frames at 60 FPS
            invincibility_duration: 0.5,
            damage_number_lifetime: 1.0,
            screen_shake_light: 2.0,
            screen_shake_heavy: 4.0,
            screen_shake_critical: 6.0,
            screen_shake_duration: 0.2,
            particle_count_light: 5,
            particle_count_heavy: 15,
            particle_count_critical: 25,
            particle_lifetime: 0.5,
            knockback_third_hit: 50.0,
        }
    }
}

impl CombatConfig {
    /// Load combat config from RON file
    ///
    /// # Arguments
    /// * `path` - Path to RON file (e.g., "combat_config.ron")
    ///
    /// # Returns
    /// Result containing CombatConfig or error message
    pub fn load_from_file(path: &str) -> Result<Self, String> {
        let contents = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read combat config file '{}': {}", path, e))?;

        let config: Self = ron::from_str(&contents)
            .map_err(|e| format!("Failed to parse combat config file '{}': {}", path, e))?;

        Ok(config)
    }

    /// Get hitfreeze duration based on hit type
    ///
    /// # Arguments
    /// * `is_critical` - Whether this is a critical hit
    /// * `is_heavy` - Whether this is a heavy hit (combo third hit)
    ///
    /// # Returns
    /// Hitfreeze duration in seconds
    pub fn get_hitfreeze_duration(&self, is_critical: bool, is_heavy: bool) -> f32 {
        if is_critical {
            self.hitfreeze_critical
        } else if is_heavy {
            self.hitfreeze_heavy
        } else {
            self.hitfreeze_light
        }
    }

    /// Get screen shake amplitude based on hit type
    pub fn get_screen_shake_amplitude(&self, is_critical: bool, is_heavy: bool) -> f32 {
        if is_critical {
            self.screen_shake_critical
        } else if is_heavy {
            self.screen_shake_heavy
        } else {
            self.screen_shake_light
        }
    }

    /// Get particle count based on hit type
    pub fn get_particle_count(&self, is_critical: bool, is_heavy: bool) -> u32 {
        if is_critical {
            self.particle_count_critical
        } else if is_heavy {
            self.particle_count_heavy
        } else {
            self.particle_count_light
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combat_config_default() {
        let config = CombatConfig::default();
        assert_eq!(config.combo_window, 1.0);
        assert_eq!(config.invincibility_duration, 0.5);
    }

    #[test]
    fn test_get_hitfreeze_duration() {
        let config = CombatConfig::default();

        assert_eq!(config.get_hitfreeze_duration(false, false), 0.05);
        assert_eq!(config.get_hitfreeze_duration(false, true), 0.083);
        assert_eq!(config.get_hitfreeze_duration(true, false), 0.117);
    }

    #[test]
    fn test_get_screen_shake_amplitude() {
        let config = CombatConfig::default();

        assert_eq!(config.get_screen_shake_amplitude(false, false), 2.0);
        assert_eq!(config.get_screen_shake_amplitude(false, true), 4.0);
        assert_eq!(config.get_screen_shake_amplitude(true, false), 6.0);
    }

    #[test]
    fn test_get_particle_count() {
        let config = CombatConfig::default();

        assert_eq!(config.get_particle_count(false, false), 5);
        assert_eq!(config.get_particle_count(false, true), 15);
        assert_eq!(config.get_particle_count(true, false), 25);
    }
}
