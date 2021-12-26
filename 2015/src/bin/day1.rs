use std::io::prelude::*;
use utils;

fn main() {
  let mut file = utils::read_input_file();

  let mut string = String::new();
  file
    .read_to_string(&mut string)
    .expect("Failed to read input file as string");

  let mut floor = 0;
  for byte in string.chars() {
    if byte == '(' {
      floor += 1;
    } else {
      floor -= 1;
    }
  }

  println!("on floor {}", floor);
}
