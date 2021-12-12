use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{prelude::*, BufReader};

struct Cli {
    path: std::path::PathBuf,
}

#[derive(Debug, Clone)]
struct Cave {
    name: String,
    is_big: bool,
    edges: Vec<usize>,
}

struct CavePath {}

fn main() {
    let path = std::env::args().nth(1).expect("no path given");
    let args = Cli {
        path: std::path::PathBuf::from(path),
    };

    let file = File::open(&args.path).expect("could not open file");
    let reader = BufReader::new(file);

    let mut caves = Vec::new();
    let mut cave_map: HashMap<String, usize> = HashMap::new();

    for line in reader.lines() {
        let text = line.expect("Cannot parse line");

        let cave_ids: Vec<usize> = text
            .split('-')
            .map(|name| match cave_map.get(name) {
                Some(cave_id) => *cave_id,
                None => {
                    let cave = Cave {
                        name: name.to_string(),
                        is_big: name.to_uppercase() == name,
                        edges: Vec::new(),
                    };
                    caves.push(cave);
                    let cave_id = caves.len() - 1;
                    cave_map.insert(name.to_string(), cave_id);
                    cave_id
                }
            })
            .collect();
        let (cave_id_a, cave_id_b) = (cave_ids[0], cave_ids[1]);

        for (id_a, id_b) in [(cave_id_a, cave_id_b), (cave_id_b, cave_id_a)] {
            let cave = &mut caves[id_a];
            cave.edges.push(id_b);
        }
    }

    println!("caves\n{:?}", caves);

    let mut path = HashSet::new();
    let start_id = caves.iter().position(|cave| cave.name == "start").unwrap();
    println!("total paths {}", count_paths(&caves, &mut path, start_id,));
}

fn count_paths(caves: &Vec<Cave>, path: &mut HashSet<usize>, cave_id: usize) -> u32 {
    let cave = &caves[cave_id];
    if cave.name == "end" {
        return 1;
    }

    let mut count = 0;

    path.insert(cave_id);
    for adjacent_id in cave.edges.iter() {
        let adjacent_cave = &caves[*adjacent_id];
        if adjacent_cave.is_big || !path.contains(&adjacent_id) {
            count += count_paths(caves, path, *adjacent_id);
        }
    }
    path.remove(&cave_id);

    count
}
