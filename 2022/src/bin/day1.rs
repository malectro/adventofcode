use std::cmp::max;
use utils;

fn main() {
  part1();
  part2();
}

fn part1() {
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

fn part2() {
  let lines = utils::read_input_file_lines();

  let mut top_3: Vec<usize> = vec![0; 3];
  let mut calories: usize = 0;

  for line in lines {
    if line == "" {
      top_3.push(calories);
      top_3.sort_unstable_by(|a, b| b.cmp(a));
      top_3.pop();
      calories = 0;
    } else {
      calories += line.parse::<usize>().unwrap();
    }
  }

  println!("the top 3 calories total is {}", top_3.iter().sum::<usize>());
}
