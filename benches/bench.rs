use advent_of_code_2019::{
    day1, day2, day3, day4, AdventOfCodeBuilder, AdventOfCodeRunner, INPUT_DAY1, INPUT_DAY2,
    INPUT_DAY3, INPUT_DAY4,
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

criterion_group!(benches, d1, d2, d3, d4);
criterion_main!(benches);

fn d1(c: &mut Criterion) {
    c.bench_function("day1", |b| {
        b.iter(|| black_box(day1::Input::build(INPUT_DAY1).unwrap().run().unwrap()))
    });
}

fn d2(c: &mut Criterion) {
    c.bench_function("day2", |b| {
        b.iter(|| black_box(day2::Input::build(INPUT_DAY2).unwrap().run().unwrap()))
    });
}

fn d3(c: &mut Criterion) {
    c.bench_function("day3", |b| {
        b.iter(|| black_box(day3::Input::build(INPUT_DAY3).unwrap().run().unwrap()))
    });
}

fn d4(c: &mut Criterion) {
    c.bench_function("day4", |b| {
        b.iter(|| black_box(day4::Input::build(INPUT_DAY4).unwrap().run().unwrap()))
    });
}
