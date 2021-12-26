use std::io::prelude::*;
use utils;

fn main() {
  let mut file = utils::read_input_file();

  let mut string = String::new();
  file
    .read_to_string(&mut string)
    .expect("Failed to read input file as string");

  let mut floor = 0;
  let mut first_basement_position = 0;
  for (i, byte) in string.chars().enumerate() {
    if byte == '(' {
      floor += 1;
    } else {
      floor -= 1;
    }
    if floor == -1 && first_basement_position == 0 {
      first_basement_position = i + 1;
    }
  }

  println!(
    "on floor {}, first basement position {}",
    floor, first_basement_position
  );
}
