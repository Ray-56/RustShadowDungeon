/// Combo system logic (pure functions)
/// 连击系统逻辑（纯函数）
use serde::{Deserialize, Serialize};

/// Combo state machine
///
/// 连击状态机：
/// - `Idle`: 无连击状态
/// - `FirstHit`: 第 1 击（轻击，10 伤害）
/// - `SecondHit`: 第 2 击（轻击，10 伤害）
/// - `ThirdHit`: 第 3 击（重击，20 伤害）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ComboState {
    Idle,
    FirstHit,
    SecondHit,
    ThirdHit,
}

impl Default for ComboState {
    fn default() -> Self {
        Self::Idle
    }
}

impl ComboState {
    /// Get the base damage multiplier for this combo state
    ///
    /// 获取此连击状态的基础伤害倍率
    pub const fn damage_multiplier(&self) -> f32 {
        match self {
            Self::Idle | Self::FirstHit | Self::SecondHit => 1.0, // Normal damage
            Self::ThirdHit => 2.0,                                // 2x damage (heavy hit)
        }
    }

    /// Check if this is a heavy hit (triggers special effects)
    ///
    /// 检查是否为重击（触发特殊效果）
    pub const fn is_heavy_hit(&self) -> bool {
        matches!(self, Self::ThirdHit)
    }
}

/// Advance to the next combo state
///
/// 推进到下一个连击状态
///
/// # Arguments
/// * `current_state` - Current combo state
/// * `window_duration` - Duration of combo window in seconds
///
/// # Returns
/// (`next_state`, `new_window_remaining`)
///
/// # Examples
/// ```
/// use rust_shadow_dungeon::domain::combat::combo::{ComboState, advance_combo};
///
/// let (next_state, window) = advance_combo(ComboState::Idle, 1.0);
/// assert_eq!(next_state, ComboState::FirstHit);
/// assert_eq!(window, 1.0);
/// ```
pub const fn advance_combo(current_state: ComboState, window_duration: f32) -> (ComboState, f32) {
    let next_state = match current_state {
        ComboState::Idle => ComboState::FirstHit,
        ComboState::FirstHit => ComboState::SecondHit,
        ComboState::SecondHit => ComboState::ThirdHit,
        ComboState::ThirdHit => ComboState::Idle, // Reset after third hit
    };

    (next_state, window_duration)
}

/// Reset combo state to Idle
///
/// 重置连击状态为 Idle
///
/// # Returns
/// (`ComboState::Idle`, `0.0` window_remaining, `0` hit_count)
pub const fn reset_combo() -> (ComboState, f32, u32) {
    (ComboState::Idle, 0.0, 0)
}

/// Check if combo window has expired
///
/// 检查连击窗口是否超时
///
/// # Arguments
/// * `window_remaining` - Remaining time in combo window (seconds)
///
/// # Returns
/// true if window expired (time <= 0)
pub fn is_combo_expired(window_remaining: f32) -> bool {
    window_remaining <= 0.0
}

/// Get the combo count (number of hits in current combo)
///
/// 获取连击计数（当前连击中的击数）
pub const fn get_combo_count(state: ComboState) -> u32 {
    match state {
        ComboState::Idle => 0,
        ComboState::FirstHit => 1,
        ComboState::SecondHit => 2,
        ComboState::ThirdHit => 3,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combo_state_advance() {
        let (next, _) = advance_combo(ComboState::Idle, 1.0);
        assert_eq!(next, ComboState::FirstHit);

        let (next, _) = advance_combo(ComboState::FirstHit, 1.0);
        assert_eq!(next, ComboState::SecondHit);

        let (next, _) = advance_combo(ComboState::SecondHit, 1.0);
        assert_eq!(next, ComboState::ThirdHit);

        let (next, _) = advance_combo(ComboState::ThirdHit, 1.0);
        assert_eq!(next, ComboState::Idle);
    }

    #[test]
    fn test_combo_reset() {
        let (state, window, count) = reset_combo();
        assert_eq!(state, ComboState::Idle);
        assert_eq!(window, 0.0);
        assert_eq!(count, 0);
    }

    #[test]
    fn test_combo_expired() {
        assert!(is_combo_expired(0.0));
        assert!(is_combo_expired(-0.1));
        assert!(!is_combo_expired(0.1));
    }

    #[test]
    fn test_damage_multiplier() {
        assert_eq!(ComboState::Idle.damage_multiplier(), 1.0);
        assert_eq!(ComboState::FirstHit.damage_multiplier(), 1.0);
        assert_eq!(ComboState::SecondHit.damage_multiplier(), 1.0);
        assert_eq!(ComboState::ThirdHit.damage_multiplier(), 2.0);
    }

    #[test]
    fn test_is_heavy_hit() {
        assert!(!ComboState::Idle.is_heavy_hit());
        assert!(!ComboState::FirstHit.is_heavy_hit());
        assert!(!ComboState::SecondHit.is_heavy_hit());
        assert!(ComboState::ThirdHit.is_heavy_hit());
    }

    #[test]
    fn test_get_combo_count() {
        assert_eq!(get_combo_count(ComboState::Idle), 0);
        assert_eq!(get_combo_count(ComboState::FirstHit), 1);
        assert_eq!(get_combo_count(ComboState::SecondHit), 2);
        assert_eq!(get_combo_count(ComboState::ThirdHit), 3);
    }
}
