use arrayvec::{ArrayString, ArrayVec};
use nom::{
  bytes::complete::{is_not, take},
  character::complete::char,
  sequence::delimited,
  IResult,
};
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub(super) struct CratesStacks {
  pub stacks: ArrayVec<VecDeque<ArrayString<1>>, 10>,
}

fn get_crate(inp: &str) -> IResult<&str, &str> {
  delimited(char('['), is_not("]"), char(']'))(inp)
}

fn get_next_crate(inp: &str) -> IResult<&str, &str> {
  let f = inp.chars().next().unwrap_or('\0');
  let inp = if f == ' ' { &inp[1..] } else { inp };
  take(3usize)(inp)
}

impl CratesStacks {
  pub(super) fn parse_crate_stacks(inp: &str) -> CratesStacks {
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
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn crates() {
    let i = "[A]
  1";
    let crates = CratesStacks::parse_crate_stacks(i);
    println!("{crates:?}");

    assert_eq!(1, crates.stacks[0].len(), "should be 1 item on the stack");

    let i = "    [A]
  1   2";
    let crates = CratesStacks::parse_crate_stacks(i);
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
    let crates = CratesStacks::parse_crate_stacks(i);
    println!("{crates:?}");

    assert_eq!(
      2,
      crates.stacks[2].len(),
      "should be 2 item on the last stack"
    );
  }
}
