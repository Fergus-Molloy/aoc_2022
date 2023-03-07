pub mod directions;

use crate::Solution;
pub use directions::Directions;

pub struct D12;
#[derive(Debug)]
struct Coord {
  x: usize,
  y: usize,
}

impl Default for Coord {
  fn default() -> Self {
    Coord { x: 0, y: 0 }
  }
}
#[derive(Debug)]
struct BoundedCoord {
  x: usize,
  y: usize,
  x_bound: usize,
  y_bound: usize,
}

#[derive(Debug)]
pub struct Map {
  map: Vec<Vec<i64>>,
  pos: BoundedCoord,
  end: Coord,
}

const S: i64 = 83;
const E: i64 = 69;

impl From<String> for Map {
  fn from(value: String) -> Self {
    let map: Vec<Vec<i64>> = value
      .lines()
      .map(|line| line.chars().map(|c| c as i64).collect())
      .collect();
    let y_bound = map.len() - 1;
    let x_bound = map[0].len() - 1;
    let mut start = Coord::default();
    let mut end = Coord::default();
    for (y, line) in map.iter().enumerate() {
      if let Some(x) = line.iter().position(|v| v == &S) {
        start = Coord { x, y };
      };
      if let Some(x) = line.iter().position(|v| v == &E) {
        end = Coord { x, y };
      };
    }
    Self {
      map,
      pos: BoundedCoord {
        x: start.x,
        y: start.y,
        x_bound,
        y_bound,
      },
      end,
    }
  }
}

impl Map {
  fn get_possible_moves(&self) -> Directions<()> {
    let h = self.get_height(&self.pos);

    let heights = self.get_height_surrounding(&self.pos);
    println!("current height: {h}");
    println!("surrounding heights: {heights:?}");

    Directions {
      up: heights.up.and_then(|v| diff1(v, h)),
      down: heights.down.and_then(|v| diff1(v, h)),
      left: heights.left.and_then(|v| diff1(v, h)),
      right: heights.right.and_then(|v| diff1(v, h)),
    }
  }

  fn get_height(&self, pos: &BoundedCoord) -> i64 {
    self.map[pos.y][pos.x]
  }

  fn get_height_surrounding(&self, pos: &BoundedCoord) -> Directions<i64> {
    Directions {
      up: if pos.y > 0 {
        Some(self.map[pos.y - 1][pos.x])
      } else {
        None
      },
      down: if pos.y < pos.y_bound {
        Some(self.map[pos.y + 1][pos.x])
      } else {
        None
      },
      left: if pos.x > 0 {
        Some(self.map[pos.y][pos.x - 1])
      } else {
        None
      },
      right: if pos.x < pos.x_bound {
        Some(self.map[pos.y][pos.x + 1])
      } else {
        None
      },
    }
  }
}

fn diff1(up: i64, h: i64) -> Option<()> {
  if (up - h > -1 && (up - h) <= 1) || (up == S || h == S || up == E || h == E) {
    Some(())
  } else {
    None
  }
}

impl Solution for D12 {
  type Output = u64;
  fn pt_1(inp: crate::Input) -> Self::Output {
    let map = Map::from(inp.get());
    println!("start: {:?}\nend:{:?}", map.pos, map.end);
    println!("{:?}", map.get_possible_moves());

    0
  }
  fn pt_2(inp: crate::Input) -> Self::Output {
    todo!()
  }
}

#[cfg(test)]
mod tests {
  use crate::Input;

  use super::*;

  #[test]
  fn test_pt1() {
    let input = Input::new(
      r"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi",
    );

    let sol = D12::pt_1(input);
    assert_eq!(31, sol);
  }
}
