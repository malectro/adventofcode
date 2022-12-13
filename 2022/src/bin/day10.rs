use utils;

fn main() {
  let mut signal = 0;
  let mut cycle = 0;
  let mut x = 1;

  let mut screen: [bool; 40 * 6] = [false; 40 * 6];

  for line in utils::read_input_file_lines() {
    let parts: Vec<&str> = line.split(" ").collect();

    let command = parts[0];

    match command {
      "noop" => {
        increment_cycle(&mut screen, &mut cycle, x, &mut signal);
      }
      "addx" => {
        increment_cycle(&mut screen, &mut cycle, x, &mut signal);
        increment_cycle(&mut screen, &mut cycle, x, &mut signal);
        x += parts[1].parse::<isize>().expect("Invalid integer");
      }
      _ => unreachable!("Invalid command"),
    }
  }

  println!("signal sum: {}", signal);

  for row in screen.chunks(40) {
    println!("{:?}", row.iter().map(|value| if *value { "#" } else { "." }).collect::<Vec<&str>>().join(""));
  }
}

fn increment_cycle(screen: &mut [bool], cycle: &mut usize, x: isize, signal: &mut isize) {
  let sprite_pos = (x - 1) as usize;
  let paint_pos = *cycle % 40;
  if paint_pos >= sprite_pos && paint_pos < sprite_pos + 3 {
    screen[*cycle] = true;
  }

  *cycle += 1;
  if (*cycle + 20) % 40 == 0 {
    *signal += x * (*cycle as isize);
  }
}
