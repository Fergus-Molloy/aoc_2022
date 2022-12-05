use crate::solutions::Solution;
use crate::Input;

pub struct D1;

impl Solution for D1 {
  type Output = u64;
  fn pt_1(inp: Input) -> u64 {
    let mut max = 0;
    let mut acc = 0;
    for snack in inp.lines() {
      if let Ok(val) = snack.parse::<u64>() {
        acc += val;
      } else {
        if acc > max {
          max = acc;
        }
        acc = 0;
      }
    }
    max
  }

  fn pt_2(inp: crate::input::Input) -> u64 {
    let mut max = vec![0, 0, 0];
    let mut acc = 0;
    for snack in inp.lines() {
      if let Ok(val) = snack.parse::<u64>() {
        acc += val;
      } else {
        let min = max.iter().min().unwrap();
        if acc > *min {
          let idx = max.iter().position(|x| x == min).unwrap();
          max[idx] = acc;
        }
        acc = 0;
      }
    }
    max.iter().sum()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::Input;

  #[test]
  fn day_1_pt_1() {
    let inp = Input::load_from_day(1);
    let sol = D1::pt_1(inp);
    assert_eq!(sol, 74_711);
  }
  #[test]
  fn day_1_pt_2() {
    let inp = Input::load_from_day(1);
    let sol = D1::pt_2(inp);
    assert_eq!(sol, 209_481);
  }
}
