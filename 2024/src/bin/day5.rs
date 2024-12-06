use std::cmp::Ordering;

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
  let mut lines = utils::read_input_file_lines();

  let pairs = lines
    .by_ref()
    .take_while(|line| line != "")
    .map(|line| {
      let (_, pair) = separated_pair(parse_int, tag("|"), parse_int)(&line).expect("Invalid pair");
      pair
    })
    .collect_vec();

  let mut order = [[0; 100]; 100];
  for (l, r) in pairs {
    order[l][r] = 1;
  }

  let lists = lines.map(|line| {
    line
      .split(',')
      .map(|num| num.parse::<usize>().expect("Invalid integer"))
      .collect_vec()
  });

  let total: usize = lists.filter(|list| {
    let mut sorted = list.clone();

    sorted.sort_by(|a, b| {
      if order[*a][*b] == 1 {
        Ordering::Less
      } else if order[*b][*a] == 1 {
        Ordering::Greater
      } else {
        Ordering::Equal
      }
    });

    sorted.iter().zip(list).all(|(a, b)| a == b)
  }).map(|list| list[list.len() / 2]).sum();

  println!("part 1 {}", total)
}

fn part2() {
  let mut lines = utils::read_input_file_lines();

  let pairs = lines
    .by_ref()
    .take_while(|line| line != "")
    .map(|line| {
      let (_, pair) = separated_pair(parse_int, tag("|"), parse_int)(&line).expect("Invalid pair");
      pair
    })
    .collect_vec();

  let mut order = [[0; 100]; 100];
  for (l, r) in pairs {
    order[l][r] = 1;
  }

  let lists = lines.map(|line| {
    line
      .split(',')
      .map(|num| num.parse::<usize>().expect("Invalid integer"))
      .collect_vec()
  });

  let total: usize = lists.map(|list| {
    let mut sorted = list.clone();

    sorted.sort_by(|a, b| {
      if order[*a][*b] == 1 {
        Ordering::Less
      } else if order[*b][*a] == 1 {
        Ordering::Greater
      } else {
        Ordering::Equal
      }
    });

    (list, sorted)
  }).filter(|(list, sorted)| {
    sorted.iter().zip(list).any(|(a, b)| a != b)
  }).map(|(_, sorted)| sorted[sorted.len() / 2]).sum();

  println!("part 1 {}", total)
}

fn parse_int(input: &str) -> IResult<&str, usize> {
  return map_res(digit1, str::parse)(input);
}
