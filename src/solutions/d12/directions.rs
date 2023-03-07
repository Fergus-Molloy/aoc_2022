#[derive(Debug)]
pub struct Directions<T> {
  pub up: Option<T>,
  pub down: Option<T>,
  pub left: Option<T>,
  pub right: Option<T>,
}
