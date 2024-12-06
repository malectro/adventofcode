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
  let strings = input
    .split('\n')
    .filter(|string| string.len() > 0)
    .collect_vec();

  let mut total: usize = 0;

  total += strings
    .iter()
    .map(|string| string.matches(target).count() + string.matches(&rtarget).count())
    .sum::<usize>();

  let row_len = strings.len();
  let col_len = strings[0].len();
  let chars: Vec<Vec<char>> = strings
    .iter()
    .map(|string| string.chars().collect())
    .collect();

  let cols: Vec<String> = (0..col_len)
    .map(|col| (0..strings.len()).map(|row| chars[row][col]).collect())
    .collect_vec();

  total += cols
    .iter()
    .map(|string| string.matches(target).count() + string.matches(&rtarget).count())
    .sum::<usize>();

  let mut diags = Vec::new();
  for start_row in 0..row_len {
    let start_col = 0;

    let mut string_down = String::new();
    let mut string_up = String::new();

    let mut row = start_row;
    let mut col = start_col;

    while row < row_len && col < col_len {
      string_down.push(chars[row][col]);
      row += 1;
      col += 1;
    }

    diags.push(string_down);

    row = start_row + 1;
    col = start_col;

    while row != 0 && col < col_len {
      row -= 1;
      string_up.push(chars[row][col]);
      col += 1;
    }

    diags.push(string_up);
  }

  for start_col in 1..col_len {
    let start_row = 0;

    let mut string_down = String::new();
    let mut string_up = String::new();

    let mut row = start_row;
    let mut col = start_col;

    while row < row_len && col < col_len {
      string_down.push(chars[row][col]);
      row += 1;
      col += 1;
    }

    diags.push(string_down);

    row = row_len;
    col = start_col;

    while row != 0 && col < col_len {
      row -= 1;
      string_up.push(chars[row][col]);
      col += 1;
    }

    diags.push(string_up);
  }

  total += diags
    .iter()
    .map(|string| string.matches(target).count() + string.matches(&rtarget).count())
    .sum::<usize>();

  println!("Part 1 {}", total);
}

fn part2() {
  let mut total = 0;

  let target = "MAS";
  let rtarget = target.chars().rev().collect::<String>();

  let input = utils::read_input_file_as_string();
  let strings = input
    .split('\n')
    .filter(|string| string.len() > 0)
    .map(|string| string.chars().collect_vec())
    .collect_vec();

  let col_len = strings[0].len();

  for row in 0..(strings.len() - target.len() + 1) {
    for col in 0..(col_len - target.len() + 1) {
      let mut down = String::new();
      for i in 0..target.len() {
          down.push(strings[row + i][col + i]);
      }
      let mut up = String::new();
      for i in 0..target.len() {
          up.push(strings[row + target.len() - 1 - i][col + i]);
      }
      if (down == target || down == rtarget) && (up == target || up == rtarget) {
        total += 1;
      }
    }
  }

  println!("Part 2 {}", total);
}
