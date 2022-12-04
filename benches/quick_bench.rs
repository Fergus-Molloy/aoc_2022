#![feature(iter_array_chunks)]
#![allow(clippy::pedantic)]
use rustc_hash::FxHashSet;

use aoc_2022::{
  solutions::{Solution, D3},
  Input,
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn get_value(c: char) -> u64 {
  if c.is_ascii_uppercase() {
    (c as u8 - 38) as u64
  } else {
    (c as u8 - 96) as u64
  }
}

fn pt_2_test(inp: Input) -> u64 {
  inp
    .lines()
    .array_chunks()
    .map(|[a, b, c]| {
      let elf_a: FxHashSet<char> = a.chars().into_iter().collect();
      let elf_b: FxHashSet<char> = b.chars().into_iter().collect();

      for item in c.chars() {
        if elf_b.get(&item).is_some() && elf_a.get(&item).is_some() {
          return get_value(item);
        }
      }
      0
    })
    .sum()
}
pub fn temp_bench(c: &mut Criterion) {
  let inp = Input::load_from_day(3);
  c.bench_function("test_new", |b| {
    b.iter(|| pt_2_test(black_box(inp.clone())));
  });
  c.bench_function("test_old", |b| {
    b.iter(|| D3::pt_2(black_box(inp.clone())));
  });
}

criterion_group!(test_bench, temp_bench);
criterion_main!(test_bench);
