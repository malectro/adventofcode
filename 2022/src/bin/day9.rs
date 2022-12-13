use euclid::*;
use std::collections::HashMap;
use utils;

struct Space {}
type Point = Point2D<isize, Space>;
type Dist = Vector2D<isize, Space>;

fn main() {
  let mut cels: HashMap<Point, bool> = HashMap::new();

  let mut head_pos = Point::origin();
  let mut tail_pos = Point::origin();

  for line in utils::read_input_file_lines() {
    let chars: Vec<&str> = line.split(" ").collect();
    let (direction, distance): (&str, usize) =
      (chars[0], chars[1].parse().expect("Invalid distance"));

    let vector = match direction {
      "U" => vec2(0, -1),
      "D" => vec2(0, 1),
      "R" => vec2(1, 0),
      "L" => vec2(-1, 0),
      _ => Dist::zero(),
    };

    for _ in 0..distance {
      head_pos += vector;

      let difference = head_pos - tail_pos;
      let distance = difference.abs();
      let normal = difference.clamp(vec2(-1, -1), vec2(1, 1));

      if distance.x + distance.y > 2 {
        tail_pos += normal;
      } else if distance.x > 1 {
        tail_pos.x += normal.x;
      } else if distance.y > 1 {
        tail_pos.y += normal.y;
      }

      // println!("head: {:?}, tail: {:?}", head_pos, tail_pos);

      cels.insert(tail_pos, true);
    }
  }

  println!("Cels touched: {}", cels.len());
}
