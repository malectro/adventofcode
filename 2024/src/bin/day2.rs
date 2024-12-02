use itertools::Itertools;
use nom::{
  character::complete::{digit1, space1},
  combinator::map_res,
  multi::separated_list0,
  IResult,
};
use utils;

fn main() {
  part1();
  part2();
}

fn part1() {
  let lines = utils::read_input_file_lines();

  let count: usize = lines
    .map(|line| {
      let (_, list) = separated_list0(space1, parse_int)(&line).expect("Invalid line");

      let is_safe = get_safety(&list);

      return if is_safe { 1 } else { 0 };
    })
    .sum();

  println!("Part 1 {}", count);
}

fn part2() {
  let lines = utils::read_input_file_lines();

  let count: usize = lines
    .map(|line| {
      let (_, list) = separated_list0(space1, parse_int)(&line).expect("Invalid line");

      let is_safe = list
        .iter()
        .enumerate()
        .map(|(i, _)| {
          let mut permutation = list[..i].to_vec();
          permutation.extend_from_slice(&list[(i+1)..]);
          return permutation;
        })
        .any(|permutation| get_safety(&permutation));

      return if is_safe { 1 } else { 0 };
    })
    .sum();

  println!("Part 2 {}", count);
}

fn parse_int(input: &str) -> IResult<&str, i16> {
  return map_res(digit1, str::parse)(input);
}

fn get_safety(list: &[i16]) -> bool {
  let group = list
    .iter()
    .tuple_windows()
    .map(|(a, b)| a - b)
    .collect_vec();

  return group.iter().map(|diff| diff.signum()).all_equal()
    && group
      .iter()
      .map(|diff| diff.abs())
      .all(|diff| diff > 0 && diff < 4);
}
