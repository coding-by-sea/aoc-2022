use criterion::criterion_main;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_09;
// ${BENCH_IMPORT_MARKER}

criterion_main! {
    day_01::benches,
    day_02::benches,
    day_03::benches,
    day_04::benches,
    day_05::benches,
    day_06::benches,
    day_09::benches,
}