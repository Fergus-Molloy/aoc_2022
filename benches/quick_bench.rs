#![feature(iter_array_chunks)]
#![allow(clippy::pedantic)]
use iai::black_box;

use aoc_2022::{
  solutions::{Solution, D2},
  Input,
};

pub fn test_pt1() {
  let inp = Input::load_from_day(2);
  D2::pt_1(black_box(inp));
}
pub fn test_pt2() {
  let inp = Input::load_from_day(2);
  D2::pt_2(black_box(inp));
}

iai::main!(test_pt1, test_pt2);
