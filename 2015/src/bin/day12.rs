use json::JsonValue;
use utils;

fn main() {
  let string = utils::read_input_file_as_string();
  let json = json::parse(&string).expect("Failed to parse json");

  println!("sum of all numbers: {}", get_sum(&json));
}

fn get_sum(value: &JsonValue) -> i64 {
  match value {
    JsonValue::Number(number) => number.as_fixed_point_i64(0).expect("Invalid number"),
    JsonValue::Object(_) => value
      .entries()
      .fold(0, |acc, (_, value)| get_sum(value) + acc),
    JsonValue::Array(_) => value.members().fold(0, |acc, value| get_sum(value) + acc),
    _ => 0,
  }
}
