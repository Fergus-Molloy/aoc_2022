use crate::Input;
use std::cmp::Ordering;

use super::Solution;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Choice {
  Rock = 1,
  Paper = 2,
  Scissors = 3,
}
impl Ord for Choice {
  fn cmp(&self, other: &Self) -> Ordering {
    self.partial_cmp(other).unwrap()
  }
}
impl PartialOrd for Choice {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    match self {
      Choice::Rock if other == &Choice::Paper => Some(Ordering::Less),
      Choice::Rock if other == &Choice::Scissors => Some(Ordering::Greater),
      Choice::Paper if other == &Choice::Scissors => Some(Ordering::Less),
      Choice::Paper if other == &Choice::Rock => Some(Ordering::Greater),
      Choice::Scissors if other == &Choice::Rock => Some(Ordering::Less),
      Choice::Scissors if other == &Choice::Paper => Some(Ordering::Greater),
      Choice::Scissors | Choice::Paper | Choice::Rock => Some(Ordering::Equal),
    }
  }
}

impl From<&str> for Choice {
  fn from(value: &str) -> Self {
    match value {
      "B" | "Y" => Choice::Paper,
      "C" | "Z" => Choice::Scissors,
      _ => Choice::Rock,
    }
  }
}

fn get_result(a: Choice, b: Choice) -> u64 {
  let result = match b.cmp(&a) {
    Ordering::Greater => 6,
    Ordering::Equal => 3,
    Ordering::Less => 0,
  };
  b as u64 + result
}
impl From<&str> for Outcome {
  fn from(value: &str) -> Self {
    match value {
      "Y" => Outcome::Draw,
      "Z" => Outcome::Win,
      _ => Outcome::Loss,
    }
  }
}

#[derive(Clone, Copy, Debug)]
enum Outcome {
  Win = 6,
  Draw = 3,
  Loss = 0,
}

fn get_result_given_outcome(a: Choice, outcome: Outcome) -> u64 {
  match outcome {
    Outcome::Draw => a as u64 + 3,
    Outcome::Loss if a == Choice::Rock => Choice::Scissors as u64,
    Outcome::Loss if a == Choice::Paper => Choice::Rock as u64,
    Outcome::Loss if a == Choice::Scissors => Choice::Paper as u64,
    Outcome::Win if a == Choice::Rock => Choice::Paper as u64 + 6,
    Outcome::Win if a == Choice::Paper => Choice::Scissors as u64 + 6,
    Outcome::Win if a == Choice::Scissors => Choice::Rock as u64 + 6,
    _ => 0,
  }
}

pub struct D2;

impl Solution for D2 {
  fn pt_1(inp: Input) -> u64 {
    inp
      .lines()
      .map(|x| {
        let round = x
          .split(' ')
          .map(std::convert::From::from)
          .collect::<Vec<Choice>>();
        get_result(round[0], round[1])
      })
      .sum::<u64>()
  }
  fn pt_2(inp: Input) -> u64 {
    inp
      .lines()
      .map(|x| {
        let round = x.split(' ').collect::<Vec<&str>>();
        get_result_given_outcome(round[0].into(), round[1].into())
      })
      .sum::<u64>()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::Input;

  #[test]
  fn day_2_pt_1() {
    let inp = Input::load_from_day(2);
    let sol = D2::pt_1(inp);
    assert_eq!(sol, 8392);
  }
  #[test]
  fn day_2_pt_2() {
    let inp = Input::load_from_day(2);
    let sol = D2::pt_2(inp);
    assert_eq!(sol, 10116);
  }
}
