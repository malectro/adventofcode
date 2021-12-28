use itertools::Itertools;
use utils;

fn main() {
  let lines = utils::read_input_file_lines();

  let mut count = 0;
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
      count += 1;
    }
  }

  println!("number of nice lines {}", count);
}
