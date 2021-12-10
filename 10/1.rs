use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader, Result};

struct Cli {
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let open_chars = ['(', '[', '{', '<'];
    let close_chars = [')', ']', '}', '>'];

    let error_scores: HashMap<_, _> = close_chars
        .into_iter()
        .zip([3, 57, 1197, 25137].into_iter())
        .collect();

    let open_char_scores: HashMap<_, _> = open_chars
        .into_iter()
        .zip([1, 2, 3, 4].into_iter())
        .collect();

    let close_to_open: HashMap<_, _> = close_chars
        .into_iter()
        .zip(open_chars.into_iter())
        .collect();

    let path = std::env::args().nth(1).expect("no path given");
    let args = Cli {
        path: std::path::PathBuf::from(path),
    };

    let file = File::open(&args.path).expect("could not open file");
    let reader = BufReader::new(file);

    let mut error_score = 0;
    let mut incomplete_scores: Vec<i64> = Vec::new();

    for line in reader.lines() {
        match parse_line(&close_to_open, line?) {
            LineResult::BadChar(c) => {
                error_score += match error_scores.get(&c) {
                    Some(score) => *score,
                    None => 0,
                }
            }
            LineResult::ExtraChars(mut extras) => {
                let mut score: i64 = 0;
                extras.reverse();
                for c in extras {
                    if let Some(char_score) = open_char_scores.get(&c) {
                        score = score * 5 + char_score;
                    }
                }
                incomplete_scores.push(score);
            }
        }
    }

    incomplete_scores.sort();
    let final_score = incomplete_scores
        .get(incomplete_scores.len() / 2)
        .expect("median");

    println!("error score {}", error_score);
    println!("final score {}", final_score);

    Ok(())
}

enum LineResult {
    BadChar(char),
    ExtraChars(Vec<char>),
}

fn parse_line(close_to_open: &HashMap<char, char>, line: String) -> LineResult {
    let mut scope: Vec<char> = Vec::new();

    for c in line.chars() {
        match close_to_open.get(&c) {
            None => scope.push(c),
            Some(open_char) => {
                match scope.last() {
                    Some(top) if top == open_char => scope.pop(),
                    _ => return LineResult::BadChar(c),
                };
                ()
            }
        }
    }

    return LineResult::ExtraChars(scope);
}
