use itertools::Itertools;
use std::io::Read;
use utils;

fn main() {
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
