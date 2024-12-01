use std::collections::HashMap;
use utils;
use nom::{
  character::complete::{digit1, space1},
  combinator::map_res,
  sequence::tuple,
  IResult,
};

fn main() {
  part1();
  part2();
}

fn part1() {
  let lines = utils::read_input_file_lines();

  let mut total = 0;

  let mut left = Vec::new();
  let mut right = Vec::new();

  for line in lines {
    let (_, (l, _, r)) = tuple((
      parse_usize,
      space1,
      parse_usize,
    ))(&line)
    .expect("Invalid line");

    left.push(l);
    right.push(r);
  }
  
  left.sort();
  right.sort();

  for (l, r) in left.iter().zip(right.iter()) {
    total += r.abs_diff(*l);
  }

  println!("The total distance is {}", total);
}

fn part2() {
  let lines = utils::read_input_file_lines();

  let mut total = 0;

  let mut left = HashMap::new();
  let mut right = HashMap::new();

  for line in lines {
    let (_, (l, _, r)) = tuple((
      parse_usize,
      space1,
      parse_usize,
    ))(&line)
    .expect("Invalid line");

    *left.entry(l).or_insert(0) += 1;
    *right.entry(r).or_insert(0) += 1;
  }
  
  for (id, count) in left.iter() {
    total += id * count * right.get(id).unwrap_or(&0);
  }

  println!("The similarity score for part 2 is {}", total);
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
  return map_res(digit1, str::parse)(input);
}
