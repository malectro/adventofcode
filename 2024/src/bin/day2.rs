use itertools::Itertools;
use nom::{
  character::complete::{digit1, space1},
  combinator::map_res,
  multi::separated_list0,
  sequence::tuple,
  IResult,
};
use std::collections::HashMap;
use utils;

fn main() {
  part1();
  //part2();
}

fn part1() {
  let lines = utils::read_input_file_lines();

  let count: usize = lines
    .map(|line| {
      let (_, list) = separated_list0(space1, parse_int)(&line).expect("Invalid line");

      let group = list
        .iter()
        .tuple_windows()
        .map(|(a, b)| a - b)
        .collect_vec();

      let is_safe = group.iter().map(|diff| diff.signum()).all_equal()
        && group
          .iter()
          .map(|diff| diff.abs())
          .all(|diff| diff > 0 && diff < 4);

      return if is_safe { 1 } else { 0 };
    })
    .sum();

  println!("Part 1 {}", count);
}

fn parse_int(input: &str) -> IResult<&str, i16> {
  return map_res(digit1, str::parse)(input);
}
