use std::collections::{HashMap, VecDeque};
use utils;

#[derive(Debug, Eq, PartialEq)]
enum ConnectionType {
  Value,
  Not,
  And,
  Or,
  LShift,
  RShift,
}

struct Connection {
  ctype: ConnectionType,
  inputs: (String, String),
  destination: String,
}

fn main() {
  let mut connections: VecDeque<Connection> = utils::read_input_file_lines()
    .into_iter()
    .map(|line| {
      let (left, destination) = line.split_once(" -> ").expect("Invalid line");
      let left_side: Vec<&str> = left.split(' ').collect();
      match left_side.len() {
        1 => Connection {
          ctype: ConnectionType::Value,
          inputs: (left.to_string(), String::from("0")),
          destination: destination.to_string(),
        },
        2 => Connection {
          ctype: ConnectionType::Not,
          inputs: (left_side[1].to_string(), String::from("0")),
          destination: destination.to_string(),
        },
        _ => {
          let ctype = match left_side[1] {
            "AND" => ConnectionType::And,
            "OR" => ConnectionType::Or,
            "LSHIFT" => ConnectionType::LShift,
            "RSHIFT" => ConnectionType::RShift,
            _ => panic!("Invalid connection type"),
          };
          Connection {
            ctype: ctype,
            inputs: (left_side[0].to_string(), left_side[2].to_string()),
            destination: destination.to_string(),
          }
        }
      }
    })
    .collect();

  let mut wires: HashMap<String, usize> = HashMap::new();

  while let Some(connection) = connections.pop_front() {
    if let (Some(wire1), Some(wire2)) = (
      get_wire_value(&wires, &connection.inputs.0),
      get_wire_value(&wires, &connection.inputs.1),
    ) {
      wires.insert(
        connection.destination,
        match connection.ctype {
          ConnectionType::Value => wire1,
          ConnectionType::Not => !wire1,
          ConnectionType::And => wire1 & wire2,
          ConnectionType::Or => wire1 | wire2,
          ConnectionType::LShift => wire1 << wire2,
          ConnectionType::RShift => wire1 >> wire2,
        },
      );
    } else {
      connections.push_back(connection);
    }
  }

  if let Some(wire) = wires.get("a") {
    println!("wire a is {}", wire);
  }
}

fn get_wire_value(wires: &HashMap<String, usize>, input: &String) -> Option<usize> {
  if let Ok(value) = input.parse() {
    return Some(value);
  }
  match wires.get(input) {
    Some(value) => Some(*value),
    _ => None,
  }
}
