mod directions;
mod traverse;
mod util;

use crate::Solution;
pub use directions::Directions;
pub use traverse::BreadthFirstSearch;
use util::*;

use self::traverse::Traverse;

pub struct D12;

#[derive(Debug)]
pub struct Map {
  map: Vec<Vec<i64>>,
  pub pos: Coord,
  end: Coord,
  bounds: Bounds,
}

impl Map {
  fn new(value: String) -> Self {
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
      bounds: Bounds {
        x: x_bound,
        y: y_bound,
      },
      pos: start,
      end,
    }
  }
}

impl Map {
  fn traverse<T: Traverse>(&self, mut method: T) -> Vec<Move> {
    method.find_end(self)
  }

  fn is_end(&self, pos: &Coord) -> bool {
    self.get_height(pos) == E
  }

  fn get_possible_moves(&self, pos: &Coord) -> Vec<Move> {
    let h = self.get_height(pos);

    let heights = self.get_height_surrounding(pos, &self.bounds);
    println!("current height: {h}");
    println!("surrounding heights: {heights:?}");

    let up = heights
      .up
      .and_then(|v| if diff1(v, h) { Some(Move::Up) } else { None });
    let down = heights
      .down
      .and_then(|v| if diff1(v, h) { Some(Move::Down) } else { None });
    let left = heights
      .left
      .and_then(|v| if diff1(v, h) { Some(Move::Left) } else { None });
    let right = heights
      .right
      .and_then(|v| if diff1(v, h) { Some(Move::Right) } else { None });
    vec![up, down, left, right]
      .into_iter()
      .filter_map(|x| x)
      .collect()
  }

  fn get_height(&self, pos: &Coord) -> i64 {
    self.map[pos.y][pos.x]
  }

  fn get_height_surrounding(&self, pos: &Coord, bounds: &Bounds) -> Directions<i64> {
    Directions {
      up: if pos.y > 0 {
        Some(self.map[pos.y - 1][pos.x])
      } else {
        None
      },
      down: if pos.y < bounds.y {
        Some(self.map[pos.y + 1][pos.x])
      } else {
        None
      },
      left: if pos.x > 0 {
        Some(self.map[pos.y][pos.x - 1])
      } else {
        None
      },
      right: if pos.x < bounds.x {
        Some(self.map[pos.y][pos.x + 1])
      } else {
        None
      },
    }
  }
}

fn diff1(up: i64, h: i64) -> bool {
  (up - h > -1 && (up - h) <= 1) || (up == S || h == S || up == E || h == E)
}

impl Solution for D12 {
  type Output = u64;
  fn pt_1(inp: crate::Input) -> Self::Output {
    let map = Map::new(inp.get());
    println!("start: {:?}\nend:{:?}", map.pos, map.end);
    println!("{:?}", map.traverse(BreadthFirstSearch::default()));

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
