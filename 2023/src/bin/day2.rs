use std::collections::HashMap;

fn main() {
  part1();
  part2();
}

fn part1() {
  let color_counts: HashMap<&str, usize> =
    HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

  let mut total = 0;

  for game in parse_games() {
    if is_valid_game(&color_counts, &game) {
      total += game.id;
    }
  }

  println!("Part 1 total {}", total)
}

fn part2() {
  let mut color_counts: HashMap<&str, usize> = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

  let mut total = 0;

  for game in parse_games() {
    // reset values
    for value in color_counts.values_mut() {
      *value = 0;
    }

    for round in game.rounds {
      for pair in round.pairs {
        let current_value = color_counts.get_mut(&*pair.color).expect("Invalid color");
        if *current_value < pair.count {
          *current_value = pair.count;
        }
      }
    }

    total += color_counts.values().fold(1, |acc, count| acc * count);
  }

  println!("Part 2 total {}", total);
}

fn is_valid_game(color_counts: &HashMap<&str, usize>, game: &Game) -> bool {
  for round in game.rounds.as_slice() {
    for pair in round.pairs.as_slice() {
      if pair.count > *color_counts.get(&*pair.color).expect("Invalid color") {
        return false;
      }
    }
  }

  return true;
}

struct Pair {
  color: String,
  count: usize,
}

struct Round {
  pairs: Vec<Pair>,
}

struct Game {
  id: usize,
  rounds: Vec<Round>,
}

fn parse_games() -> impl std::iter::Iterator<Item = Game> {
  let lines = utils::read_input_file_lines();

  return lines.enumerate().map(|(i, line)| {
    let (_, string) = line.split_once(": ").expect("Invalid line");

    let mut rounds: Vec<Round> = Vec::new();

    for round in string.split("; ") {
      let mut pairs: Vec<Pair> = Vec::new();

      for pick in round.split(", ") {
        let (count_string, color) = pick.split_once(" ").expect("Invalid pair");
        let count: usize = count_string.parse().expect("Invalid size");

        pairs.push(Pair {
          color: color.to_string(),
          count,
        })
      }

      rounds.push(Round { pairs });
    }

    return Game {
      id: i + 1,
      rounds,
    }
  });
}
