use crate::input::Input;

pub trait Solution {
  type Output;
  fn pt_1(inp: Input) -> Self::Output;
  fn pt_2(inp: Input) -> Self::Output;

  #[inline]
  fn time_pt_1(inp: Input) -> std::time::Duration {
    let start = std::time::Instant::now();
    Self::pt_1(inp);
    start.elapsed()
  }

  #[inline]
  fn time_pt_2(inp: Input) -> std::time::Duration {
    let start = std::time::Instant::now();
    Self::pt_2(inp);
    start.elapsed()
  }

  #[inline]
  fn time_day(inp: Input) -> std::time::Duration {
    let start = std::time::Instant::now();
    Self::pt_1(inp.clone());
    Self::pt_2(inp);
    start.elapsed()
  }
}
