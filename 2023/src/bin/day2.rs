use std::collections::HashMap;

fn main() {
    part1();
}

fn part1() {
    let color_counts: HashMap<&str, usize> =
        HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    let mut total = 0;

    let lines = utils::read_input_file_lines();
    let mut game = 1;

    for line in lines {
        if is_valid_game(&color_counts, &line) {
            total += game;
        }

        game += 1;
    }

    println!("Part 1 total {}", total)
}

fn is_valid_game(color_counts: &HashMap<&str, usize>, line: &str) -> bool {
    let (_, string) = line.split_once(": ").expect("Invalid line");
    let rounds = string.split("; ");

    for round in rounds {
        for pick in round.split(", ") {
            let (count_string, color) = pick.split_once(" ").expect("Invalid pair");
            let count: usize = count_string.parse().expect("Invalid size");

            if count > *color_counts.get(color).expect("Invalid color") {
                return false;
            }
        }
    }

    return true;
}
