use regex::Regex;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Point {
  x: usize,
  y: usize,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Rect {
  origin: Point,
  end: Point,
}

enum Command {
  TurnOn,
  TurnOff,
  Toggle,
}

fn main() {
  let lines = utils::read_input_file_lines();

  let mut grid = [[0u8; 1000]; 1000];

  let re = Regex::new(r"^(?P<command>turn on|turn off|toggle) (?P<ox>\d+),(?P<oy>\d+) through (?P<ex>\d+),(?P<ey>\d+)$").expect("Invalid regex");
  for line in lines {
    let captured = re.captures(line.as_ref()).expect("Invalid line");
    let new_rect = Rect {
      origin: Point {
        x: captured["ox"].parse::<usize>().expect("Invalid origin x"),
        y: captured["oy"].parse::<usize>().expect("Invalid origin y"),
      },
      end: Point {
        x: captured["ex"].parse::<usize>().expect("Invalid end x") + 1,
        y: captured["ey"].parse::<usize>().expect("Invalid end y") + 1,
      },
    };

    let command = match &captured["command"] {
      "turn on" => Command::TurnOn,
      "turn off" => Command::TurnOff,
      _ => Command::Toggle,
    };

    for x in new_rect.origin.x..new_rect.end.x {
      for y in new_rect.origin.y..new_rect.end.y {
        grid[x][y] = match command {
          Command::TurnOn => 1,
          Command::TurnOff => 0,
          _ => (grid[x][y] + 1) % 2,
        }
      }
    }
  }

  let count: usize = grid.iter().fold(0, |acc, row| {
    row.iter().fold(acc, |acc, item| {
      acc
        + match item {
          1 => 1,
          _ => 0,
        }
    })
  });

  println!("count {}", count);
}
