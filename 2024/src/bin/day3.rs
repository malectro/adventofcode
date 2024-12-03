use itertools::Itertools;
use nom::{
  bytes::complete::{is_not, tag, take_until},
  character::complete::{digit1, space1},
  combinator::{iterator, map_res, opt, verify},
  multi::{many_till, separated_list0},
  sequence::{delimited, preceded, separated_pair},
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

  let total: usize = re.captures_iter(&input).map(|c| c.extract()).map(
    |(_, [num1, num2])| num1.parse::<usize>().unwrap() * num2.parse::<usize>().unwrap()
  ).sum();

  println!("Part 1 {}", total);

  return;

  /*
    let parsed = preceded(
      take_until("mul("),
      delimited(
        tag("mul("),
        separated_pair(parse_mult_num, tag(","), parse_mult_num),
        tag(")"),
      ),
    )(input.as_str())
    .expect("help");
    println!("parsed {:?}", parsed);

    return;
  */

  /*
    let mut mults = iterator(
      input.as_str(),
      preceded(
        take_until("mul("),
        delimited(
          tag("mul("),
          separated_pair(parse_mult_num, tag(","), parse_mult_num),
          tag(")"),
        ),
      ),
    );
  */

  /*
      opt(preceded(
        is_not("mult("),
        separated_pair(parse_mult_num, tag(","), parse_mult_num),
      )),
  */

  /*
  for mult in mults.into_iter() {
    println!("{:?}", mult);
  }
*/
}

fn part2() {}

fn parse_usize(input: &str) -> IResult<&str, usize> {
  return map_res(digit1, str::parse)(input);
}

fn parse_mult_num(input: &str) -> IResult<&str, usize> {
  return map_res(verify(digit1, |digits: &str| digits.len() < 4), str::parse)(input);
}
