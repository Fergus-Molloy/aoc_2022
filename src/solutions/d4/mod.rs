use super::Solution;
use crate::Input;
use arrayvec::ArrayVec;
use std::cmp::Ordering;

pub struct D4;

struct Elf {
  range_min: u32,
  range_max: u32,
}

fn get_elf_from_range(input: &str) -> Elf {
  let nums = input
    .split('-')
    .map(|x| x.parse().expect(""))
    .collect::<ArrayVec<u32, 2>>();
  Elf {
    range_min: nums[0],
    range_max: nums[1],
  }
}

impl Solution for D4 {
  type Output = u64;
  fn pt_1(inp: Input) -> u64 {
    inp
      .lines()
      .filter(|l| {
        let elfs = l
          .split(',')
          .map(get_elf_from_range)
          .collect::<ArrayVec<Elf, 2>>();

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
          .split(',')
          .map(get_elf_from_range)
          .collect::<ArrayVec<Elf, 2>>();

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
