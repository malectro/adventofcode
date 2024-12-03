use nom::{
  bytes::complete::tag,
  character::complete::digit1,
  combinator::{map, map_res, verify},
  sequence::{delimited, separated_pair},
  IResult,
};
use regex::Regex;
use utils;

fn main() {
  part1();
  part2();
}

fn part1() {
  let input = utils::read_input_file_as_string();

  let re = Regex::new("mul\\((\\d{1,3}),(\\d{1,3})\\)").expect("Invalid regex");

  let total: usize = re
    .captures_iter(&input)
    .map(|c| c.extract())
    .map(|(_, [num1, num2])| num1.parse::<usize>().unwrap() * num2.parse::<usize>().unwrap())
    .sum();

  println!("Part 1 {}", total);
}

fn part2() {
  let input = utils::read_input_file_as_string();

  let mut current = input.as_str();
  let mut total = 0;
  let mut is_on = true;

  while current.len() > 0 {
    if let Ok((rest, num)) = parse_mult(current) {
      if is_on {
        total += num;
      }
      current = rest;
    } else if let Ok((rest, _)) = parse_do(current) {
      is_on = true;
      current = rest; 
    } else if let Ok((rest, _)) = parse_dont(current) {
      is_on = false;
      current = rest;
    } else {
      current = &current[1..]
    }
  }

  println!("Part 2 {}", total);
}

fn parse_mult_num(input: &str) -> IResult<&str, usize> {
  map_res(verify(digit1, |digits: &str| digits.len() < 4), str::parse)(input)
}

fn parse_mult(input: &str) -> IResult<&str, usize> {
  map(
    delimited(
      tag("mul("),
      separated_pair(parse_mult_num, tag(","), parse_mult_num),
      tag(")"),
    ),
    |(a, b)| a * b,
  )(input)
}

fn parse_dont(input: &str) -> IResult<&str, &str> {
  tag("don't()")(input)
}

fn parse_do(input: &str) -> IResult<&str, &str> {
  tag("do()")(input)
}
