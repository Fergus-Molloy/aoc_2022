use crate::{Input, Solution};

pub struct D6;

impl Solution for D6 {
  type Output = usize;
  fn pt_1(inp: Input) -> Self::Output {
    (inp.get().chars().collect::<Vec<char>>()[..]
      .windows(4)
      .map(|window| match window {
        [a, b, c, d] => a != b && a != c && a != d && b != c && b != d && c != d,
        _ => false,
      }))
    .position(|a| a)
    .unwrap()
      + 4
  }
  fn pt_2(inp: Input) -> Self::Output {
    (inp.get().chars().collect::<Vec<char>>()[..]
      .windows(14)
      .map(|window| {
        for c in window {
          if window.iter().filter(|a| a == &c).count() > 1 {
            return false;
          }
        }
        true
      }))
    .position(|a| a)
    .unwrap()
      + 14
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example_d6_pt1() {
    let inp = Input::new("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string());
    let sol = D6::pt_1(inp);
    assert_eq!(5, sol);
  }
  #[test]
  fn example_d6_pt2() {
    let inp = Input::new("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string());
    let sol = D6::pt_2(inp);
    assert_eq!(23, sol);
  }
  #[test]
  fn day_6_pt1() {
    let inp = Input::load_from_day(6);
    let sol = D6::pt_1(inp);
    assert_eq!(1356, sol);
  }
  #[test]
  fn day_6_pt2() {
    let inp = Input::load_from_day(6);
    let sol = D6::pt_2(inp);
    assert_eq!(2564, sol);
  }
}
