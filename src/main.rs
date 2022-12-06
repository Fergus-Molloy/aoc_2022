use aoc_2022::solutions::{Solution, D6};
use aoc_2022::Input;
fn main() {
  let inp = Input::load_from_day(6);
  let sol = D6::pt_2(inp);

  println!("{sol}");
}
