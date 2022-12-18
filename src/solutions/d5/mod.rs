mod crate_stack;
mod instruction;
use lazy_format::lazy_format;
use std::collections::VecDeque;

use super::Solution;
use crate::Input;
use arrayvec::ArrayString;
use crate_stack::CratesStacks;
use instruction::Instruction;
use joinery::JoinableIterator;

pub struct D5;

impl Solution for D5 {
  type Output = String;
  fn pt_1(inp: Input) -> String {
    let inp = inp.get();
    let mut parts = inp.split("\n\n");
    let mut crates = CratesStacks::parse_crate_stacks(parts.next().unwrap());
    let instructions = Instruction::parse_instructions(parts.next().unwrap());
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
    let mut crates = CratesStacks::parse_crate_stacks(parts[0]);
    let instructions = Instruction::parse_instructions(parts[1]);
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
  use super::D5;
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
move 1 from 1 to 2",
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
move 1 from 1 to 2",
    );
    let sol = D5::pt_2(inp);
    assert_eq!("MCD".to_string(), sol);
  }
}
