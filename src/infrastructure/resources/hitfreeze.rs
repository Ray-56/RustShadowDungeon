/// Hitfreeze (hit stop / time freeze) resource
/// 打击定格资源
use bevy::prelude::*;

/// Global hitfreeze timer
///
/// 全局打击定格计时器
///
/// Controls Time<Virtual> dilation for hit stop effect.
/// When `remaining > 0`, game time is paused (except UI/audio).
#[derive(Resource, Debug, Clone)]
pub struct HitfreezeTimer {
    /// Remaining hitfreeze duration (seconds, using Time<Real>)
    pub remaining: f32,
}

impl Default for HitfreezeTimer {
    fn default() -> Self {
        Self { remaining: 0.0 }
    }
}

impl HitfreezeTimer {
    /// Create a new HitfreezeTimer
    pub fn new() -> Self {
        Self::default()
    }

    /// Trigger hitfreeze for a duration
    ///
    /// If hitfreeze is already active, takes the maximum of current and new duration.
    /// This prevents hitfreeze from being cut short by weaker hits.
    ///
    /// # Arguments
    /// * `duration` - Hitfreeze duration in seconds
    pub fn trigger(&mut self, duration: f32) {
        self.remaining = self.remaining.max(duration);
    }

    /// Check if hitfreeze is currently active
    pub fn is_active(&self) -> bool {
        self.remaining > 0.0
    }

    /// Update hitfreeze timer (use Time<Real> delta)
    ///
    /// # Arguments
    /// * `delta` - Time delta from Time<Real>
    ///
    /// # Returns
    /// true if hitfreeze just ended this frame
    pub fn update(&mut self, delta: f32) -> bool {
        if self.remaining > 0.0 {
            self.remaining -= delta;
            if self.remaining <= 0.0 {
                self.remaining = 0.0;
                return true; // Hitfreeze just ended
            }
        }
        false
    }

    /// Get progress (0.0 = just started, 1.0 = finished)
    ///
    /// Useful for visual effects that scale with hitfreeze progress.
    pub fn progress(&self, original_duration: f32) -> f32 {
        if original_duration <= 0.0 {
            return 1.0;
        }
        1.0 - (self.remaining / original_duration).clamp(0.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hitfreeze_timer_new() {
        let timer = HitfreezeTimer::new();
        assert_eq!(timer.remaining, 0.0);
        assert!(!timer.is_active());
    }

    #[test]
    fn test_hitfreeze_trigger() {
        let mut timer = HitfreezeTimer::new();
        timer.trigger(0.1);
        assert_eq!(timer.remaining, 0.1);
        assert!(timer.is_active());
    }

    #[test]
    fn test_hitfreeze_trigger_max() {
        let mut timer = HitfreezeTimer::new();
        timer.trigger(0.1);
        timer.trigger(0.05); // Should not reduce
        assert_eq!(timer.remaining, 0.1);

        timer.trigger(0.15); // Should increase
        assert_eq!(timer.remaining, 0.15);
    }

    #[test]
    fn test_hitfreeze_update() {
        let mut timer = HitfreezeTimer::new();
        timer.trigger(0.1);

        let ended = timer.update(0.05);
        assert!(!ended);
        assert_eq!(timer.remaining, 0.05);

        let ended = timer.update(0.05);
        assert!(ended); // Should return true when finishing
        assert_eq!(timer.remaining, 0.0);
        assert!(!timer.is_active());
    }

    #[test]
    fn test_hitfreeze_progress() {
        let mut timer = HitfreezeTimer::new();
        timer.trigger(1.0);

        assert_eq!(timer.progress(1.0), 0.0); // Just started

        timer.update(0.5);
        assert_eq!(timer.progress(1.0), 0.5); // Halfway

        timer.update(0.5);
        assert_eq!(timer.progress(1.0), 1.0); // Finished
    }
}
