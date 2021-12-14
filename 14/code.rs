use itertools::Itertools;
use std::collections::HashMap;
use utils;

fn main() {
    let mut lines = utils::read_input_file_lines();

    let template = lines.next().expect("must have a template");
    lines.next();

    let mut pair_counts: HashMap<(char, char), usize> = template.chars().tuple_windows().counts();

    let rules: HashMap<(char, char), char> = lines
        .map(|line| {
            let (pair, result) = line.split_once(" -> ").expect("should split");
            (
                pair.chars().next_tuple().expect("invalid pair"),
                result.chars().take(1).next().unwrap(),
            )
        })
        .collect();

    for _ in 0..10 {
        let mut new_pair_counts = HashMap::new();
        for ((c1, c2), count) in pair_counts {
            if let Some(insertion) = rules.get(&(c1, c2)) {
                *new_pair_counts.entry((c1, *insertion)).or_insert(0) += count;
                *new_pair_counts.entry((*insertion, c2)).or_insert(0) += count;
            }
        }
        pair_counts = new_pair_counts;
    }

    let mut char_counts: HashMap<char, usize> =
        HashMap::from([(template.chars().next().expect("first char"), 1)]);
    for ((_c1, c2), count) in &pair_counts {
        *char_counts.entry(*c2).or_insert(0) += count;
    }

    match char_counts.values().minmax() {
        itertools::MinMaxResult::MinMax(min, max) => {
            println!("final value {}", max - min);
        }
        _ => {
            println!("failed to get a min max");
        }
    }
}
