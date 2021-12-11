use std::cmp::min;
use std::fs::File;
use std::io::{prelude::*, BufReader};

struct Cli {
    path: std::path::PathBuf,
}

#[derive(Debug, Copy, Clone)]
struct Jellyfish {
    level: u32,
    has_flashed: bool,
}

fn main() {
    let path = std::env::args().nth(1).expect("no path given");
    let args = Cli {
        path: std::path::PathBuf::from(path),
    };

    let file = File::open(&args.path).expect("could not open file");
    let reader = BufReader::new(file);

    let mut grid: [[Jellyfish; 10]; 10] = [[Jellyfish {
        level: 0,
        has_flashed: false,
    }; 10]; 10];

    for (y, line) in reader.lines().enumerate() {
        for (x, number) in line.expect("Failed to read line").chars().enumerate() {
            grid[x][y] = Jellyfish {
                level: number.to_digit(10).expect("Failed to parse digit."),
                has_flashed: false,
            }
        }
    }

    let mut flash_count: u64 = 0;
    for _cycle in 0..100 {
        for col in grid.iter_mut() {
            for mut jelly in col.iter_mut() {
                jelly.level += 1;
                jelly.has_flashed = false;
            }
        }

        let mut flashers = vec![];
        for (x, col) in grid.iter().enumerate() {
            for (y, jelly) in col.iter().enumerate() {
                if jelly.level > 9 {
                    flashers.push((x, y));
                }
            }
        }

        for (x, y) in flashers {
            if !grid[x][y].has_flashed {
                flash_count += flash(&mut grid, x, y);
            }
        }
    }

    println!("flash count {}", flash_count);
}

fn flash(grid: &mut [[Jellyfish; 10]; 10], x: usize, y: usize) -> u64 {
    let mut count = 1;

    let jelly = &mut grid[x][y];
    jelly.level = 0;
    jelly.has_flashed = true;

    for i in (x.saturating_sub(1))..min(x + 2, 10) {
        for j in (y.saturating_sub(1))..min(y + 2, 10) {
            let jelly = &mut grid[i][j];
            if !jelly.has_flashed {
                jelly.level += 1;
                if jelly.level > 9 {
                    count += flash(grid, i, j);
                }
            }
        }
    }

    count
}
