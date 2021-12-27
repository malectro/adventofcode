use utils;

fn main() {
  let string = utils::read_input_file_as_string();

  let mut floor = 0;
  let mut first_basement_position = 0;
  for (i, byte) in string.chars().enumerate() {
    if byte == '(' {
      floor += 1;
    } else {
      floor -= 1;
    }
    if floor == -1 && first_basement_position == 0 {
      first_basement_position = i + 1;
    }
  }

  println!(
    "on floor {}, first basement position {}",
    floor, first_basement_position
  );
}
