#![feature(iter_array_chunks)]
#![allow(clippy::pedantic)]
use iai::black_box;

use aoc_2022::{solutions::*, Input};

fn day1_pt1() {
  let inp = Input::load_from_day(1);
  D1::pt_1(black_box(inp));
}
fn day1_pt2() {
  let inp = Input::load_from_day(1);
  D1::pt_2(black_box(inp));
}

fn day2_pt1() {
  let inp = Input::load_from_day(2);
  D2::pt_1(black_box(inp));
}
fn day2_pt2() {
  let inp = Input::load_from_day(2);
  D2::pt_2(black_box(inp));
}

fn day3_pt1() {
  let inp = Input::load_from_day(3);
  D3::pt_1(black_box(inp));
}
fn day3_pt2() {
  let inp = Input::load_from_day(3);
  D3::pt_2(black_box(inp));
}

fn day4_pt1() {
  let inp = Input::load_from_day(4);
  D4::pt_1(black_box(inp));
}
fn day4_pt2() {
  let inp = Input::load_from_day(4);
  D4::pt_2(black_box(inp));
}

fn day5_pt1() {
  let inp = Input::load_from_day(5);
  D5::pt_1(black_box(inp));
}
fn day5_pt2() {
  let inp = Input::load_from_day(5);
  D5::pt_2(black_box(inp));
}

fn day6_pt1() {
  let inp = Input::load_from_day(6);
  D6::pt_1(black_box(inp));
}
fn day6_pt2() {
  let inp = Input::load_from_day(6);
  D6::pt_2(black_box(inp));
}

iai::main!(
  day1_pt1, day1_pt2, day2_pt1, day2_pt2, day3_pt1, day3_pt2, day4_pt1, day4_pt2, day5_pt1,
  day5_pt2, day6_pt1, day6_pt2
);
