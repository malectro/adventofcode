use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Element {
  name: String,
  derivations: Vec<String>,
  string: String,
}

fn main() {
  let string = std::env::args().nth(1).expect("No input given.");

  let growth_period = 50;

  /*
  let part_1_string = grow(&string, growth_period);
  println!("final number {}", part_1_string.len());
  */

  let elements: Vec<Element> = utils::read_file_lines("./data/10/elements")
    .map(|line| {
      let items: Vec<&str> = line.split('\t').collect();
      let (name, derivations, string) = (items[1], items[2], items[3]);
      Element {
        name: name.trim().to_string(),
        derivations: derivations
          .trim()
          .split(' ')
          .map(|d| d.trim().to_string())
          .collect(),
        string: string.trim().to_string(),
      }
    })
    .collect();

  let elements_by_name: HashMap<String, usize> = elements
    .iter()
    .enumerate()
    .map(|(i, element)| (element.name.clone(), i))
    .collect();

  let mut growth_matrix = [[0; 92]; 92];
  for i in 0..92 {
    let element = &elements[i];
    for d in &element.derivations {
      let d_id = elements_by_name.get(d).expect("invalid name");
      growth_matrix[i][*d_id] = 1;
    }
  }

  //println!("element 0 {:?}", elements[1]);

  /*
  let mut components = Vec::new();
  let mut candidates: Vec<usize> = (0..elements.len()).collect();
  let mut index = 0;
  let chars: Vec<char> = string.chars().collect();
  let mut i = 0;
  while i < chars.len() {
    let c = chars[i];
    let mut filtered = Vec::new();

    for element_index in &candidates {
      let element = &elements[*element_index];
      if let Some(element_c) = element.string.chars().nth(index) {
        if element_c == c {
          filtered.push(*element_index);
        }
      }
    }

    println!("filtered {} {:?}", c, filtered);

    if filtered.len() == 0 || i == chars.len() - 1 {
      components.push(candidates[0]);
      index = 0;
      candidates = (0..elements.len()).collect();
    } else {
      candidates = filtered;
      index += 1;
      i += 1;
    }
  }

  println!("components {:?}", components);
    */

  println!("87 {:?}", elements[86]);

  let mut counts = [0usize; 92];
  counts[86] = 1;

  for _ in 0..growth_period {
    let mut new_counts = [0; 92];
    for i in 0..counts.len() {
      let count = counts[i];
      if count > 0 {
        let element = &elements[i];
        for d in &element.derivations {
          let d_id = elements_by_name.get(d).expect("invalid name");
          new_counts[*d_id] += count;
        }
      }
    }
    counts = new_counts;
  }

  let sum = counts
    .into_iter()
    .enumerate()
    .fold(0, |acc, (i, v)| acc + elements[i].string.len() * v);
  println!("part 2 length {}", sum);
}

fn grow(string: &String, count: usize) -> String {
  let mut result = string.clone();

  for _ in 0..count {
    let mut new_string = String::new();

    let mut current_char = 'a';
    let mut current_char_count = 0;

    for c in result.chars() {
      if c != current_char {
        if current_char != 'a' {
          new_string = format!("{}{}", new_string, current_char_count);
          new_string.push(current_char);
        }
        current_char = c;
        current_char_count = 1;
      } else {
        current_char_count += 1;
      }
    }

    new_string = format!("{}{}", new_string, current_char_count);
    new_string.push(current_char);

    //println!("{} {}", new_string.len(), new_string);

    result = new_string;
  }

  result
}
