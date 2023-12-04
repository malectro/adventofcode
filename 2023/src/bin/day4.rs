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
  let mut total = 0;

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
      total += score;
    }
  }

  println!("Part 1 {}", total);
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
  return map_res(digit1, str::parse)(input);
}
