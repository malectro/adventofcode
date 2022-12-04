use std::collections::HashMap;
use utils;

fn main() {
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

  println!("the final score is {}", score);
}
