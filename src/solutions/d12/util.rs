pub const S: i64 = 83;
pub const E: i64 = 69;

#[derive(Debug, Clone, Copy)]
pub struct Coord {
  pub x: usize,
  pub y: usize,
}

impl Default for Coord {
  fn default() -> Self {
    Coord { x: 0, y: 0 }
  }
}
#[derive(Debug)]
pub struct Bounds {
  pub x: usize,
  pub y: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum Move {
  Up,
  Down,
  Left,
  Right,
}

impl Move {
  pub fn coord_after_move(&self, start: &Coord) -> Coord {
    match self {
      Self::Up => Coord {
        y: start.y - 1,
        x: start.x,
      },
      Self::Down => Coord {
        y: start.y + 1,
        x: start.x,
      },
      Self::Left => Coord {
        y: start.y,
        x: start.x - 1,
      },
      Self::Right => Coord {
        y: start.y,
        x: start.x + 1,
      },
    }
  }
}
