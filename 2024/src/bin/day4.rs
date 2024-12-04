use itertools::Itertools;
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
  let target = "XMAS";
  let rtarget = target.chars().rev().collect::<String>();

  let input = utils::read_input_file_as_string();
  let strings = input.split('\n').filter(|string| string.len() > 0).collect_vec();

  let mut total: usize = 0;

  total += strings.iter().map(|string|
    string.matches(target).count() + string.matches(&rtarget).count()
  ).sum::<usize>();

  let col_len = strings[0].len();
  let chars: Vec<Vec<char>> = strings.iter().map(|string| string.chars().collect()).collect();
  let cols: Vec<String> = (0..col_len).map(|col| 
    (0..strings.len()).map(|row| chars[row][col]).collect()
  ).collect_vec();

  total += cols.iter().map(|string|
    string.matches(target).count() + string.matches(&rtarget).count()
  ).sum::<usize>();


  /*
  total += (0..col_len).map(|col| {
    let column = ColumnIterator::of(&strings, col).collect::<String>();
    column.matches(target).count() + column.matches(&rtarget).count()
  }).sum();
*/

  println!("Part 1 {}", total);
}

fn part2() {
  let mut total = 0;

  println!("Part 2 {}", total);
}

/*
struct ColumnIterator<'a> {
  i: usize,
  col: usize,
  strings: &'a Vec<&'a [char]>,
}

impl<'a> ColumnIterator<'a> {
  fn of(strings: &'a Vec<&'a str>, col: usize) -> ColumnIterator<'a> {
    ColumnIterator { i: 0, col, strings: strings.m }
  }
}

impl Iterator for ColumnIterator<'_> {
  type Item = char;

  fn next(&mut self) -> Option<Self::Item> {
    if self.i < self.strings.len() {
      let i = self.i;
      self.i += 1;
      return Some(self.strings[i][self.col]);
    }
    None
  }
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
*/
