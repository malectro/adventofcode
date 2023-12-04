use nom::{
  bytes::complete::tag,
  character::complete::{digit1, space1},
  combinator::map_res,
  multi::separated_list0,
  sequence::{pair, preceded, tuple},
  IResult,
};
use std::collections::HashSet;

fn main() {
  part1();
}

fn part1() {
  let mut part1_total = 0;
  let mut part2_total = 0;

  let mut multipliers: Vec<(usize, usize)> = Vec::new();

  for line in utils::read_input_file_lines() {
    let (_, (card_id, _, winners, _, numbers)) = preceded(
      pair(tag("Card"), space1),
      tuple((
        parse_usize,
        pair(tag(":"), space1),
        separated_list0(space1, parse_usize),
        tuple((space1, tag("|"), space1)),
        separated_list0(space1, parse_usize),
      )),
    )(&line)
    .expect("Invalid card");

    let winner_set: HashSet<usize> = HashSet::from_iter(winners);

    let count = numbers.iter().filter(|n| winner_set.contains(n)).count();
    if count > 0 {
      let score = 2usize.pow((count - 1).try_into().expect("Invalid integer"));
      part1_total += score;
    }

    let card_count = 1 + multipliers.iter().fold(0, |acc, pair| pair.1 + acc);
    part2_total += card_count;

    for count in multipliers.iter_mut() {
      count.0 -= 1;
    }
    multipliers.retain(|&count| count.0 > 0);

    if count > 0 {
      multipliers.push((count, card_count));
    }
  }

  println!("Part 1 {}", part1_total);
  println!("Part 2 {}", part2_total);
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
  return map_res(digit1, str::parse)(input);
}
