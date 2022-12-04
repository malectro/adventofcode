use std::cmp::max;
use utils;

fn main() {
  let lines = utils::read_input_file_lines();

  let mut most_calories = 0;
  let mut calories: usize = 0;
  for line in lines {
    if line == "" {
      most_calories = max(most_calories, calories);
      calories = 0;
    } else {
      calories += line.parse::<usize>().unwrap();
    }
  }

  println!("the most calories is {}", most_calories);
}
