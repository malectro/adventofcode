use regex::Regex;
use std::cmp::max;
use std::collections::{HashSet};
use std::fs::File;
use std::io::{prelude::*, BufReader};

struct Cli {
    path: std::path::PathBuf,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Copy, Clone)]
enum Axis {
    X,
    Y,
}

#[derive(Debug, Copy, Clone)]
struct Fold {
    axis: Axis,
    value: usize,
}

fn main() {
    let path = std::env::args().nth(1).expect("no path given");
    let args = Cli {
        path: std::path::PathBuf::from(path),
    };

    let file = File::open(&args.path).expect("could not open file");
    let reader = BufReader::new(file);

    let mut lines = reader
        .lines()
        .map(|line| line.expect("failed to read line"));

    let point_regex = Regex::new(r"^(\d+),(\d+)$").expect("invalid regex");
    let mut points: HashSet<Point> = lines
        .by_ref()
        .take_while(|line| line != "")
        .map(|line| {
            let capture = point_regex.captures(line.as_str()).expect("invalid point");
            Point {
                x: capture
                    .get(1)
                    .expect("invalid x")
                    .as_str()
                    .parse()
                    .expect("invalid x"),
                y: capture
                    .get(2)
                    .expect("invalid y")
                    .as_str()
                    .parse()
                    .expect("invalid y"),
            }
        })
        .collect();

    let fold_regex = Regex::new(r"([xy])=(\d+)$").expect("invalid regex");
    let folds: Vec<Fold> = lines
        .map(|line| {
            let capture = fold_regex.captures(line.as_str()).expect("invalid fold");
            Fold {
                axis: match capture.get(1).expect("invalid axis").as_str() {
                    "x" => Axis::X,
                    _ => Axis::Y,
                },
                value: capture
                    .get(2)
                    .expect("invalid fold value")
                    .as_str()
                    .parse()
                    .expect("invalid fold value"),
            }
        })
        .collect();

    for fold in folds {
        match fold {
            Fold {
                axis: Axis::Y,
                value,
            } => {
                points = points
                    .into_iter()
                    .filter_map(|mut point| {
                        if point.y == value {
                            return None;
                        }

                        if point.y > value {
                            point.y = value - (point.y - value);
                        }

                        Some(point)
                    })
                    .collect()
            }
            Fold {
                axis: Axis::X,
                value,
            } => {
                points = points
                    .into_iter()
                    .filter_map(|mut point| {
                        if point.x == value {
                            return None;
                        }

                        if point.x > value {
                            point.x = value - (point.x - value);
                        }

                        Some(point)
                    })
                    .collect()
            }
        }
    }

    let mut size = points.iter().fold(Point { x: 0, y: 0 }, |mut size, point| {
        size.x = max(size.x, point.x);
        size.y = max(size.y, point.y);
        size
    });

    size.x += 1;
    size.y += 1;

    let mut grid = vec!['.'; size.x * size.y];

    for point in points {
        grid[point.x + point.y * size.x] = '#';
    }

    for y in 0..size.y {
        for x in 0..size.x {
            print!("{}", grid[x + y * size.x]);
        }
        print!("\n");
    }
}
