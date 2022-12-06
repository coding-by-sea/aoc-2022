use aoc_2022::{day_06, utils};
use criterion::{black_box, criterion_group, Criterion};

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 6: Tuning Trouble");
    let lines = utils::read_lines("inputs/day_06").unwrap();

    group.bench_function("part 1", |b| {
        b.iter(|| black_box(day_06::part_1(&lines)))
    });
    group.bench_function("part 2", |b| {
        b.iter(|| black_box(day_06::part_2(&lines)))
    });
    group.finish();
}

criterion_group!(benches, benchmark);