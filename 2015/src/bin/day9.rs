use std::collections::{HashMap, HashSet};
use utils;

struct Edge {
  place_1: usize,
  place_2: usize,
  distance: usize,
}

fn main() {
  let mut places = HashMap::new();
  let edges: Vec<Edge> = utils::read_input_file_lines()
    .map(|line| {
      let (left, distance) = line.split_once(" = ").expect("Invalid line");
      let (place1, place2) = left.split_once(" to ").expect("Invalid line");

      let place1_id = get_place_id(&mut places, place1);
      let place2_id = get_place_id(&mut places, place2);

      Edge {
        place_1: place1_id,
        place_2: place2_id,
        distance: distance.parse().expect("Invalid distance"),
      }
    })
    .collect();

  //let mut places = Vec::new();
  let mut distances = vec![vec![0; places.len()]; places.len()];

  for edge in edges {
    distances[edge.place_1][edge.place_2] = edge.distance;
    distances[edge.place_2][edge.place_1] = edge.distance;
  }

  let mut visited = HashSet::new();
  let mut shortest: usize = usize::MAX;
  for (_, id) in places {
    let distance = get_shortest_distance(&distances, &mut visited, id);
    if distance < shortest {
      shortest = distance;
    }
  }

  println!("shortest distance {}", shortest);
}

fn get_place_id(places: &mut HashMap<String, usize>, place: &str) -> usize {
  match places.get(place) {
    Some(place_id) => *place_id,
    _ => {
      let place_id = places.len();
      places.insert(place.to_string(), place_id);
      place_id
    }
  }
}

fn get_shortest_distance(
  distances: &Vec<Vec<usize>>,
  visited: &mut HashSet<usize>,
  start: usize,
) -> usize {
  let mut got_one = false;
  let mut shortest = 0;
  visited.insert(start);
  for (id, distance) in distances[start].iter().enumerate() {
    if !visited.contains(&id) {
      let total = distance + get_shortest_distance(distances, visited, id);
      if !got_one || total < shortest {
        shortest = total;
        got_one = true;
      }
    }
  }
  visited.remove(&start);
  shortest
}
