use super::Solution;
use crate::Input;

pub struct D3;
fn get_value(c: char) -> u64 {
  #![allow(clippy::cast_lossless)]
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
        let compartment_1 = l.chars().take(midpoint);
        for item in l.chars().skip(midpoint) {
          if compartment_1.clone().any(|a| a == item) {
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
        let elf_a = a.chars();
        let elf_b = b.chars();

        for item in c.chars() {
          if elf_b.clone().any(|b| b == item) && elf_a.clone().any(|a| a == item) {
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
