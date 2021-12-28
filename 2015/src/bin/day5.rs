use itertools::Itertools;
use std::collections::HashMap;
use utils;

fn main() {
  let lines = utils::read_input_file_lines();

  let mut part1_count = 0;
  let mut part2_count = 0;
  for line in lines {
    let mut vowel_count = 0;
    for c in line.chars() {
      if ['a', 'e', 'i', 'o', 'u'].contains(&c) {
        vowel_count += 1;
      }
      if vowel_count > 2 {
        break;
      }
    }

    let has_vowel = vowel_count > 2;

    let mut has_repeat = false;
    let mut has_bad_pair = false;
    for (c1, c2) in line.chars().tuple_windows() {
      has_repeat = has_repeat || c1 == c2;
      has_bad_pair = has_bad_pair
        || match (c1, c2) {
          ('a', 'b') => true,
          ('c', 'd') => true,
          ('p', 'q') => true,
          ('x', 'y') => true,
          _ => false,
        }
    }

    if has_vowel && has_repeat && !has_bad_pair {
      part1_count += 1;
    }

    let mut has_repeat_pair = false;
    let mut pairs = HashMap::new();
    for (i, (c1, c2)) in line.chars().tuple_windows().enumerate() {
      if let Some(index) = pairs.get(&(c1, c2)) {
        if i > index + 1 {
          has_repeat_pair = true;
        }
      } else {
        pairs.insert((c1, c2), i);
      }
    }

    let has_sandwich = line.chars().tuple_windows().any(|(c1, c2, c3)| c1 == c3);

    if has_repeat_pair && has_sandwich {
      part2_count += 1;
    }
  }

  println!("part 1 {}", part1_count);
  println!("part 2 {}", part2_count);
}
