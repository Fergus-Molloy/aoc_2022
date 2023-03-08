use std::collections::VecDeque;

use super::{util::Move, Coord, Map};
pub trait Traverse {
  fn find_end(&mut self, map: &Map) -> Vec<Move>;
}

#[derive(Default, Debug)]
pub struct BreadthFirstSearch {
  queue: VecDeque<(Move, Coord)>,
  taken: Vec<Move>,
}

impl Traverse for BreadthFirstSearch {
  fn find_end(&mut self, map: &Map) -> Vec<Move> {
    let moves = map.get_possible_moves(&map.pos);
    self
      .queue
      .extend(moves.into_iter().map(|m| (m, m.coord_after_move(&map.pos))));

    while let Some((m, current)) = self.queue.pop_front() {
      self.taken.push(m);
      if map.is_end(&current) {
        break;
      }
      let moves = map.get_possible_moves(&current);
      self
        .queue
        .extend(moves.into_iter().map(|m| (m, m.coord_after_move(&map.pos))));
    }
    self.taken.clone()
  }
}
