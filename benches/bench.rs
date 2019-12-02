use advent_of_code_2019::{day1::Day1, day2::Day2, AdventOfCodeBuilder, AdventOfCodeRunner};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

criterion_group!(benches, day1, day2);
criterion_main!(benches);

fn day1(c: &mut Criterion) {
    c.bench_function("day1", |b| {
        b.iter(|| black_box(Day1::build().unwrap().run().unwrap()))
    });
}

fn day2(c: &mut Criterion) {
    c.bench_function("day2", |b| {
        b.iter(|| black_box(Day2::build().unwrap().run().unwrap()))
    });
}
