use json::JsonValue;
use utils;

fn main() {
  let string = utils::read_input_file_as_string();
  let json = json::parse(&string).expect("Failed to parse json");

  println!("sum of all numbers: {}", get_sum(&json, None));
  println!(
    "sum of all non-red numbers: {}",
    get_sum(&json, Some("red"))
  );
}

fn get_sum(value: &JsonValue, ignore_object_prop: Option<&str>) -> i64 {
  match value {
    JsonValue::Number(_) => value.as_i64().expect("Invalid number"),
    JsonValue::Object(_) => {
      let mut sum = 0;
      if let Some(_) = ignore_object_prop {
        for (_, value) in value.entries() {
          if value.as_str() == ignore_object_prop {
            return 0;
          }
          sum += get_sum(value, ignore_object_prop);
        }
      } else {
        for (_, value) in value.entries() {
          sum += get_sum(value, ignore_object_prop);
        }
      }
      sum
    }
    JsonValue::Array(_) => value
      .members()
      .fold(0, |acc, value| get_sum(value, ignore_object_prop) + acc),
    _ => 0,
  }
}
