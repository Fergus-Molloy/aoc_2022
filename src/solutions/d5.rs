use lazy_format::lazy_format;
use std::collections::VecDeque;

use super::Solution;
use crate::Input;
use arrayvec::{ArrayString, ArrayVec};
use joinery::JoinableIterator;
use nom::{
  bytes::complete::{is_not, take},
  character::complete::{alpha0, char, digit1},
  combinator::map_res,
  sequence::delimited,
  IResult,
};

pub struct D5;

#[derive(Debug, Clone)]
struct CratesStacks {
  stacks: ArrayVec<VecDeque<ArrayString<1>>, 10>,
}

fn get_crate(inp: &str) -> IResult<&str, &str> {
  delimited(char('['), is_not("]"), char(']'))(inp)
}

fn get_next_crate(inp: &str) -> IResult<&str, &str> {
  let f = inp.chars().next().unwrap_or('\0');
  let inp = if f == ' ' { &inp[1..] } else { inp };
  take(3usize)(inp)
}

fn parse_crate_stacks(inp: &str) -> CratesStacks {
  let inp = inp.lines().rev();
  let mut stacks: ArrayVec<VecDeque<ArrayString<1>>, 10> = ArrayVec::default();
  for _ in 0..10 {
    stacks.push(VecDeque::new());
  }
  for mut line in inp.skip(1) {
    let mut idx = 0;
    loop {
      let Ok((rest, crate_str)) = get_next_crate(line) else {
        break;
      };
      line = rest;
      if let Ok((_, c)) = get_crate(crate_str) {
        stacks[idx].push_back(ArrayString::from(c).unwrap());
      }
      idx += 1;
    }
  }
  CratesStacks { stacks }
}

#[derive(Debug, PartialEq, Eq)]
struct Instruction {
  count: usize,
  from: usize,
  to: usize,
}

fn get_next_digit(inp: &str) -> IResult<&str, usize> {
  let (next, _) = alpha0(inp.trim_start())?;
  map_res(digit1, str::parse)(next.trim_start())
}

fn parse_instructions(inp: &str) -> Vec<Instruction> {
  inp
    .lines()
    .map(|l| {
      let (rest, count) = get_next_digit(l).unwrap();
      let (rest, from) = get_next_digit(rest).unwrap();
      let (_, to) = get_next_digit(rest).unwrap();
      Instruction {
        count,
        from: (from - 1),
        to: (to - 1),
      }
    })
    .collect()
}

impl Solution for D5 {
  type Output = String;
  fn pt_1(inp: Input) -> String {
    let inp = inp.get();
    let mut parts = inp.split("\n\n");
    let mut crates = parse_crate_stacks(parts.next().unwrap());
    let instructions = parse_instructions(parts.next().unwrap());
    for instruction in instructions {
      for _ in 0..instruction.count {
        let moved = crates.stacks[instruction.from].pop_back().unwrap();
        crates.stacks[instruction.to].push_back(moved);
      }
    }

    crates
      .stacks
      .iter()
      .map(|s| lazy_format!("{}", s.iter().last().unwrap_or(&ArrayString::new())))
      .join_with("")
      .to_string()
  }

  fn pt_2(inp: Input) -> String {
    let inp = inp.get();
    let parts = inp.split("\n\n").collect::<Vec<&str>>();
    let mut crates = parse_crate_stacks(parts[0]);
    let instructions = parse_instructions(parts[1]);
    for instruction in instructions {
      let mut moved = VecDeque::new();
      for _ in 0..instruction.count {
        let move_crate = crates.stacks[instruction.from].pop_back().unwrap();
        moved.push_front(move_crate);
      }
      crates.stacks[instruction.to].append(&mut moved);
    }

    crates
      .stacks
      .iter()
      .map(|s| lazy_format!("{}", s.iter().last().unwrap_or(&ArrayString::new())))
      .join_with("")
      .to_string()
  }
}

#[cfg(test)]
mod tests {
  use super::{parse_crate_stacks, parse_instructions, Instruction, D5};
  use crate::{solutions::Solution, Input};

  #[test]
  fn example_day_5_pt_1() {
    let inp = Input::new(
      "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"
        .to_string(),
    );
    let sol = D5::pt_1(inp);
    assert_eq!("CMZ".to_string(), sol);
  }

  #[test]
  fn example_day_5_pt_2() {
    let inp = Input::new(
      "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"
        .to_string(),
    );
    let sol = D5::pt_2(inp);
    assert_eq!("MCD".to_string(), sol);
  }

  #[test]
  fn crates() {
    let i = "[A]
  1";
    let crates = parse_crate_stacks(i);
    println!("{crates:?}");

    assert_eq!(1, crates.stacks[0].len(), "should be 1 item on the stack");

    let i = "    [A]
  1   2";
    let crates = parse_crate_stacks(i);
    println!("{crates:?}");

    assert_eq!(
      0,
      crates.stacks[0].len(),
      "should be 0 item on the 1st stack"
    );
    assert_eq!(
      1,
      crates.stacks[1].len(),
      "should be 1 item on the 2nd stack"
    );

    let i = "[A]     [B]
[C] [D] [E]
 1   2   3";
    let crates = parse_crate_stacks(i);
    println!("{crates:?}");

    assert_eq!(
      2,
      crates.stacks[2].len(),
      "should be 2 item on the last stack"
    );
  }

  #[test]
  fn example_instructions() {
    let i = "move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    let instructions = parse_instructions(i);
    assert_eq!(
      Instruction {
        count: 1,
        from: 1,
        to: 0,
      },
      instructions[0]
    );
  }
}
