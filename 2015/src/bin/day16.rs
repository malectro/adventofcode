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

    let is_valid = right.split(", ").all(|pair| {
      let (attr, value) = pair.split_once(": ").expect("Invalid attribute pair");
      let count: usize = value.parse().expect("Invalid value");
      Some(&count) == info.get(attr)
    });

    if is_valid {
      println!("{} is valid", id);
    }
  }
}
