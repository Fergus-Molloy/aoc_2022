use std::fs::{self, DirEntry};
use std::io;
use std::path::{Path, PathBuf};

use crate::{Input, Solution};
pub struct D7;

fn recreate_file_system(inp: &Input) {
  let root = PathBuf::from("/tmp/aoc");
  if root.exists() {
    println!("Dir {root:?} already exists deleting it");
    fs::remove_dir_all(&root).unwrap();
  }
  fs::create_dir(&root).unwrap();
  let mut path = root.clone();
  for cmd in inp.lines() {
    let parts = cmd.split(' ').collect::<Vec<&str>>();
    let command = parts[0].chars().next().unwrap() == '$';
    if command {
      match parts[1].trim() {
        "cd" => match parts[2].trim() {
          ".." => {
            path.pop();
          }
          "/" => {
            path = root.clone();
          }
          _ => path.push(parts[2].trim()),
        },
        "ls" => continue,
        _ => unreachable!(),
      }
    } else {
      // contents of an ls
      match parts[0] {
        "dir" => fs::create_dir(path.join(parts[1])).unwrap(),
        _ => fs::write(path.join(parts[1]), parts[0]).unwrap(),
      }
    }
  }
}

fn visit_dirs(dir: &Path) -> io::Result<u64> {
  if dir.is_dir() {
    let mut sum = 0;
    for entry in fs::read_dir(dir)? {
      let entry = entry?;
      let path = entry.path();
      if path.is_dir() {
        visit_dirs(&path)?;
      } else {
        let s = fs::read_to_string(path)?;
        sum += s.parse::<u64>().unwrap();
      }
    }
    if sum > 100_000 {
      Ok(0)
    } else {
      Ok(sum)
    }
  } else {
    let s = fs::read_to_string(dir)?;
    Ok(s.parse().unwrap())
  }
}

impl Solution for D7 {
  type Output = u64;
  fn pt_1(inp: Input) -> Self::Output {
    recreate_file_system(&inp);
    visit_dirs(&PathBuf::from("/tmp/aoc")).unwrap()
  }
  fn pt_2(inp: Input) -> Self::Output {
    todo!()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn example_file_system() {
    recreate_file_system(&Input::new(
      "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k",
    ));

    let mut path = PathBuf::from("/tmp/aoc");
    assert!(path.exists() && path.is_dir());
    path.push("a");
    assert!(path.exists() && path.is_dir());
  }
  #[test]
  fn example_d7_pt_1() {
    let inp = Input::new(
      "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k",
    );

    let sol = D7::pt_1(inp);
    assert_eq!(0, sol);
  }
}
