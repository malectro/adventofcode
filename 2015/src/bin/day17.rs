use utils;

fn main() {
  let amount = 150;

  let containers: Vec<usize> = utils::read_input_file_lines()
    .map(|line| line.parse().expect("Invalid container size"))
    .collect();

  println!(
    "permutation count: {}",
    get_permutation_count(&containers, 0, amount)
  );
}

fn get_permutation_count(containers: &Vec<usize>, index: usize, amount: usize) -> usize {
  if amount == 0 {
    return 1;
  } else if index == containers.len() {
    return 0;
  }

  let mut count = 0;

  //println!("getting permutations for index {} and amount {}", index, amount);

  let size = containers[index];

  let possibles = if size > amount { 1 } else { 2 };
  for multiple in 0..possibles {
    count += get_permutation_count(containers, index + 1, amount - multiple * size);
  }

  count
}
