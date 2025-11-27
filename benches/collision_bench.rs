/// Performance benchmarks for collision detection system
///
/// 碰撞检测系统性能基准测试
///
/// T108: Tests collision detection performance with 15 enemies
/// Target: <3ms per frame for collision detection
use criterion::{criterion_group, criterion_main, Criterion};
use rust_shadow_dungeon::domain::combat::collision::{Rect, aabb_intersects};

/// Benchmark AABB intersection check (pure function)
///
/// 基准测试：AABB 相交检测（纯函数）
fn aabb_intersection_benchmark(c: &mut Criterion) {
    let rect_a = Rect::new(0.0, 0.0, 32.0, 32.0);
    let rect_b = Rect::new(16.0, 16.0, 32.0, 32.0); // Overlapping
    let rect_c = Rect::new(100.0, 100.0, 32.0, 32.0); // Not overlapping

    c.bench_function("aabb_intersects_overlapping", |b| {
        b.iter(|| {
            let _result = aabb_intersects(&rect_a, &rect_b);
        });
    });

    c.bench_function("aabb_intersects_not_overlapping", |b| {
        b.iter(|| {
            let _result = aabb_intersects(&rect_a, &rect_c);
        });
    });
}

/// Benchmark collision detection with multiple entities
///
/// 基准测试：多实体碰撞检测
fn multiple_entity_collision_benchmark(c: &mut Criterion) {
    // Create 15 enemy hurtboxes (simulating 15 enemies)
    let mut enemy_hurtboxes = Vec::new();
    for i in 0..15 {
        enemy_hurtboxes.push(Rect::new(
            (i as f32) * 50.0, // Spread enemies horizontally
            0.0,
            16.0, // 16x16 enemy size
            16.0,
        ));
    }

    // Create 5 player hitboxes (simulating player attacks)
    let mut player_hitboxes = Vec::new();
    for i in 0..5 {
        player_hitboxes.push(Rect::new(
            (i as f32) * 30.0,
            0.0,
            32.0, // 32x32 hitbox size
            32.0,
        ));
    }

    c.bench_function("collision_detection_15_enemies_5_hitboxes", |b| {
        b.iter(|| {
            let mut hit_count = 0;
            // Check each hitbox against each hurtbox
            for hitbox in &player_hitboxes {
                for hurtbox in &enemy_hurtboxes {
                    if aabb_intersects(hitbox, hurtbox) {
                        hit_count += 1;
                    }
                }
            }
            // Prevent optimization
            criterion::black_box(hit_count);
        });
    });
}

/// Benchmark collision detection with many overlapping entities
///
/// 基准测试：大量重叠实体的碰撞检测
fn dense_collision_benchmark(c: &mut Criterion) {
    // Create 20 tightly packed hurtboxes
    let mut hurtboxes = Vec::new();
    for i in 0..20 {
        hurtboxes.push(Rect::new(
            (i as f32) * 10.0, // Very close together (overlapping)
            0.0,
            16.0,
            16.0,
        ));
    }

    // Create 10 hitboxes
    let mut hitboxes = Vec::new();
    for i in 0..10 {
        hitboxes.push(Rect::new(
            (i as f32) * 15.0,
            0.0,
            32.0,
            32.0,
        ));
    }

    c.bench_function("collision_detection_dense_20_hurtboxes_10_hitboxes", |b| {
        b.iter(|| {
            let mut hit_count = 0;
            for hitbox in &hitboxes {
                for hurtbox in &hurtboxes {
                    if aabb_intersects(hitbox, hurtbox) {
                        hit_count += 1;
                    }
                }
            }
            criterion::black_box(hit_count);
        });
    });
}

criterion_group!(benches, aabb_intersection_benchmark, multiple_entity_collision_benchmark, dense_collision_benchmark);
criterion_main!(benches);

