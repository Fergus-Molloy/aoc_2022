use std::str::Lines;

#[derive(Clone, Debug)]
pub struct Input(String);

impl Input {
  #[allow(clippy::expect_fun_call)]
  #[must_use]
  pub fn load_from_day(day: u8) -> Self {
    Self(
      std::fs::read_to_string(format!("./inputs/{day}"))
        .expect(&format!("Could not read ./inputs/{day}")),
    )
  }

  pub fn new<T: AsRef<str>>(s: T) -> Self {
    Self(s.as_ref().to_string())
  }
}

impl<'a> Input {
  #[inline]
  pub fn lines(&'a self) -> Lines<'a> {
    self.0.lines()
  }

  #[inline]
  pub fn get(self) -> String {
    self.0
  }
}
