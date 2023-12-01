use utils;

fn main() {
  part1();
}

fn part1() {
  let lines = utils::read_input_file_lines();

  let mut total = 0;

  for line in lines {
    let numbers: Vec<u32> = line.chars().filter_map(
      |c| c.to_digit(10)
    ).collect();

    total += numbers.first().unwrap_or(&0) * 10 + numbers.last().unwrap_or(&0);
  }

  println!("The total callibration is {}", total);
}
