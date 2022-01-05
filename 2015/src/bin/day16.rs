use std::collections::HashMap;
use utils;

fn main() {
  let tape_info = "children: 3
cats: 7
samoyeds: 2
pomeranians: 3
akitas: 0
vizslas: 0
goldfish: 5
trees: 3
cars: 2
perfumes: 1";

  let info: HashMap<&str, usize> = tape_info
    .split('\n')
    .map(|line| {
      let (attr, value) = line.split_once(": ").expect("Invalid line");
      (attr, value.parse().expect("Invalid value"))
    })
    .collect();

  println!("info {:?}", info);

  let lines = utils::read_input_file_lines();

  for line in lines {
    let (left, right) = line.split_once(": ").expect("Invalid line");
    let (_, id) = left.split_once(' ').expect("Invalid line");

    let attributes: Vec<(&str, usize)> = right
      .split(", ")
      .map(|pair| {
        let (attr, value) = pair.split_once(": ").expect("Invalid attribute pair");
        let count: usize = value.parse().expect("Invalid value");
        (attr, count)
      })
      .collect();

    let is_valid = attributes
      .iter()
      .all(|(attr, count)| Some(count) == info.get(attr));

    let is_part_2_valid = attributes.iter().all(|(attr, count)| {
      let gift_count = info.get(attr).expect("Invalid attr");

      match *attr {
        "cats" | "trees" => count > gift_count,
        "pomeranians" | "goldfish" => count < gift_count,
        _ => count == gift_count,
      }
    });

    if is_valid {
      println!("{} is valid for part 1", id);
    }

    if is_part_2_valid {
      println!("{} is valid for part 2", id);
    }
  }
}
