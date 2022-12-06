use std::io::Read;
use std::collections::HashSet;
use std::collections::VecDeque;
use itertools::Itertools;
use utils;

fn main() {
    part1();
    part2();
}

fn part1() {
  let bytes = utils::read_input_file().bytes();

  let mut index = 0;

  for (i, (a, b, c, d)) in bytes
    .map(|b| b.expect("failed to read byte"))
    .tuple_windows()
    .enumerate()
  {
    if a != b && a != c && a != d && b != c && b != d && c != d {
      index = i;
      break;
    }
  }

  println!("found marker at {}", index + 4);
}

fn part2() {
  let size = 14;
  let bytes = utils::read_input_file().bytes();

  let mut window = VecDeque::new();
  let mut index = 0;

  for (i, byte) in bytes.map(|b| b.expect("failed to read byte")).enumerate() {
    if window.len() == size {
      window.pop_front();
    }
    window.push_back(byte);

    let set: HashSet<u8> = HashSet::from_iter(window.clone());

    if set.len() == size {
      index = i;
      break;
    }
  }

  println!("found marker at {}", index + 1);
}
