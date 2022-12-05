use itertools::Itertools;
use std::collections::HashSet;
use utils;

fn main() {
  part1();
  part2();
}

fn part1() {
  let lines = utils::read_input_file_lines();

  let mut score: usize = 0;

  for line in lines {
    let bytes = line.as_bytes();
    let half_len = bytes.len() / 2;

    let left_set: HashSet<&u8> = HashSet::from_iter(bytes[0..half_len].iter());

    let mut repeat_byte: u8 = 0;
    for byte in bytes[half_len..].iter() {
      if left_set.contains(byte) {
        repeat_byte = *byte;
      }
    }

    score += get_score(repeat_byte);
  }

  println!("final score is {}", score);
}

fn part2() {
  let lines = utils::read_input_file_lines();

  let mut score: usize = 0;

  for group in &lines.chunks(3) {
    let mut set: HashSet<u8> = HashSet::new();

    for pack in group {
      let group_set: HashSet<u8> = HashSet::from_iter(pack.as_bytes().iter().copied());

      set = if set.len() == 0 {
        group_set
      } else {
        set.intersection(&group_set).copied().collect()
      };
    }

    let badge = set.iter().next().unwrap();
    score += get_score(*badge);
  }

  println!("final score is {}", score);
}

fn get_score(byte: u8) -> usize {
  return if byte >= 97 {
    byte as usize - 97 + 1
  } else {
    byte as usize - 65 + 27
  };
}
