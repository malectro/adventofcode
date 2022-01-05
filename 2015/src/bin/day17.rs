use utils;

fn main() {
  let amount = 150;

  let containers: Vec<usize> = utils::read_input_file_lines()
    .map(|line| line.parse().expect("Invalid container size"))
    .collect();

  let mut valid_combos = Vec::new();
  let mut tries = vec![(Vec::new(), amount, 0)];

  while let Some((combo, amount, index)) = tries.pop() {
    if amount == 0 {
      valid_combos.push(combo);
      continue;
    } else if index == containers.len() {
      continue;
    }

    let size = containers[index];

    if size <= amount {
      let mut new_combo = combo.clone();
      new_combo.push(size);
      tries.push((new_combo, amount - size, index + 1));
    }
    tries.push((combo, amount, index + 1));
  }

  println!("permutation count: {}", valid_combos.len());

  let minimum_containers = valid_combos.iter().fold(usize::MAX, |acc, combo| {
    if combo.len() < acc {
      combo.len()
    } else {
      acc
    }
  });

  let total_minimum_combos = valid_combos.iter().fold(0, |acc, combo| {
    if combo.len() == minimum_containers {
      acc + 1
    } else {
      acc
    }
  });

  println!("minimum permutation count: {}", total_minimum_combos);
}
