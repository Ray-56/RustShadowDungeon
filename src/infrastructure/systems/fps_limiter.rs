//! FPS limiter to cap frame rate at 60 FPS

use bevy::prelude::*;
use std::time::{Duration, Instant};

/// Resource to track frame timing for FPS limiting
#[derive(Resource)]
pub struct FpsLimiter {
    /// Target frame duration (1/60 second = ~16.67ms)
    target_duration: Duration,
    /// Last frame time
    last_frame: Instant,
}

impl Default for FpsLimiter {
    fn default() -> Self {
        Self {
            target_duration: Duration::from_micros(16_667), // 60 FPS = 16.667ms
            last_frame: Instant::now(),
        }
    }
}

/// System to limit FPS to 60
pub fn fps_limit_system(mut limiter: ResMut<FpsLimiter>) {
    let now = Instant::now();
    let elapsed = now.duration_since(limiter.last_frame);

    // If frame finished too quickly, sleep for the remaining time
    if elapsed < limiter.target_duration {
        let sleep_duration = limiter.target_duration - elapsed;
        std::thread::sleep(sleep_duration);
    }

    limiter.last_frame = Instant::now();
}
