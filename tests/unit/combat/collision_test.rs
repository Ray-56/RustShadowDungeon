/// Unit tests for AABB collision detection
/// 
/// These tests verify the domain layer collision detection logic:
/// - AABB intersection detection
/// - Edge cases (touching, no overlap, partial overlap)
/// - Pixel-perfect collision (1 pixel overlap = hit, 0 pixels = miss)

#[cfg(test)]
mod collision_tests {
    use rust_shadow_dungeon::domain::combat::collision::Rect;
    
    /// T022: Test AABB intersects - basic overlap
    #[test]
    fn test_aabb_intersects() {
        let rect_a = Rect {
            x: 0.0,
            y: 0.0,
            width: 32.0,
            height: 32.0,
        };
        let rect_b = Rect {
            x: 16.0,
            y: 16.0,
            width: 32.0,
            height: 32.0,
        };

        assert!(rect_a.intersects(&rect_b));
        assert!(rect_b.intersects(&rect_a)); // Symmetric
    }

    /// T022: Test AABB no intersect - completely separate
    #[test]
    fn test_aabb_no_intersect() {
        let rect_a = Rect {
            x: 0.0,
            y: 0.0,
            width: 32.0,
            height: 32.0,
        };
        let rect_b = Rect {
            x: 100.0,
            y: 100.0,
            width: 32.0,
            height: 32.0,
        };

        assert!(!rect_a.intersects(&rect_b));
        assert!(!rect_b.intersects(&rect_a));
    }

    /// T022: Test 1 pixel overlap (should hit)
    #[test]
    fn test_one_pixel_overlap() {
        let rect_a = Rect {
            x: 0.0,
            y: 0.0,
            width: 32.0,
            height: 32.0,
        };
        let rect_b = Rect {
            x: 31.0, // 1 pixel overlap
            y: 0.0,
            width: 32.0,
            height: 32.0,
        };

        assert!(rect_a.intersects(&rect_b));
    }

    /// T022: Test 0 pixel overlap (should miss)
    #[test]
    fn test_zero_pixel_overlap() {
        let rect_a = Rect {
            x: 0.0,
            y: 0.0,
            width: 32.0,
            height: 32.0,
        };
        let rect_b = Rect {
            x: 32.0, // Exactly touching, no overlap
            y: 0.0,
            width: 32.0,
            height: 32.0,
        };

        assert!(!rect_a.intersects(&rect_b));
    }

    /// T022: Test rect A contains rect B
    #[test]
    fn test_contains() {
        let rect_a = Rect {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 100.0,
        };
        let rect_b = Rect {
            x: 25.0,
            y: 25.0,
            width: 50.0,
            height: 50.0,
        };

        assert!(rect_a.intersects(&rect_b));
        assert!(rect_b.intersects(&rect_a));
    }

    /// T022: Test vertical overlap only (should miss)
    #[test]
    fn test_vertical_overlap_only() {
        let rect_a = Rect {
            x: 0.0,
            y: 0.0,
            width: 32.0,
            height: 32.0,
        };
        let rect_b = Rect {
            x: 100.0, // No horizontal overlap
            y: 16.0,  // Vertical overlap
            width: 32.0,
            height: 32.0,
        };

        assert!(!rect_a.intersects(&rect_b));
    }

    /// T022: Test horizontal overlap only (should miss)
    #[test]
    fn test_horizontal_overlap_only() {
        let rect_a = Rect {
            x: 0.0,
            y: 0.0,
            width: 32.0,
            height: 32.0,
        };
        let rect_b = Rect {
            x: 16.0,  // Horizontal overlap
            y: 100.0, // No vertical overlap
            width: 32.0,
            height: 32.0,
        };

        assert!(!rect_a.intersects(&rect_b));
    }

    /// T022: Test corner touch (should miss)
    #[test]
    fn test_corner_touch() {
        let rect_a = Rect {
            x: 0.0,
            y: 0.0,
            width: 32.0,
            height: 32.0,
        };
        let rect_b = Rect {
            x: 32.0, // Corner touch
            y: 32.0,
            width: 32.0,
            height: 32.0,
        };

        assert!(!rect_a.intersects(&rect_b));
    }

    /// Test negative coordinates
    #[test]
    fn test_negative_coordinates() {
        let rect_a = Rect {
            x: -16.0,
            y: -16.0,
            width: 32.0,
            height: 32.0,
        };
        let rect_b = Rect {
            x: 0.0,
            y: 0.0,
            width: 32.0,
            height: 32.0,
        };

        assert!(rect_a.intersects(&rect_b));
    }

    /// Test rect center calculation
    #[test]
    fn test_rect_center() {
        let rect = Rect {
            x: 10.0,
            y: 20.0,
            width: 40.0,
            height: 60.0,
        };

        let (cx, cy) = rect.center();
        assert_eq!(cx, 30.0); // 10 + 40/2
        assert_eq!(cy, 50.0); // 20 + 60/2
    }

    /// Test rect with zero size (degenerate case)
    #[test]
    fn test_zero_size_rect() {
        let rect_a = Rect {
            x: 10.0,
            y: 10.0,
            width: 0.0,
            height: 0.0,
        };
        let rect_b = Rect {
            x: 10.0,
            y: 10.0,
            width: 32.0,
            height: 32.0,
        };

        // Zero-size rect should not intersect
        assert!(!rect_a.intersects(&rect_b));
    }

    /// Test typical HitBox vs HurtBox scenario
    #[test]
    fn test_hitbox_vs_hurtbox_scenario() {
        // Player HitBox (32x32 at position 50, 100)
        let hitbox = Rect {
            x: 50.0,
            y: 100.0,
            width: 32.0,
            height: 32.0,
        };

        // Enemy HurtBox (16x16 at position 70, 110)
        let hurtbox = Rect {
            x: 70.0,
            y: 110.0,
            width: 16.0,
            height: 16.0,
        };

        // Should overlap (70-86 overlaps with 50-82 in X, 110-126 overlaps with 100-132 in Y)
        assert!(hitbox.intersects(&hurtbox));
    }

    /// Test HitBox misses HurtBox by 1 pixel
    #[test]
    fn test_hitbox_miss_by_one_pixel() {
        let hitbox = Rect {
            x: 0.0,
            y: 0.0,
            width: 32.0,
            height: 32.0,
        };

        let hurtbox = Rect {
            x: 33.0, // 1 pixel away
            y: 0.0,
            width: 16.0,
            height: 16.0,
        };

        assert!(!hitbox.intersects(&hurtbox));
    }

    /// Test rect right() method
    #[test]
    fn test_rect_right() {
        let rect = Rect {
            x: 10.0,
            y: 20.0,
            width: 40.0,
            height: 60.0,
        };
        assert_eq!(rect.right(), 50.0); // 10 + 40
    }

    /// Test rect bottom() method
    #[test]
    fn test_rect_bottom() {
        let rect = Rect {
            x: 10.0,
            y: 20.0,
            width: 40.0,
            height: 60.0,
        };
        assert_eq!(rect.bottom(), 80.0); // 20 + 60
    }

    /// Test Rect Default implementation
    #[test]
    fn test_rect_default() {
        let rect = Rect::default();
        assert_eq!(rect.x, 0.0);
        assert_eq!(rect.y, 0.0);
        assert_eq!(rect.width, 0.0);
        assert_eq!(rect.height, 0.0);
    }

    /// Test aabb_intersects function (alternative API)
    #[test]
    fn test_aabb_intersects_function() {
        use rust_shadow_dungeon::domain::combat::collision::aabb_intersects;
        
        let a = Rect {
            x: 0.0,
            y: 0.0,
            width: 32.0,
            height: 32.0,
        };
        let b = Rect {
            x: 16.0,
            y: 16.0,
            width: 32.0,
            height: 32.0,
        };
        assert!(aabb_intersects(&a, &b));
        
        let c = Rect {
            x: 100.0,
            y: 100.0,
            width: 32.0,
            height: 32.0,
        };
        assert!(!aabb_intersects(&a, &c));
    }

    /// Test Rect::new() constructor
    #[test]
    fn test_rect_new() {
        let rect = Rect::new(10.0, 20.0, 30.0, 40.0);
        assert_eq!(rect.x, 10.0);
        assert_eq!(rect.y, 20.0);
        assert_eq!(rect.width, 30.0);
        assert_eq!(rect.height, 40.0);
    }

    /// Test rect contains_point with various points
    #[test]
    fn test_rect_contains_point_various() {
        let rect = Rect::new(0.0, 0.0, 100.0, 100.0);
        
        // Inside
        assert!(rect.contains_point(50.0, 50.0));
        assert!(rect.contains_point(0.0, 0.0)); // Top-left corner
        assert!(rect.contains_point(100.0, 100.0)); // Bottom-right corner
        assert!(rect.contains_point(1.0, 1.0)); // Just inside
        
        // Outside
        assert!(!rect.contains_point(101.0, 50.0));
        assert!(!rect.contains_point(50.0, 101.0));
        assert!(!rect.contains_point(-1.0, 50.0));
        assert!(!rect.contains_point(50.0, -1.0));
    }

    /// Test rect intersection with same rect
    #[test]
    fn test_rect_intersects_self() {
        let rect = Rect::new(0.0, 0.0, 32.0, 32.0);
        assert!(rect.intersects(&rect));
    }

    /// Test rect intersection with contained rect
    #[test]
    fn test_rect_intersects_contained() {
        let outer = Rect::new(0.0, 0.0, 100.0, 100.0);
        let inner = Rect::new(25.0, 25.0, 50.0, 50.0);
        assert!(outer.intersects(&inner));
        assert!(inner.intersects(&outer));
    }

    /// Test rect intersection with overlapping edges
    #[test]
    fn test_rect_intersects_edge_cases() {
        // Overlapping by 1 pixel
        let a = Rect::new(0.0, 0.0, 32.0, 32.0);
        let b = Rect::new(31.0, 0.0, 32.0, 32.0);
        assert!(a.intersects(&b));

        // Overlapping by 0.5 pixels (floating point)
        let c = Rect::new(0.0, 0.0, 32.0, 32.0);
        let d = Rect::new(31.5, 0.0, 32.0, 32.0);
        assert!(c.intersects(&d));

        // Exactly touching (no overlap)
        let e = Rect::new(0.0, 0.0, 32.0, 32.0);
        let f = Rect::new(32.0, 0.0, 32.0, 32.0);
        assert!(!e.intersects(&f));
    }

    /// Test rect with negative coordinates
    #[test]
    fn test_rect_negative_coordinates_intersection() {
        let a = Rect::new(-16.0, -16.0, 32.0, 32.0);
        let b = Rect::new(0.0, 0.0, 32.0, 32.0);
        assert!(a.intersects(&b));
    }

    /// Test rect area calculation
    #[test]
    fn test_rect_area_various() {
        assert_eq!(Rect::new(0.0, 0.0, 10.0, 20.0).area(), 200.0);
        assert_eq!(Rect::new(5.0, 5.0, 10.0, 10.0).area(), 100.0);
        assert_eq!(Rect::new(0.0, 0.0, 1.0, 1.0).area(), 1.0);
        assert_eq!(Rect::new(0.0, 0.0, 0.0, 0.0).area(), 0.0);
    }

    /// Test rect right() and bottom() with various values
    #[test]
    fn test_rect_right_bottom_various() {
        let rect = Rect::new(10.0, 20.0, 30.0, 40.0);
        assert_eq!(rect.right(), 40.0); // 10 + 30
        assert_eq!(rect.bottom(), 60.0); // 20 + 40

        let rect2 = Rect::new(0.0, 0.0, 100.0, 200.0);
        assert_eq!(rect2.right(), 100.0);
        assert_eq!(rect2.bottom(), 200.0);
    }

    /// Test rect center with various sizes
    #[test]
    fn test_rect_center_various() {
        let rect1 = Rect::new(0.0, 0.0, 100.0, 100.0);
        let (cx, cy) = rect1.center();
        assert_eq!(cx, 50.0);
        assert_eq!(cy, 50.0);

        let rect2 = Rect::new(10.0, 20.0, 40.0, 60.0);
        let (cx, cy) = rect2.center();
        assert_eq!(cx, 30.0);
        assert_eq!(cy, 50.0);

        let rect3 = Rect::new(-10.0, -20.0, 20.0, 40.0);
        let (cx, cy) = rect3.center();
        assert_eq!(cx, 0.0);
        assert_eq!(cy, 0.0);
    }

    /// Test rect intersection with negative width/height (edge case)
    #[test]
    fn test_rect_negative_dimensions() {
        let rect_a = Rect::new(0.0, 0.0, -10.0, -10.0); // Negative dimensions
        let rect_b = Rect::new(0.0, 0.0, 32.0, 32.0);
        // Negative dimensions should not intersect
        assert!(!rect_a.intersects(&rect_b));
    }

    /// Test rect intersection with very small overlap
    #[test]
    fn test_rect_very_small_overlap() {
        let rect_a = Rect::new(0.0, 0.0, 32.0, 32.0);
        let rect_b = Rect::new(31.999, 0.0, 32.0, 32.0); // Very small overlap
        assert!(rect_a.intersects(&rect_b));
    }

    /// Test rect contains_point with edge points
    #[test]
    fn test_rect_contains_point_edges() {
        let rect = Rect::new(0.0, 0.0, 100.0, 100.0);
        // Edge points should be contained
        assert!(rect.contains_point(0.0, 0.0)); // Top-left
        assert!(rect.contains_point(100.0, 100.0)); // Bottom-right
        assert!(rect.contains_point(50.0, 0.0)); // Top edge
        assert!(rect.contains_point(0.0, 50.0)); // Left edge
    }

    /// Test rect contains_point with points just outside
    #[test]
    fn test_rect_contains_point_just_outside() {
        let rect = Rect::new(0.0, 0.0, 100.0, 100.0);
        assert!(!rect.contains_point(100.1, 50.0)); // Just outside right
        assert!(!rect.contains_point(50.0, 100.1)); // Just outside bottom
        assert!(!rect.contains_point(-0.1, 50.0)); // Just outside left
        assert!(!rect.contains_point(50.0, -0.1)); // Just outside top
    }

    /// Test rect area with zero dimensions
    #[test]
    fn test_rect_area_zero_dimensions() {
        assert_eq!(Rect::new(0.0, 0.0, 0.0, 10.0).area(), 0.0);
        assert_eq!(Rect::new(0.0, 0.0, 10.0, 0.0).area(), 0.0);
        assert_eq!(Rect::new(0.0, 0.0, 0.0, 0.0).area(), 0.0);
    }

    /// Test rect area with very large dimensions
    #[test]
    fn test_rect_area_large_dimensions() {
        let rect = Rect::new(0.0, 0.0, 10000.0, 10000.0);
        assert_eq!(rect.area(), 100000000.0);
    }

    /// Test rect right() and bottom() with negative coordinates
    #[test]
    fn test_rect_right_bottom_negative() {
        let rect = Rect::new(-10.0, -20.0, 30.0, 40.0);
        assert_eq!(rect.right(), 20.0); // -10 + 30
        assert_eq!(rect.bottom(), 20.0); // -20 + 40
    }

    /// Test rect intersection with one rect having zero area
    #[test]
    fn test_rect_intersects_zero_area() {
        let rect_a = Rect::new(0.0, 0.0, 0.0, 0.0); // Zero area
        let rect_b = Rect::new(0.0, 0.0, 32.0, 32.0);
        assert!(!rect_a.intersects(&rect_b));
        assert!(!rect_b.intersects(&rect_a));
    }

    /// Test aabb_intersects function with non-overlapping rects
    #[test]
    fn test_aabb_intersects_no_overlap() {
        use rust_shadow_dungeon::domain::combat::collision::aabb_intersects;
        
        let a = Rect::new(0.0, 0.0, 32.0, 32.0);
        let b = Rect::new(100.0, 100.0, 32.0, 32.0);
        assert!(!aabb_intersects(&a, &b));
    }

    /// Test rect center with zero dimensions
    #[test]
    fn test_rect_center_zero_dimensions() {
        let rect = Rect::new(10.0, 20.0, 0.0, 0.0);
        let (cx, cy) = rect.center();
        assert_eq!(cx, 10.0);
        assert_eq!(cy, 20.0);
    }
}

