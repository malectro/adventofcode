use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use utils;

fn main() {
    part1();
    part2();
}

fn part1() {
    let lines = utils::read_input_file_lines();

    let mut total = 0;

    for line in lines {
        let numbers: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
        total += numbers.first().unwrap_or(&0) * 10 + numbers.last().unwrap_or(&0);
    }

    println!("The total callibration for part 1 is {}", total);
}

fn part2() {
    let number_map: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let lines = utils::read_input_file_lines();

    let forward_keys = number_map.keys().join("|");
    let forward_re = Regex::new(&format!("({}|\\d)", forward_keys)).expect("Invalid regex");

    let backward_keys: String = forward_keys.chars().rev().collect();
    let backward_re = Regex::new(&format!("({}|\\d)", backward_keys)).expect("Invalid regex");

    let mut total = 0;

    for line in lines {
        let first = forward_re.find(&line).expect("Invalid line").as_str();
        if let Some(number) = parse_string(&number_map, first) {
            total += number * 10;
        }

        let reversed_line: String = line.chars().rev().collect();
        let last: String = backward_re
            .find(&reversed_line)
            .expect("Invalid line")
            .as_str()
            .chars()
            .rev()
            .collect();

        if let Some(number) = parse_string(&number_map, &last) {
            total += number;
        }
    }

    println!("The total callibration for part 2 is {}", total);
}

fn parse_string(number_map: &HashMap<&str, u32>, string: &str) -> Option<u32> {
    if let Some(number) = number_map.get(string) {
        return Some(*number);
    } else if let Some(number) = string.parse::<u32>().ok() {
        return Some(number);
    } else {
        return None;
    }
}
