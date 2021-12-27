use std::cmp::min;
use std::io::prelude::*;
use utils;

fn main() {
  let mut file = utils::read_input_file_lines();

  let mut total = 0;
  for line in file {
    let size: Vec<usize> = line
      .split('x')
      .map(|n| n.parse().expect("Could not parse box"))
      .collect();

    let (w, h, l) = (size[0], size[1], size[2]);
    let sides = [w * h, h * l, l * w];

    let smallest = sides.into_iter().fold(usize::MAX, |acc, val| min(acc, val));
    let surface_area = sides.iter().fold(0, |acc, val| val * 2 + acc);

    total += surface_area + smallest;
  }

  println!("total wrapping paper required: {}", total);
}
