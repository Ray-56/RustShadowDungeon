//! Telegraph area calculations
//!
//! Pure domain logic for telegraph (warning area) geometry calculations.

/// Telegraph shape type
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TelegraphShape {
    /// Circle shape
    Circle {
        /// Radius (pixels)
        radius: f32,
    },
    /// Rectangle shape
    Rectangle {
        /// Width (pixels)
        width: f32,
        /// Height (pixels)
        height: f32,
    },
    /// Sector shape (arc)
    Sector {
        /// Radius (pixels)
        radius: f32,
        /// Angle (radians)
        angle: f32,
    },
}

/// Telegraph area definition
#[derive(Debug, Clone, PartialEq)]
pub struct TelegraphArea {
    /// Shape type
    pub shape: TelegraphShape,
    /// Center point coordinates
    pub center: (f32, f32),
    /// Rotation angle (radians)
    pub rotation: f32,
}

impl TelegraphArea {
    /// Check if a point is contained within the telegraph area
    ///
    /// # Arguments
    /// * `point` - Point coordinates (x, y)
    ///
    /// # Returns
    /// True if point is within the area
    pub fn contains_point(&self, point: (f32, f32)) -> bool {
        match self.shape {
            TelegraphShape::Circle { radius } => {
                let dx = point.0 - self.center.0;
                let dy = point.1 - self.center.1;
                (dx * dx + dy * dy).sqrt() <= radius
            }
            TelegraphShape::Rectangle { width, height } => {
                // Simple axis-aligned rectangle check (rotation not implemented yet)
                let dx = (point.0 - self.center.0).abs();
                let dy = (point.1 - self.center.1).abs();
                dx <= width / 2.0 && dy <= height / 2.0
            }
            TelegraphShape::Sector { radius, angle: _angle } => {
                // Check if point is within radius
                let dx = point.0 - self.center.0;
                let dy = point.1 - self.center.1;
                let distance = (dx * dx + dy * dy).sqrt();
                
                if distance > radius {
                    return false;
                }

                // Check if point is within angle (simplified - assumes rotation = 0)
                // TODO: Implement proper angle check with rotation
                true
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_contains_point() {
        let area = TelegraphArea {
            shape: TelegraphShape::Circle { radius: 10.0 },
            center: (0.0, 0.0),
            rotation: 0.0,
        };

        assert!(area.contains_point((0.0, 0.0)));
        assert!(area.contains_point((5.0, 0.0)));
        assert!(area.contains_point((0.0, 10.0)));
        assert!(!area.contains_point((11.0, 0.0)));
        assert!(!area.contains_point((0.0, 11.0)));
    }

    #[test]
    fn test_rectangle_contains_point() {
        let area = TelegraphArea {
            shape: TelegraphShape::Rectangle {
                width: 20.0,
                height: 10.0,
            },
            center: (0.0, 0.0),
            rotation: 0.0,
        };

        assert!(area.contains_point((0.0, 0.0)));
        assert!(area.contains_point((10.0, 5.0)));
        assert!(area.contains_point((-10.0, -5.0)));
        assert!(!area.contains_point((11.0, 0.0)));
        assert!(!area.contains_point((0.0, 6.0)));
    }
}

