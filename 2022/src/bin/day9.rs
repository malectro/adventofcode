use euclid::*;
use itertools::Itertools;
use std::collections::HashMap;
use utils;

struct Space {}
type Point = Point2D<isize, Space>;
type Dist = Vector2D<isize, Space>;

fn main() {
  pull_rope(2);
  pull_rope(10);
}

fn pull_rope(knot_count: usize) {
  let mut cels: HashMap<Point, bool> = HashMap::new();

  let mut knots: Vec<Point> = vec![Point::origin(); knot_count];

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
      knots[0] += vector;

      for i in 1..knots.len() {
        let head_pos = knots[i - 1];
        let mut tail_pos = knots[i];

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

        knots[i] = tail_pos;

        //println!("head: {:?}, tail: {:?}", head_pos, tail_pos);
      }

      cels.insert(knots[knots.len() - 1], true);
    }
  }

  println!("Cels touched: {}", cels.len());
}
