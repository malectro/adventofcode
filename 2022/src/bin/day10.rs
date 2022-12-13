use utils;

fn main() {
  let mut signal = 0;
  let mut cycle = 0;
  let mut x = 1;

  for line in utils::read_input_file_lines() {
    let parts: Vec<&str> = line.split(" ").collect();

    let command = parts[0];

    match command {
      "noop" => {
        increment_cycle(&mut cycle, x, &mut signal);
      }
      "addx" => {
        increment_cycle(&mut cycle, x, &mut signal);
        increment_cycle(&mut cycle, x, &mut signal);
        x += parts[1].parse::<isize>().expect("Invalid integer");
      }
      _ => unreachable!("Invalid command"),
    }
  }

  println!("signal sum: {}", signal);
}

fn increment_cycle(cycle: &mut usize, x: isize, signal: &mut isize) {
  *cycle += 1;
  if (*cycle + 20) % 40 == 0 {
    *signal += x * (*cycle as isize);
    //println!("{} {} signal {}", cycle, x, signal);
  }
}
