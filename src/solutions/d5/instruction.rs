use nom::{
  character::complete::{alpha0, digit1},
  combinator::map_res,
  IResult,
};

#[derive(Debug, PartialEq, Eq)]
pub(super) struct Instruction {
  pub count: usize,
  pub from: usize,
  pub to: usize,
}

fn get_next_digit(inp: &str) -> IResult<&str, usize> {
  let (next, _) = alpha0(inp.trim_start())?;
  map_res(digit1, str::parse)(next.trim_start())
}

impl Instruction {
  pub(super) fn parse_instructions(inp: &str) -> Vec<Instruction> {
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
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example_instructions() {
    let i = "move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    let instructions = Instruction::parse_instructions(i);
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
