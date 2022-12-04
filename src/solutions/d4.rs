use std::cmp::Ordering;

use crate::Input;

use super::Solution;

pub struct D4;

struct Elf {
  range_min: u32,
  range_max: u32,
}

fn get_range(input: &str) -> Result<(u32, u32), String> {
  let nums = input.split('-').collect::<Vec<&str>>();
  let lower = nums[0]
    .parse::<u32>()
    .map_err(|_| "could not parse lower bound".to_string())?;
  let upper = nums[1]
    .parse::<u32>()
    .map_err(|_| "could not parse upper bound".to_string())?;
  Ok((lower, upper))
}

impl Solution for D4 {
  fn pt_1(inp: Input) -> u64 {
    inp
      .lines()
      .filter(|l| {
        let elfs = l
          .split(",")
          .map(|r| {
            let range = get_range(r).expect("Could not get range");
            Elf {
              range_max: range.1,
              range_min: range.0,
            }
          })
          .collect::<Vec<Elf>>();

        let min_cmp = elfs[0].range_min.cmp(&elfs[1].range_min);
        let max_cmp = elfs[0].range_max.cmp(&elfs[1].range_max);

        match min_cmp {
          Ordering::Less if max_cmp == Ordering::Equal || max_cmp == Ordering::Greater => true,
          Ordering::Equal => true,
          Ordering::Greater if max_cmp == Ordering::Equal || max_cmp == Ordering::Less => true,
          _ => false,
        }
      })
      .count() as u64
  }
  fn pt_2(inp: Input) -> u64 {
    inp
      .lines()
      .filter(|l| {
        let elfs = l
          .split(",")
          .map(|r| {
            let range = get_range(r).expect("Could not get range");
            Elf {
              range_max: range.1,
              range_min: range.0,
            }
          })
          .collect::<Vec<Elf>>();

        (elfs[0].range_min < elfs[1].range_min && elfs[0].range_max >= elfs[1].range_min)
          || (elfs[0].range_min > elfs[1].range_min && elfs[0].range_min <= elfs[1].range_max)
          || (elfs[0].range_min == elfs[1].range_min)
          || (elfs[0].range_max == elfs[1].range_max)
      })
      .count() as u64
  }
}

#[cfg(test)]
mod tests {
  use super::D4;
  use crate::{solutions::Solution, Input};

  #[test]
  fn day_4_pt_1() {
    let inp = Input::load_from_day(4);
    let sol = D4::pt_1(inp);
    assert_eq!(305, sol);
  }
  #[test]
  fn day_4_pt_2() {
    let inp = Input::load_from_day(4);
    let sol = D4::pt_2(inp);
    assert_eq!(811, sol);
  }
}
