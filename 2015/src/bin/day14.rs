use regex::Regex;
use std::cmp;
use utils;

#[derive(Debug)]
struct Reindeer {
  id: usize,
  name: String,
  speed: usize,
  stamina: usize,
  recovery: usize,
}

fn main() {
  let re = Regex::new(
        r"^(?P<name>[^ ]+) can fly (?P<speed>\d+) km/s for (?P<stamina>\d+) seconds, but then must rest for (?P<recovery>\d+) seconds."
    ).expect("Invalid regex");

  let reindeers: Vec<Reindeer> = utils::read_input_file_lines()
    .enumerate()
    .map(|(i, line)| {
      let captured = re.captures(line.as_ref()).expect("Invalid line");
      Reindeer {
        id: i,
        name: captured["name"].to_string(),
        speed: captured["speed"].parse().expect("Invalid speed"),
        stamina: captured["stamina"].parse().expect("Invalid stamina"),
        recovery: captured["recovery"].parse().expect("Invalid recovery"),
      }
    })
    .collect();

  println!("reindeers {:?}", reindeers);

  let total_time = 2503;
  //let total_time = 1000;

  let max_distance = reindeers.iter().fold(0, |max, reindeer| {
    let period = reindeer.stamina + reindeer.recovery;
    let periods = total_time / period;

    let mut distance = periods * reindeer.stamina * reindeer.speed;

    let remainder = total_time % period;
    distance += cmp::min(reindeer.stamina, remainder) * reindeer.speed;

    cmp::max(distance, max)
  });

  println!("max reindeer distance: {}", max_distance);

  let mut scores = vec![0usize; reindeers.len()];
  let mut distances = vec![0usize; reindeers.len()];
  for i in 0..total_time {
    for reindeer in reindeers.iter() {
      let period = reindeer.stamina + reindeer.recovery;
      let moment = i % period;
      if moment < reindeer.stamina {
        distances[reindeer.id] += reindeer.speed;
      }
    }

    let max_distance = *distances.iter().fold(&0, std::cmp::max);

    for (id, distance) in distances.iter().enumerate() {
      if *distance == max_distance {
        scores[id] += 1;
      }
    }
  }

  let best_distance = distances.into_iter().fold(0, std::cmp::max);
  println!("best reindeer distance: {}", best_distance);

  let best_score = scores.into_iter().fold(0, std::cmp::max);
  println!("best reindeer score: {}", best_score);
}
