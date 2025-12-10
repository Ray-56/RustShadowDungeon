/// Performance benchmarks for dungeon system
///
/// 地下城系统性能基准测试
///
/// T081: Tests dungeon room loading and transition performance
/// Target: <16ms per frame for room operations (60 FPS budget)
use criterion::{criterion_group, criterion_main, Criterion};
use rust_shadow_dungeon::domain::dungeon::progression::{
    can_transition_to_room, check_room_cleared, get_target_room_entrance, is_room_cleared,
    mark_room_cleared, should_spawn_enemies, should_unlock_doors, RoomState,
};
use rust_shadow_dungeon::infrastructure::components::dungeon::DoorState;

/// Benchmark room state checking (pure function)
///
/// 基准测试：房间状态检查（纯函数）
fn room_state_check_benchmark(c: &mut Criterion) {
    let cleared_state = RoomState::Cleared;
    let uncleared_state = RoomState::Uncleared;
    let active_state = RoomState::Active;

    c.bench_function("is_room_cleared", |b| {
        b.iter(|| {
            let _result1 = is_room_cleared(cleared_state);
            let _result2 = is_room_cleared(uncleared_state);
            let _result3 = is_room_cleared(active_state);
        });
    });
}

/// Benchmark room state marking (pure function)
///
/// 基准测试：房间状态标记（纯函数）
fn room_state_marking_benchmark(c: &mut Criterion) {
    let uncleared_state = RoomState::Uncleared;
    let active_state = RoomState::Active;

    c.bench_function("mark_room_cleared", |b| {
        b.iter(|| {
            let _result1 = mark_room_cleared(uncleared_state);
            let _result2 = mark_room_cleared(active_state);
        });
    });
}

/// Benchmark spawn decision logic (pure function)
///
/// 基准测试：生成决策逻辑（纯函数）
fn spawn_decision_benchmark(c: &mut Criterion) {
    let cleared_state = RoomState::Cleared;
    let uncleared_state = RoomState::Uncleared;
    let active_state = RoomState::Active;

    c.bench_function("should_spawn_enemies", |b| {
        b.iter(|| {
            let _result1 = should_spawn_enemies(cleared_state);
            let _result2 = should_spawn_enemies(uncleared_state);
            let _result3 = should_spawn_enemies(active_state);
        });
    });
}

/// Benchmark door unlock decision logic (pure function)
///
/// 基准测试：门解锁决策逻辑（纯函数）
fn door_unlock_decision_benchmark(c: &mut Criterion) {
    let cleared_state = RoomState::Cleared;
    let uncleared_state = RoomState::Uncleared;
    let active_state = RoomState::Active;

    c.bench_function("should_unlock_doors", |b| {
        b.iter(|| {
            let _result1 = should_unlock_doors(cleared_state);
            let _result2 = should_unlock_doors(uncleared_state);
            let _result3 = should_unlock_doors(active_state);
        });
    });
}

/// Benchmark room clear checking (pure function)
///
/// 基准测试：房间清理检查（纯函数）
fn room_clear_check_benchmark(c: &mut Criterion) {
    // Simulate different enemy counts
    let enemy_counts = vec![0, 1, 3, 5, 10];

    c.bench_function("check_room_cleared", |b| {
        b.iter(|| {
            for &count in &enemy_counts {
                let _result = check_room_cleared(count);
            }
        });
    });
}

/// Benchmark door transition logic (pure function)
///
/// 基准测试：门过渡逻辑（纯函数）
fn door_transition_benchmark(c: &mut Criterion) {
    let unlocked_state = DoorState::Unlocked;
    let locked_state = DoorState::Locked;

    c.bench_function("can_transition_to_room", |b| {
        b.iter(|| {
            let _result1 = can_transition_to_room(unlocked_state);
            let _result2 = can_transition_to_room(locked_state);
        });
    });
}

/// Benchmark entrance position calculation (pure function)
///
/// 基准测试：入口位置计算（纯函数）
fn entrance_position_benchmark(c: &mut Criterion) {
    use bevy::math::Vec2;

    let entrance_positions =
        vec![Vec2::new(100.0, 0.0), Vec2::new(200.0, -170.0), Vec2::new(300.0, 50.0)];

    c.bench_function("get_target_room_entrance", |b| {
        b.iter(|| {
            for pos in &entrance_positions {
                let _result = get_target_room_entrance(*pos);
            }
        });
    });
}

criterion_group!(
    benches,
    room_state_check_benchmark,
    room_state_marking_benchmark,
    spawn_decision_benchmark,
    door_unlock_decision_benchmark,
    room_clear_check_benchmark,
    door_transition_benchmark,
    entrance_position_benchmark
);
criterion_main!(benches);
