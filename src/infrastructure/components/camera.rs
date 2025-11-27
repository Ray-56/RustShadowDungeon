use bevy::prelude::*;

/// Component to make entity follow the player
#[derive(Component, Debug, Clone)]
pub struct CameraFollow {
    /// Offset from the target
    pub offset: Vec3,
    /// Smoothness factor (0.0-1.0)
    pub smoothness: f32,
}

impl Default for CameraFollow {
    fn default() -> Self {
        Self { offset: Vec3::new(0.0, 0.0, 0.0), smoothness: 0.1 }
    }
}

/// Screen shake component for camera
///
/// 屏幕震动组件
///
/// Added to camera entity to create screen shake effect.
/// System will apply random offset based on amplitude and duration.
#[derive(Component, Debug, Clone)]
pub struct ScreenShake {
    /// Shake amplitude (pixels)
    /// 震动幅度（像素）
    pub amplitude: f32,
    /// Remaining duration (seconds)
    /// 剩余持续时间（秒）
    pub duration: f32,
    /// Timer for shake effect (used for decay)
    /// 震动计时器（用于衰减）
    pub timer: f32,
}

impl ScreenShake {
    /// Create a new ScreenShake
    pub fn new(amplitude: f32, duration: f32) -> Self {
        Self { amplitude, duration, timer: 0.0 }
    }

    /// Update shake timer (returns true if still active)
    pub fn update(&mut self, delta: f32) -> bool {
        self.timer += delta;
        self.timer < self.duration
    }

    /// Get current shake intensity (decays over time)
    pub fn intensity(&self) -> f32 {
        if self.duration <= 0.0 {
            return 0.0;
        }
        let progress = (self.timer / self.duration).min(1.0);
        // Decay: start at full amplitude, fade to 0
        self.amplitude * (1.0 - progress)
    }
}
