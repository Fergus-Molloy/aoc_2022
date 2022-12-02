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
      _ => None,
    }
  }
}

impl TryFrom<&str> for Choice {
  type Error = String;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    match value {
      "A" | "X" => Ok(Choice::Rock),
      "B" | "Y" => Ok(Choice::Paper),
      "C" | "Z" => Ok(Choice::Scissors),
      _ => Err(format!("Un-recognized choice {value}")),
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
impl TryFrom<&str> for Outcome {
  type Error = String;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    match value {
      "X" => Ok(Outcome::Loss),
      "Y" => Ok(Outcome::Draw),
      "Z" => Ok(Outcome::Win),
      _ => Err(format!("Un-recognized choice {value}")),
    }
  }
}

#[derive(Clone, Copy, Debug)]
enum Outcome {
  Win = 6,
  Draw = 3,
  Loss = 0,
}

fn get_result_given_outcome(a: Choice, outcome: Outcome) -> Result<u64, String> {
  match outcome {
    Outcome::Draw => Ok(a as u64 + 3),
    Outcome::Loss if a == Choice::Rock => Ok(Choice::Scissors as u64),
    Outcome::Loss if a == Choice::Paper => Ok(Choice::Rock as u64),
    Outcome::Loss if a == Choice::Scissors => Ok(Choice::Paper as u64),
    Outcome::Win if a == Choice::Rock => Ok(Choice::Paper as u64 + 6),
    Outcome::Win if a == Choice::Paper => Ok(Choice::Scissors as u64 + 6),
    Outcome::Win if a == Choice::Scissors => Ok(Choice::Rock as u64 + 6),
    _ => Err(format!(
      "Could not get result for opponent choice {:?} and outcome {:?}",
      a, outcome
    )),
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
          .map(std::convert::TryInto::try_into)
          .into_iter()
          .collect::<Result<Vec<Choice>, String>>()
          .unwrap();
        get_result(round[0], round[1])
      })
      .sum::<u64>()
  }
  fn pt_2(inp: Input) -> u64 {
    inp
      .lines()
      .map(|x| {
        let round = x.split(' ').collect::<Vec<&str>>();
        get_result_given_outcome(round[0].try_into().unwrap(), round[1].try_into().unwrap())
          .unwrap()
      })
      .sum::<u64>()
  }
}
