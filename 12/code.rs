use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{prelude::*, BufReader};

struct Cli {
    path: std::path::PathBuf,
}

// #[derive(Debug, Copy, Clone)]
struct Cave {
    name: String,
    edges: Vec<Cave>,
}

struct CavePath {}

fn main() {
    let path = std::env::args().nth(1).expect("no path given");
    let args = Cli {
        path: std::path::PathBuf::from(path),
    };

    let file = File::open(&args.path).expect("could not open file");
    let reader = BufReader::new(file);

    let mut edge_map = HashMap::new();
    for line in reader.lines() {
        let text = line.expect("Cannot parse line");
        let parts: Vec<&str> = text.split('-').collect();

        //let (name_a, name_b) = (parts[0].to_string(), parts[1].to_string());
        let (name_a, name_b) = (parts[0], parts[1]);

        add_edge(&mut edge_map, &name_a, &name_b);
        add_edge(&mut edge_map, &name_b, &name_a);

        println!("{} = {}", name_a, name_b);
    }

    let mut path = HashSet::new();
    let total_paths = count_paths(&edge_map, &mut path, &"start".to_string());

    println!("total paths {}", total_paths);
}

fn add_edge(edge_map: &mut HashMap<String, Vec<String>>, name_a: &str, name_b: &str) {
    let string_b = name_b.to_string();
    let edge = edge_map.entry(name_a.to_string()).or_insert(Vec::new());
    if !edge.contains(&string_b) {
        edge.push(string_b);
    }
}

fn count_paths(
    edge_map: &HashMap<String, Vec<String>>,
    path: &mut HashSet<String>,
    from_node: &String,
) -> u32 {
    if from_node == "end" {
        return 1;
    }

    let mut count = 0;

    let edges = edge_map.get(from_node).expect("Must have edges");

    path.insert(String::from(from_node));
    //path.insert(*from_node);
    for to_node in edges {
        if !path.contains(to_node) {
            count += count_paths(edge_map, path, to_node);
        }
    }
    path.remove(from_node);

    count
}
