use criterion::{black_box, Criterion, criterion_group};

use aoc_2022::{day_01, utils};

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day 01: Calorie Counting");
    let lines = utils::read_lines("inputs/day_01").unwrap();

    group.bench_function("part 1", |b| {
        b.iter(|| black_box(day_01::part_1(&lines)))
    });
    group.bench_function("part 2", |b| {
        b.iter(|| black_box(day_01::part_2(&lines)))
    });
    group.finish();
}

criterion_group!(benches, benchmark);