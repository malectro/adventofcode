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

    score += if repeat_byte >= 97 {
      repeat_byte as usize - 97 + 1
    } else {
      repeat_byte as usize - 65 + 27
    }
  }

  println!("final score is {}", score);
}

fn part2() {
}
