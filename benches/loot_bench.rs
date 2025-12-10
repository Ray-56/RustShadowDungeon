//! Performance benchmarks for loot drop calculations
//!
//! 掉落计算性能基准测试
//!
//! Performance budget: <0.1ms per loot drop calculation

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_shadow_dungeon::domain::loot::drop_table::{calculate_loot_drops, LootTable, LootTableEntry};
use rand::thread_rng;

/// 基准测试：单个掉落表计算
fn bench_calculate_loot_drops_single(c: &mut Criterion) {
    let loot_table = LootTable {
        id: "bench_test".to_string(),
        entries: vec![LootTableEntry {
            item_id: 1,
            chance: 0.5,
            quantity_min: 1,
            quantity_max: 10,
        }],
    };

    c.bench_function("calculate_loot_drops_single", |b| {
        b.iter(|| {
            let mut rng = thread_rng();
            black_box(calculate_loot_drops(black_box(&loot_table), &mut rng))
        })
    });
}

/// 基准测试：多个掉落条目计算
fn bench_calculate_loot_drops_multiple(c: &mut Criterion) {
    let loot_table = LootTable {
        id: "bench_test".to_string(),
        entries: vec![
            LootTableEntry {
                item_id: 1,
                chance: 0.8,
                quantity_min: 5,
                quantity_max: 15,
            },
            LootTableEntry {
                item_id: 2,
                chance: 0.3,
                quantity_min: 1,
                quantity_max: 2,
            },
            LootTableEntry {
                item_id: 3,
                chance: 0.1,
                quantity_min: 1,
                quantity_max: 1,
            },
        ],
    };

    c.bench_function("calculate_loot_drops_multiple", |b| {
        b.iter(|| {
            let mut rng = thread_rng();
            black_box(calculate_loot_drops(black_box(&loot_table), &mut rng))
        })
    });
}

criterion_group!(benches, bench_calculate_loot_drops_single, bench_calculate_loot_drops_multiple);
criterion_main!(benches);
