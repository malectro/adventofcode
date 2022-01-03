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
}
