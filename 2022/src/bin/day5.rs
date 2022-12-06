use itertools::Itertools;
use regex::Regex;
use utils;

fn main() {
  part1();
  part2();
}

fn part1() {
  let mut lines = utils::read_input_file_lines();

  let mut stacks = Vec::new();

  for line in lines.by_ref().take_while(|l| l != "") {
    for (i, mut block) in line.chars().chunks(4).into_iter().enumerate() {
      if stacks.len() <= i {
        stacks.push(Vec::new());
      }
      let possible_char = block.nth(1).unwrap();
      if possible_char != ' ' {
        stacks[i].push(possible_char);
      }
    }
  }

  for stack in stacks.iter_mut() {
    stack.reverse();
  }

  let re =
    Regex::new(r"^move (?P<amount>\d+) from (?P<from>\d+) to (?P<to>\d+)$").expect("Invalid regex");

  for line in lines {
    let captured = re.captures(line.as_ref()).expect("Invalid line");

    let amount: usize = captured["amount"].parse().expect("Invalid amount");
    let from: usize = captured["from"].parse::<usize>().expect("Invalid from") - 1;
    let to: usize = captured["to"].parse::<usize>().expect("Invalid to") - 1;

    for _ in 0..amount {
      let char = stacks[from].pop().unwrap();
      stacks[to].push(char);
    }
  }

  let final_string: String = stacks.iter().map(|stack| stack.last().unwrap()).collect();
  println!("stack string {}", final_string);
}

fn part2() {}
