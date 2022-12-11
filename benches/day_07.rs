use aoc_2022::{day_07, utils};
use criterion::{black_box, criterion_group, Criterion};

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 7: No Space Left On Device");
    let lines = utils::read_lines("inputs/day_07").unwrap();


    group.bench_function("combined", |b| {
        b.iter(|| black_box({
            let map = day_07::get_directory_layout(&lines);
            day_07::part_1(&map);
            day_07::part_2(&map);
        }))
    });
    group.finish();
}

criterion_group!(benches, benchmark);