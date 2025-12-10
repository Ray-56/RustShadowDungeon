//! Performance benchmarks for Enemy AI System
//!
//! Tests AI update performance to ensure it meets the <2ms budget per frame.
//!
//! 敌人 AI 系统性能基准测试
//! 测试 AI 更新性能，确保符合每帧 <2ms 预算

use bevy::prelude::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_shadow_dungeon::domain::enemy::ai::*;
use rust_shadow_dungeon::infrastructure::components::enemy::{
    AggroTarget, AttackConfig, EnemyAI, PatrolConfig, Perception,
};
use rust_shadow_dungeon::infrastructure::components::{Enemy, Player};

/// Benchmark: AI state transition calculations
///
/// Tests the performance of state transition logic (domain layer functions).
fn bench_ai_state_transitions(c: &mut Criterion) {
    c.bench_function("ai_state_transitions", |b| {
        b.iter(|| {
            // Test should_transition_to_chase
            black_box(should_transition_to_chase(100.0, 200.0, true));
            black_box(should_transition_to_chase(300.0, 200.0, false));

            // Test should_drop_aggro
            black_box(should_drop_aggro(500.0, 400.0, 0.0, 5.0));
            black_box(should_drop_aggro(200.0, 400.0, 6.0, 5.0));

            // Test should_transition_to_attack
            black_box(should_transition_to_attack(30.0, 32.0, true));
            black_box(should_transition_to_attack(50.0, 32.0, false));

            // Test calculate_chase_direction
            black_box(calculate_chase_direction(Vec2::new(0.0, 0.0), Vec2::new(100.0, 0.0)));

            // Test calculate_patrol_target
            black_box(calculate_patrol_target(Vec2::new(0.0, 0.0), Vec2::new(200.0, 0.0), 100.0));
        });
    });
}

/// Benchmark: Line of sight check
///
/// Tests the performance of line of sight raycast.
fn bench_line_of_sight(c: &mut Criterion) {
    c.bench_function("line_of_sight_no_obstacles", |b| {
        let enemy_pos = Vec2::new(0.0, 0.0);
        let target_pos = Vec2::new(100.0, 0.0);
        let obstacles: Vec<(f32, f32, f32, f32)> = vec![];

        b.iter(|| {
            black_box(check_line_of_sight(enemy_pos, target_pos, &obstacles));
        });
    });

    c.bench_function("line_of_sight_with_obstacles", |b| {
        let enemy_pos = Vec2::new(0.0, 0.0);
        let target_pos = Vec2::new(200.0, 0.0);
        let obstacles =
            vec![(50.0, -10.0, 20.0, 20.0), (100.0, -10.0, 20.0, 20.0), (150.0, -10.0, 20.0, 20.0)];

        b.iter(|| {
            black_box(check_line_of_sight(enemy_pos, target_pos, &obstacles));
        });
    });
}

/// Benchmark: Perception system simulation
///
/// Simulates perception checks for multiple enemies.
/// Target: <0.5ms per check (throttled to every 3-5 frames)
fn bench_perception_simulation(c: &mut Criterion) {
    c.bench_function("perception_check_10_enemies", |b| {
        let mut perception = Perception::new(200.0, 32.0, 400.0);
        let player_pos = Vec2::new(100.0, 0.0);
        let obstacles: Vec<(f32, f32, f32, f32)> = vec![];

        b.iter(|| {
            for i in 0..10 {
                let enemy_pos = Vec2::new(i as f32 * 50.0, 0.0);
                let distance = enemy_pos.distance(player_pos);

                if distance <= perception.detection_range {
                    black_box(check_line_of_sight(enemy_pos, player_pos, &obstacles));
                    black_box(should_transition_to_chase(
                        distance,
                        perception.detection_range,
                        true,
                    ));
                }
            }
        });
    });

    c.bench_function("perception_check_50_enemies", |b| {
        let mut perception = Perception::new(200.0, 32.0, 400.0);
        let player_pos = Vec2::new(100.0, 0.0);
        let obstacles: Vec<(f32, f32, f32, f32)> = vec![];

        b.iter(|| {
            for i in 0..50 {
                let enemy_pos = Vec2::new(i as f32 * 20.0, 0.0);
                let distance = enemy_pos.distance(player_pos);

                if distance <= perception.detection_range {
                    black_box(check_line_of_sight(enemy_pos, player_pos, &obstacles));
                }
            }
        });
    });
}

/// Benchmark: State machine update simulation
///
/// Simulates state machine updates for multiple enemies.
/// Target: <0.3ms per update
fn bench_state_machine_simulation(c: &mut Criterion) {
    c.bench_function("state_machine_10_enemies", |b| {
        b.iter(|| {
            for _ in 0..10 {
                let mut ai = EnemyAI::new();
                let perception = Perception::new(200.0, 32.0, 400.0);
                let aggro = AggroTarget::default();
                
                // Simulate state transition logic
                match ai.state {
                    AIState::Patrol => {
                        if perception.target_position.is_some() {
                            black_box(should_transition_to_chase(
                                100.0,
                                perception.detection_range,
                                perception.has_line_of_sight,
                            ));
                        }
                    }
                    AIState::Chase => {
                        let attack_config = AttackConfig::new(
                            32.0, 1.5, 5.0,
                            rust_shadow_dungeon::infrastructure::components::enemy::AttackType::Melee,
                            0.5, 0.2,
                        );
                        black_box(should_transition_to_attack(
                            30.0,
                            attack_config.attack_range,
                            attack_config.is_cooldown_ready(),
                        ));
                    }
                    _ => {}
                }
            }
        });
    });

    c.bench_function("state_machine_100_enemies", |b| {
        b.iter(|| {
            for _ in 0..100 {
                let ai = EnemyAI::new();
                let perception = Perception::new(200.0, 32.0, 400.0);

                // Simulate state check
                black_box(ai.state);
                black_box(perception.target_position);
            }
        });
    });
}

/// Benchmark: Full AI update cycle
///
/// Simulates a complete AI update cycle (perception + state machine + movement).
/// Target: <2ms total per frame for 100 enemies
fn bench_full_ai_update(c: &mut Criterion) {
    c.bench_function("full_ai_update_10_enemies", |b| {
        b.iter(|| {
            for i in 0..10 {
                let enemy_pos = Vec2::new(i as f32 * 50.0, 0.0);
                let player_pos = Vec2::new(100.0, 0.0);
                let distance = enemy_pos.distance(player_pos);

                // Perception check
                let obstacles: Vec<(f32, f32, f32, f32)> = vec![];
                let has_los = check_line_of_sight(enemy_pos, player_pos, &obstacles);

                // State transition
                let should_chase = should_transition_to_chase(distance, 200.0, has_los);

                // Movement calculation (simplified)
                if should_chase {
                    black_box(calculate_chase_direction(enemy_pos, player_pos));
                }
            }
        });
    });

    c.bench_function("full_ai_update_100_enemies", |b| {
        b.iter(|| {
            for i in 0..100 {
                let enemy_pos = Vec2::new(i as f32 * 10.0, 0.0);
                let player_pos = Vec2::new(500.0, 0.0);
                let distance = enemy_pos.distance(player_pos);

                // Simplified check (only for enemies in range)
                if distance <= 200.0 {
                    let obstacles: Vec<(f32, f32, f32, f32)> = vec![];
                    black_box(check_line_of_sight(enemy_pos, player_pos, &obstacles));
                }
            }
        });
    });
}

criterion_group!(
    benches,
    bench_ai_state_transitions,
    bench_line_of_sight,
    bench_perception_simulation,
    bench_state_machine_simulation,
    bench_full_ai_update
);
criterion_main!(benches);
