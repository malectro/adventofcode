use std::fs::File;
use std::io::{prelude::*, BufReader};

struct Cli {
    path: std::path::PathBuf,
}

pub fn read_input_file() -> BufReader<File> {
    let path = std::env::args().nth(1).expect("no path given");
    let args = Cli {
        path: std::path::PathBuf::from(path),
    };

    let file = File::open(&args.path).expect("could not open file");
    BufReader::new(file)
}

pub fn read_input_file_lines() -> impl std::iter::Iterator<Item = String> {
    let reader = read_input_file();

    reader
        .lines()
        .map(|line| line.expect("failed to read line"))
}
