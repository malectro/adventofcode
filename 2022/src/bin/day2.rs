use std::collections::HashMap;
use utils;

fn main() {
  part1();
  part2();
}

fn part1() {
  let lines = utils::read_input_file_lines();

  let conversion = HashMap::from([("A", 1), ("B", 2), ("C", 3)]);
  let values = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);

  let mut score = 0;
  for line in lines {
    let chars: Vec<&str> = line.split(" ").collect();
    let player1move = conversion.get(chars[0]).unwrap();
    let player2move = values.get(chars[1]).unwrap();

    score += player2move;

    let diff = player2move - player1move;
    score += if diff == 0 {
      3
    } else if [1, -2].contains(&diff) {
      6
    } else {
      0
    }
  }

  println!("the final score for part 1 is {}", score);
}

fn part2() {
  let lines = utils::read_input_file_lines();

  let conversion = HashMap::from([("A", 0), ("B", 1), ("C", 2)]);

  let mut score = 0;
  for line in lines {
    let chars: Vec<&str> = line.split(" ").collect();
    let player1move = conversion.get(chars[0]).unwrap();

    score += match chars[1] {
      "X" => 1 + (player1move + 2) % 3,
      "Y" => 3 + 1 + player1move,
      "Z" => 6 + 1 + ((player1move + 1) % 3),
      &_ => 0,
    }
  }

  println!("the final score for part 2 is {}", score);
}
