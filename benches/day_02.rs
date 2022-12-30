use criterion::{black_box, Criterion, criterion_group};

use aoc_2022::{day_02, utils};

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day 02: Rock Paper Scissors");
    let lines = utils::read_lines("inputs/day_02").unwrap();

    group.bench_function("part 1", |b| {
        b.iter(|| black_box(day_02::part_1(&lines)))
    });
    group.bench_function("part 2", |b| {
        b.iter(|| black_box(day_02::part_2(&lines)))
    });
    group.finish();
}

criterion_group!(benches, benchmark);