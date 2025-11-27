/// Performance benchmarks for combat system
///
/// 战斗系统性能基准测试
///
/// T107: Tests combat logic performance (damage calculation, combo system, status effects)
/// Target: <2.5ms per frame for combat logic
use criterion::{criterion_group, criterion_main, Criterion};
use rust_shadow_dungeon::domain::combat::{ComboState, Element, Stats};
use rust_shadow_dungeon::domain::combat::damage::calculate_damage;
use rust_shadow_dungeon::domain::combat::combo::{advance_combo, get_combo_count};

/// Benchmark damage calculation (pure function)
///
/// 基准测试：伤害计算（纯函数）
fn damage_calculation_benchmark(c: &mut Criterion) {
    let attacker_stats = Stats {
        attack: 10.0,
        defense: 0.0,
        crit_rate: 0.1,
        crit_multiplier: 2.0,
        element_resistances: std::collections::HashMap::new(),
    };

    let defender_stats = Stats {
        attack: 5.0,
        defense: 2.0,
        crit_rate: 0.0,
        crit_multiplier: 1.0,
        element_resistances: std::collections::HashMap::new(),
    };

    c.bench_function("damage_calculation", |b| {
        b.iter(|| {
            let _result = calculate_damage(
                10.0, // base damage
                &attacker_stats,
                &defender_stats,
                Element::Physical,
            );
        });
    });
}

/// Benchmark combo state transitions
///
/// 基准测试：连击状态转换
fn combo_state_benchmark(c: &mut Criterion) {
    c.bench_function("combo_advance", |b| {
        let mut state = ComboState::Idle;
        b.iter(|| {
            let (next_state, _window) = advance_combo(state, 1.0);
            state = next_state;
            if state == ComboState::Idle {
                state = ComboState::FirstHit; // Reset for next iteration
            }
        });
    });

    c.bench_function("combo_count", |b| {
        b.iter(|| {
            let _count = get_combo_count(ComboState::ThirdHit);
        });
    });
}

criterion_group!(benches, damage_calculation_benchmark, combo_state_benchmark);
criterion_main!(benches);

