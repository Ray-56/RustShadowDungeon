/// AABB collision detection for combat system
/// AABB 碰撞检测
use serde::{Deserialize, Serialize};

/// Axis-Aligned Bounding Box rectangle
///
/// 轴对齐包围盒矩形
///
/// Used for `HitBox` and `HurtBox` collision detection.
/// Supports pixel-perfect collision (1 pixel overlap = hit).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Rect {
    /// Left edge X coordinate (in pixels)
    pub x: f32,
    /// Top edge Y coordinate (in pixels)
    pub y: f32,
    /// Width (in pixels)
    pub width: f32,
    /// Height (in pixels)
    pub height: f32,
}

impl Rect {
    /// Create a new Rect
    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }

    /// Check if this rect intersects with another rect (AABB collision)
    ///
    /// 检查此矩形是否与另一个矩形相交（AABB 碰撞检测）
    ///
    /// Returns true if there is any overlap (even 1 pixel).
    /// Returns false if rects are only touching (0 pixel overlap).
    ///
    /// # Arguments
    /// * `other` - The other rectangle to test against
    ///
    /// # Examples
    /// ```
    /// use rust_shadow_dungeon::domain::combat::collision::Rect;
    ///
    /// let a = Rect::new(0.0, 0.0, 32.0, 32.0);
    /// let b = Rect::new(16.0, 16.0, 32.0, 32.0);
    /// assert!(a.intersects(&b));
    ///
    /// let c = Rect::new(32.0, 0.0, 32.0, 32.0);
    /// assert!(!a.intersects(&c)); // Touching but not overlapping
    /// ```
    pub fn intersects(&self, other: &Self) -> bool {
        // AABB intersection check:
        // A and B overlap if:
        // - A.left < B.right AND A.right > B.left (horizontal overlap)
        // - A.top < B.bottom AND A.bottom > B.top (vertical overlap)
        self.x < other.x + other.width  // A.left < B.right
            && self.x + self.width > other.x  // A.right > B.left
            && self.y < other.y + other.height  // A.top < B.bottom
            && self.y + self.height > other.y // A.bottom > B.top
    }

    /// Get the center point of the rectangle
    ///
    /// 获取矩形中心点坐标
    ///
    /// # Returns
    /// (`center_x`, `center_y`) tuple
    pub fn center(&self) -> (f32, f32) {
        (self.x + self.width / 2.0, self.y + self.height / 2.0)
    }

    /// Check if this rect contains a point
    ///
    /// 检查点是否在矩形内
    pub fn contains_point(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }

    /// Get the right edge X coordinate
    pub fn right(&self) -> f32 {
        self.x + self.width
    }

    /// Get the bottom edge Y coordinate
    pub fn bottom(&self) -> f32 {
        self.y + self.height
    }

    /// Get the area of the rectangle
    pub fn area(&self) -> f32 {
        self.width * self.height
    }
}

impl Default for Rect {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, width: 0.0, height: 0.0 }
    }
}

/// Check if two AABBs intersect (alternative function-based API)
///
/// 检查两个 AABB 是否相交（函数式 API）
pub fn aabb_intersects(a: &Rect, b: &Rect) -> bool {
    a.intersects(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rect_intersects() {
        let a = Rect::new(0.0, 0.0, 32.0, 32.0);
        let b = Rect::new(16.0, 16.0, 32.0, 32.0);
        assert!(a.intersects(&b));
    }

    #[test]
    fn test_rect_no_intersect() {
        let a = Rect::new(0.0, 0.0, 32.0, 32.0);
        let b = Rect::new(100.0, 100.0, 32.0, 32.0);
        assert!(!a.intersects(&b));
    }

    #[test]
    fn test_rect_touching_no_overlap() {
        let a = Rect::new(0.0, 0.0, 32.0, 32.0);
        let b = Rect::new(32.0, 0.0, 32.0, 32.0);
        assert!(!a.intersects(&b));
    }

    #[test]
    fn test_rect_center() {
        let rect = Rect::new(10.0, 20.0, 40.0, 60.0);
        let (cx, cy) = rect.center();
        assert_eq!(cx, 30.0);
        assert_eq!(cy, 50.0);
    }

    #[test]
    fn test_rect_contains_point() {
        let rect = Rect::new(0.0, 0.0, 100.0, 100.0);
        assert!(rect.contains_point(50.0, 50.0));
        assert!(!rect.contains_point(150.0, 50.0));
    }

    #[test]
    fn test_rect_area() {
        let rect = Rect::new(0.0, 0.0, 10.0, 20.0);
        assert_eq!(rect.area(), 200.0);
    }

    #[test]
    fn test_aabb_intersects_function() {
        let a = Rect::new(0.0, 0.0, 32.0, 32.0);
        let b = Rect::new(16.0, 16.0, 32.0, 32.0);
        assert!(aabb_intersects(&a, &b));
    }
}
