use std::collections::HashSet;
use utils;

fn main() {
  let string = utils::read_input_file_as_string();

  let mut location = (0, 0);

  let mut visited = HashSet::new();
  visited.insert(location);

  for c in string.chars() {
    visit(&mut visited, &mut location, c);
  }

  println!("part 1 locations visited {}", visited.len());


  let mut santas = [
    (0, 0),
    (0, 0),
  ];

  visited.clear();
  visited.insert(santas[0]);

  let mut i = 0;
  for c in string.chars() {
    visit(&mut visited, &mut santas[i], c);
    i = (i + 1) % 2;
  }

  println!("part 2 locations visited {}", visited.len());
}

fn visit(visited: &mut HashSet<(isize, isize)>, location: &mut (isize, isize), c: char) {
  match c {
    '^' => location.1 += 1,
    '>' => location.0 += 1,
    'v' => location.1 -= 1,
    '<' => location.0 -= 1,
    _ => panic!("Invalid character"),
  }
  visited.insert(*location);
}
