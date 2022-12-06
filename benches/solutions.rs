#![allow(clippy::pedantic)]
use aoc_2022::{solutions::*, Input};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn day_1(c: &mut Criterion) {
  let inp = Input::load_from_day(1);
  c.bench_function("Day 01, pt 1", |b| {
    b.iter(|| D1::pt_1(black_box(inp.clone())));
  });
  c.bench_function("Day 01, pt 2", |b| {
    b.iter(|| D1::pt_2(black_box(inp.clone())));
  });
}

pub fn day_2(c: &mut Criterion) {
  let inp = Input::load_from_day(2);
  c.bench_function("Day 02, pt 1", |b| {
    b.iter(|| D2::pt_1(black_box(inp.clone())));
  });
  c.bench_function("Day 02, pt 2", |b| {
    b.iter(|| D2::pt_2(black_box(inp.clone())));
  });
}

pub fn day_3(c: &mut Criterion) {
  let inp = Input::load_from_day(3);
  c.bench_function("Day 03, pt 1", |b| {
    b.iter(|| D3::pt_1(black_box(inp.clone())));
  });
  c.bench_function("Day 03, pt 2", |b| {
    b.iter(|| D3::pt_2(black_box(inp.clone())));
  });
}

pub fn day_4(c: &mut Criterion) {
  let inp = Input::load_from_day(4);
  c.bench_function("Day 04, pt 1", |b| {
    b.iter(|| D4::pt_1(black_box(inp.clone())));
  });
  c.bench_function("Day 04, pt 2", |b| {
    b.iter(|| D4::pt_2(black_box(inp.clone())));
  });
}

pub fn day_5(c: &mut Criterion) {
  let inp = Input::load_from_day(5);
  c.bench_function("Day 05, pt 1", |b| {
    b.iter(|| D5::pt_1(black_box(inp.clone())));
  });
  c.bench_function("Day 05, pt 2", |b| {
    b.iter(|| D5::pt_2(black_box(inp.clone())));
  });
}

pub fn day_6(c: &mut Criterion) {
  let inp = Input::load_from_day(6);
  c.bench_function("Day 06, pt 1", |b| {
    b.iter(|| D6::pt_1(black_box(inp.clone())));
  });
  c.bench_function("Day 06, pt 2", |b| {
    b.iter(|| D6::pt_2(black_box(inp.clone())));
  });
}

criterion_group!(day_six, day_6);
criterion_group!(day_five, day_5);
criterion_group!(day_four, day_4);
criterion_group!(day_three, day_3);
criterion_group!(day_two, day_2);
criterion_group!(day_one, day_1);
criterion_main!(day_one, day_two, day_three, day_four, day_five, day_six);
