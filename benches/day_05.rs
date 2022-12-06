use aoc_2022::{day_05, utils};
use criterion::{black_box, criterion_group, Criterion};

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 5: Supply Stacks");
    let lines = utils::read_lines("inputs/day_05").unwrap();
    let (stacks, mut commands) = day_05::parsing(&lines);


    group.bench_function("part 1", |b| {
        b.iter(|| black_box(day_05::part_1(&mut stacks.clone(), &commands)))
    });
    group.bench_function("part 2", |b| {
        b.iter(|| black_box(day_05::part_2(&mut stacks.clone(), &commands)))
    });
    group.finish();
}

criterion_group!(benches, benchmark);