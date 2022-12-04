use crate::Input;
use rustc_hash::FxHashSet;

use super::Solution;

pub struct D3;
fn get_value(c: char) -> u64 {
  if c.is_ascii_uppercase() {
    (c as u8 - 38) as u64
  } else {
    (c as u8 - 96) as u64
  }
}

impl Solution for D3 {
  fn pt_1(inp: Input) -> u64 {
    inp
      .lines()
      .map(|l| {
        let midpoint = l.len() / 2;
        let compartment_1: FxHashSet<char> = l.chars().take(midpoint).collect();
        for item in l.chars().skip(midpoint) {
          if compartment_1.get(&item).is_some() {
            return get_value(item);
          }
        }
        0
      })
      .sum()
  }
  fn pt_2(inp: Input) -> u64 {
    inp
      .lines()
      .array_chunks()
      .map(|[a, b, c]| {
        let elf_a: FxHashSet<char> = a.chars().into_iter().collect();
        let elf_b: FxHashSet<char> = b.chars().into_iter().collect();

        for item in c.chars() {
          if elf_b.get(&item).is_some() && elf_a.get(&item).is_some() {
            return get_value(item);
          }
        }
        0
      })
      .sum()
  }
}

#[cfg(test)]
mod tests {
  use crate::{solutions::Solution, Input};

  use super::D3;

  #[test]
  fn day_3_pt_1() {
    let inp = Input::load_from_day(3);
    assert_eq!(8515, D3::pt_1(inp));
  }
  #[test]
  fn day_3_pt_2() {
    let inp = Input::load_from_day(3);
    assert_eq!(2434, D3::pt_2(inp));
  }
}
