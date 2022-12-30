use criterion::{black_box, Criterion, criterion_group};

use aoc_2022::{day_09, utils};

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 9: Rope Bridge");
    let lines = utils::read_lines("inputs/day_09").unwrap();
    let moves = day_09::parsing(&lines);


    group.bench_function("part 1", |b| {
        b.iter(|| black_box(day_09::part_1(&moves)))
    });
    group.bench_function("part 2", |b| {
        b.iter(|| black_box(day_09::part_2(&moves)))
    });
    group.finish();
}

criterion_group!(benches, benchmark);