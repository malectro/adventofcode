use std::cmp::min;
use utils;

fn main() {
  let file = utils::read_input_file_lines();

  let mut total_area = 0;
  let mut total_ribbon = 0;

  for line in file {
    let size: Vec<usize> = line
      .split('x')
      .map(|n| n.parse().expect("Could not parse box"))
      .collect();

    let (w, h, l) = (size[0], size[1], size[2]);

    let sides = [w * h, h * l, l * w];
    let smallest = sides.into_iter().fold(usize::MAX, |acc, val| min(acc, val));
    let surface_area = sides.iter().fold(0, |acc, val| val * 2 + acc);

    total_area += surface_area + smallest;

    let ribbon_length = [w + h, h + l, l + w]
      .into_iter()
      .fold(usize::MAX, |acc, val| min(acc, val));
    total_ribbon += w * h * l + 2 * ribbon_length;
  }

  println!("total wrapping paper required: {}", total_area);
  println!("total ribbon required: {}", total_ribbon);
}
