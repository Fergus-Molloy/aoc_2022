use aoc_2022::solutions::{Solution, D12};
use aoc_2022::Input;
fn main() {
  let inp = Input::new(
    r"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi",
  );
  let sol = D12::pt_1(inp);

  println!("{sol}");
}
