use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use utils;
use nom::{
  bytes::complete::tag,
  character::complete::{digit1, space1},
  combinator::map_res,
  multi::separated_list0,
  sequence::{pair, preceded, tuple},
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
  let mut total = 0;

  println!("The total callibration for part 2 is {}", total);
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
  return map_res(digit1, str::parse)(input);
}
