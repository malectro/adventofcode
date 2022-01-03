use regex::Regex;
use std::collections::{BinaryHeap, HashMap, HashSet};
use utils;

#[derive(Clone)]
struct Edge {
  guest: usize,
  other_guest: usize,
  amount: isize,
}

#[derive(Eq, PartialEq, Clone)]
struct Node {
  score: isize,
  guest: usize,
  to_visit: HashSet<usize>,
}

impl Ord for Node {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.score.cmp(&other.score)
  }
}

impl PartialOrd for Node {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

fn main() {
  let lines = utils::read_input_file_lines();

  let re = Regex::new(
    r"^(?P<name>[^ ]+) would (?P<sign>gain|lose) (?P<amount>\d+) happiness units by sitting next to (?P<other_name>[^ ]+).$",
  ).expect("Invalid regex");
  let mut guests = HashMap::new();

  let edges: Vec<Edge> = lines
    .map(|line| {
      let captured = re.captures(line.as_ref()).expect("Invalid line");

      let name = captured["name"].to_string();
      let other_name = captured["other_name"].to_string();

      let edge_guests = [name, other_name].map(|name| match guests.get(&name) {
        Some(id) => *id,
        None => {
          let id = guests.len();
          guests.insert(name, id);
          id
        }
      });

      let abs_amount: isize = captured["amount"].parse().expect("Invalid amount number");
      let amount = match &captured["sign"] {
        "lose" => -1 * abs_amount,
        _ => abs_amount,
      };

      Edge {
        guest: edge_guests[0],
        other_guest: edge_guests[1],
        amount: amount,
      }
    })
    .collect();

  println!("best score {}", get_best_score(&edges, guests.len()));

  let myself = guests.len();
  let mut new_edges = edges.clone();
  for guest in 0..myself {
    new_edges.push(Edge {
      guest: myself,
      other_guest: guest,
      amount: 0,
    });
    new_edges.push(Edge {
      guest: guest,
      other_guest: myself,
      amount: 0,
    });
  }

  println!(
    "best score with me {}",
    get_best_score(&new_edges, guests.len() + 1)
  );
}

fn get_best_score(edges: &Vec<Edge>, guest_count: usize) -> isize {
  let mut amounts = vec![vec![0isize; guest_count]; guest_count];
  for edge in edges {
    amounts[edge.guest][edge.other_guest] = edge.amount;
  }

  let mut max = 0;

  for start in 0..amounts.len() {
    let mut totals = vec![0isize; guest_count];

    // NOTE (kyle): using a heap here to be "more efficient", but we're exhausting
    // all options so a VecDeque would be just as good.
    let mut queue = BinaryHeap::from([Node {
      score: 0,
      guest: start,
      to_visit: (0..amounts.len()).collect(),
    }]);

    while let Some(current) = queue.pop() {
      totals[current.guest] = current.score;

      let mut new_to_visit = current.to_visit.clone();
      new_to_visit.remove(&current.guest);

      if new_to_visit.len() == 0 {
        let final_score = get_score(&amounts, current.guest, start) + current.score;
        if final_score > max {
          max = final_score;
        }
      }

      for og in new_to_visit.iter() {
        let other_guest = *og;
        let both = get_score(&amounts, current.guest, other_guest);
        let total = both + current.score;

        queue.push(Node {
          score: total,
          guest: other_guest,
          to_visit: new_to_visit.clone(),
        });
      }
    }
  }

  max
}

fn get_score(scores: &Vec<Vec<isize>>, guest1: usize, guest2: usize) -> isize {
  scores[guest1][guest2] + scores[guest2][guest1]
}
